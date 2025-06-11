// 🎯 Exercice 2: Trait Objects + Lifetimes - Data Processor
// 📊 Niveau: Medium
// ⏱️ Durée: 15min
//
// 📋 MISSION COMPLETE:
// Créer un système de processors avec trait objects qui peuvent traiter
// des données avec différentes lifetimes. Le processor doit pouvoir être
// stocké et utilisé avec des références temporaires.
//
// 📥 ENTRÉES:
// - Trait DataProcessor<'a> avec méthode process(&self, data: &'a str) -> String
// - ProcessorBox pour stocker Box<dyn DataProcessor<'a> + 'a>
// - Implémentations concrètes: UpperProcessor, LowerProcessor
//
// 📤 SORTIES:
// - ProcessorBox<'a> contenant trait object
// - process_data(&self, input: &'a str) -> String
// - Les processors transforment le texte selon leur règle
//
// 📏 RÈGLES MÉTIER:
// 1. UpperProcessor: convertit en MAJUSCULES
// 2. LowerProcessor: convertit en minuscules
// 3. ProcessorBox stocke ANY processor via trait object
// 4. La lifetime 'a lie les données d'entrée au processor
// 5. Retourne String owned (pas de lifetime dans output)
//
// 🧪 EXEMPLES:
// let upper = UpperProcessor;
// let processor_box = ProcessorBox::new(Box::new(upper));
// assert_eq!(processor_box.process_data("Hello"), "HELLO");

// TODO: Définir trait avec lifetime parameter
trait DataProcessor<'a> {
    fn process(&self, data: &'a str) -> String;
}

// TODO: Implémenter UpperProcessor
struct UpperProcessor;

// TODO: Implémenter trait pour UpperProcessor
impl<'a> DataProcessor<'a> for UpperProcessor {
    fn process(&self, data: &'a str) -> String {
        data.to_uppercase()
    }
}

// TODO: Implémenter LowerProcessor
struct LowerProcessor;

// TODO: Implémenter trait pour LowerProcessor
impl<'a> DataProcessor<'a> for LowerProcessor {
    fn process(&self, data: &'a str) -> String {
        data.to_lowercase()
    }
}

// TODO: Définir ProcessorBox avec trait object + lifetime
struct ProcessorBox<'a> {
    processor: Box<dyn DataProcessor<'a> + 'a>,
}

impl<'a> ProcessorBox<'a> {
    // TODO: Constructeur qui accepte n'importe quel DataProcessor
    fn new(processor: Box<dyn DataProcessor<'a> + 'a>) -> Self {
        Self { processor }
    }

    // TODO: Méthode qui délègue au processor interne
    fn process_data(&self, input: &'a str) -> String {
        self.processor.process(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper_processor() {
        let upper = UpperProcessor;
        let processor_box = ProcessorBox::new(Box::new(upper));

        assert_eq!(processor_box.process_data("hello world"), "HELLO WORLD");
        assert_eq!(processor_box.process_data("Rust"), "RUST");
    }

    #[test]
    fn test_lower_processor() {
        let lower = LowerProcessor;
        let processor_box = ProcessorBox::new(Box::new(lower));

        assert_eq!(processor_box.process_data("HELLO WORLD"), "hello world");
        assert_eq!(processor_box.process_data("Rust"), "rust");
    }

    #[test]
    fn test_mixed_case() {
        let upper = UpperProcessor;
        let processor_box = ProcessorBox::new(Box::new(upper));

        assert_eq!(processor_box.process_data("CamelCase"), "CAMELCASE");
    }

    #[test]
    fn test_empty_string() {
        let lower = LowerProcessor;
        let processor_box = ProcessorBox::new(Box::new(lower));

        assert_eq!(processor_box.process_data(""), "");
    }
}
