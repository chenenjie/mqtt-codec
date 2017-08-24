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
        PacketIdentifierError(::control::variable_header::PacketIdentifierError, ::control::variable_header::PacketIdentifierErrorKind);
    }
    
}

#[derive(Debug)]
struct SubscribeFixedHeader{
    packet_type: u8,
    remaining_length: u32,
}

impl SubscribeFixedHeader{
    fn new() -> SubscribeFixedHeader {
        SubscribeFixedHeader{
            packet_type: 8,
            remaining_length: 0
        }
    }
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

#[derive(Debug)]
pub struct Subscribe {
    fixed_header: SubscribeFixedHeader,
    packet_identifier: PacketIdentifier,
    payload: SubscribePayload,
}

impl Subscribe {
    fn new() -> Subscribe{
        let mut subscribe = Subscribe{
            fixed_header: SubscribeFixedHeader::new(),
            packet_identifier: PacketIdentifier(82u16),
            payload: {
                let mut vec = Vec::new();
                for i in 0..5 {
                    let topic_name = TopicName("enjie".into());
                    let qos = QualityOfService::Level0;
                    vec.push((topic_name, qos));
                }
                SubscribePayload{
                    subscribes: vec,
                }
            }
        };
        subscribe.calculate_remaining_length();
        subscribe
    }

    fn calculate_remaining_length(&mut self) -> Result<(), SubscribeError>{
        self.fixed_header.remaining_length = self.packet_identifier.encode_length()? + self.payload.encode_length()?;
        Ok(())
    }
}

impl<'a> Decodable<'a> for Subscribe {
    type Error = SubscribeError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let fixed_header = Decodable::decode(byte)?; 
        let packet_identifier = Decodable::decode(byte)?;
        let payload = Decodable::decode(byte)?;

        Ok(Subscribe{
            fixed_header: fixed_header,
            packet_identifier: packet_identifier,
            payload: payload,
        })
    }
}

impl Encodable for Subscribe {
    type Error = SubscribeError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        let mut vec = Vec::new();
        let fixed_header = self.fixed_header.encode()?;
        let packet_identifier = self.packet_identifier.encode()?;
        let payload = self.payload.encode()?;

        vec.extend(fixed_header);
        vec.extend(packet_identifier);
        vec.extend(payload);

        Ok(vec)
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        let length = self.fixed_header.encode_length()?
                    + self.packet_identifier.encode_length()?
                    + self.payload.encode_length()?;
        Ok(length)
    }
}

#[derive(Debug)]
struct SubscribePayload {
    subscribes: Vec<(TopicName, QualityOfService)>,
}

impl<'a> Decodable<'a> for SubscribePayload {
    type Error = SubscribeError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let mut vec = Vec::new();
        while byte.len() > 0{
           let topic_name = Decodable::decode(byte)?;
           let qos = Decodable::decode(byte)?;
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
        for &(ref topic_name, ref qos) in &self.subscribes {
            vec.extend(topic_name.encode()?);
            vec.extend(qos.encode()?);
        }
        Ok(vec)

    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        let (left, right): (Vec<Result<u32, Self::Error>>, Vec<Result<u32, Self::Error>>) = self.subscribes.iter().map(|&(ref topic_name, ref qos)| {
            (topic_name.encode_length().map_err(From::from), qos.encode_length().map_err(From::from))
        }).unzip();

        let mut iter = left.into_iter().chain(right.into_iter());

        let mut sum = iter.fold(Ok(0), |acc, b| {
            let sum = match acc {
                Err(err) => return Err(err),
                Ok(num) => {
                    match b {
                        Err(err) => return Err(err),
                        Ok(len) => {
                            len + num
                        }
                    } 
                },
            };
            Ok(sum)
        });
        sum
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use bytes::BytesMut;

    //#[test]
    fn test_subscribe_payload(){
        let mut vec = vec![];
        for i in 0..5 {
            let topic_name = TopicName("enjie".into());
            let qos = QualityOfService::Level0;
            vec.push((topic_name, qos));
        }
        let payload = SubscribePayload{
            subscribes: vec  
        };

        //println!("{:?}", payload.encode_length());  
    }

    #[test]
    fn test_subscribe_encode_decode(){
        let subscribe = Subscribe::new();
        let vecbytes = subscribe.encode();
        //println!("{:?}", vecbytes);
        
        let mut bytes = BytesMut::from(vecbytes.unwrap());
        let result = Subscribe::decode(&mut bytes);
        //println!("{:?}", result) ;
        
    }
}
