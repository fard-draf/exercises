use std::cell::{Cell, Ref, RefCell};

pub struct SmartSensor {
    value: Cell<u32>,
    log: RefCell<Vec<String>>
}

impl SmartSensor {
    pub fn new() -> Self {
        Self { value: Cell::new(0), log: RefCell::new(Vec::new()) }
    }

    pub fn record_value(&self, new_val: u32) {
        self.value.set(new_val);
    }

    pub fn add_log(&self, message: &str) {
        let mut logs = self.log.borrow_mut();
        logs.push(String::from(message));

        self.add_log_resilient(message);
    }


    fn add_log_resilient(&self, message: &str) {
        if let Ok(mut logs) = self.log.try_borrow_mut() {
            logs.push(String::from(message));
            println!("Message added!");
        } else {
            eprintln!("Error: log unreachable");
        }
    }
    pub fn read_value(&self) -> u32 {
        self.value.get()
    }
}