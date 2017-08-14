use bytes::BytesMut;
use {Encodable, Decodable};
use packet::FixedHeader;
use control::variable_header::{PacketIdentifierError, PacketIdentifierErrorKind, PacketIdentifier};

error_chain!{
    types{
        PubCompError, ErrorKind, ResultExt, PubCompResult;
    }

    links{
        FixedHeader(::packet::FixedHeaderError, ::packet::ErrorKind);
        PacketIdentifierError(::control::variable_header::PacketIdentifierError, ::control::variable_header::PacketIdentifierErrorKind);
    }
}


#[derive(Debug)]
struct PubCompFixedHeader{
    packet_type: u8,
    remaining_length: u32,
}

impl PubCompFixedHeader {
    fn new() -> PubCompFixedHeader {
        PubCompFixedHeader {
            packet_type: 7,
            remaining_length: 2u32,
        }
    }
}

impl FixedHeader for PubCompFixedHeader{

    fn set_remaining_length(&mut self, len: u32) {
        self.remaining_length = len;
    }
}

impl<'a> Decodable<'a> for PubCompFixedHeader {
    type Error = PubCompError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _: Option<Self::Cond>) -> Result<Self, Self::Error>{
        
        match Self::get_fixheader(byte) {
            Ok((packet_type, _reserved, remaining_length, n)) => {
                byte.split_to(1 + n);

                Ok(PubCompFixedHeader{
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

impl Encodable for PubCompFixedHeader {
    type Error = PubCompError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        Self::encode_fixedheader(self.packet_type, 0u8, self.remaining_length).map_err(From::from)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        Ok(2u32)
    }
}

#[derive(Debug)]
struct PubComp{
    fixed_header: PubCompFixedHeader,
    packet_identifier: PacketIdentifier,
}

impl PubComp {
    pub fn new() -> PubComp{
        PubComp{
            fixed_header: PubCompFixedHeader::new(),
            packet_identifier: PacketIdentifier(100),
        }
    }
}

impl<'a> Decodable<'a> for PubComp {
    
    type Error = PubCompError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let fixed_header = Decodable::decode(byte)?;
        let packet_identifier = Decodable::decode(byte)?;

        Ok(PubComp {
            fixed_header: fixed_header,
            packet_identifier: packet_identifier,
        })
    }
}

impl Encodable for PubComp{
    type Error = PubCompError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        let mut vec = vec![];
        let fixed_header = self.fixed_header.encode()?;
        let packet_identifier = self.packet_identifier.encode()?;


        vec.extend(fixed_header);
        vec.extend(packet_identifier);

        Ok(vec)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        let length = self.fixed_header.encode_length()? 
                    + self.packet_identifier.encode_length()?;
        Ok(length)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_encode_decode_pubComp() {
        let pub_comp = PubComp::new();
        let vec = pub_comp.encode().unwrap();
        println!("{:?}", vec);

        let mut bytes = BytesMut::from(vec);
        println!("{:?}", PubComp::decode(&mut bytes));
    }
}


