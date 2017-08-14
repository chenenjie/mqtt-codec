use bytes::BytesMut;
use {Decodable, Encodable};
use packet::FixedHeader;
use control::variable_header::{PacketIdentifier, TopicName};



error_chain!{
    types {
        PublishError, ErrorKind, ResultExt, PublishResult;
    }

    errors {
        PublishPayloadError(r: String)
    }

    links{
        FixedHeader(::packet::FixedHeaderError, ::packet::ErrorKind);
        TopicName(::control::variable_header::TopicNameError, ::control::variable_header::TopicNameErrorKind);
        PacketIdentifier(::control::variable_header::PacketIdentifierError, ::control::variable_header::PacketIdentifierErrorKind);
    }
}

#[derive(Debug)]
struct PublishFixedHeader{
    packet_type: u8,
    dup_flag: bool,
    qos_level: u8,
    retain: bool,
    remaining_length: u32,
}

impl PublishFixedHeader {
    fn new() -> PublishFixedHeader {
        PublishFixedHeader{
            packet_type: 3,
            dup_flag: false,
            qos_level: 0,
            retain: false,
            remaining_length: 0,
        }
    }
     
}


impl FixedHeader for PublishFixedHeader {
    fn set_remaining_length(&mut self, len: u32) {
        self.remaining_length = len;
    }
}

impl<'a> Decodable<'a> for PublishFixedHeader {
    type Error = PublishError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _: Option<Self::Cond>) -> Result<Self, Self::Error> {
        match Self::get_fixheader(byte) {
            Ok((packet_type, reserved, remaining_length, n)) => {
                let dup_flag = if (reserved >> 3) == 0x01 {
                    true
                }else {
                    false
                };
                let qos_level = {
                    reserved >> 1 & 0x03
                };
                let retain = if (reserved & 0x01) == 0x01 {
                    true
                }else {
                    false
                };

                byte.split_to(1 + n);

                Ok(PublishFixedHeader{
                    packet_type: packet_type,
                    dup_flag: dup_flag,
                    qos_level: qos_level,
                    retain: retain,
                    remaining_length: remaining_length,
                })
            },
            Err(err) => Err(From::from(err))
        } 
    }
}

impl Encodable for PublishFixedHeader {
    type Error = PublishError;
    type Cond = ();

    fn encode_with(&self, _: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        let mut byte = 0u8;
        if self.dup_flag {
            byte |= 8;
        }
        byte |= self.qos_level << 1;
        if self.retain {
            byte |= 1;
        }

        Self::encode_fixedheader(self.packet_type, byte, self.remaining_length).map_err(From::from)
    }


    fn encode_length(&self) -> Result<u32, Self::Error> {
        Self::get_remaining_length_bytes(self.remaining_length).map_err(From::from)
    }
}

#[derive(Debug)]
struct PublishPayload(Vec<u8>);

impl Encodable for PublishPayload{
    type Error = PublishError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        let mut v = vec![];
        v.extend(self.0.iter().cloned());
        Ok(v)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        Ok(self.0.len() as u32)
    }
}

impl<'a> Decodable<'a> for PublishPayload{
    type Error = PublishError;
    type Cond = usize;

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        println!("{:?}", decode_size);
        if let Some(len) = decode_size {
            if len >= 0 {

                if len >= byte.len() {
                    Ok(PublishPayload(byte.split_to(len).to_vec()))
                }else {
                    bail!(ErrorKind::PublishPayloadError("no enough byte to decode".into()))
                }
            } else{
                Ok(PublishPayload(vec![]))
            }
        } else {
            bail!(ErrorKind::PublishPayloadError("param is none is avaiable".into()))
        }
    }
}


#[derive(Debug)]
pub struct Publish{
    fixed_header: PublishFixedHeader,
    topic_name: TopicName,    
    packet_identifier: PacketIdentifier,
    payload: PublishPayload,
}

impl Publish{
    fn new() -> Publish {
        let mut publish = Publish {
            fixed_header: PublishFixedHeader::new(),
            topic_name: TopicName("a/b".into()),
            packet_identifier: PacketIdentifier(10),
            payload: PublishPayload(vec![32,32,32])
        };
        publish.calculate_remaining_length();
        publish
    }

    fn calculate_remaining_length(&mut self) -> Result<(), PublishError> {
        let remaining_length = self.topic_name.encode_length()?
                                + self.packet_identifier.encode_length()?
                                + self.payload.encode_length()?;
        self.fixed_header.remaining_length = remaining_length;
        Ok(())
    }
}


impl<'a> Decodable<'a> for Publish {
    type Error = PublishError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let fixed_header :PublishFixedHeader = Decodable::decode(byte)?;
        let topic_name :TopicName= Decodable::decode(byte)?;
        let packet_identifier: PacketIdentifier = Decodable::decode(byte)?;
        
        let paylaod_length = fixed_header.remaining_length - topic_name.encode_length()? - packet_identifier.encode_length()?;

        let payload = Decodable::decode_with(byte, Some(paylaod_length as usize))?;

        Ok(Publish{
            fixed_header: fixed_header,
            topic_name: topic_name,
            packet_identifier: packet_identifier,
            payload: payload,
        })
    }
}

impl Encodable for Publish {
    type Error = PublishError;
    type Cond = ();

    fn encode_with(&self, _: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        let mut v = vec![];
        let fixed_header = self.fixed_header.encode()?;
        let topic_name = self.topic_name.encode()?;
        let packet_identifier = self.packet_identifier.encode()?;
        let payload = self.payload.encode()?;

        v.extend(fixed_header);
        v.extend(topic_name);
        v.extend(packet_identifier);
        v.extend(payload);

        Ok(v)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        let len = self.fixed_header.encode_length()?
                + self.topic_name.encode_length()?
                + self.packet_identifier.encode_length()?
                + self.packet_identifier.encode_length()?;

        Ok(len)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use bytes::BytesMut;
    #[test]
    fn test_encode_decode_publish_header(){
        let publish = PublishFixedHeader::new();
        let bytes = publish.encode();
        //println!("{:?}", bytes);
    }

    #[test]
    fn test_encode_decode_publish(){
        let mut publish = Publish::new(); 
        let bytes = publish.encode().unwrap();
        //println!("{:?}", bytes);

        
        let mut bytesmut = BytesMut::from(bytes); 
        //println!("{:?}", Publish::decode(&mut bytesmut));        

    }

}

