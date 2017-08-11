use bytes::BytesMut;
use {Decodable, Encodable};

pub struct TopicName(pub String);

error_chain!{
    types {
        TopicNameError, ErrorKind, ResultExt, TopicNameResult;
    }
}

impl<'a> Decodable<'a> for TopicName {
    type Error = TopicNameError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _: Option<Self::Cond>) -> Result<Self, Self::Error> {
        Ok(TopicName(Decodable::decode(byte).chain_err(|| "decode topic name avaiable")?))
    }
}


impl Encodable for TopicName {
    type Error = TopicNameError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        self.0.encode().chain_err(||"topic name encode avaiable")
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        self.0.encode_length().chain_err(||"topic name encode length avaiable")
    }
}

