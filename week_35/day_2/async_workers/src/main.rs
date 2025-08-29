use std::sync::Arc;

use rand::random;
use tokio::{
    sync::{
        Mutex,
        mpsc::{self, Sender},
    },
    time::Duration,
};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(60);
    let counter: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let centralizer = tokio::spawn(async move {
        let mut vec = Vec::new();
        while let Some(message) = rx.recv().await {
            vec.push(message);
        }
        vec
    });

    let sensor1 = Sensor::new(1)
        .with_sender(tx.clone())
        .with_atomic_counter(Arc::clone(&counter));

    let sensor2 = Sensor::new(2)
        .with_sender(tx.clone())
        .with_atomic_counter(Arc::clone(&counter))
        .with_start(100);

    let sensor3 = Sensor::new(3)
        .with_sender(tx.clone())
        .with_atomic_counter(Arc::clone(&counter))
        .with_start(150)
        .with_timer(200);

    let t1 = send_message(sensor1);

    let t2 = send_message(sensor2);

    let t3 = send_message(sensor3);

    drop(tx);

    tokio::join!(t1, t2, t3);

    let total_messages = centralizer.await.unwrap();
    let final_count = *counter.lock().await;

    assert_eq!(final_count as usize, total_messages.len(), "Counting error");
    println!("Assertion valid. Proceed..");
    println!("Messages received... Total count: {:?}", counter)
}

async fn send_message(sensor: Sensor) {
    let mut counter = 0;
    tokio::time::sleep(Duration::from_millis(sensor.start)).await;
    while counter < 20 {
        counter += 1;
        if let Some(atomic_counter) = &sensor.atomic_counter {
            let mut total_counter = atomic_counter.lock().await;
            *total_counter += 1;
        }

        let value = random::<u32>();

        let message = format!("[SENSOR{}] value:{}", sensor.id, value);

        if let Some(tx) = &sensor.sender {
            if (tx.send(message).await).is_err() {
                break;
            }
        }

        tokio::time::sleep(Duration::from_millis(sensor.timer)).await;
    }
}

#[derive(Default)]
struct Sensor {
    id: u8,
    sender: Option<Sender<String>>,
    atomic_counter: Option<Arc<Mutex<u32>>>,
    start: u64,
    timer: u64,
}

impl Sensor {
    fn new(id: u8) -> Self {
        Self {
            id,
            sender: None,
            atomic_counter: None,
            start: 0,
            timer: 0,
        }
    }

    fn with_sender(mut self, sender: Sender<String>) -> Self {
        self.sender = Some(sender);
        self
    }

    fn with_atomic_counter(mut self, counter: Arc<Mutex<u32>>) -> Self {
        self.atomic_counter = Some(counter);
        self
    }

    fn with_start(mut self, start: u64) -> Self {
        self.start = start;
        self
    }

    fn with_timer(mut self, timer: u64) -> Self {
        self.timer = timer;
        self
    }
}
