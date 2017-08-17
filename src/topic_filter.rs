use bytes::BytesMut;
use {Encodable, Decodable};

error_chain!{
    types{
        TopicFilterError, ErrorKind, ResultExt, TopicFilterResult;
    }
}

#[derive(Debug)]
pub struct TopicFilter(pub String);

impl<'a> Decodable<'a> for TopicFilter {
    type Error = TopicFilterError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _: Option<Self::Cond>) -> Result<Self, Self::Error> {
        Ok(TopicFilter(Decodable::decode(byte).chain_err(||"decode topic filter fail")?))
    }
}


impl Encodable for TopicFilter {
    type Error = TopicFilterError;
    type Cond = ();

    fn encode_with(&self, _: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        self.0.encode().chain_err(||"encode topic filter fail")
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        self.0.encode_length().chain_err(||"encode topic filter length fail")
    }
}
