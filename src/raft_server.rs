use std::collections::HashMap;
use std::io;
use std::sync::Arc;

/// Each instance of a raft server should get a unique ID.
type ServerId = u8;

type CandidateId = ServerId;
type Term = u64;
type LogIndex = u64;

#[derive(Clone,Debug)]
struct LogEntry {
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
}

#[derive(Clone,Debug)]
pub struct RaftServer {
    /// The state of this raft instance, initialized as ServerState::Follower
    state: ServerState,

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

impl RaftServer {
    /// Initializes a RaftServer in the default Follower state
    pub fn new(config: Configuration) -> RaftServer {
        println!("RaftServer::new({:?})", config);
        RaftServer {
            state: ServerState::Follower,
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
                      leaderId: ServerId,
                      prevLogIndex: LogIndex,
                      entries: Vec<LogEntry>,
                      leaderCommit: LogIndex) -> (Term, bool) {
        println!("AppendEntries{{{}, {}, {}, {:?}, {}}})", term, leaderId, prevLogIndex, entries, leaderCommit);
        (12345678u64, false)
    }

    fn request_vote(&mut self) {
        // TODO: This is triggered from a RequestVote RPC to have the server cast a vote.
        println!("RequestVote");
    }
}

pub fn start(server: &mut RaftServer) -> Result<(), String> {
    println!("Starting: {:?}", server);
    Ok(())
}


use capnp::capability::Promise;

impl ::rpc::Server for RaftServer {
    fn append_entries(&mut self,
                     params: ::rpc::AppendEntriesParams,
                     mut results: ::rpc::AppendEntriesResults) -> Promise<(), ::capnp::Error> {
        let append_entries = pry!(params.get());

        let term = append_entries.get_term();
        let leader_id = append_entries.get_leader_id();
        let prev_log_index = append_entries.get_prev_log_index();
        let leader_commit = append_entries.get_leader_commit();
        let entries = {
            let e = match append_entries.get_entries() {
                Ok(r) => r,
                Err(e) => panic!(e),
            };
            let size = e.len();
            let mut entries = Vec::with_capacity(size as usize);
            fn to_log_entry(e: ::log_entry::Reader) -> LogEntry {
                LogEntry{term: e.get_term(),
                         key: e.get_key().expect("").to_string(),
                         value: e.get_value().expect("").to_string()}
            }
            for i in 0..size {
                entries.push(to_log_entry(e.get(i)));
            }
            entries
        };

        let (term, success) = self.append_entries(term, leader_id, prev_log_index, entries, leader_commit);
        
        results.get().set_term(term);
        results.get().set_success(success);
        
        Promise::ok(())
    }
}