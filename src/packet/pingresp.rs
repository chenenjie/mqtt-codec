
use {Encodable, Decodable};
use packet::FixedHeader;
use bytes::BytesMut;

error_chain!{
    types{
        PingRespError, ErrorKind, ResultExt, PingRespResult;
    }
    links{
        FixedHeaderError(::packet::FixedHeaderError, ::packet::ErrorKind);
    }
}

#[derive(Debug)]
struct PingRespFixedHeader{
    packet_type: u8,
    remaining_length: u32,
}

impl PingRespFixedHeader{
    fn new() -> PingRespFixedHeader{
        PingRespFixedHeader{
            packet_type: 13,
            remaining_length: 0,
        }
    }
}

impl FixedHeader for PingRespFixedHeader {
    fn set_remaining_length(&mut self, len: u32) {
        self.remaining_length = len;
    }
}

impl<'a> Decodable<'a> for PingRespFixedHeader{

    type Error = PingRespError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        match Self::get_fixheader(byte) {
            Ok((packet_type, _, remaining_length, n)) => {
                byte.split_to(1 + n);
                Ok(PingRespFixedHeader {
                    packet_type: packet_type,
                    remaining_length: remaining_length,
                })
            },
            Err(err) => Err(err.into()),
        }
    }
}

impl Encodable for PingRespFixedHeader{
    type Error = PingRespError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        Self::encode_fixedheader(self.packet_type, 0u8, self.remaining_length).map_err(From::from)
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        Self::get_remaining_length_bytes(self.remaining_length).map_err(From::from)
    }
}

#[derive(Debug)]
pub struct PingResp{
    fixed_header: PingRespFixedHeader,  
}

impl PingResp {
    fn new() -> PingResp{
        PingResp{
            fixed_header: PingRespFixedHeader::new()
        }
    }
}

impl<'a> Decodable<'a> for PingResp{
    type Error = PingRespError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let fixed_header = Decodable::decode(byte)?;

        Ok(PingResp{
            fixed_header: fixed_header,
        })
    }
}

impl Encodable for PingResp {
    type Error = PingRespError;
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
    fn test_pingresp_encode_decode(){
        let packet = PingResp::new();
        let vecbytes = packet.encode().unwrap();

        //println!("{:?}", vecbytes);

        let mut bytes = BytesMut::from(vecbytes);
        let result = PingResp::decode(&mut bytes);
        //println!("{:?}", result);
    }
}
