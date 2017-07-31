extern crate bytes;

mod packet;

use bytes::BytesMut;
use bytes::BigEndian;
use bytes::ByteOrder;
use std::fmt;
use packet::FixedHeaderError;


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
            return Err(PacketError::NoEnoughBytesToDecode);
        }

        if len < (size as usize) + 2 {
            return Err(PacketError::NoEnoughBytesToDecode);
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
            return Err(PacketError::NoEnoughBytesToDecode)
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
            return Err(PacketError::NoEnoughBytesToDecode)
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
}


impl Encodable for u16 {
    type Error = PacketError;
    type Cond = ();
    fn encode_with(&self, _cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        let mut vec = vec![0u8; 2];
        BigEndian::write_u16(&mut vec, *self);
        Ok(vec)
    }
}


pub enum PacketError {
    NoEnoughBytesToDecode,
    FromUtf8Error(::std::string::FromUtf8Error),
    FixedHeaderError(FixedHeaderError),
}

impl From<::std::string::FromUtf8Error> for PacketError {
    fn from(err: ::std::string::FromUtf8Error) -> PacketError {
        PacketError::FromUtf8Error(err)
    }
}

impl From<FixedHeaderError> for PacketError{
    fn from(err: FixedHeaderError) -> PacketError{
        PacketError::FixedHeaderError(err)
    }
} 

impl fmt::Debug for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &PacketError::NoEnoughBytesToDecode => write!(f, "No EnougnBytes"),
            &PacketError::FromUtf8Error(ref e) => write!(f, "error from utf8 error"),
            &PacketError::FixedHeaderError(ref e) => write!(f, "error from decode fixedHeader")
        }
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
