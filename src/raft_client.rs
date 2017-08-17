use std::net::SocketAddr;

use raft_server as server;

pub struct RaftClient {
    last_known_leader: Option<SocketAddr>,
    all_nodes: Vec<SocketAddr>,
}

impl RaftClient {
    /// This will construct a new `RaftClient` by connecting to the given
    /// `SocketAddr` and populating a list of all nodes, by making an
    /// initial call to that node.
    pub fn new(initial_node: SocketAddr) -> RaftClient {
        let mut client = RaftClient {
            last_known_leader: Some(initial_node),
            all_nodes: Vec::new(),
        };

        // TODO - make a call to the last_known_leader to determine current leader and server list.
        Self::update_nodes(&mut client);
        client
    }

    /// Calls append_entries on the leader. This must take an `&mut self` so that the
    /// node_list in the client can be udpated if it has changed.
    pub fn append_entries(&mut self,
                          term: server::Term,
                          leader_id: server::ServerId,
                          prev_log_index: server::LogIndex,
                          prev_log_term: server::Term,
                          entries: &Vec<server::LogEntry>,
                          leader_commit: server::LogIndex) -> Result<(server::Term, bool), String> {
        Ok((1, false))
    }
    
    /// This method will attempt to make a call to the last_known_leader to determine the current leader
    /// and node list. If this call fails or times out, then a call will be made to each of the items in the
    /// all_nodes vector. If this is empty, or they also all time out, then the method will fail.
    ///
    /// This means that in the initial state, a valid node must be passed in so that the client can discover the leader.
    fn update_nodes(&mut self) -> Result<(), String> {
        Ok(())
    }
}
