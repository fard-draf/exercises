
pub struct FrameParser<'a> {
    buffer: &'a str,
    builder: ParsedDataBuilder<'a>,
}

impl <'a>Iterator for FrameParser<'a> {
    type Item = ParsedData<'a>;

    fn next(&mut self) -> Option<Self::Item> 

    {
        if self.buffer.is_empty() {
            return None;
        }
        
        if let Some((focus, rest)) = self.buffer.split_once(';') {
            if let Ok(parsed_data) = parse(focus, &mut self.builder) {
                self.buffer = rest;
                
                if self.builder.is_complete() {
                    let build = self.builder.build();
                    self.builder = ParsedDataBuilder::default();
                    return Some(build);
                } else {
                    return Some(self.builder.build())
                }
                                
            } else { None }
        } else { 
            if let Ok(parsed_data) = parse(self.buffer, &mut self.builder) {
                if self.builder.is_complete() {
                    let build = self.builder.build();
                    self.builder = ParsedDataBuilder::default();
                    return Some(build);
                } else {
                    None
                }
            } else {
                None
            }
         } 
    } 

        
}



#[derive(Debug, Default)]
pub struct ParsedData<'a> {
    id: &'a str,
    temp: &'a str,
    status: &'a str,
}

#[derive(Debug, Default)]
pub struct ParsedDataBuilder<'a> {
    id: Option<&'a str>,
    temp: Option<&'a str>,
    status: Option<&'a str>
}

impl<'a> ParsedDataBuilder<'a> {
    pub fn set_id(&mut self, id: &'a str) -> &Self {
        self.id = Some(id);
        self
    }

    pub fn set_temp(&mut self, temp: &'a str) -> &Self {
        self.temp = Some(temp);
        self
    }

    pub fn set_status(&mut self, status: &'a str) -> &Self {
        self.status = Some(status);
        self
    }

    pub fn build(&self) -> ParsedData<'a> {
        if let (Some(id), Some(temp), Some(status)) = (self.id, self.temp, self.status) {
            return ParsedData {
                id,
                temp,
                status
            };
        } else {
            ParsedData::default()
        }
    }

    pub fn fill(&mut self, input: &ParsedDataBuilder<'a>) -> &Self {
        if self.id.is_none() {
            self.id = input.id;
        }
        if self.temp.is_none() {
            self.temp = input.temp;
        }
        if self.status.is_none() {
            self.status = input.status;
        }
        self
    }

    pub fn is_complete(&self) -> bool {
        self.id.is_some() && self.temp.is_some() && self.status.is_some()
    }
}


fn parse<'a, 'b>(input: &'a str, builder: &'b mut ParsedDataBuilder<'a>) -> Result<&'b ParsedDataBuilder<'a>, &'static str> {

    Ok(match input {
            id_parts if id_parts.starts_with("ID") => {
                let (_, id) = id_parts.split_once("=").ok_or("Unvalid parts id")?;
                builder.set_id(id)
            }
            temp_parts if temp_parts.starts_with("TEMP") => {
                let (_, temp) = temp_parts.split_once('=').ok_or("Unvalid parts temp")?;
                builder.set_temp(temp)
            }
            status_parts if status_parts.starts_with("STATUS") => {
                let (_, status) = status_parts.split_once('=').ok_or("Unvalid parts status")?;
                builder.set_status(status)
            }
            _ => {
                return Err("Unvalid tram");
            }
        })


    
}

fn main() {
    // let input_data = String::from("ID=A451;TEMP=22.5;STATUS=OK");
    
    // match parse(&input_data) {
    //     Ok(parsed) => {
    //         println!("Parsing réussi :");
    //         println!("{:?}", parsed);
    //     }
    //     Err(e) => {
    //         println!("Erreur de parsing : {}", e);
    //     }
    // }

    let input_stream = "ID=1;TEMP=20;STATUS=OK;ID=2;TEMP=21;STATUS=WARN;ID=3;TEMP=22;STATUS=OK";

    let parser = FrameParser { buffer: input_stream, builder: ParsedDataBuilder::default() };

    // Grâce à l'implémentation d'Iterator, on peut utiliser une boucle for !
    for frame in parser {
        println!("Trame parsée : {:?}", frame);
    }


}

// #[cfg(test)]
// mod tests {
//     use super::*; // Importe les éléments du module parent (votre code)

//     #[test]
//     fn test_parse_succes() {
//         let input = "ID=A451;TEMP=22.5;STATUS=OK";
//         let result = parse(input).unwrap(); // .unwrap() paniquera si le test échoue, ce qui est attendu
//         assert_eq!(result.id, "A451");
//         assert_eq!(result.temp, "22.5");
//         assert_eq!(result.status, "OK");
//     }

//     // VOTRE MISSION : AJOUTEZ AU MOINS DEUX TESTS D'ÉCHEC
//     // 1. Un test pour une entrée malformée (ex: "ID=A451;TEMP22.5;STATUS=OK")
//     // 2. Un test pour une entrée incomplète (ex: "ID=A451;TEMP=22.5")

//     #[test]
//     fn test_parse_malformed() {
//         let input = "ID=A451TEMP=22.5;STATUS=OK";
//         assert!(parse(input).is_err()); // .unwrap() paniquera si le test échoue, ce qui est attendu

//     }
//     //

//     #[test]
//     fn test_parse_tooshort() {
//         let input = "ID=A451;TEMP=22.5";
//         assert!(parse(input).is_err()); // .unwrap() paniquera si le test échoue, ce qui est attendu
//     }
//     // Utilisez `assert!(parse(votre_input).is_err());` pour vérifier que la fonction retourne bien une erreur.
// }