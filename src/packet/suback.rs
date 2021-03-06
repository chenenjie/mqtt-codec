use {Encodable, Decodable};
use bytes::BytesMut;
use control::variable_header::PacketIdentifier;
use return_code::SubscribeReturnCode;
use packet::FixedHeader;

error_chain!{
    types{
        SubAckError, ErrorKind, ResultExt, SubAckResult;
    }

    links{
        SubscribeReturnCodeError(::return_code::SubscribeReturnCodeError, ::return_code::ErrorKind);
        FixedHeaderError(::packet::FixedHeaderError, ::packet::ErrorKind);
        PacketIdentifierError(::control::variable_header::PacketIdentifierError, ::control::variable_header::PacketIdentifierErrorKind);
    }
}

#[derive(Debug)]
struct SubAckFixedHeader{
    packet_type: u8,
    remaining_length: u32,
}

impl SubAckFixedHeader{
    fn new() -> SubAckFixedHeader {
        SubAckFixedHeader{
            packet_type: 9,
            remaining_length: 0,
        }
    }
}


impl FixedHeader for SubAckFixedHeader{
    fn set_remaining_length(&mut self, len: u32) {
        self.remaining_length = len;
    }
} 

impl<'a> Decodable<'a> for SubAckFixedHeader {
    type Error = SubAckError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {

        match Self::get_fixheader(byte) {
            Ok((packet_type, _, remaining_length, n)) => {
                byte.split_to(1 + n);
                Ok(SubAckFixedHeader{
                    packet_type: packet_type,
                    remaining_length: remaining_length,
                })
            },
            Err(err) => Err(err.into()),
        }
    }
}

impl Encodable for SubAckFixedHeader {
    type Error = SubAckError;
    type Cond =();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        Self::encode_fixedheader(self.packet_type, 0u8, self.remaining_length).map_err(From::from)
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        Self::get_remaining_length_bytes(self.remaining_length).map_err(From::from) 
    }
}

#[derive(Debug)]
struct SubAckPayload {
    subscribes: Vec<SubscribeReturnCode>,
}

impl<'a> Decodable<'a> for SubAckPayload {
    type Error = SubAckError;
    type Cond = u32;

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let mut size = match decode_size {
            Some(length) => length,
            None => bail!("unavaiable no param to decode sub ack payload "),
        };

        let mut vec = Vec::new();
        while size > 0 {
            let code = Decodable::decode(byte)?;
            vec.push(code);
            size -= 1;
        }

        Ok(SubAckPayload{
            subscribes: vec
        })
    }
}

impl Encodable for SubAckPayload {
    type Error = SubAckError;
    type Cond =();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        let mut vec = Vec::new();
        
        for &code in &self.subscribes{
            vec.push(code as u8);
        }
        Ok(vec)
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        Ok(self.subscribes.len() as u32)
    }
}

#[derive(Debug)]
pub struct SubAck {
    fixed_header: SubAckFixedHeader,
    packet_identifier: PacketIdentifier,
    payload: SubAckPayload,
}

impl SubAck {
    fn new() -> SubAck{
        let mut vec = vec![];
        for i in 1..5 {
            vec.push(SubscribeReturnCode::MaximumQos2);
        }
        let mut suback = SubAck{
            fixed_header: SubAckFixedHeader::new(),
            packet_identifier: PacketIdentifier(81),
            payload: SubAckPayload{
                subscribes: vec
            }
        };

        suback.calculate_remaining_length();
        suback
    }

    fn calculate_remaining_length(&mut self) -> Result<(), SubAckError> {
        let length = self.packet_identifier.encode_length()? + self.payload.encode_length()?; 
        self.fixed_header.remaining_length = length;
        Ok(())
    }
}

impl<'a> Decodable<'a> for SubAck {
    type Error = SubAckError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let fixed_header: SubAckFixedHeader = Decodable::decode(byte)?;
        let packet_identifier: PacketIdentifier = Decodable::decode(byte)?;

        let payload_length = fixed_header.remaining_length - packet_identifier.encode_length()?;
        let payload = Decodable::decode_with(byte, Some(payload_length))?;

        Ok(SubAck{
            fixed_header: fixed_header,
            packet_identifier: packet_identifier,
            payload: payload,
        })
    }
}

impl Encodable for SubAck {
    type Error = SubAckError;
    type Cond =();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        let fixed_header = self.fixed_header.encode()?;
        let packet_identifier = self.packet_identifier.encode()?;
        let payload = self.payload.encode()?;

        let mut vec = Vec::new();
        vec.extend(fixed_header);
        vec.extend(packet_identifier);
        vec.extend(payload);

        Ok(vec)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        let length = self.fixed_header.encode_length()? + self.packet_identifier.encode_length()? + self.payload.encode_length()?;
        Ok(length)
    }
}

#[cfg(test)]
mod test{
    use bytes::BytesMut;
    use super::*;

    #[test]
    fn test_suback_encode_decode(){
        let suback = SubAck::new();
        let mut vecbyte = suback.encode().unwrap();
        //println!("{:?}", vecbyte);

        let mut byte = BytesMut::from(vecbyte);
        let result = SubAck::decode(&mut byte);
        //println!("{:?}", result);


    }
}




