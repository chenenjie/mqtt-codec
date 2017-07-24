extern crate bytes;

use bytes::BytesMut;
use bytes::BigEndian;
use bytes::ByteOrder;
use std::fmt;

mod packet;

pub trait Decodable: Sized {
    type Error;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error> {
        Self::decode_with(bytes, None)
    }

    fn decode_with(byte: &mut BytesMut, decode_size: Option<usize>) -> Result<Self, Self::Error>;
}

impl Decodable for String {
    type Error = PacketError;
    fn decode_with(bytes: &mut BytesMut, _size: Option<usize>) -> Result<Self, Self::Error> {
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

        //println!("size :{}", size);
        //println!("len :{}", len);
        Ok(String::from_utf8(bytes.split_to(2 + ( size as usize )).split_off(2).to_vec())?)

    }
}


pub enum PacketError {
    NoEnoughBytesToDecode,
    FromUtf8Error(::std::string::FromUtf8Error),
}

impl From<::std::string::FromUtf8Error> for PacketError {
    fn from(err: ::std::string::FromUtf8Error) -> PacketError {
        PacketError::FromUtf8Error(err)
    }
}

impl fmt::Debug for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &PacketError::NoEnoughBytesToDecode => write!(f, "No EnougnBytes"),
            &PacketError::FromUtf8Error(ref e) => write!(f, "error from utf8 error"),
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
        println!("{:?}", result);
        println!("{:?}", bytes);
    }
}
