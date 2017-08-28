#![recursion_limit="128"]

#[macro_use]
extern crate log;
extern crate bytes;
#[macro_use]
extern crate error_chain;

pub mod packet;
mod control;
mod qos;
mod topic_name;
mod return_code;
mod topic_filter;


use bytes::BytesMut;
use bytes::BigEndian;
use bytes::ByteOrder;
use std::fmt;
use packet::FixedHeaderError;
use std::error::Error;

pub trait Decodable<'a>: Sized {
    type Error;
    type Cond;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error> {
        Self::decode_with(bytes, None)
    }

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>;
}

pub trait Encodable{
    type Error;
    type Cond;

    fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        Self::encode_with(&self, None)
    }

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>;

    fn encode_length(&self) -> Result<u32, Self::Error>;
}

impl<'a> Decodable<'a> for String {
    type Error = PacketError;
    type Cond = ();

    fn decode_with(bytes: &mut BytesMut, _size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let len = bytes.len();
        let mut size = 0u16;
        if len >= 2 {
            size = BigEndian::read_u16(bytes);
        } else {
            error!("not enough bytes encode String header bytes");
            // return Err(PacketError::NoEnoughBytesToDecode);
            bail!(ErrorKind::NoEnoughBytesToDecode)
        }

        
        if len < (size as usize) + 2 {
            error!("expect size len : {}, acutal len : {}", size, len);
            error!("not enough bytes encode String content bytes");
            // return Err(PacketError::NoEnoughBytesToDecode);
            bail!(ErrorKind::NoEnoughBytesToDecode)
        }

        Ok(String::from_utf8(bytes.split_to(2 + ( size as usize )).split_off(2).to_vec())?)
    }
}

impl<'a> Decodable<'a> for u8 {
    type Error = PacketError;
    type Cond = ();

    fn decode_with(bytes: &mut BytesMut, _size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let len = bytes.len();
        if len >= 1 {
            let code = bytes[0];
            bytes.split_to(1);
            Ok(code)
        } else {
            error!("u8 enough code to decode");
            // return Err(PacketError::NoEnoughBytesToDecode)
            bail!(ErrorKind::NoEnoughBytesToDecode)
        }
    }
}

impl<'a> Decodable<'a> for u16 {
    type Error = PacketError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        let len = byte.len();
        let mut result = 0u16;
        if len >= 2 {
            result = BigEndian::read_u16(byte);
            byte.split_to(2);
            Ok(result)
        }else {
            error!("u16 not enough code to decode");
            // return Err(PacketError::NoEnoughBytesToDecode)
            bail!(ErrorKind::NoEnoughBytesToDecode)
        }
    }
}


impl Encodable for String {
    type Error = PacketError;
    type Cond = ();
    fn encode_with(&self, _cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        let length = self.len() as u16;
        let mut vec = vec![0u8, 0u8];
        BigEndian::write_u16(&mut vec, length);
        vec.extend(self.as_bytes());
        Ok(vec)
    }
    
    fn encode_length(&self) -> Result<u32, Self::Error>{
        Ok(2 + (self.len() as u32))
    }
}


impl Encodable for u16 {
    type Error = PacketError;
    type Cond = ();
    fn encode_with(&self, _cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        let mut vec = vec![0u8; 2];
        BigEndian::write_u16(&mut vec, *self);
        Ok(vec)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        Ok(2u32)
    }
}

impl Encodable for u8{
    type Error = PacketError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        Ok(vec![*self])
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        Ok(1u32)
    }
}


error_chain!{
    types {
        PacketError, ErrorKind, ResultExt, PacketResult;
    }
    
    errors{
        NoEnoughBytesToDecode
        InvalidEncode
    }

    links {
        ConnectPacket(::packet::ConnectError, ::packet::ConnectErrorKind);
    }

    foreign_links {
        FromUtf8Error(::std::string::FromUtf8Error);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}

    #[test]
    fn check_string_decode() {
        let mut vec = vec![0x00, 0x04];
        vec.extend(String::from("mqtt").into_bytes().iter().clone());
        let mut bytes = BytesMut::from(vec);

        let result = String::decode(&mut bytes);
        // println!("{:?}", result);
        // println!("{:?}", bytes);
    }

    #[test]
    fn check_u8_decode() {
        let mut vec = vec![];
        let mut bytes = BytesMut::from(vec);

        let result = u8::decode(&mut bytes);
        // println!("{:?}", result);
    }

    #[test]
    fn check_string_encode(){
        let target = String::from("enjie");
        // println!("{:?}", target.encode));
    }


    #[test]
    fn check_u16_encode(){
        let number = 65535u16;

        let mut bytes = BytesMut::from(number.encode().unwrap());
        
        let encode: Result<u16, PacketError> = Decodable::decode(&mut bytes);
        // println!("{:?}", encode);
        
    }
}
