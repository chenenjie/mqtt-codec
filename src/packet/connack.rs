struct Connack;

use super::FixedHeader;

struct ConnackFixedHeader {
    packet_type: u8,
    reserveed: u8,
    remaining_length: u32,
}


struct Connack {
    fixed_header: ConnackFixedHeader,
}



