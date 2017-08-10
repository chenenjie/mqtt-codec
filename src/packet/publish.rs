use bytes::BytesMut;
use {Decodable, Encodable};
use packet::FixedHeader;

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


