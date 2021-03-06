use std::io::Error;
use std::net::SocketAddr;

use capnp::capability::Promise;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tokio_io::AsyncRead;

use server;
use rpc;

pub struct RaftClient {
    last_known_leader: Option<SocketAddr>,
    all_nodes: Vec<SocketAddr>,

    rpc_core: Core,
}

impl RaftClient {
    /// This will construct a new `RaftClient` by connecting to the given
    /// `SocketAddr` and populating a list of all nodes, by making an
    /// initial call to that node.
    pub fn new(initial_node: SocketAddr) -> Result<RaftClient, Error> {
        let mut client = RaftClient {
            last_known_leader: Some(initial_node),
            all_nodes: Vec::new(),
            rpc_core: Core::new()?,
        };

        // TODO - make a call to the last_known_leader to determine current leader and server list.
        Self::update_nodes(&mut client)?;
        Ok(client)
    }

    /// Calls append_entries on the leader. This must take an `&mut self` so that the
    /// node_list in the client can be udpated if it has changed.
    pub fn append_entries(&mut self,
                          term: server::Term,
                          leader_id: server::ServerId,
                          prev_log_index: server::LogIndex,
                          prev_log_term: server::Term,
                          entries: &Vec<server::LogEntry>,
                          leader_commit: server::LogIndex) -> Result<(server::Term, bool), Error> {
        let client = self.open_connection()?;
        {
            let mut req = client.append_entries_request();
            req.get().set_term(term);
            req.get().set_leader_id(leader_id);
            req.get().set_prev_log_index(prev_log_index);
            req.get().set_prev_log_term(prev_log_term);
            req.get().set_leader_commit(leader_commit);

            let mut t: server::Term = 0;
            let mut b: bool = false;
            self.rpc_core.run(req.send().promise.and_then(|response| {
                let resp = pry!(response.get());
                t = resp.get_term();
                b = resp.get_success();
                Promise::ok(())
            })).unwrap();
            return Ok((t, b));
        }
    }

    fn open_connection(&mut self) -> Result<rpc::raft::Client, Error> {
        self.update_nodes()?;
        let handle = self.rpc_core.handle();
        let stream = self.rpc_core.run(TcpStream::connect(&self.last_known_leader.unwrap(), &handle))?;
        stream.set_nodelay(true);
        let (reader, writer) = stream.split();
        let network = Box::new(twoparty::VatNetwork::new(reader, writer,
                                                         rpc_twoparty_capnp::Side::Client,
                                                         Default::default()));
        let mut rpc_system = RpcSystem::new(network, None);
        let client: rpc::raft::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        handle.spawn(rpc_system.map_err(|e| {
            panic!("RPC System Error: {:?}", e);
        }));

        Ok(client)
    }
    
    /// This method will attempt to make a call to the last_known_leader to determine the current leader
    /// and node list. If this call fails or times out, then a call will be made to each of the items in the
    /// all_nodes vector. If this is empty, or they also all time out, then the method will fail.
    ///
    /// This means that in the initial state, a valid node must be passed in so that the client can discover the leader.
    fn update_nodes(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
