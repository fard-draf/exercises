use std::cell::Cell;

#[derive(Default)]
pub struct Config {
    pub api_key: String,
    pub reading: Cell<u32>
}

impl Config {
    pub fn get_api_key(&self) -> String {
        self.reading.set(self.reading.get() + 1);
        self.api_key.clone()
    }
}

