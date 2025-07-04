use log::{Level, Metadata, Record};
use std::fmt::Arguments;

fn main() {
    println!("Size_of {}", std::mem::size_of::<Record>());
    println!("Align_of {}", std::mem::align_of::<Record>())
}