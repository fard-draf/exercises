// TODO: Ajouter lifetime parameter approprié
struct TextAnalyzer<'a> {
    text: &'a str,
    min_length: usize,
}

// TODO: Implémenter avec lifetime parameter
impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str, min_length: usize) -> Self {
        TextAnalyzer { text, min_length }
    }

    fn word_count(&self) -> usize {
        self.text
            .split_whitespace()
            .filter(|e| e.len() >= self.min_length)
            .count()
    }

    fn longest_word(&self) -> Option<&str> {
        self.text
            .split_whitespace()
            .filter(|e| e.len() >= self.min_length)
            .max_by_key(|s| s.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let analyzer = TextAnalyzer::new("hello world rust", 4);
        assert_eq!(analyzer.word_count(), 3);

        let analyzer2 = TextAnalyzer::new("a bb ccc", 3);
        assert_eq!(analyzer2.word_count(), 1);
    }

    #[test]
    fn test_longest_word() {
        let analyzer = TextAnalyzer::new("hellos world rust", 1);
        assert_eq!(analyzer.longest_word(), Some("hellos"));

        let analyzer2 = TextAnalyzer::new("", 1);
        assert_eq!(analyzer2.longest_word(), None);
    }

    #[test]
    fn test_first_longest() {
        let analyzer = TextAnalyzer::new("abce def gh", 1);
        assert_eq!(analyzer.longest_word(), Some("abce")); // Premier trouvé
    }
}
