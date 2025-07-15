use std::convert::{TryFrom, From};

#[derive(Debug, PartialEq, Clone)]
pub struct PacketHeader {
    pub version: u8,
    pub length: u8,
    pub is_urgent: bool,
    pub packet_type: u8,
}

// Erreur customisée pour la conversion
#[derive(Debug)]
pub struct PacketParseError;

enum MASK {
    PROTOCOLE = 0xF,
    PAYLOAD = 0xFF,
    FLAG = 0b0001,
    TYPE = 0b111,
}

enum SHIFT {
    PROTOCOLE = 0,
    PAYLOAD = 4,
    FLAG = 12,
    TYPE = 13
}


// ÉTAPE 1: Implémentez la conversion de u16 VERS PacketHeader
impl TryFrom<u16> for PacketHeader {
    type Error = PacketParseError;

    fn try_from(header: u16) -> Result<Self, Self::Error> {
        // Reprenez votre logique de parsing ici.
        // C'est presque un copier-coller de votre fonction précédente,
        // mais enveloppé dans un `Ok(...)`.
        let version = ((header >> SHIFT::PROTOCOLE as u16) & MASK::PROTOCOLE as u16) as u8;
        let length = ((header >> SHIFT::PAYLOAD as u16) & MASK::PAYLOAD as u16) as u8;
        let is_urgent = ((header >> SHIFT::FLAG as u16) & MASK::FLAG as u16) as u8 == 1;
        let packet_type = ((header >> SHIFT::TYPE as u16) & MASK::TYPE as u16) as u8;

        Ok(
            PacketHeader {
                version,
                length,
                is_urgent,
                packet_type
            }
        )
    }
}

// ÉTAPE 2: Implémentez la conversion de PacketHeader VERS u16
impl From<PacketHeader> for u16 {
    fn from(packet: PacketHeader) -> u16 {
        
        (packet.version as u16) << SHIFT::PROTOCOLE as u16
        | (packet.length as u16) << SHIFT::PAYLOAD as u16
        | (packet.is_urgent as u16) << SHIFT::FLAG as u16     
        | (packet.packet_type as u16) << SHIFT::TYPE as u16
 

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u16_to_packet_header() {
        let header_data: u16 = 0b101_1_11001001_0110;
        let packet = PacketHeader::try_from(header_data).unwrap();

        assert_eq!(packet.version, 6);
        assert_eq!(packet.length, 201);
        assert_eq!(packet.is_urgent, true);
        assert_eq!(packet.packet_type, 5);
    }

    #[test]
    fn test_packet_header_to_u16() {
        let packet = PacketHeader {
            version: 6,
            length: 201,
            is_urgent: true,
            packet_type: 5,
        };
        let expected_header_data: u16 = 0b101_1_11001001_0110;
        let result_data = u16::from(packet);
        
        assert_eq!(result_data, expected_header_data);
    }
    
    #[test]
    fn test_round_trip() {
        // Teste si on peut convertir dans les deux sens et retrouver la donnée d'origine.
        let original_packet = PacketHeader {
            version: 15,
            length: 42,
            is_urgent: false,
            packet_type: 7,
        };

        let header_representation = u16::from(original_packet.clone());
        let reconstructed_packet = PacketHeader::try_from(header_representation).unwrap();

        assert_eq!(original_packet, reconstructed_packet);
    }
}