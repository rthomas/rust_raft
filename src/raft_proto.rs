use std::io;

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;

use raft_codec::RaftCodec;

/// Basic impl of a raft proto based on the tokio LineProto
pub struct RaftProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for RaftProto {
    type Request = String;
    type Response = String;
    type Transport = Framed<T, RaftCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(RaftCodec))
    }
}
