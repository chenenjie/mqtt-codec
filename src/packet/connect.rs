use super::super::Decodable;
use super::super::PacketError;
use bytes::BytesMut;

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

impl Decodable for ConnectFlags {
    type Error = PacketError;
    fn decode_with(bytes: &mut BytesMut, decode_size: Option<usize>) -> Result<Self, Self::Error> {
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
            
            Ok(connect_flags)
        }else {
            Err(PacketError::NoEnoughBytesToDecode)
        }
    }
} 

struct KeepAlive(u16);

struct Connect{
    protocol_name: ProtocolName,
    protocol_level: ProtocolLevel,
    connect_flags: ConnectFlags,
    keep_alive: KeepAlive,
}


impl Decodable for Connect{
    type Error = PacketError;

    fn decode_with(byte: &mut BytesMut, _decode_size: Option<usize>) -> Result<Self, Self::Error> {
        let protocol_name = ProtocolName(Decodable::decode(byte)?);
        let protocol_level = ProtocolLevel(Decodable::decode(byte)?);
        let connect_flags = Decodable::decode(byte)?;
        let keep_alive = KeepAlive(Decodable::decode(byte)?);

        let connect = Connect{
            protocol_name: protocol_name,
            protocol_level: protocol_level,
            connect_flags: connect_flags,
            keep_alive: keep_alive,
        };

        Ok(connect)
    }
}

struct ConnectPayload{
    client_identifier: String,
    will_topic: Option<String>,
    will_message: Option<VecBytes>,
    user_name: Option<String>,
    password: Option<String>,
}

struct VecBytes(Vec<u8>);



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_connect_flag(){
        let vec = vec![0x13];
        let mut bytes = BytesMut::from(vec);
        let connect_flag = ConnectFlags::decode(&mut bytes);
        println!("{:?}", connect_flag);
    }

}


