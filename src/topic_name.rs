use bytes::BytesMut;
use {Encodable, Decodable};

pub struct TopicName(String);

error_chain!{
    types{
        TopicNameError, ErrorKind, ResultExt, TopicNameResult;
    }
}

impl<'a> Decodable<'a> for TopicName {
    type Error = TopicNameError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        Ok(TopicName(Decodable::decode(byte).chain_err(||"decode topic name string fail")?))
    }
}

impl Encodable for TopicName {
    type Error = TopicNameError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        self.0.encode().chain_err(||"encode topic name string fail")
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        self.0.encode_length().chain_err(||"encode topic name string length fail")
    }
}


