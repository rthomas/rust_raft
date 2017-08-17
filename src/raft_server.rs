use std::cmp;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use capnp;
use capnp::capability::Promise;
use ::rpc;


/// Each instance of a raft server should get a unique ID.
pub type ServerId = u8;

pub type CandidateId = ServerId;
pub type Term = u64;
pub type LogIndex = u64;

#[derive(Clone,Debug)]
pub struct LogEntry {
    term: Term,
    key: String,
    value: String,
}

#[derive(Clone,Debug)]
enum ServerState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Clone,Debug)]
pub struct Configuration {
    /// Host / IP and port to bind this server on.
    pub addr: String,
    pub follower_timeout: time::Duration,
}

#[derive(Clone,Debug)]
pub struct RaftServer {
    /// The state of this raft instance, initialized as ServerState::Follower
    state: ServerState,

    /// The `Instant` that the last heartbeat was recieved at.
    last_heartbeat: time::Instant,
    
    /// Configuration for the raft cluster
    config: Configuration,
    
    /// Latest term the server has seen, initialized to 0 and increases monotonically.
    current_term: Term,

    /// Candidate that recieved the vote in the current term (or None if not voted).
    voted_for: Option<CandidateId>,

    log: Vec<LogEntry>,

    /// Index of the highest log entry that is known to be committed.
    commit_index: LogIndex,

    /// Index of the highest log entry applied to the state machine.
    last_applied: LogIndex,

    /// Leader specific entries that are reinitialized on election:
    
    /// For each server, the index of the next LogEntry to send that server.
    next_index: Option<HashMap<ServerId, LogIndex>>,

    /// For each server, the index of the highest known replicated log entry.
    match_index: Option<HashMap<ServerId, LogIndex>>,
}

/// This exists as a thread-safe wrapper to the RaftServer instance.
#[derive(Debug)]
pub struct Raft {
    server: Arc<Mutex<RaftServer>>,
}

impl Raft {
    pub fn new_server(config: Configuration) -> Raft {
        let timeout = config.follower_timeout;
        let raft_server = Arc::new(Mutex::new(RaftServer::new(config)));
        let server = raft_server.clone();
        
        let mut raft = Raft {
            server: raft_server,
        };
        
        thread::spawn(move || {
            loop {
                {
                    // We need to make sure the lock scope closes before the sleep
                    // otherwise we never release the lock.
                    let s = match server.lock() {
                        Ok(s) => s,
                        Err(poisoned) => panic!("Poisoned lock"),
                    };
                    match s.state {
                        ServerState::Follower => {
                            println!("Follower...");
                            let now = time::Instant::now();
                            if now.duration_since(s.last_heartbeat) > timeout {
                                println!("Follower timed out - triggering election!");
                                // Start election
                            }
                            else {
                                println!("Still following...");
                            }
                        }
                        ref state => {
                            // TODO - impl other states
                            println!("Other state... {:?}", state);
                        }
                    }
                }
                thread::sleep(timeout);
            }
        });
        
        return raft;
    }
}

impl RaftServer {
    /// Initializes a RaftServer in the default Follower state
    pub fn new(config: Configuration) -> RaftServer {
        println!("RaftServer::new({:?})", config);
        RaftServer {
            state: ServerState::Follower,
            last_heartbeat: time::Instant::now(),
            config: config,
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
            // Leader specifics init'd to None due to new creation.
            next_index: None,
            match_index: None,
        }
    }

    fn start_election(&mut self) {
        // TODO: This is triggered from a timeout causing this server to start al election cycle.
        println!("Trigger Election");
    }

    fn append_entries(&mut self,
                      term: Term,
                      leader_id: ServerId,
                      prev_log_index: LogIndex,
                      prev_log_term: Term,
                      entries: &mut Vec<LogEntry>,
                      leader_commit: LogIndex) -> (Term, bool) {
        println!("AppendEntries{{{}, {}, {}, {}, {:?}, {}}})", term, leader_id, prev_log_index, prev_log_term, entries, leader_commit);        
        // Naive impl - any append entries counts as a heartbeat.
        self.last_heartbeat = time::Instant::now();
        
        if term < self.current_term {
            return (self.current_term, false)
        }

        if prev_log_index < (self.log.len() as u64) && self.log[prev_log_index as usize].term != prev_log_term {
            return (self.current_term, false)
        }
        else if prev_log_index != 0 && prev_log_index >= (self.log.len() as u64) {
            println!("Recieved prevLogIndex >= actual log length. {} >= {}", prev_log_index, self.log.len() as u64);
            return (self.current_term, false)
        }

        let new_index = match prev_log_index {
            0 => 0,
            i => i + 1,
        };

        // new_index should be the next item in the log
        // This means it should be equal to the length - otherwise we need to discard all further entries.
        if (self.log.len() as u64) == new_index {
            self.log.append(entries);
        }
        else if (self.log.len() as u64) > new_index {
            // Discard all items from new_index after
            self.log.split_off(new_index as usize);
            self.log.append(entries);
        }
        else {
            panic!("Unknown state. new_index: {}, log length: {}", new_index, self.log.len());
        }

        if leader_commit > self.commit_index {
            let last_index = match self.log.len() as u64 {
                0 => 0,
                i => i - 1,
            };
            self.commit_index = cmp::min(leader_commit, last_index);
        }
        
        (self.current_term, true)
    }

    fn request_vote(&mut self, term: Term, candidate_id: CandidateId, last_log_index: LogIndex, last_log_term: Term) -> (Term, bool) {
        // Do not vote as the term is less than ours.
        if term < self.current_term {
            return (self.current_term, false);
        }

        // Do not vote as we have already voted for someone else.
        if self.voted_for.is_some() && self.voted_for.unwrap() != candidate_id {
            return (self.current_term, false);
        }

        if last_log_index < self.log.len() as u64 && self.log[last_log_index as usize].term == last_log_term {
            // The candidate is at least as up to date as we are, so we can vote for them
            self.voted_for = Some(candidate_id);
            return (self.current_term, true);
        }

        (self.current_term, false)
    }
}

pub fn start(server: &mut RaftServer) -> Result<(), String> {
    println!("Starting: {:?}", server);
    Ok(())
}

fn to_log_entry(e: ::log_entry::Reader) -> LogEntry {
    LogEntry{term: e.get_term(),
             key: match e.get_key() {
                 Ok(s) => s.to_string(),
                 Err(_) => "".to_string(),
             },
             value: match e.get_value() {
                 Ok(s) => s.to_string(),
                 Err(_) => "".to_string(),
             }
    }
}

impl rpc::Server for Raft {
    fn append_entries(&mut self,
                     params: rpc::AppendEntriesParams,
                     mut results: rpc::AppendEntriesResults) -> Promise<(), capnp::Error> {
        let append_entries = pry!(params.get());

        let term = append_entries.get_term();
        let leader_id = append_entries.get_leader_id();
        let prev_log_index = append_entries.get_prev_log_index();
        let prev_log_term = append_entries.get_prev_log_term();
        let leader_commit = append_entries.get_leader_commit();
        let mut entries = {
            let e = match append_entries.get_entries() {
                Ok(r) => r,
                Err(e) => panic!(e),
            };
            let size = e.len();
            let mut entries = Vec::with_capacity(size as usize);
            for i in 0..size {
                entries.push(to_log_entry(e.get(i)));
            }
            entries
        };

        let (term, success) = self.server.lock().unwrap().append_entries(term, leader_id, prev_log_index, prev_log_term, &mut entries, leader_commit);

        println!("DONE... {:?}", self);
        
        results.get().set_term(term);
        results.get().set_success(success);
        
        Promise::ok(())
    }
}
