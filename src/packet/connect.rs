use super::super::Decodable;
use super::super::PacketError;
use super::FixedHeader;
use bytes::BytesMut;
use bytes::BigEndian;
use bytes::ByteOrder;

struct ProtocolName(String);

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


impl<'a> Decodable<'a> for ConnectFlags {
    type Error = PacketError;
    type Cond = ();
    fn decode_with(bytes: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let len = bytes.len();
        if len >= 1 {
            let byte = bytes[0];
            bytes.split_to(1);

            let user_name_flag = {
                if byte >> 7 & 0x01 == 0x01 {
                    true
                }else {
                    false
                }
            };
            let password_flag = {
                if byte >> 6 & 0x01 == 0x01 {
                    true
                } else {
                    false
                }
            };
            let will_retain = {
                if byte >> 5 & 0x01 == 0x01 {
                    true
                } else{
                    false
                }
            };
            let will_QoS = {
                byte >> 3 & 0x03
            };
            let will_flag = {
                if byte >> 2 & 0x01 == 0x01 {
                    true
                }else{
                    false
                }
            };
            let clean_session = {
                if byte >> 1 & 0x01 == 0x01 {
                    true
                } else {
                    false
                }
            };
            let reserved = {
                if byte & 0x01 == 0x01 {
                    true
                }else {
                    false
                }
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
            
            println!("{:?}",connect_flags.will_flag);
            
            Ok(connect_flags)
        }else {
            Err(PacketError::NoEnoughBytesToDecode)
        }
    }
} 

#[derive(Debug)]
struct KeepAlive(u16);

struct ConnectFixedHeader{
    packet_type: u8,
    reserved: u8,
    remaining_length: u32,
}

impl FixedHeader for ConnectFixedHeader{}

impl<'a, T: FixedHeader> Decodable<'a> for ConnectFixedHeader{
    type Error = PacketError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        match Self::get_fixheader() {
            Ok((packet_type, reserved, remaining_length, n)) => {
                byte.split_to(2 + n);
                ConnectFixHeader{
                    packet_type: packet_type,
                    reserved: reserved,
                    remaining_length: remaining_length,
                }
            }
            Err(err) => {
                err.into()
            }
        } 
    }
}

#[derive(Debug)]
struct Connect{
    fix_header: ConnectFixHeader,
    protocol_name: ProtocolName,
    protocol_level: ProtocolLevel,
    connect_flags: ConnectFlags,
    keep_alive: KeepAlive,
    payload: ConnectPayload,
}


impl<'a> Decodable<'a> for Connect{
    type Error = PacketError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, _decode_size: Option<Self::Cond>) -> Result<Self, Self::Error> {
        let protocol_name = ProtocolName(Decodable::decode(byte)?);
        let protocol_level = ProtocolLevel(Decodable::decode(byte)?);
        let connect_flags = Decodable::decode(byte)?;
        let keep_alive = KeepAlive(Decodable::decode(byte)?);
        let payload = Decodable::decode_with(byte, Some(connect_flags).as_ref())?;


        let connect = Connect{
            protocol_name: protocol_name,
            protocol_level: protocol_level,
            connect_flags: connect_flags,
            keep_alive: keep_alive,
            payload: payload,
        };

        Ok(connect)
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

impl<'a> Decodable<'a> for ConnectPayload{
    type Error = PacketError;
    type Cond = &'a ConnectFlags;

    fn decode_with(byte: &mut BytesMut, connect_flags: Option<Self::Cond>) -> Result<Self, Self::Error>{
        let client_identifier = Decodable::decode(byte)?;

        if let Some(connect_flag) = connect_flags{
            let will_topic = if connect_flag.will_flag {
                Some(Decodable::decode(byte)?)
            }else{
                None
            }; 
            let will_message = if connect_flag.will_flag {
                Some(Decodable::decode(byte)?)
            }else{
                None
            };
            let user_name = if connect_flag.user_name_flag {
                Some(Decodable::decode(byte)?)
            }else{
                None
            };
            let password = if connect_flag.password_flag {
                Some(Decodable::decode(byte)?)
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
            Err(PacketError::NoEnoughBytesToDecode)
        } 
    }
}

#[derive(Debug)]
struct VecBytes(Vec<u8>);

impl<'a> Decodable<'a> for VecBytes{
    type Error = PacketError;
    type Cond = ();

    fn decode_with(byte: &mut BytesMut, decode_size: Option<Self::Cond>) -> Result<Self, Self::Error>{
        let len = byte.len();
        let mut size = 0u16;
        if len >= 2 {
            size = BigEndian::read_u16(byte);
        }else {
            return Err(PacketError::NoEnoughBytesToDecode);
        }

        let split_len = (size + 2 ) as usize;
        if len >= split_len {
            let result = VecBytes(byte[2..split_len].to_vec());
            byte.split_to(split_len);

            Ok(result)
        }else {
            Err( PacketError::NoEnoughBytesToDecode )
        }
    }
} 



#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_connect_packet(){
        let vec = vec![];
        let mut bytes = BytesMut::from(vec);
        let packet = Connect::decode(bytes);
    }

}


