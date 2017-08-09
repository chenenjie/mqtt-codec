use Encodable; 
use Decodable;
use bytes::BytesMut;

error_chain!{
    types{
        ConnectAckFlagsError, ErrorKind, ResultExt, ConnectAckFlagsResult;
    }
}
#[derive(Debug)]
pub struct ConnectAckFlags(pub bool);

impl<'a> Decodable<'a> for ConnectAckFlags {
    type Error = ConnectAckFlagsError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _: Option<Self::Cond>) -> Result<Self, Self::Error>{
        let len = byte.len();
        if len >= 1{
            let session_present = if byte[0] & 0x01 == 0x01 {
                true
            }else {
                false
            };
            Ok(ConnectAckFlags(session_present))
        } else {
            bail!("no enough byte to decode connect_ack_flag");
        }
    }
}

impl Encodable for ConnectAckFlags{
    type Error = ConnectAckFlagsError;
    type Cond = ();

    fn encode_with(&self, _: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        let mut vec = vec![];
        if self.0 {
            vec.push(1);
        }else{
            vec.push(0);
        }
        Ok(vec)
    }


    fn encode_length(&self) -> Result<u32, Self::Error> {
        Ok(1u32)
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    use bytes::BytesMut;

    #[test]
    fn test_encode_decode_connect_ack_flag(){
        let vec = vec![0u8];
        let mut bytes = BytesMut::from(vec);
        let caf = ConnectAckFlags::decode(&mut bytes);
        let result = caf.unwrap().encode();
    }
}
