extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate mqtt_codec;

use tokio_io::codec::{Encoder, Decoder};
use std::io;
use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use tokio_proto::TcpServer;
use bytes::BytesMut;

use mqtt_codec::packet::{Connect, Connack, Disconnect, PingReq, PingResp, PubAck, PubComp, Publish, PubRec, PubRel, SubAck, Subscribe, Unsubscribe, UnSubAck, ValuePacket};
use mqtt_codec::{Encodable, Decodable};



pub struct PackectCodec;

impl Decoder for PackectCodec {
    type Item = ValuePacket;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error>{
        if let Ok(n) = ValuePacket::get_packet_length(src) {
            if src.len() < (n as usize) {
                Ok(None)
            }else {
                let mut packet_byte = src.split_to(n as usize);
                match ValuePacket::decode(&mut packet_byte) {
                    Ok(item) => Ok(Some(item)),
                    Err(err) => Err(io::Error::new(io::ErrorKind::Other, "decode available")),
                }
            }
        }else {
            Err(io::Error::new(io::ErrorKind::Other, "decode available"))
        }
    }
}

impl Encoder for PackectCodec {
    type Item = ValuePacket;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        if let Ok(code) = item.encode() {
            dst.extend(code);
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "encode available"))
        }
    }
}


pub struct PackectProto;
    
impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for PackectProto{

    type Request = ValuePacket;

    type Response = ValuePacket;

    type Transport = Framed<T, PackectCodec>;

    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(PackectCodec))
    }
}


pub struct Mqtt;

impl Service for Mqtt {
    type Request = ValuePacket;
    type Response = ValuePacket;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}

fn main(){
    let addr = "0.0.0.0:12345".parse().unwrap();

    let server = TcpServer::new(PackectProto, addr);

    server.serve(|| Ok(Mqtt))
}
    


