use std::cell::RefCell;

pub trait EventHandler<'a> {
    fn handle(&self, event_data: &'a str);
}

pub struct EventProcessor<'a> {
    handlers: Vec<Box<dyn EventHandler<'a> + 'a>>,
}

impl<'a> EventProcessor<'a> {
    fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    fn add_handler(&mut self, handler: Box<dyn EventHandler<'a> + 'a>) {
        self.handlers.push(handler);
    }

    fn process(&self, data: &'a str) {
        self.handlers.iter().for_each(|e| e.handle(data));
    }
}

pub struct ScopedLogger<'a> {
    buffer: &'a RefCell<String>,
}

impl<'a> ScopedLogger<'a> {
    fn new(buffer: &'a RefCell<String>) -> Self {
        Self { buffer }
    }
}

impl<'a> EventHandler<'a> for ScopedLogger<'a> {
    fn handle(&self, event_data: &'a str) {
        self.buffer.borrow_mut().push_str(event_data);
    }
}

pub struct StaticLogger {}

impl<'a> EventHandler<'a> for StaticLogger {
    fn handle(&self, event_data: &'a str) {
        println!("[STATIC_LOGGER]: {}", event_data)
    }
}

fn register_global_validator(validator: fn(&str)) {
    println!("Validator registred");
}

fn static_validator(data: &'static str) {
    println!("Validator static on: {}", data);
}

trait MutatingEventHandler<'a> {
    fn mutate(&self, data: &'a mut String);
}

struct Mutator;
impl<'a> MutatingEventHandler<'a> for Mutator {
    fn mutate(&self, data: &'a mut String) {
        data.push_str("...mutated");
    }
}

fn main() {
    // {
    //     let log_buffer = RefCell::new(String::new());
    //     let mut processor: EventProcessor<'_> = EventProcessor::new();

    //     let static_handler = StaticLogger {};
    //     processor.add_handler(Box::new(static_handler));

    //     let scoped_handler = ScopedLogger::new(&log_buffer);
    //     processor.add_handler(Box::new(scoped_handler));

    //     processor.process("DATA TEST");
    // }
    //
    //

    let mut static_mutator: Vec<Box<dyn MutatingEventHandler<'static>>> = Vec::new();
    {
        let short_lived_mutator = Mutator;
        static_mutator.push(Box::new(short_lived_mutator));
    }
}
