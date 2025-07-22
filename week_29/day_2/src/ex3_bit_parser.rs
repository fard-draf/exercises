#[derive(Debug, PartialEq)]
pub struct PacketHeader {
    pub version: u8,
    pub length: u8,
    pub is_urgent: bool,
    pub packet_type: u8,
}

enum Mask {
    Protocole = 0xF,
    Payload = 0x0FF,
    UrgentFlag = 0b0001,
    PaquetType = 0b111,
}

enum Shift {
    Protocole = 0,
    Payload = 4,
    UrgentFlag = 12,
    PaquetType = 13
}

pub fn parse_header(header: u16) -> PacketHeader {

    let version = ((header >> Shift::Protocole as u8) & Mask::Protocole as u16) as u8;
    let length = ((header >> Shift::Payload as u8) & Mask::Payload as u16) as u8;
    let is_urgent = ((header >> Shift::UrgentFlag as u8) & Mask::UrgentFlag as u16);
    let packet_type = ((header >> Shift::PaquetType as u8) & Mask::PaquetType as u16) as u8;


    PacketHeader { version, length, is_urgent: is_urgent == 1, packet_type } 

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_complex_header() {

        let header_data: u16 = 0b101_1_11001001_0110;

        let expected = PacketHeader {
            version: 6,
            length: 201,
            is_urgent: true,
            packet_type: 5,
        };

        let result = parse_header(header_data);
        assert_eq!(result, expected);
    }
}