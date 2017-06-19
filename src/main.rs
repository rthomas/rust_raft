extern crate raft;
extern crate tokio_core;
extern crate tokio_io;
extern crate capnp;
#[macro_use]
extern crate capnp_rpc;
extern crate futures;

use capnp::capability::Promise;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use futures::{Future, Stream};
use tokio_io::{AsyncRead};

use raft::{Configuration, RaftServer};

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

    let config = Configuration {addr: args[2].to_string()};
    println!("{:?}", config);
    
    let raft = raft::raft_capnp::bar::ToClient::new(RaftServer::new(config)).from_server::<::capnp_rpc::Server>();

    let done = socket.incoming().for_each(move |(socket, _addr)| {
        try!(socket.set_nodelay(true));
        let (reader, writer) = socket.split();

        let handle = handle.clone();

        let network =
            twoparty::VatNetwork::new(reader, writer,
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

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();

    let addr = args[2].to_socket_addrs().unwrap().next().expect("could not parse address");
    let stream = core.run(::tokio_core::net::TcpStream::connect(&addr, &handle)).unwrap();
    stream.set_nodelay(true);
    let (reader, writer) = stream.split();

    let network =
        Box::new(twoparty::VatNetwork::new(reader, writer,
                                           rpc_twoparty_capnp::Side::Client,
                                           Default::default()));
    let mut rpc_system = RpcSystem::new(network, None);

    let raft: raft::raft_capnp::bar::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

    handle.spawn(rpc_system.map_err(|_e| ()));
    
    {
        println!("Calling RPC");
        let mut req = raft.baz_request();
        req.get().set_x(11);

        core.run(req.send().promise.and_then(|response| {
            println!("VALUE: {}", pry!(response.get()).get_y());
            Promise::ok(())
        })).unwrap()

//        let ret = Box::new(req.send().promise.and_then(|response| {
//            println!("handler");
//            Ok(try!(response.get()).get_y())
//        })).wait().unwrap();
//        println!("Response: {:?}", ret);
    }
}
