pub trait EventHandler<'a> {
    fn handle(&self, event_data: &'a str);
}

pub struct EventProcessor<'a> {
    handlers: Vec<Box<dyn EventHandler<'a>>>
}

impl<'a> EventProcessor<'a> {
    fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    fn add_handler(&mut self, handler: Box<dyn EventHandler<'a>>) {
        self.handlers.push(handler);
    }

    fn process(&self, data: &'a str) {
        self.handlers.iter().for_each(|e| e.handle(data));
    } 

}

fn main() {

    let event_process = EventProcessor::new();    
    let static_logger = 
}
