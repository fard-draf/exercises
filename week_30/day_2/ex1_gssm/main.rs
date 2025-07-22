#[derive(Debug, PartialEq)]
pub struct NmeaFrame<'a> {
    pub data: &'a [u8],
}

pub struct FrameIterator<'a> {
    pub buffer: &'a [u8],
}

impl<'a> Iterator for FrameIterator<'a> {
    type Item = NmeaFrame<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() {
            return None;
        }

        loop {

            if let Some(position) = self.buffer.iter().position(|e| *e == b'\n') {
            
                let first = &self.buffer[..position - 1usize];
                let rest = &self.buffer[position + 1 ..];
                self.buffer = rest;
                if let Some(data) = parse_tram(first) {
                    return Some(NmeaFrame {
                            data: data,
                        });
                    } else {
                        continue;
                    }

            } else {

                if let Some(data) = parse_tram(self.buffer) {
                    self.buffer = b"";
                    return Some(NmeaFrame {
                        data: data,
                    });
                } else {
                    return None;
                }
            }
        }
    }
    
}

fn parse_tram<'a>(input: &'a [u8]) -> Option<&'a [u8]> {
    if let Some(position) = input.iter().position(|e| *e == b'$') {
        match &input[position..] {
            
            gp if gp[0] == b'$'  => {
                if let Some(stars) = gp.iter().rev().nth(2) {
                    if *stars == b'*' {         
                        return Some(gp)
                    } else { 
                        return None
                    }  
                } else { 
                    return None
                }
            } 
            _ => return None,
        }
    } else {
        return None
    }
}

fn main() {
    let raw_tram = b"teyuiopst\r\nd$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n$GPZDA,201530.00,04,07,2002,00,00*60\r\nb1s515us\r\nl051516\r\n$GPZMMMA,201530.00,04,07,2002,00,00*60";
    // let raw_tram = b"polluedhere$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n$GPZDA,201530.00,04,07,2002,00,00*60\r\nfalseendingtram\r\n$GPZDA,201530.00,04,07,2002,";
    let iter = FrameIterator { buffer: raw_tram };

    for tram in iter {
        if let Some(val) = core::str::from_utf8(tram.data).ok() {
            println!("Nmea: {:?}", val)
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_valid_tram() {
        let raw_tram = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n$GPZDA,201530.00,04,07,2002,00,00*60";
        let mut raw_iterator = FrameIterator { buffer: raw_tram };
        let pretty1 = NmeaFrame {
            data: b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47",
        };
        let pretty2 = NmeaFrame {
            data: b"$GPZDA,201530.00,04,07,2002,00,00*60",
        };
        let iter1 = raw_iterator.next().unwrap();
        let iter2 = raw_iterator.next().unwrap();
        assert_eq!(iter1, pretty1);
        // println!("iter1: {:?}", iter1);
        assert_eq!(iter2, pretty2);
    }

    #[test]
    fn test_empty_buffer() {
        let raw_tram = b"";
        let mut raw_iterator = FrameIterator { buffer: raw_tram };

        let iter1 = raw_iterator.next();
        assert!(iter1.is_none());
    }

    #[test]
    fn test_parasited_startnend_tram() {
        let raw_tram = b"polluedhere$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n$GPZDA,201530.00,04,07,2002,00,00*60\r\nfalseendingtram";
        let mut raw_iterator = FrameIterator { buffer: raw_tram };
        let pretty1 = NmeaFrame {
            data: b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47",
        };
        let pretty2 = NmeaFrame {
            data: b"$GPZDA,201530.00,04,07,2002,00,00*60",
        };
        let iter1 = raw_iterator.next().unwrap();
        let iter2 = raw_iterator.next().unwrap();
        let iter3 = raw_iterator.next();
        assert_eq!(iter1, pretty1);
        assert_eq!(iter2, pretty2);
        assert!(iter3.is_none())
    }

    #[test]
    fn test_parasite_between_tram() {
        let raw_tram = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\nUasdp'i'j;iuBF\r\n$GPZDA,201530.00,04,07,2002,00,00*60\r\n";
        let mut raw_iterator = FrameIterator { buffer: raw_tram };
        let pretty1 = NmeaFrame {
            data: b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47",
        };
        let pretty2 = NmeaFrame {
            data: b"$GPZDA,201530.00,04,07,2002,00,00*60",
        };
        let iter1 = raw_iterator.next().unwrap();
        let iter2 = raw_iterator.next().unwrap();
        assert_eq!(iter1, pretty1);
        assert_eq!(iter2, pretty2);
    }

    #[test]
    fn test_uncompleted_tram() {
        let raw_tram = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4\r\n$GPZDA,201530.00,04,07,2002,00,00*60\r\n";
        let mut raw_iterator = FrameIterator { buffer: raw_tram };
        let pretty2 = NmeaFrame {
            data: b"$GPZDA,201530.00,04,07,2002,00,00*60",
        };
        let iter1 = raw_iterator.next().unwrap();
        assert_eq!(iter1, pretty2);
    }
}
