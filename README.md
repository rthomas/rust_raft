# Rust Raft

A basic, work in progress implementation of the Raft consensus protocol.

For more details on Raft see [this paper](https://raft.github.io/raft.pdf).

This implementation is using the [capnp-rpc](https://github.com/dwrensha/capnp-rpc-rust) library for RPC on top of [tokio](tokio.rs). 
