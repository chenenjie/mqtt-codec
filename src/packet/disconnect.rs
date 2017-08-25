use {Encodable, Decodable};
use packet::FixedHeader;
use bytes::BytesMut;

error_chain!{
    types{
        DisconnectError, ErrorKind, ResultExt, DisconnectResult;
    }
    links{
        FixedHeaderError(::packet::FixedHeaderError, ::packet::ErrorKind);
    }
}

#[derive(Debug)]
struct DisconnectFixedHeader{
    packet_type: u8,
    remaining_length: u32,
}

impl DisconnectFixedHeader{
    fn new() -> DisconnectFixedHeader{
        DisconnectFixedHeader{
            packet_type: 14,
            remaining_length: 0,
        }
    }
}

impl FixedHeader for DisconnectFixedHeader {
    fn set_remaining_length(&mut self, len: u32) {
        self.remaining_length = len;

    }
}

impl<'a> Decodable<'a> for DisconnectFixedHeader{

    type Error = DisconnectError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        match Self::get_fixheader(byte) {
            Ok((packet_type, _, remaining_length, n)) => {
                byte.split_to(1 + n);
                Ok(DisconnectFixedHeader {
                    packet_type: packet_type,
                    remaining_length: remaining_length,
                })
            },
            Err(err) => Err(err.into()),
        }
    }
}

impl Encodable for DisconnectFixedHeader{
    type Error = DisconnectError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        Self::encode_fixedheader(self.packet_type, 0u8, self.remaining_length).map_err(From::from)
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        Self::get_remaining_length_bytes(self.remaining_length).map_err(From::from)
    }
}

#[derive(Debug)]
pub struct Disconnect{
    fixed_header: DisconnectFixedHeader,  
}

impl Disconnect {
    fn new() -> Disconnect{
        Disconnect{
            fixed_header: DisconnectFixedHeader::new()
        }
    }
}

impl<'a> Decodable<'a> for Disconnect{
    type Error = DisconnectError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let fixed_header = Decodable::decode(byte)?;

        Ok(Disconnect{
            fixed_header: fixed_header,
        })
    }
}

impl Encodable for Disconnect {
    type Error = DisconnectError;
    type Cond = ();


    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        self.fixed_header.encode().map_err(From::from)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        self.fixed_header.encode_length().map_err(From::from)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_disconnect_encode_decode(){
        let packet = Disconnect::new();
        let vecbytes = packet.encode().unwrap();

        //println!("{:?}", vecbytes);

        let mut bytes = BytesMut::from(vecbytes);
        let result = Disconnect::decode(&mut bytes);
        //println!("{:?}", result);
    }
}
