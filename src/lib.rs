extern crate capnp;
#[macro_use]
extern crate capnp_rpc;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

pub mod raft_server;
pub mod raft_client;

pub use raft_server as server;
pub use raft_client as client;

mod raft_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/raft_capnp.rs"));
}

pub use raft_capnp::*;
