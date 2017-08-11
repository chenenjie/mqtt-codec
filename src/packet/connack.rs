use {Decodable, Encodable};
use packet::FixedHeader;
use control::variable_header::{ConnectAckFlags, ConnectAckFlagsErrorKind, ConnectAckFlagsError, ConnectReturnCode, ConnectReturnCodeError, ConnectReturnCodeErrorKind};
use bytes::BytesMut;

error_chain!{
    types{
        ConnackError, ErrorKind, ResultExt, ConnackResult;
    }


    links{
        FixedHeader(::packet::FixedHeaderError, ::packet::ErrorKind);
        ConnectAckFlags(ConnectAckFlagsError, ConnectAckFlagsErrorKind);
        ConnectReturnCode(ConnectReturnCodeError, ConnectReturnCodeErrorKind);
    }
}

#[derive(Debug)]
struct ConnackFixedHeader {
    packet_type: u8,
    reserved: u8,
    remaining_length: u32,
}

impl ConnackFixedHeader{

    fn new() -> ConnackFixedHeader {
        ConnackFixedHeader{
            packet_type: 2,
            reserved: 0,
            remaining_length: 0,
        }
    }
}

impl FixedHeader for ConnackFixedHeader{
    fn set_remaining_length(&mut self, len: u32) {
        self.remaining_length = len;
    }
}

impl<'a> Decodable<'a> for ConnackFixedHeader {
    type Error =  ConnackError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        match Self::get_fixheader(byte) {
            Ok((packet_type, reserved, remaining_length, n)) => {
                byte.split_to(1 + n);
                Ok(ConnackFixedHeader{
                    packet_type: packet_type,
                    reserved: reserved,
                    remaining_length: remaining_length,
                })
            },
            Err(err) => {
                Err(From::from(err))
            }
        }
    }
}

impl Encodable for ConnackFixedHeader {
    type Error = ConnackError;
    type Cond = ();

    
    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        Self::encode_fixedheader(self.packet_type, self.reserved, self.remaining_length).map_err(From::from)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        Self::get_remaining_length_bytes(self.remaining_length).map_err(From::from)
    }
}

#[derive(Debug)]
pub struct Connack {
    fixed_header: ConnackFixedHeader,
    connect_ack_flag: ConnectAckFlags,
    connect_return_code: ConnectReturnCode,
}

impl Connack{
    fn new() -> Connack {
        let mut connack = Connack{
            fixed_header: ConnackFixedHeader::new(),
            connect_ack_flag: ConnectAckFlags(false),
            connect_return_code: ConnectReturnCode(0u8),
        };
        connack.calculate_remaining_length();
        
        connack
    }

    fn calculate_remaining_length(&mut self) -> Result<(), ConnackError> {
        let remaining_length = self.connect_ack_flag.encode_length()? 
                            + self.connect_return_code.encode_length()?;
        self.fixed_header.set_remaining_length(remaining_length);
        Ok(())
    }

    fn set_connect_ack_flag(&mut self, flag: bool){
        self.connect_ack_flag.0 = flag;
        self.calculate_remaining_length();
    }

    fn set_connect_return_code(&mut self, code: u8) {
        self.connect_return_code.0 = code;
        self.calculate_remaining_length();
    }
}

impl<'a> Decodable<'a> for Connack {
    type Error = ConnackError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let fixed_header = Decodable::decode(byte)?;
        let connect_ack_flag = Decodable::decode(byte)?;
        let connect_return_code = Decodable::decode(byte)?;

        Ok(Connack{
            fixed_header: fixed_header,
            connect_ack_flag: connect_ack_flag,
            connect_return_code: connect_return_code,
        })
    }
}

impl Encodable for Connack {
    type Error = ConnackError;
    type Cond = ();

    fn encode_with(&self, _: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        let mut vec = vec![];

        let fixed_header = self.fixed_header.encode()?;
        let connect_ack_flag = self.connect_ack_flag.encode()?;
        let connect_return_code = self.connect_return_code.encode()?;

        vec.extend(fixed_header);
        vec.extend(connect_ack_flag);
        vec.extend(connect_return_code);

        Ok(vec)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        let length = self.fixed_header.encode_length()? + self.connect_ack_flag.encode_length()? + self.connect_return_code.encode_length()?;
        Ok(length)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;
    #[test]
    fn test_encode_decode_connack_packet(){
        let connack = Connack::new(); 
        //println!("{:?}",connack.encode());
        let mut bytes = BytesMut::from(connack.encode().unwrap());
        let connack_copy = Connack::decode(&mut bytes);
        //println!("{:?}", connack_copy);
    }
}
