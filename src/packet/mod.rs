use bytes::BytesMut;

trait FixedHeader {
    fn decode(bytes: &mut BytesMut) -> Result<(u8, u8, u32), FixedHeaderError> {
        let len = bytes.len();
        let mut control_packet_type = 0u8;
        let mut reserved_code = 0u8;
        if len < 2 {
            control_packet_type = bytes[0] >> 4;
            reserved_code =  bytes[0] & 0x0f 
        } else {
            return Err(FixedHeaderError::NoEnoughBytes)
        }

        let mut n = 1;
        while n < 5 {
            let mut sum = 0u32;
            if bytes[n] / 128 > 0 {
                sum |= 128;
                n += 1;
                if n + 1 > len {
                    return Err(FixedHeaderError::NoEnoughBytes) 
                }
            }else {
                return Ok((control_packet_type, reserved_code, sum))
            }
        }
        Err(FixedHeaderError::RemainLengthAvailable)

    }
}

pub enum FixedHeaderError {
    NoEnoughBytes,
    RemainLengthAvailable,
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fixheader() {
        let vec = vec![240u8, 120u8];
        let b = BytesMut::from(vec);

        println!("{:?}",FixedHeader::decode(b));
        // println!("{:?}", b[0]);
        // println!("{:?}", b[1]);

        // println!("{:?}", b.len());
    }
}