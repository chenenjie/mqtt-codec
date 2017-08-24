use {Encodable, Decodable};
use bytes::BytesMut;
use control::variable_header::PacketIdentifier;
use topic_filter::TopicFilter;
use packet::FixedHeader;

error_chain!{

    types {
        UnsubscribeError, ErrorKind, ResultExt, UnsubscribeResult;
    }

    links{
        FixedHeaderError(::packet::FixedHeaderError, ::packet::ErrorKind);
        PacketIdentifierError(::control::variable_header::PacketIdentifierError, ::control::variable_header::PacketIdentifierErrorKind);
        TopicFilterError(::topic_filter::TopicFilterError, ::topic_filter::ErrorKind);
    }
}

#[derive(Debug)]
struct UnsubscribeFixedHeader {
    packet_type: u8,
    remaining_length: u32,
}

impl UnsubscribeFixedHeader {
    fn new() -> UnsubscribeFixedHeader{
        UnsubscribeFixedHeader{
            packet_type: 10,
            remaining_length: 0,
        }
    }
}

impl FixedHeader for UnsubscribeFixedHeader {
    
    fn set_remaining_length(&mut self, len: u32){
        self.remaining_length = len;
    }
}

impl<'a> Decodable<'a> for UnsubscribeFixedHeader{
    type Error = UnsubscribeError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _: Option<Self::Cond>) -> Result<Self, Self::Error>{
        match Self::get_fixheader(byte) {
            Ok((packet_type, _, remaining_length, n)) => {
                byte.split_to(1 + n);
                Ok(UnsubscribeFixedHeader {
                    packet_type: packet_type,
                    remaining_length: remaining_length,
                })
            },
            Err(err) => Err(err.into()),
        }
    }
}

impl Encodable for UnsubscribeFixedHeader{
    type Error = UnsubscribeError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        Self::encode_fixedheader(self.packet_type, 0u8, self.remaining_length).map_err(From::from)
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        Self::get_remaining_length_bytes(self.remaining_length).map_err(From::from)
    }
}

#[derive(Debug)]
struct UnsubscribePayload {
    filters: Vec<TopicFilter>,
}

impl UnsubscribePayload {
    fn new() -> UnsubscribePayload {
        let vec = vec![TopicFilter("fuck".into())];
        UnsubscribePayload {
            filters: vec,
        }
    }
}

impl<'a> Decodable<'a> for UnsubscribePayload{

    type Error = UnsubscribeError;
    type Cond = u32;

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let mut size = match decode_size {
            Some(n) => n,
            None => bail!("unavaiable no param to decode Unsubscribe payload.")
        };

        let mut vec = Vec::new();
        while size > 0 {
            let topic_filter: TopicFilter = Decodable::decode(byte)?;
            size -= topic_filter.encode_length()?;
            vec.push(topic_filter);
        }

        Ok(UnsubscribePayload{
            filters: vec,
        })
    }
}

impl Encodable for UnsubscribePayload{
    type Error = UnsubscribeError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        let mut vec = Vec::new();
        for topic_filter in &self.filters {
            vec.extend(topic_filter.encode()?);
        }

        Ok(vec)
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        let mut size = 0u32;
        for topic_filter in &self.filters {
            size += topic_filter.encode_length()?;
        }

        Ok(size)
    }
}

#[derive(Debug)]
pub struct Unsubscribe {
    fixed_header: UnsubscribeFixedHeader,
    packet_identifier: PacketIdentifier,
    payload: UnsubscribePayload,
}

impl Unsubscribe {
    fn new() -> Unsubscribe {
        let mut result = Unsubscribe{
            fixed_header: UnsubscribeFixedHeader::new(),
            packet_identifier: PacketIdentifier(32),
            payload: UnsubscribePayload::new(),
        };
        result.calculate_remaining_length();
        result
    }

    fn calculate_remaining_length(&mut self) -> Result<(), UnsubscribeError>{
        let length = self.packet_identifier.encode_length()? + self.payload.encode_length()?; 
        self.fixed_header.remaining_length = length;
        Ok(())
    }
}

impl<'a> Decodable<'a> for Unsubscribe{

    type Error = UnsubscribeError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        let fixed_header: UnsubscribeFixedHeader = Decodable::decode(byte)?;
        let packet_identifier: PacketIdentifier = Decodable::decode(byte)?;

        let payload_length = fixed_header.remaining_length - packet_identifier.encode_length()?;
        
        let payload = Decodable::decode_with(byte, Some(payload_length))?;

        Ok(Unsubscribe{
            fixed_header: fixed_header,
            packet_identifier: packet_identifier,
            payload: payload,
        })

    }
}

impl Encodable for Unsubscribe{
    type Error = UnsubscribeError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        let mut vec = vec![];
        let fixed_header = self.fixed_header.encode()?;
        let packet_identifier = self.packet_identifier.encode()?;
        let payload = self.payload.encode()?;

        vec.extend(fixed_header);
        vec.extend(packet_identifier);
        vec.extend(payload);

        Ok(vec)
    }

    fn encode_length(&self) -> Result<u32, Self::Error>{
        let length = self.fixed_header.encode_length()? 
                    + self.packet_identifier.encode_length()?
                    + self.payload.encode_length()?;
        Ok(length)
    }
}


#[cfg(test)]
mod test{
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_unsubscribe_encode_decode(){
        let fuck = Unsubscribe::new();
        let vecbytes = fuck.encode().unwrap();
        //println!("{:?}", vecbytes);

        let mut bytes = BytesMut::from(vecbytes);
        let result = Unsubscribe::decode(&mut bytes);
        //println!("{:?}", result);
    }
}
