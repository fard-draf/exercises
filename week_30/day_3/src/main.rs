use std::cell::RefCell;

// Trait qui définit un gestionnaire d'événements
pub trait EventHandler<'a> {
    fn handle(&self, event_data: &'a str);
}

// Le processeur, générique sur la lifetime 'a de ses handlers et de ses données
pub struct EventProcessor<'a> {
    handlers: Vec<Box<dyn EventHandler<'a> + 'a>>,
}

impl<'a> EventProcessor<'a> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Box<dyn EventHandler<'a> + 'a>) {
        self.handlers.push(handler);
    }

    pub fn process(&self, data: &'a str) {
        self.handlers.iter().for_each(|e| e.handle(data));
    }
}

// Un logger qui emprunte un buffer local
pub struct ScopedLogger<'a> {
    buffer: &'a RefCell<String>,
}

impl<'a> ScopedLogger<'a> {
    pub fn new(buffer: &'a RefCell<String>) -> Self {
        Self { buffer }
    }
}

// Implémentation du trait pour le logger local
impl<'a> EventHandler<'a> for ScopedLogger<'a> {
    fn handle(&self, event_data: &'a str) {
        self.buffer.borrow_mut().push_str(event_data);
        // Pour vérifier que ça fonctionne :
        // println!("Scoped log: {}", self.buffer.borrow());
    }
}

// Un logger statique qui ne dépend de rien
pub struct StaticLogger {}

// Implémentation du trait pour le logger statique
// Il peut gérer n'importe quelle lifetime 'a
impl<'a> EventHandler<'a> for StaticLogger {
    fn handle(&self, event_data: &'a str) {
        println!("[STATIC_LOGGER]: {}", event_data);
    }
}

fn main() {
    // On crée un scope pour limiter la durée de vie des emprunts
    {
        // LA MODIFICATION CLÉ : On annote le type de processor avec une
        // lifetime anonyme `'_` pour forcer le compilateur à utiliser
        // la durée de vie du scope local.
        let mut processor: EventProcessor<'_> = EventProcessor::new();

        // Ajout du handler statique. Valide car il peut s'adapter à la lifetime `'_`.
        let static_handler = StaticLogger {};
        processor.add_handler(Box::new(static_handler));

        // Ajout du handler local. Valide car sa lifetime correspond à `'_`.
        let log_buffer = RefCell::new(String::new());
        let scoped_handler = ScopedLogger::new(&log_buffer);
        processor.add_handler(Box::new(scoped_handler));

        // Traitement des données. Valide car "DATA TEST" est &'static, qui
        // peut être contraint à la lifetime `'_`.
        processor.process("DATA TEST");

        // Preuve que le logger local a fonctionné
        println!("Final buffer content: \"{}\"", log_buffer.borrow());
    }
}
