use bytes::BytesMut;
use {Encodable, Decodable};

error_chain!{
    types {
        QualityOfServiceError, ErrorKind, ResultExt, QualityOfServiceResult;
    }
}

#[derive(Debug, Clone)]
pub enum QualityOfService{
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
}


impl<'a> Decodable<'a> for QualityOfService{
    type Error = QualityOfServiceError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        let flag: u8 = Decodable::decode(byte).chain_err(||"decode quality of service byte fail")?;
        let mut result = match flag {
            0 => Ok(QualityOfService::Level0),
            1 => Ok(QualityOfService::Level1),
            2 => Ok(QualityOfService::Level2),
            _ => bail!("QualityOfService unavaiable u8"),
        };
        result
    }
}

impl Encodable for QualityOfService{
    type Error = QualityOfServiceError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        match *self {
            QualityOfService::Level0 => Ok(vec![0]),
            QualityOfService::Level1 => Ok(vec![1]),
            QualityOfService::Level2 => Ok(vec![2]),
        }
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        Ok(1u32)
    }
}
