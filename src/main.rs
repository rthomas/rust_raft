extern crate raft;

use raft::{Configuration, RaftServer};

pub fn main() {
    let config1 = Configuration {addr: "127.0.0.1:12300".to_string()};
    println!("{:?}", config1);
    let raft1 = RaftServer::new(config1);
    {
        raft::start(&raft1);
    }
}