use Decodable;
use Encodable;

pub struct ConnectReturnCode(u8);

error_chain!{
    types{
        ConnectReturnCodeError, ErrorKind, ResultExt, ConnectReturnCodeResult;
    }
}

impl<'a> Decodable<'a> for ConnectReturnCode {
    type Error = ConnectReturnCodeError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        Ok(ConnectReturnCode(Decodable::decode(byte)))
    }
}

impl Encodable for ConnectReturnCode{
    type Error;
    type Cond;

    fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        Self::encode_with(&self, None)
    }

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>;

    fn encode_length(&self) -> Result<u32, Self::Error>;
}



