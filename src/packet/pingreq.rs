use {Encodable, Decodable};
use packet::FixedHeader;
use bytes::BytesMut;

error_chain!{
    types{
        PingReqError, ErrorKind, ResultExt, PingReqResult;
    }
    links{
        FixedHeaderError(::packet::FixedHeaderError, ::packet::ErrorKind);
    }
}

#[derive(Debug)]
struct PingReqFixedHeader{
    packet_type: u8,
    remaining_length: u32,
}

impl PingReqFixedHeader{
    fn new() -> PingReqFixedHeader{
        PingReqFixedHeader{
            packet_type: 12,
            remaining_length: 0,
        }
    }
}

impl FixedHeader for PingReqFixedHeader {
    fn set_remaining_length(&mut self, len: u32) {
        self.remaining_length = len;
    }
}

impl<'a> Decodable<'a> for PingReqFixedHeader{

    type Error = PingReqError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        match Self::get_fixheader(byte) {
            Ok((packet_type, _, remaining_length, n)) => {
                byte.split_to(1 + n);
                Ok(PingReqFixedHeader {
                    packet_type: packet_type,
                    remaining_length: remaining_length,
                })
            },
            Err(err) => Err(err.into()),
        }
    }
}

impl Encodable for PingReqFixedHeader{
    type Error = PingReqError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        Self::encode_fixedheader(self.packet_type, 0u8, self.remaining_length).map_err(From::from)
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        Self::get_remaining_length_bytes(self.remaining_length).map_err(From::from)
    }
}

#[derive(Debug)]
pub struct PingReq{
    fixed_header: PingReqFixedHeader,  
}

impl PingReq {
    fn new() -> PingReq{
        PingReq{
            fixed_header: PingReqFixedHeader::new()
        }
    }
}

impl<'a> Decodable<'a> for PingReq{
    type Error = PingReqError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let fixed_header = Decodable::decode(byte)?;

        Ok(PingReq{
            fixed_header: fixed_header,
        })
    }
}

impl Encodable for PingReq {
    type Error = PingReqError;
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
    fn test_pingreq_encode_decode(){
        let packet = PingReq::new();
        let vecbytes = packet.encode().unwrap();

        println!("{:?}", vecbytes);

        let mut bytes = BytesMut::from(vecbytes);
        let result = PingReq::decode(&mut bytes);
        println!("{:?}", result);
    }
}
