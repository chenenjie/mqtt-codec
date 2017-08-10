use Decodable;
use Encodable;
use bytes::BytesMut;

#[derive(Debug)]
pub struct ConnectReturnCode(pub u8);

error_chain!{
    types{
        ConnectReturnCodeError, ErrorKind, ResultExt, ConnectReturnCodeResult;
    }
}

impl<'a> Decodable<'a> for ConnectReturnCode {
    type Error = ConnectReturnCodeError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        Ok(ConnectReturnCode(Decodable::decode(byte).chain_err(||"decode connect return code error")?))
    }
}

impl Encodable for ConnectReturnCode{
    type Error = ConnectReturnCodeError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        Ok(vec![self.0])
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        Ok(1u32)
    }
}



