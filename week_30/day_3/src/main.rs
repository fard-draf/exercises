use core::cell::RefCell;
use std::{io::BufRead, marker::PhantomData};

pub struct ParsedSensorData<'a> {
    pub bat: &'a [u8],
    pub hum: &'a [u8],
    pub temp: &'a [u8],
}
pub struct VirtualSensorParser<'a> {
    pub buffer: &'a [u8],
    pub counter: RefCell<u16>,
}

impl<'a> VirtualSensorParser<'a> {
    /// Returns the parser of this [`VirtualSensorParser`].
    fn parser(&mut self) -> Option<ParsedSensorData> {
        let mut splitted = self.buffer.splitn(3, |&b| b == b';');
        let mut parsed_data = ParsedSensorData {
            bat: &[0u8],
            hum: &[0u8],
            temp: &[0u8],
        };

        if let Some(temp_raw) = splitted.next() {
            if let Some(position) = temp_raw.iter().position(|e| *e == b':') {
                match temp_raw {
                    temp if &temp[..position] == b"TEMP" => {}
                }
            }
        }

        if let Some(hum) = splitted[1] {
            match hum {
                hum if hum[..=2] - b"HUM" => {
                    parsed_data.hum = hum;
                }
                _ => return None,
            }
        }
        if let Some(bat) = splitted[2] {
            match bat {
                bat if bat[..=2] == b"BAT" => {
                    parsed_data.bat = bat;
                }
                _ => return None,
            }
        }

        Some(parsed_data)
    }
}
fn main() {
    let a = 44;
}
