use packet::connect::Connect;
use packet::connack::Connack;
use packet::publish::Publish;
use packet::puback::PubAck;
use packet::pubrec::PubRec;
use packet::pubrel::PubRel;
use packet::pubcomp::PubComp;
use packet::subscribe::Subscribe;
use packet::suback::SubAck;
use packet::unsubscribe::Unsubscribe;
use packet::unsuback::UnSubAck;
use packet::pingreq::PingReq;
use packet::pingresp::PingResp;
use packet::disconnect::Disconnect;
use {Decodable, Encodable};
use packet::FixedHeader;
use bytes::BytesMut;

#[derive(Debug)]
pub enum ValuePacket{
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

    links{
        FixedHeaderError(::packet::FixedHeaderError, ::packet::ErrorKind);
        ConnackError(::packet::ConnackError, ::packet::ConnackErrorKind);
        ConnectError(::packet::ConnectError, ::packet::ConnectErrorKind);
        DisconnectError(::packet::DisconnectError, ::packet::DisconnectErrorKind);
        PingReqError(::packet::PingReqError, ::packet::PingReqErrorKind);
        PingRespError(::packet::PingRespError, ::packet::PingRespErrorKind);
        PubAckError(::packet::PubAckError, ::packet::PubAckErrorKind);
        PubCompError(::packet::PubCompError, ::packet::PubCompErrorKind);
        PublishError(::packet::PublishError, ::packet::PublishErrorKind);
        PubRecError(::packet::PubRecError, ::packet::PubRecErrorKind);
        PubRelError(::packet::PubRelError, ::packet::PubRelErrorKind);
        SubAckError(::packet::SubAckError, ::packet::SubAckErrorKind);
        SubscribeError(::packet::SubscribeError, ::packet::SubscribeErrorKind);
        UnSubAckError(::packet::UnSubAckError, ::packet::UnSubAckErrorKind);
        UnsubscribeError(::packet::UnsubscribeError, ::packet::UnsubscribeErrorKind);
    }
}

impl FixedHeader for ValuePacket{
    fn set_remaining_length(&mut self, len: u32){
        unreachable!();
    }
}

impl ValuePacket {
    pub fn get_packet_length(bytes: &mut BytesMut) -> Result<u32, ValuePacketError> {
        if let Ok((_, _, n, _,)) = Self::get_fixheader(bytes) {
            Ok(n + 2)
        }else{
            bail!("get packet length error");
        }
    }
}


impl<'a> Decodable<'a> for ValuePacket {
    type Error = ValuePacketError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        let result = match Self::get_fixheader(byte) {
            Ok((packet_type, _, _, _)) => {
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
            Err(err) => return Err(err.into()),
        };
        Ok(result)
    }
}

impl Encodable for ValuePacket {
    type Error = ValuePacketError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        match self {
            &ValuePacket::ConnectPacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::ConnackPacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::PublishPacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::PubAckPacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::PubRecPacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::PubRelPacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::PubCompPacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::SubscribePacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::SubAckPacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::UnsubscribePacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::UnSubAckPacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::PingReqPacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::PingRespPacket(ref packet) => packet.encode().map_err(From::from),
            &ValuePacket::DisconnecPacket(ref packet) => packet.encode().map_err(From::from),
            _ => bail!("not found value packet error"),
        }
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        match self {
            &ValuePacket::ConnectPacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::ConnackPacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::PublishPacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::PubAckPacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::PubRecPacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::PubRelPacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::PubCompPacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::SubscribePacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::SubAckPacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::UnsubscribePacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::UnSubAckPacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::PingReqPacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::PingRespPacket(ref packet) => packet.encode_length().map_err(From::from),
            &ValuePacket::DisconnecPacket(ref packet) => packet.encode_length().map_err(From::from),
            _ => bail!("not found value packet error"),
        }
    }
}
