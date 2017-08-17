extern crate raft;
extern crate tokio_core;
extern crate tokio_io;
extern crate capnp;
#[macro_use]
extern crate capnp_rpc;
extern crate futures;

use std::time;

use capnp::capability::Promise;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use futures::{Future, Stream};
use tokio_io::{AsyncRead};

use raft::server::{Configuration, Raft};
use raft::client::RaftClient;

pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "client" => return client(),
            "server" => return server(),
            _ => ()
        }
    }

    println!("usage: {} [client | server] ADDRESS", args[0]);
}

fn server() {
    use std::net::ToSocketAddrs;

    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} server ADDRESS[:PORT]", args[0]);
        return;
    }

    let mut core = ::tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();

    let addr = args[2].to_socket_addrs().unwrap().next().expect("could not parse address");
    let socket = ::tokio_core::net::TcpListener::bind(&addr, &handle).unwrap();

    let config = Configuration {addr: args[2].to_string(), follower_timeout: time::Duration::from_millis(250)};
    println!("{:?}", config);

    let raft = raft::rpc::ToClient::new(Raft::new_server(config)).from_server::<::capnp_rpc::Server>();

    let done = socket.incoming().for_each(move |(socket, _addr)| {
        try!(socket.set_nodelay(true));
        let (reader, writer) = socket.split();

        let handle = handle.clone();

        let network = twoparty::VatNetwork::new(reader, writer,
                                                rpc_twoparty_capnp::Side::Server, Default::default());

        let rpc_system = RpcSystem::new(Box::new(network), Some(raft.clone().client));
        handle.spawn(rpc_system.map_err(|e| println!("error: {:?}", e)));
        Ok(())
    });

    core.run(done).unwrap();
}

fn client() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} client HOST:PORT", args[0]);
        return;
    }

    use std::net::ToSocketAddrs;

    let addr = args[2].to_socket_addrs().unwrap().next().expect("could not parse address");


    let mut client = RaftClient::new(addr).unwrap();
    let entries: Vec<raft::server::LogEntry> = Vec::new();
    let (term, success) = client.append_entries(12345u64, 1u8, 0u64, 1, &entries, 999u64).unwrap();

    println!("Term: {:?}, Success: {}", term, success);
}
