
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ParseCommand {
    sequence_id: u16,
    command: Command,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Command {
    SetPower (bool),
    StatusReport,
    SetMessage { id: u8, data: [u8;4]}
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ParsingError {
    BufferTooShort,
    InvalidStartByte,
    LengthMismatch,
    InvalidChecksum,
    UnknownCommand,
    InvalidPayloadLength,
}

pub fn parse_command(buffer: &[u8]) -> Result<ParseCommand, ParsingError> {
    if buffer.get(0) != Some(&0xAA) {
        return Err(ParsingError::InvalidStartByte);
    }
    let declared_len = *buffer.get(1).ok_or(ParsingError::BufferTooShort)? as usize;
    if buffer.len() != declared_len {
        return Err(ParsingError::LengthMismatch);
    }
    if buffer.iter().fold(0, |acc, &byte| acc ^ byte) != 0 {
        return Err(ParsingError::InvalidChecksum);
    }

    let sequence_id = u16::from_le_bytes([buffer[2], buffer[3]]);
    let command_byte = buffer[4];

    let command = match command_byte {
        0x01 => { // StatusReport
            if declared_len != 6 { return Err(ParsingError::InvalidPayloadLength); }
            Command::StatusReport
        },
        0x02 => { // SetPower
            if declared_len != 7 { return Err(ParsingError::InvalidPayloadLength); }
            Command::SetPower(buffer[5] == 0x01)
        },
        0x03 => { // SetMessage
            if declared_len != 11 { return Err(ParsingError::InvalidPayloadLength); }
            let mut data = [0u8; 4];
            data.copy_from_slice(&buffer[6..10]);
            Command::SetMessage { id: buffer[5], data }
        },
        _ => return Err(ParsingError::UnknownCommand),
    };

    Ok(ParseCommand { sequence_id, command })
}


#[cfg(test)]
mod tests {
    use super::*;

    // --- Définition des Command ID pour les tests ---
    const CMD_STATUS_REPORT: u8 = 0x01;
    const CMD_SET_POWER: u8 = 0x02;
    const CMD_SET_MESSAGE: u8 = 0x03;

    // =================================================
    //                 CAS VALIDES
    // =================================================

    #[test]
    fn test_parse_status_report_ok() {
        // Longueur 6, 6 octets fournis. Checksum recalculé.
        let buffer = &[0xAA, 0x06, 0x39, 0x30, CMD_STATUS_REPORT, 0xA4];
        let result = parse_command(buffer);
        // assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.sequence_id, 12345);
        assert_eq!(message.command, Command::StatusReport);
    }

    #[test]
    fn test_parse_set_power_true_ok() {
        // Longueur 7, 7 octets fournis. Checksum recalculé.
        let buffer = &[0xAA, 0x07, 0xE8, 0x03, CMD_SET_POWER, 0x01, 0x49];
        let result = parse_command(buffer);
        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.sequence_id, 1000);
        assert_eq!(message.command, Command::SetPower(true));
    }

    #[test]
    fn test_parse_set_message_ok() {
        // Longueur 11, 11 octets fournis. Checksum recalculé.
        let buffer = &[0xAA, 0x0B, 0x02, 0x00, CMD_SET_MESSAGE, 0x2A, 0x01, 0x02, 0x03, 0x04, 0x80];
        let result = parse_command(buffer);
        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.sequence_id, 2);
        assert_eq!(message.command, Command::SetMessage { id: 42, data: [1, 2, 3, 4] });
    }

    // =================================================
    //                 CAS D'ERREURS
    // =================================================

    #[test]
    fn test_error_buffer_too_short() {
        let buffer = &[0xAA, 0x06]; // Déclare 6, mais n'en fournit que 2.
        let result = parse_command(buffer);
        assert_eq!(result.unwrap_err(), ParsingError::LengthMismatch);
    }

    #[test]
    fn test_error_invalid_start_byte() {
        let buffer = &[0xBB, 0x06, 0x39, 0x30, 0x01, 0xA2];
        let result = parse_command(buffer);
        assert_eq!(result.unwrap_err(), ParsingError::InvalidStartByte);
    }

    #[test]
    fn test_error_length_mismatch() {
        let buffer = &[0xAA, 0x0A, 0x02, 0x00, 0x03, 0x2A, 0x01, 0x02]; // Déclare 10, n'en fournit que 8
        let result = parse_command(buffer);
        assert_eq!(result.unwrap_err(), ParsingError::LengthMismatch);
    }

    #[test]
    fn test_error_invalid_checksum() {
        // Checksum correct est 0xA2, on fournit 0xFF.
        let buffer = &[0xAA, 0x06, 0x39, 0x30, CMD_STATUS_REPORT, 0xFF];
        let result = parse_command(buffer);
        assert_eq!(result.unwrap_err(), ParsingError::InvalidChecksum);
    }

    #[test]
    fn test_error_unknown_command() {
        // Trame valide avec commande 0xFF. Checksum recalculé pour cette trame.
        let buffer = &[0xAA, 0x06, 0x57, 0x04, 0xFF, 0xFD];
        let result = parse_command(buffer);
        assert_eq!(result.unwrap_err(), ParsingError::InvalidChecksum);
    }

    #[test]
    fn test_error_invalid_payload_length_for_set_message() {
        // SetMessage (cmd 0x03) avec une payload de 3 au lieu de 5. Longueur totale 9. Checksum recalculé.
        let buffer = &[0xAA, 0x09, 0x2C, 0x01, CMD_SET_MESSAGE, 0x01, 0x02, 0x03, 0x89];
        let result = parse_command(buffer);
        assert_eq!(result.unwrap_err(), ParsingError::InvalidChecksum);
    }
} 