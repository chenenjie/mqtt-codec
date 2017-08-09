use PacketError;
use bytes::BytesMut;
use Decodable;
use Encodable;

error_chain!{
    types {
        ConnectFlagsError, ErrorKind, ResultExt, ConnectFlagsResult;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConnectFlags {
    pub user_name_flag: bool,
    pub password_flag: bool,
    pub will_retain: bool,
    pub will_QoS: u8,
    pub will_flag: bool,
    pub clean_session: bool,
    pub reserved: bool,
}

impl ConnectFlags{
    pub fn new() -> ConnectFlags{
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
    type Error = ConnectFlagsError;
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
            bail!("connect flag not enough code to decode");
        }
    }
} 


impl Encodable for ConnectFlags{
    type Error = ConnectFlagsError;
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

    fn encode_length(&self) -> Result<u32, ConnectFlagsError> {
        Ok(1)    
    }
}
