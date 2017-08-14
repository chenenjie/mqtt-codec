use bytes::BytesMut;
use {Encodable, Decodable};

error_chain!{
    types{
        PacketIdentifierError, ErrorKind, ResultExt, PacketIdentifierResult;
    }
}

#[derive(Debug)]
pub struct PacketIdentifier(pub u16);

impl<'a> Decodable<'a> for PacketIdentifier{
    type Error = PacketIdentifierError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _: Option<Self::Cond>) -> Result<Self, Self::Error> {
        Ok(PacketIdentifier(Decodable::decode(byte).chain_err(||"decode packet identifier avaiable")?))
    }
}

impl Encodable for PacketIdentifier {
    type Error = PacketIdentifierError;
    type Cond = ();

    fn encode_with(&self, _: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        self.0.encode().chain_err(||"encode packet identifier error")
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        self.0.encode_length().chain_err(||"encode length packet identifier error")
    }
}

