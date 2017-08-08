use super::super::{Decodable, Encodable};
use super::super::PacketError;
use super::FixedHeader;
use bytes::BytesMut;
use bytes::BigEndian;
use bytes::ByteOrder;


error_chain!{
    types {
        ConnectPacketError, ErrorKind, ResultExt, ConnectPacketResult;
    }

    errors{
        ConnectFlagsDecodeError(r: String)
        WillMessageError
        ConnectPayloadError(r: String)
    }

    links {
        FixedHeaderError(::packet::FixedHeaderError, ::packet::ErrorKind);
    }
}

#[derive(Debug)]
struct ProtocolName(String);

#[derive(Debug)]
struct ProtocolLevel(u8);

#[derive(Debug, Clone, Copy)]
struct ConnectFlags {
    user_name_flag: bool,
    password_flag: bool,
    will_retain: bool,
    will_QoS: u8,
    will_flag: bool,
    clean_session: bool,
    reserved: bool,
}

impl ConnectFlags{
    fn new() -> ConnectFlags{
        ConnectFlags{
            user_name_flag: false,
            password_flag: false,
            will_retain: false,
            will_QoS: 0,
            will_flag: false,
            clean_session: false,
            reserved: false,
        }
    }
}


impl<'a> Decodable<'a> for ConnectFlags {
    type Error = ConnectPacketError;
    type Cond = ();
    fn decode_with(bytes: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let len = bytes.len();
        if len >= 1 {
            let byte = bytes[0];
            bytes.split_to(1);

            let user_name_flag = if byte >> 7 & 0x01 == 0x01 {
                    true
                }else {
                    false
            };
            let password_flag = 
                if byte >> 6 & 0x01 == 0x01 {
                    true
                } else {
                    false
            };
            let will_retain = 
                if byte >> 5 & 0x01 == 0x01 {
                    true
                } else{
                    false
            };
            let will_QoS = {
                byte >> 3 & 0x03
            };
            let will_flag = 
                if byte >> 2 & 0x01 == 0x01 {
                    true
                }else{
                    false
            };
            let clean_session = 
                if byte >> 1 & 0x01 == 0x01 {
                    true
                } else {
                    false
            };
            let reserved = 
                if byte & 0x01 == 0x01 {
                    true
                }else {
                    false
            };

            let connect_flags = ConnectFlags {
                    user_name_flag: user_name_flag,
                    password_flag: password_flag,
                    will_flag: will_flag,
                    will_retain: will_retain,
                    will_QoS: will_QoS,
                    clean_session: clean_session,
                    reserved: reserved,
            };
            
            Ok(connect_flags)
        }else {
            error!("connect flag not enough code to decode");
            Err(ErrorKind::ConnectFlagsDecodeError("connect flag not enough code to decode".into()).into())
        }
    }
} 


impl Encodable for ConnectFlags{
    type Error = ConnectPacketError;
    type Cond = ();
    fn encode_with(&self, _cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        let mut connect_flag = 0u8;
        if self.user_name_flag {
            connect_flag |= 0x01;
        };
        connect_flag = connect_flag << 1;
        if self.password_flag {
            connect_flag |= 0x01;
        };
        connect_flag = connect_flag << 1;
        if self.will_retain {
            connect_flag |= 0x01;
        };
        connect_flag = connect_flag << 2;
        connect_flag |= self.will_QoS;
        connect_flag = connect_flag << 1;
        if self.will_flag {
            connect_flag |= 0x01;
        };
        connect_flag = connect_flag << 1;
        if self.clean_session {
            connect_flag |= 0x01;
        };
        connect_flag = connect_flag << 1;
        if self.reserved {
            connect_flag |= 0x01;
        };
        Ok(vec![connect_flag])
    }

    fn encode_length(&self) -> Result<u32, ConnectPacketError> {
        Ok(1)    
    }
}

#[derive(Debug)]
struct KeepAlive(u16);

#[derive(Debug)]
struct ConnectFixedHeader{
    packet_type: u8,
    reserved: u8,
    remaining_length: u32,
}


impl FixedHeader for ConnectFixedHeader{
    fn new() -> Self{
        ConnectFixedHeader{
            packet_type: 1,
            reserved: 0,
            remaining_length: 0,
        }
    }
     
    fn set_remaining_length(&mut self, len: u32) {
        self.remaining_length = len;
    }
}

impl<'a> Decodable<'a> for ConnectFixedHeader{
    type Error = ConnectPacketError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        match Self::get_fixheader(byte) {
            Ok((packet_type, reserved, remaining_length, n)) => {
                byte.split_to(2 + n);
                Ok(ConnectFixedHeader{
                    packet_type: packet_type,
                    reserved: reserved,
                    remaining_length: remaining_length,
                })
            }
            Err(err) => {
                Err(From::from(err))
            }
        } 
    }
}

impl Encodable for ConnectFixedHeader {
    type Error = ConnectPacketError;
    type Cond = ();
    
    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        Self::encode_fixedheader(self.packet_type, self.reserved, self.remaining_length).map_err(From::from)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        Self::get_remaining_length_bytes(self.remaining_length).map_err(From::from)
    } 
}


#[derive(Debug)]
struct Connect{
    fix_header: ConnectFixedHeader,
    protocol_name: ProtocolName,
    protocol_level: ProtocolLevel,
    connect_flags: ConnectFlags,
    keep_alive: KeepAlive,
    payload: ConnectPayload,
}

impl Connect {
    fn with_level<P, C>(protocol_name: P, client_identifier: C, level: u8) -> Connect 
        where P: Into<String>,
              C: Into<String>
    {
        let mut connect = Connect{
            fix_header: ConnectFixedHeader::new(),
            protocol_name: ProtocolName(protocol_name.into()),
            protocol_level: ProtocolLevel(level),
            connect_flags: ConnectFlags::new(),
            keep_alive: KeepAlive(0),
            payload: ConnectPayload::new(client_identifier.into()),
        };
        connect.calculate_remaining_length();

        connect
    }

    fn calculate_remaining_length(&mut self) -> Result<(), ConnectPacketError> {
        let remaining_length = self.protocol_name.0.encode_length().chain_err(||"encode protocol name length error")? 
                        + self.protocol_level.0.encode_length().chain_err(||"encode protocol level length error")?
                        + self.connect_flags.encode_length().chain_err(||"encode connect flags length error")?
                        + self.keep_alive.0.encode_length().chain_err(||"encode keep alive length error")?
                        + self.payload.encode_length().chain_err(||"encode payload length error")?;
        self.fix_header.remaining_length = remaining_length;
        Ok(())
    }

    fn set_will(&mut self, will: Option<(String, Vec<u8>)>) -> Result<(), ConnectPacketError>{
        self.connect_flags.will_flag = will.is_some();
        
        match will {
            Some((topic_name, message)) => {
                self.payload.will_topic = Some(topic_name);
                self.payload.will_message = Some(VecBytes(message));
            },
            None => {
                self.payload.will_topic = None;
                self.payload.will_message = None;
            }
        }
        self.calculate_remaining_length()
    }

    fn set_user_name(&mut self, user_name: Option<String>) -> Result<(), ConnectPacketError>{
        self.connect_flags.user_name_flag = user_name.is_some();
        self.payload.user_name = user_name;        
        self.calculate_remaining_length()
    }

    fn set_password(&mut self, password: Option<String>) -> Result<(), ConnectPacketError> {
        self.connect_flags.password_flag = password.is_some();
        self.payload.password = password;
        self.calculate_remaining_length()
    }

    fn set_clean_session(&mut self, clean_session: bool) {
        self.connect_flags.clean_session = clean_session;
    }

    fn set_will_retain(&mut self, will_retain: bool) {
        self.connect_flags.will_retain = will_retain;
    }

    fn set_will_qos(&mut self, will_qos: u8){
        assert!(will_qos <= 2);
        self.connect_flags.will_QoS = will_qos;
    }
}

impl<'a> Decodable<'a> for Connect{
    type Error = ConnectPacketError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        //byte is fixable length according remaining length
        let fix_header = Decodable::decode(byte)?;
        let protocol_name = ProtocolName(Decodable::decode(byte).chain_err(||"decode protocol name fail")?);
        let protocol_level = ProtocolLevel(Decodable::decode(byte).chain_err(||"decode protocol level fail")?);
        let connect_flags = Decodable::decode(byte)?;
        let keep_alive = KeepAlive(Decodable::decode(byte).chain_err(||"decode keep alive fail")?);
        let payload = Decodable::decode_with(byte, Some(connect_flags).as_ref())?;

        

        let connect = Connect{
            fix_header: fix_header,
            protocol_name: protocol_name,
            protocol_level: protocol_level,
            connect_flags: connect_flags,
            keep_alive: keep_alive,
            payload: payload,
        };

        Ok(connect)
    }
}

impl Encodable for Connect {
    type Error = ConnectPacketError;
    type Cond = ();

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        let mut result = vec![];
        
        let fix_header = self.fix_header.encode().chain_err(|| "encode fix header fail")?;
        let protocol_name = self.protocol_name.0.encode().chain_err(|| "encode protocol name fail")?;
        let protocol_level = self.protocol_level.0.encode().chain_err(|| "encode protocol level fail")?;
        let connect_flag = self.connect_flags.encode().chain_err(|| "encode connect flag fail")?;
        let keep_alive = self.keep_alive.0.encode().chain_err(|| "encode keep alive fail")?;
        let payload = self.payload.encode_with(Some(self.connect_flags)).chain_err(|| "encode payload fail")?;

        result.extend(fix_header);
        result.extend(protocol_name);
        result.extend(protocol_level);
        result.extend(connect_flag);
        result.extend(keep_alive);
        result.extend(payload);

        Ok(result)
    }

    fn encode_length(&self) -> Result<u32, ConnectPacketError> {
        let mut length = self.fix_header.encode_length().chain_err(||"encode fix header length error")?;
        length += self.protocol_name.0.encode_length().chain_err(||"encode protocol name length error")?;
        length += self.protocol_level.0.encode_length().chain_err(||"encode protocol level length error")?;
        length += self.connect_flags.encode_length().chain_err(||"encode connect flags length error")?;
        length += self.keep_alive.0.encode_length().chain_err(||"encode keep alive length error")?;
        length += self.payload.encode_length().chain_err(||"encode payload length error")?;

        Ok(length)
    }
}

#[derive(Debug)]
struct ConnectPayload{
    client_identifier: String,
    will_topic: Option<String>,
    will_message: Option<VecBytes>,
    user_name: Option<String>,
    password: Option<String>,
}

impl ConnectPayload {
    fn new(client_identifier: String) -> ConnectPayload {
        ConnectPayload {
            client_identifier: client_identifier,
            will_topic: None,
            will_message: None,
            user_name: None,
            password: None,
        }
    }
}

impl<'a> Decodable<'a> for ConnectPayload{
    type Error = ConnectPacketError;
    type Cond = &'a ConnectFlags;

    fn decode_with(byte: &mut BytesMut, connect_flags: Option<Self::Cond>) -> Result<Self, Self::Error>{
        let client_identifier = Decodable::decode(byte).chain_err(|| ErrorKind::ConnectPayloadError("decode clien identifier error".into()))?;

        if let Some(connect_flag) = connect_flags{
            let will_topic = if connect_flag.will_flag {
                Some(Decodable::decode(byte).chain_err(|| ErrorKind::ConnectPayloadError("decode will topic error".into()))?)
            }else{
                None
            }; 
            let will_message = if connect_flag.will_flag {
                Some(Decodable::decode(byte).chain_err(|| ErrorKind::ConnectPayloadError("decode will message error".into()))?)
            }else{
                None
            };
            let user_name = if connect_flag.user_name_flag {
                Some(Decodable::decode(byte).chain_err(|| ErrorKind::ConnectPayloadError("decode user name error".into()))?)
            }else{
                None
            };
            let password = if connect_flag.password_flag {
                Some(Decodable::decode(byte).chain_err(|| ErrorKind::ConnectPayloadError("decode password error".into()))?)
            }else{
                None
            };

            Ok(ConnectPayload{
                client_identifier: client_identifier,
                will_topic: will_topic,
                will_message: will_message,
                user_name: user_name,
                password: password,
            })
        }else {
            // error!("connect payload is not encode code to decode");
            // Err(ConnectPacketError::NoEnoughBytesToDecode)
            bail!(ErrorKind::ConnectPayloadError("decode connect payload fail ".into()))
        } 
    }
}

impl Encodable for ConnectPayload{
    type Error = ConnectPacketError;
    type Cond = ConnectFlags;

    fn encode_with(&self, cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error>{
        let mut vec = vec![];
        match cond {
            Some(connect_flag) => {
                vec.extend(self.client_identifier.encode().chain_err(||"encode client identifier is fail")?);
                if connect_flag.will_flag {
                    //TODO eles return connectflag and content unmatchable error
                    if let Some(ref topic) = self.will_topic {
                        vec.extend(topic.encode().chain_err(||"encode will topic is fail")?);        
                    };

                    if let Some(ref message) = self.will_message{
                        vec.extend(message.encode().chain_err(||"encode will message is fail")?);
                    };
                };

                if connect_flag.user_name_flag {
                    if let Some(ref user_name) = self.user_name {
                        vec.extend(user_name.encode().chain_err(||"encode username is fail")?);
                    };
                };

                if connect_flag.password_flag {
                    if let Some(ref password) = self.password {
                        vec.extend(password.encode().chain_err(||"encode password is fail")?);
                    };
                };
                Ok(vec)
            },
            _ => {
                error!("connect payload encoding payload is none");
                // Err(ConnectPacketError::InvalidEncode)
                bail!(ErrorKind::ConnectPayloadError("connect payload parm is none".into()))
            }
        }
    }

    fn encode_length(&self) -> Result<u32, ConnectPacketError> {
        let mut length = self.client_identifier.encode_length().chain_err(|| "client identifier encode lenght fail")?;
        if let Some(ref will_topic) = self.will_topic {
            length += will_topic.encode_length().chain_err(|| "will topic encode lenght fail")?;
        }
        if let Some(ref will_message) = self.will_message{
            length += will_message.encode_length().chain_err(|| "will message encode lenght fail")?;
        }
        if let Some(ref user_name) = self.user_name {
            length += user_name.encode_length().chain_err(|| "username encode lenght fail")?;
        }
        if let Some(ref password) = self.password {
            length += password.encode_length().chain_err(|| "password enncode lenght fail")?;
        }
        Ok(length)
    }
}

#[derive(Debug)]
struct VecBytes(Vec<u8>);

impl<'a> Decodable<'a> for VecBytes{
    type Error = ConnectPacketError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        let len = byte.len();
        let mut size = 0u16;
        if len >= 2 {
            size = BigEndian::read_u16(byte);
        }else {
            error!("will message header is not enough code to decode");
            bail!(ErrorKind::WillMessageError);
        }

        let split_len = (size + 2 ) as usize;
        if len >= split_len {
            let result = VecBytes(byte[2..split_len].to_vec());
            byte.split_to(split_len);

            Ok(result)
        }else {
            error!("will message header is not enough code to decode");
            // Err( ConnectPacketError::NoEnoughBytesToDecode )
            bail!(ErrorKind::WillMessageError);
        }
    }
} 

impl Encodable for VecBytes {
    type Error = ConnectPacketError;
    type Cond = ();
    fn encode_with(&self, _cond: Option<Self::Cond>) -> Result<Vec<u8>, Self::Error> {
        let mut result = vec![0u8; 2];
        BigEndian::write_u16(&mut result, self.0.len() as u16);
        result.extend(self.0.iter().cloned());
        Ok(result)
    }

    fn encode_length(&self) -> Result<u32, Self::Error> {
        Ok( 2 + ( self.0.len() as u32 ) )
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    extern crate env_logger;

    struct Enjie{
        a: i32,
        b: i32,
    }

    #[test]
    fn test_connect_flag(){
        let vec = vec![0x13];
        let mut bytes = BytesMut::from(vec);
        let connect_flag = ConnectFlags::decode(&mut bytes);
        // println!("{:?}", connect_flag);
    }

    #[test]
    fn test_vecbytes(){
        let vec = vec![0x00, 0x02, 0x13, 0x32, 0x33];
        let mut bytes = BytesMut::from(vec);
        let vec_bytes = VecBytes::decode(&mut bytes);
        // println!("{:?}", vec_bytes); 
    }

    #[test]
    fn test_encode_vecbytes(){
        let vec = vec![0x00, 0x02, 0x13, 0x32, 0x33];
        let param = VecBytes(vec);
        // println!("{:?}", param.encode()); 
    }


    #[test]
    fn test_connect_packet(){
        let vec = vec![];
        let mut bytes = BytesMut::from(vec);
        let packet = Connect::decode(&mut bytes);
        //println!("{:?}", packet);
    }

    #[test]
    fn test_encode_connectflags() {
        let connect_flag = ConnectFlags{
            user_name_flag: true,
            password_flag: false,
            will_retain: true,
            will_QoS: 3u8,
            will_flag: true,
            clean_session: false,
            reserved: true,
        };
        // println!("{:?}", connect_flag.encode());
    }

    #[test]
    fn test_encode_connect_packet(){
        let _ = env_logger::init().unwrap();
        let packet = Connect::with_level("MQTT", "123", 4);

        let vec = packet.encode().unwrap();
        let mut bytes = BytesMut::from(vec);
        println!("{:?}", Connect::decode(&mut bytes));
        //match Connect::decode(&mut bytes) {
            //Ok(result) => println!("{:?}", result),
            //Err(err) => println!("{:?}", err)
        //}
    }

}


