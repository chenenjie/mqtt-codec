use PacketError;
use bytes::BytesMut;
use Decodable;
use Encodable;
use bytes::BigEndian;
use bytes::ByteOrder;

error_chain!{
    types {
        VecBytesError, ErrorKind, ResultExt, VecBytesResult;
    }
}

#[derive(Debug)]
pub struct VecBytes(pub Vec<u8>);

impl<'a> Decodable<'a> for VecBytes{
    type Error = VecBytesError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        let len = byte.len();
        let mut size = 0u16;
        if len >= 2 {
            size = BigEndian::read_u16(byte);
        }else {
            error!("will message header is not enough code to decode");
            bail!("will message header is not enough code to decode");
        }

        let split_len = (size + 2 ) as usize;
        if len >= split_len {
            let result = VecBytes(byte[2..split_len].to_vec());
            byte.split_to(split_len);

            Ok(result)
        }else {
            error!("will message header is not enough code to decode");
            // Err( ConnectPacketError::NoEnoughBytesToDecode )
            bail!("will message header is not enough code to decode");
        }
    }
} 

impl Encodable for VecBytes {
    type Error = VecBytesError;
    type Cond = ();
    fn encode_with(&self, _cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        let mut result = vec![0u8; 2];
        BigEndian::write_u16(&mut result, self.0.len() as u16);
        result.extend(self.0.iter().cloned());
        Ok(result)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        Ok( 2 + ( self.0.len() as u32 ) )
    }
}
