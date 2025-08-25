use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

#[derive(Debug)]
pub struct Connection(u32);

pub struct ConnectionPool {
    available: Mutex<VecDeque<Connection>>,
    max_size: usize,
}

fn main() {}
