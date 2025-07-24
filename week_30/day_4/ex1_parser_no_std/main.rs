use core::cell::RefCell;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VirtualParseError {
    #[error("Unvalid Data")]
    UnvalidData,
    #[error("Unable to parse")]
    UnableToParse,
    #[error("Empty buffer")]
    EmptyBuffer,
}

#[derive(Debug, Default)]
pub struct ParsedSensorData<'a> {
    pub bat: &'a [u8],
    pub hum: &'a [u8],
    pub temp: &'a [u8],
}

#[derive(Debug, PartialEq, Eq)]
pub enum SensorType {
    Battery,
    Humidity,
    Temperature,
}

pub struct VirtualSensorParser<'a> {
    pub buffer: &'a [u8],
    pub counter: RefCell<usize>,
}

impl<'a> VirtualSensorParser<'a> {
    pub fn to_data(&self) -> Result<ParsedSensorData, VirtualParseError> {
        if self.buffer.is_empty() {
            return Err(VirtualParseError::EmptyBuffer);
        }

        let mut search_area = &self.buffer[..];
        let mut parsed_data = ParsedSensorData::default();

        while let Some(relative_pos) = search_area.iter().position(|&e| e == b';') {
            let next_start = relative_pos + 1;
            if next_start >= search_area.len() + 1 {
                break;
            }
            let data_packed = self.parsing_slice(&search_area[..relative_pos])?;

            self.match_packed_data(data_packed, &mut parsed_data);
            search_area = &search_area[next_start..];
            self.counter.replace_with(|&mut old| old + next_start);
        }

        if let None = search_area.iter().position(|&e| e == b';') {
            let data_packed = self.parsing_slice(search_area)?;
            self.match_packed_data(data_packed, &mut parsed_data);
        }

        Ok(parsed_data)
    }

    pub fn match_packed_data(
        &self,
        data_packed: (&'a [u8], SensorType),
        parsed_data: &mut ParsedSensorData<'a>,
    ) {
        match data_packed.1 {
            SensorType::Temperature => {
                parsed_data.temp = data_packed.0;
            }
            SensorType::Humidity => {
                parsed_data.hum = data_packed.0;
            }
            SensorType::Battery => {
                parsed_data.bat = data_packed.0;
            }
        };
    }

    pub fn parsing_slice<'b>(
        &self,
        raw_slice: &'b [u8],
    ) -> Result<(&'b [u8], SensorType), VirtualParseError> {
        if let Some(pos) = raw_slice.iter().position(|e| *e == b':') {
            match raw_slice {
                t if t.starts_with(b"TEMP") => return Ok((&t[pos + 1..], SensorType::Temperature)),
                h if h.starts_with(b"HUM") => return Ok((&h[pos + 1..], SensorType::Humidity)),
                b if b.starts_with(b"BAT") => return Ok((&b[pos + 1..], SensorType::Battery)),
                _ => return Err(VirtualParseError::UnvalidData),
            }
        } else {
            return Err(VirtualParseError::UnableToParse);
        }
    }
}
fn main() -> Result<(), VirtualParseError> {
    let raw_data = b"TEMP:25.5;HUM:60;BAT:3.7";
    let parser = VirtualSensorParser {
        buffer: raw_data,
        counter: RefCell::new(0),
    };

    let data = parser.to_data()?;
    println!("raw_data_as_bytes: {:?}", raw_data);
    println!("[PARSED_DATA]: {:?}", data);

    Ok(())
}
