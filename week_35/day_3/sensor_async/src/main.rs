use std::sync::Arc;

use tokio::sync::{Mutex, mpsc::Sender};

struct Sensor {
    id: u8,
    tx: Sender<u32>,
    starter: u8,
}

impl Sensor {
    fn new(id: u8, sender: Sender<u32>, starter: u8) -> Self {
        Self {
            id,
            tx: sender,
            starter,
        }
    }

    async fn run(self) {
        let mut starter = self.starter;
        let end = starter + 4;
        while starter <= end {
            let message = starter;

            if (self.tx.send(message as u32).await).is_err() {
                break;
            } else {
                starter += 1;
            }
        }
        println!("[SENSOR{}] data sended.", self.id);
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<u32>(15);

    let counter = Arc::new(Mutex::new(Vec::new()));
    let centralizer_tools = Arc::clone(&counter);
    let centralizer = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            centralizer_tools.lock().await.push(message);
        }
        println!("centralizer task finished");
    });

    let sensor1 = Sensor::new(1, tx.clone(), 10);
    let sensor2 = Sensor::new(2, tx.clone(), 20);
    let sensor3 = Sensor::new(3, tx.clone(), 30);

    let s1 = sensor1.run();
    let s2 = sensor2.run();
    let s3 = sensor3.run();

    tokio::join!(s1, s2, s3);
    drop(tx);

    centralizer.await.unwrap();
    let mut final_messages = counter.lock().await.clone();
    final_messages.sort_unstable();
    assert_eq!(
        final_messages,
        vec![10, 11, 12, 13, 14, 20, 21, 22, 23, 24, 30, 31, 32, 33, 34],
        "Operation failed"
    );

    println!("Final vec: {:?}", final_messages);
}
