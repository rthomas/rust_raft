use raft_codec;
use raft_proto;

use futures::{future, Future, BoxFuture};
use std::collections::HashMap;
use std::io;
use tokio_proto::TcpServer;
use tokio_service::Service;

/// Each instance of a raft server should get a unique ID.
type ServerId = u8;

type CandidateId = ServerId;
type Term = u32;
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

/// Starts the RaftServer based on the configuration set in ::new()
/// This will block until the server terminates.
pub fn start(config: Configuration) -> Result<(), String> {
//    println!("Starting: {:?}", config);
//    let server = TcpServer::new(raft_proto::RaftProto, config.addr.parse().unwrap());
//    let raft = RaftServer::new(config.clone());
//    server.serve(move || {
//
//        println!("{:?}", raft);
//        Ok(raft)
//    });

    Ok(())
}

impl RaftServer {
    /// Initializes a RaftServer in the default Follower state
    pub fn new(config: Configuration) -> RaftServer {
        println!("NEW RAFT");
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
    
    fn start_election(&self) {
        // TODO: This is triggered from a timeout causing this server to start al election cycle.
        println!("Trigger Election");
    }

    fn append_entries(&self) {
        // TODO: This is triggered from an AppendEntries RPC to update the log.
        println!("AppendEntries");
    }

    fn request_vote(&self) {
        // TODO: This is triggered from a RequestVote RPC to have the server cast a vote.
        println!("RequestVote");
    }
}

pub struct RaftService {
    pub server: RaftServer,
}

impl Service for RaftService {
    type Request = String;
    type Response = String;

    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        println!("RECIEVED MESSAGE: {:?}", req);
        self.server.append_entries();
        future::ok(req).boxed()
    }
}
