
use PacketError;
use bytes::BytesMut;
use Decodable;
use Encodable;

#[derive(Debug)]
pub struct KeepAlive(pub u16);


impl<'a> Decodable<'a> for KeepAlive{
    type Error = PacketError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _: Option<Self::Cond>) -> Result<Self, Self::Error> {
        Ok(KeepAlive(Decodable::decode(byte)?))
    }
}

impl Encodable for KeepAlive{
    type Error = PacketError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        self.0.encode()
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        self.0.encode_length()
    }
}





