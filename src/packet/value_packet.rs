mod connect;
mod connack;
mod publish;
mod puback;
mod pubrec;
mod pubrel;
mod pubcomp;
mod subscribe;
mod suback;
mod unsubscribe;
mod unsuback;
mod pingreq;
mod pingresp;
mod disconnect;

use connect::Connect;
use connack::Connack;
use publish::Publish;
use puback::PubAck;
use pubrec::PubRec;
use pubrel::PubRel;
use pubcomp::PubComp;
use subscribe::Subscribe;
use suback::SubAck;
use unsubscribe::Unsubscribe;
use unsuback::UnSubAck;
use pingreq::PingReq;
use pingresp::PingResp;
use disconnect::Disconnect;
use {Decodable, Encodable};
use packet::FixedHeader;

enum ValuePacket{
    ConnectPacket(Connect),
    ConnackPacket(Connack),
    PublishPacket(Publish),
    PubAckPacket(PubAck),
    PubRecPacket(PubRec),
    PubRelPacket(PubRel),
    PubCompPacket(PubComp),
    SubscribePacket(Subscribe),
    SubAckPacket(SubAck),
    UnsubscribePacket(Unsubscribe),
    UnSubAckPacket(UnSubAck),
    PingReqPacket(PingReq),
    PingRespPacket(PingResp),
    DisconnecPacket(Disconnect),
}

error_chain!{
    types{
        ValuePacketError, ErrorKind, ResultExt, ValuePacketResult;
    }
}

impl FixedHeader for ValuePacket{}


impl<'a> Decodable<'a> for ValuePacket {
    type Error = ValuePacketError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        let result = match Self::get_fixedheader(byte) {
            Ok(packet_type, _, _, _) => {
                match packet_type {
                    1 => ValuePacket::ConnectPacket(Decodable::decode(byte)?),
                    2 => ValuePacket::ConnackPacket(Decodable::decode(byte)?),
                    3 => ValuePacket::PublishPacket(Decodable::decode(byte)?),
                    4 => ValuePacket::PubAckPacket(Decodable::decode(byte)?),
                    5 => ValuePacket::PubRecPacket(Decodable::decode(byte)?),
                    6 => ValuePacket::PubRelPacket(Decodable::decode(byte)?),
                    7 => ValuePacket::PubCompPacket(Decodable::decode(byte)?),
                    8 => ValuePacket::SubscribePacket(Decodable::decode(byte)?),
                    9 => ValuePacket::SubAckPacket(Decodable::decode(byte)?),
                    10 => ValuePacket::UnsubscribePacket(Decodable::decode(byte)?),
                    11 => ValuePacket::UnSubAckPacket(Decodable::decode(byte)?),
                    12 => ValuePacket::PingReqPacket(Decodable::decode(byte)?),
                    13 => ValuePacket::PingRespPacket(Decodable::decode(byte)?),
                    14 => ValuePacket::DisconnecPacket(Decodable::decode(byte)?),
                    _ => bail!("error packet type , no specify packet to decode"),
                }
            },
            Err(err) => err.into(),
        };
        Ok(result)
    }
}

impl Encodable for ValuePacket {
    type Error = ValuePacketError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        match self {
            ValuePacket::ConnectPacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::ConnackPacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::PublishPacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::PubAckPacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::PubRecPacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::PubRelPacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::PubCompPacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::SubscribePacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::SubAckPacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::UnsubscribePacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::UnSubAckPacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::PingReqPacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::PingRespPacket(packet) => packet.encode().map_err(From::from),
            ValuePacket::DisconnecPacket(packet) => packet.encode().map_err(From::from),
            _ => bail!("not found value packet error"),
        }
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        match self {
            ValuePacket::ConnectPacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::ConnackPacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::PublishPacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::PubAckPacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::PubRecPacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::PubRelPacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::PubCompPacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::SubscribePacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::SubAckPacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::UnsubscribePacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::UnSubAckPacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::PingReqPacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::PingRespPacket(packet) => packet.encode_length().map_err(From::from),
            ValuePacket::DisconnecPacket(packet) => packet.encode_length().map_err(From::from),
            _ => bail!("not found value packet error"),
        }
    }
}
