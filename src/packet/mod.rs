use bytes::BytesMut;
use std::fmt;

mod connect;

pub trait FixedHeader {
    fn get_fixheader(bytes: &mut BytesMut) -> Result<(u8, u8, u32, usize), FixedHeaderError> {
        let len = bytes.len();
        let mut control_packet_type = 0u8;
        let mut reserved_code = 0u8;
        if len >= 2 {
            control_packet_type = bytes[0] >> 4;
            reserved_code = bytes[0] & 0x0f
        } else {
            return Err(FixedHeaderError::NoEnoughBytes);
        }

        let mut n = 1;
        let mut sum = 0u32;
        while n < 5 {
            let a = bytes[n];
            let k = (((a & 0x7f) as u32) << (7 * (n - 1))) as u32;
            sum = sum | k;
            if a / 128 > 0 {
                n += 1;
                if n + 1 > len {
                    return Err(FixedHeaderError::NoEnoughBytes);
                }
            } else {
                return Ok((control_packet_type, reserved_code, sum, n));
            }
        }
        Err(FixedHeaderError::RemainLengthAvailable)
    }

    fn encode_fixedheader(packet_type: u8, reserved: u8, remaining_length: u32) -> Result<Vec<u8>, FixedHeaderError> {
        let mut vec = vec![];
        let first_u8 = (packet_type << 4) | reserved;
        vec.push(first_u8);

        if remaining_length >= 0 && remaining_length <= 127 {
            vec.push(remaining_length as u8);
        }else if remaining_length >= 128 && remaining_length <= 16_383 {
            vec.push((remaining_length & 0x7f) as u8);
            vec.push(( remaining_length >> 7 | 0x80 ) as u8);
        }else if remaining_length >= 16_384 && remaining_length <= 2_097_151 {
            vec.push(( remaining_length & 0x7f ) as u8);
            vec.push(( remaining_length >> 7 & 0x7f ) as u8);
            vec.push(( remaining_length >> 14 | 0x80 ) as u8);
        }else if remaining_length >= 2_097_152 && remaining_length <= 268_435_455 {
            vec.push(( remaining_length & 0x7f ) as u8);
            vec.push(( remaining_length >> 7 & 0x7f ) as u8);
            vec.push(( remaining_length >> 14 & 0x7f ) as u8);
            vec.push(( remaining_length >> 21 | 0x80 ) as u8);
        }else{
            return Err(FixedHeaderError::RemainLengthAvailable);
        }
        Ok(vec)
        
    }
}

pub enum FixedHeaderError {
    NoEnoughBytes,
    RemainLengthAvailable,
}

impl fmt::Debug for FixedHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FixedHeaderError::NoEnoughBytes => write!(f, "No EnougnBytes"),
            &FixedHeaderError::RemainLengthAvailable => write!(f, "Remailength Avaialable"),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fixheader() {
        struct Enjie;
        impl FixedHeader for Enjie {}

        // let enjie = Enjie;

        // let vec = vec![241u8, 120u8];
        // let mut b = BytesMut::from(vec);


        // println!("{:?}", enjie.decode(&mut b));

        let vec = vec![241u8, 0xFF, 0xFF, 0xFF, 0x8f];
        let mut b = BytesMut::from(vec);
        //let result = Enjie::decode(&mut b);
        //println!("{:?}", result);
        // println!("{:?}", b[0]);
        //println!("{}", b.len());
    }

    //#[test]
    fn test_plus() {
        let a = 128 | 1;
        println!("{}", a);
    }
}
