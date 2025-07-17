use std::cell::RefCell;

#[derive(Default)]
pub struct Logger {
    pub messages: RefCell<Vec<String>>
}

impl Logger {
    pub fn log(&self, message: String) {
        self.messages.borrow_mut().push(message);
    }
}

pub fn service_a(logger: &Logger, message: String) {
    logger.log(message);
}

pub fn service_b(logger: &Logger, message: String) {
    logger.log(message);
}
