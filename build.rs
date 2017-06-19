extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new().file("schema/raft.capnp").run().unwrap()
}
