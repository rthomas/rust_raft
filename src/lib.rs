extern crate capnp;
#[macro_use]
extern crate capnp_rpc;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

pub mod server;
pub mod client;

pub mod raft_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/raft_capnp.rs"));
}

pub use raft_capnp as rpc;
