#![no_std]

use thiserror::Error;

//==================================================================================ERRORS
#[derive(Debug, Error)]
pub enum ParseErr {
    #[error("Unvalid Entry")]
    InvalidEntry,
    #[error("Unable to parse")]
    UnableToParse,
    #[error("Checksum Error")]
    ChecksumError,
    #[error("Command Error")]
    CommandError,
    #[error("Empty Buffer")]
    EmptyBuffer,
}
//==================================================================================STRUCTURE

#[derive(Debug, Default)]
pub struct ParsedLine<'a> {
    pub command: &'a str,
    pub payload: Option<&'a str>,
    pub checksum: &'a str,
}
pub enum Categories {
    Commands,
    Payload,
    Checksum,
}
//==================================================================================PARSER
#[derive(Debug, PartialEq, Eq)]
pub struct ParserLine<'a> {
    buffer: &'a str,
    //raw_income "CMD:<NOM_CMD>;PAYLOAD:<DATA>;CHECKSUM:<HEX>\n"
}

impl<'a> ParserLine<'a> {
    pub fn parse_line(&mut self) -> Result<ParsedLine, ParseErr> {
        if self.buffer.is_empty() {
            return Err(ParseErr::EmptyBuffer);
        }

        let mut parsed_data = ParsedLine::default();
        while let Some((line, rest)) = self.buffer.split_once('\n') {
            self.buffer = rest;
            let mut temp_buf = line;
            while let Some((line, rest)) = temp_buf.split_once(';') {
                temp_buf = rest;
                self.match_lines(parse_line(line)?, &mut parsed_data);
            }
            if let None = temp_buf.split_once(';') {
                self.match_lines(parse_line(temp_buf)?, &mut parsed_data);
            }
        }

        Ok(parsed_data)
    }

    fn match_lines(&self, data_packed: (&'a str, Categories), parsed_data: &mut ParsedLine<'a>) {
        match data_packed.1 {
            Categories::Commands => parsed_data.command = data_packed.0,
            Categories::Payload => parsed_data.payload = Some(data_packed.0),
            Categories::Checksum => parsed_data.checksum = data_packed.0,
        }
    }
}

fn parse_line<'a>(line: &'a str) -> Result<(&'a str, Categories), ParseErr> {
    if line.is_empty() {
        return Err(ParseErr::InvalidEntry);
    }
    if let Some((first, rest)) = line.split_once(':') {
        match first {
            cmd if cmd.starts_with("CMD") => return Ok((rest, Categories::Commands)),
            pyld if pyld.starts_with("PAYLOAD") => return Ok((rest, Categories::Payload)),
            chcksm if chcksm.starts_with("CHECKSUM") => return Ok((rest, Categories::Checksum)),
            _ => return Err(ParseErr::UnableToParse),
        }
    } else {
        return Err(ParseErr::UnableToParse);
    }
}
//==================================================================================MAIN
fn main() -> Result<(), ParseErr> {
    Ok(())
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn test_valid_entry() {
        let mut parser = ParserLine {
            buffer: "CMD:SET_THRUST;PAYLOAD:FWD,100;CHECKSUM:A3\n",
        };
        assert!(parser.parse_line().is_ok());
    }

    #[test]
    fn test_unvalid_entry() {
        let mut parser = ParserLine {
            buffer: "CMD:SET_THRUST;asadad:PAYLOAD:FWD,100;CHECKSUM:A3\n",
        };
        assert!(parser.parse_line().is_err());
    }
}
