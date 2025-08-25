use std::io;

use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

#[tokio::main]
async fn main() {
    tokio::select! {
        first = read_file("files/canboat1.json") => {
            println!("First win: {first:?}");
        }

        second = read_file("files/canboat2.json") => {
            println!("Second win: {second:?}");
        }
    }
}

async fn read_file(path: &str) -> Result<String, io::Result<()>> {
    let file = File::open(path).await.unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).await.unwrap();
    let string = String::new();

    Ok(String::from_utf8_lossy(&buffer).to_string())
}
