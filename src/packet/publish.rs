use bytes::BytesMut;
use {Decodable, Encodable};
use packet::FixedHeader;


error_chain!{
    types {
        PublishError, ErrorKind, ResultExt, PublishError;
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


impl FixedHeader for PublishFixedHeader {
    fn new() -> Self {
        PublishFixedHeader{
            packet_type: 3,
            dup_flag: false,
            qos_level: 0,
            retain: false,
            remaining_length: 0,
        }
    }
     
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
                }

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

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>;

    fn encode_length(&self) -> Result<u32, Self::Error> {
        Self::get_remaining_length_bytes(self.remaining_length)
    }
}


