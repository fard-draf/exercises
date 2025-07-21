pub struct FrameParser<'a> {
    buffer: &'a str,
}

impl<'a> Iterator for FrameParser<'a> {
    type Item = ParsedData<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() {
            return None;
        }
        let mut parsed_data = ParsedData::default();

        loop {
            if let Some((value, rest)) = self.buffer.split_once(';') {
                self.buffer = rest;

                parsing_val(value, &mut parsed_data);

                if parsed_data.id.is_some()
                    && parsed_data.status.is_some()
                    && parsed_data.temp.is_some()
                {
                    return Some(parsed_data);
                }
            } else {
                let old_buffer = self.buffer;
                self.buffer = "";

                parsing_val(old_buffer, &mut parsed_data);
            }

            if parsed_data.id.is_some()
                && parsed_data.status.is_some()
                && parsed_data.temp.is_some()
            {
                return Some(parsed_data);
            }
        }
    }
}

fn parsing_val<'a>(value: &'a str, parsed_data: &mut ParsedData<'a>) {
    match value {
        id if id.starts_with("ID") => {
            if let Some((_, last)) = id.split_once('=') {
                parsed_data.id = Some(last);
            }
        }
        temp if temp.starts_with("TEMP") => {
            if let Some((_, last)) = temp.split_once('=') {
                parsed_data.temp = Some(last);
            }
        }
        status if status.starts_with("STATUS") => {
            if let Some((_, last)) = status.split_once('=') {
                parsed_data.status = Some(last);
            }
        }
        _ => (),
    }
}

#[derive(Debug)]
pub struct ParsedData<'a> {
    id: Option<&'a str>,
    temp: Option<&'a str>,
    status: Option<&'a str>,
}

impl<'a> Default for ParsedData<'a> {
    fn default() -> Self {
        Self {
            id: None,
            temp: None,
            status: None,
        }
    }
}

fn main() {
    let input_stream = "ID=1;TEMP=20;STATUS=OK;ID=2;TEMP=21;STATUS=WARN;ID=3;TEMP=22;STATUS=OK";

    let parser = FrameParser {
        buffer: input_stream,
    };

    // Grâce à l'implémentation d'Iterator, on peut utiliser une boucle for !
    for frame in parser {
        println!("Trame parsée : {:?}", frame);
    }
}
