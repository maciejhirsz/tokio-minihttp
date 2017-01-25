extern crate futures;
extern crate httparse;
extern crate net2;
extern crate time;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate smallvec;
extern crate inlinable_string;

mod date;
mod request;
mod response;
mod headers;

use std::io;

pub use request::Request;
pub use response::Response;
pub use headers::Headers;

use tokio_proto::pipeline::ServerProto;
use tokio_core::io::{Io, Codec, Framed, EasyBuf};

pub struct Http;

impl<T: Io + 'static> ServerProto<T> for Http {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, HttpCodec>;
    type BindTransport = io::Result<Framed<T, HttpCodec>>;

    fn bind_transport(&self, io: T) -> io::Result<Framed<T, HttpCodec>> {
        Ok(io.framed(HttpCodec))
    }
}

pub struct HttpCodec;

impl Codec for HttpCodec {
    type In = Request;
    type Out = Response;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Request>> {
        request::decode(buf)
    }

    fn encode(&mut self, msg: Response, buf: &mut Vec<u8>) -> io::Result<()> {
        response::encode(msg, buf);
        Ok(())
    }
}
