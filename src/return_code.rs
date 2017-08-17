use {Encodable, Decodable};
use bytes::BytesMut;

error_chain!{
    types{
        SubscribeReturnCodeError, ErrorKind, ResultExt, SubscribeReturnCodeResult;
    }
}

//#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SubscribeReturnCode{
    MaximumQos0 = 0x00,
    MaximumQos1 = 0x01,
    MaximumQos2 = 0x02,
    Failure =  0x80,
}

impl<'a> Decodable<'a> for SubscribeReturnCode {
    type Error = SubscribeReturnCodeError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _: Option<Self::Cond>) -> Result<Self, Self::Error>{
        let code = match Decodable::decode(byte).chain_err(||"decode subscribe reture code u8 error")? {
            0u8 => SubscribeReturnCode::MaximumQos0, 
            1 => SubscribeReturnCode::MaximumQos1, 
            2 => SubscribeReturnCode::MaximumQos2, 
            128 => SubscribeReturnCode::MaximumQos0, 
            _ => bail!("unavaiable subscribe return code"),
        };
        Ok(code)
    }
}

impl Encodable for SubscribeReturnCode {
    type Error = SubscribeReturnCodeError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        let vec = vec![*self as u8];
        Ok(vec)
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        Ok(1u32)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_repr_decode_encode(){
        let return_code = SubscribeReturnCode::MaximumQos1;
        let vec = return_code.encode();

        //println!("{:?}", vec);
    }
}
