use ex1_parser::{parse, ParseErr, ParsedMessage};

fn main() {
    let buffer = b"$THISISAMESSAGE,ANDTHISISTHEPAYLOAD*FF";

    let parsed = parse(buffer).unwrap();
    println!("Parsed: {:?}", core::str::from_utf8(parsed.message));
}
