extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new()
        .file("schema/append_entries.capnp")
        .run().expect("Failed to compile capnp");
}