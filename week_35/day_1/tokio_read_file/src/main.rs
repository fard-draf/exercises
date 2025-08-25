use std::io;

use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

#[tokio::main]
async fn main() {
    let content1 = "test".repeat(100000);
    let content2 = "test".repeat(100004);
    tokio::fs::write("files/test1.json", &content1)
        .await
        .unwrap();
    tokio::fs::write("files/test2.json", &content2)
        .await
        .unwrap();
    tokio::select! {
    _ = read_file("files/test1.json") => {
                        println!("First win");
                }

    _ = read_file("files/test2.json") => {
                        println!("second win");
                }
    }
}

async fn read_file(path: &str) -> Result<String, io::Result<()>> {
    let file = File::open(path).await.unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).await.unwrap();

    Ok(String::from_utf8_lossy(&buffer).to_string())
}
