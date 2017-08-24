use {Encodable, Decodable};
use bytes::BytesMut;
use control::variable_header::PacketIdentifier;
use packet::FixedHeader;

error_chain!{
    types{
        UnSubAckError, ErrorKind, ResultExt, UnSubAckResult;
    }

    links{
        FixedHeaderError(::packet::FixedHeaderError, ::packet::ErrorKind);
        PacketIdentifierError(::control::variable_header::PacketIdentifierError, ::control::variable_header::PacketIdentifierErrorKind);
    }
}
#[derive(Debug)]
struct UnSubAckFixedHeader{
    packet_type: u8,
    remaining_length: u32,
}

impl UnSubAckFixedHeader{
    fn new() -> UnSubAckFixedHeader{
        UnSubAckFixedHeader{
            packet_type:11,
            remaining_length: 2
        }
    }
}

impl FixedHeader for UnSubAckFixedHeader{
    fn set_remaining_length(&mut self, len: u32) {
        self.remaining_length = len;
    }
}

impl<'a> Decodable<'a> for UnSubAckFixedHeader{
    type Error = UnSubAckError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        match Self::get_fixheader(byte) {
            Ok((packet_type, _, remaining_length, n)) => {
                byte.split_to(1 + n);
                Ok(UnSubAckFixedHeader {
                    packet_type: packet_type,
                    remaining_length: remaining_length,
                })
            },
            Err(err) => Err(err.into()),
        }
    }
}

impl Encodable for UnSubAckFixedHeader{
    type Error = UnSubAckError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        Self::encode_fixedheader(self.packet_type, 0u8, self.remaining_length).map_err(From::from)
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        Self::get_remaining_length_bytes(self.remaining_length).map_err(From::from)
    }
}


#[derive(Debug)]
pub struct UnSubAck{
    fixed_header: UnSubAckFixedHeader,
    packet_identifier: PacketIdentifier,
}

impl UnSubAck{
    fn new() -> UnSubAck {
        UnSubAck{
            fixed_header: UnSubAckFixedHeader::new(),
            packet_identifier: PacketIdentifier(323),
        }
    }
}


impl<'a> Decodable<'a> for UnSubAck {
    type Error = UnSubAckError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let fixed_header = Decodable::decode(byte)?;
        let packet_identifier = Decodable::decode(byte)?;

        Ok(UnSubAck{
            fixed_header: fixed_header,
            packet_identifier: packet_identifier,
        })
    }
}

impl Encodable for UnSubAck {
    type Error = UnSubAckError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        let mut vec = vec![];
        let fixed_header = self.fixed_header.encode()?;
        let packet_identifier = self.packet_identifier.encode()?;

        vec.extend(fixed_header);
        vec.extend(packet_identifier);

        Ok(vec)
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        let length = self.fixed_header.encode_length()?
                    + self.packet_identifier.encode_length()?;
        Ok(length)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_unsuback_encode_decode(){
        let packet = UnSubAck::new();
        let vecbyte = packet.encode().unwrap();
        //println!("{:?}", vecbyte);

        let mut bytes = BytesMut::from(vecbyte);
        let result = UnSubAck::decode(&mut bytes);
        //println!("{:?}", result);
    }
}

