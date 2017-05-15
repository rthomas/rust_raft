extern crate raft;
extern crate tokio_proto;

use tokio_proto::TcpServer;

use raft::{Configuration, RaftServer, RaftService};
use raft::raft_proto;

pub fn main() {
    let config1 = Configuration {addr: "127.0.0.1:12300".to_string()};
    println!("{:?}", config1);

    let raft1 = RaftServer::new(config1.clone());

    let server = TcpServer::new(raft_proto::RaftProto, config1.addr.parse().unwrap());
    server.serve(move || Ok(RaftService{server: raft1.clone()}))

}