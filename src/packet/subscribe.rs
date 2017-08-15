use {Encodable, Decodable};
use packet::FixedHeader;
use control::variable_header::{PacketIdentifier, PacketIdentifierError, PacketIdentifierErrorKind};
use bytes::BytesMut;

use topic_name::TopicName;
use qos::QualityOfService;

error_chain!{
    types {
        SubscribeError, ErrorKind, ResultExt, SubscribeResult;
    }

    

    links {
        FixedHeaderError(::packet::FixedHeaderError, ::packet::ErrorKind);
        TopicNameError(::topic_name::TopicNameError, ::topic_name::ErrorKind);
        QualityOfServiceError(::qos::QualityOfServiceError, ::qos::ErrorKind);
    }
    
}

struct SubscribeFixedHeader{
    packet_type: u8,
    remaining_length: u32,
}

impl FixedHeader for SubscribeFixedHeader {
    
    fn set_remaining_length(&mut self, len: u32){
        self.remaining_length = len;
    }
}

impl<'a> Decodable<'a> for SubscribeFixedHeader {
    type Error = SubscribeError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        match Self::get_fixheader(byte) {
            Ok((packet_type, _, remaining_length, n)) => {
                byte.split_to(1 + n);
                Ok(SubscribeFixedHeader{
                    packet_type: packet_type,
                    remaining_length: remaining_length,
                })
            },
            Err(err) => {
                Err(err.into())
            }
        }
    }
}

impl Encodable for SubscribeFixedHeader {
    type Error = SubscribeError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        Self::encode_fixedheader(self.packet_type, 0u8, self.remaining_length).map_err(From::from)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        Self::get_remaining_length_bytes(self.remaining_length).map_err(From::from)
    }
}

struct Subscribe {
    fixed_header: SubscribeFixedHeader,
    packet_identifier: PacketIdentifier,
    payload: SubscribePayload,
}

//impl<'a> Decodable<'a> for Subscribe {
    //type Error = SubscribeError;
    //type Cond = ();

    //fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>;
//}

//impl Encodable for Subscribe {
    //type Error;
    //type Cond;

    //fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>;

    //fn encode_length(&self) -> Result<u32, Self::Error>;
//}

struct SubscribePayload {
    subscribes: Vec<(TopicName, QualityOfService)>,
}

impl<'a> Decodable<'a> for SubscribePayload {
    type Error = SubscribeError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let vec = Vec::new();
        while byte.len() > 0{
           let topic_name = Decodable::decode(byte).map_err(From::from)?;
           let qos = Decodable::decode(byte).map_err(From::from)?;
           vec.push((topic_name, qos));
        } 
        Ok(SubscribePayload{
            subscribes: vec,
        })
    }
}

impl Encodable for SubscribePayload {
    type Error = SubscribeError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        let mut vec = Vec::new();
        for (topic_name, qos) in self.subscribes {
            vec.extend(topic_name.encode()?);
            vec.extend(qos.encode()?);
        }
        Ok(vec)

    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        let (left, right) = self.subscribes.iter().unzip();
    }
}
