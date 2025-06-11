// 🔄 WARM-UP : TextAnalyzer Enhanced (10min)
//
// 📋 MISSION:
// Révision concepts J1 + ajout méthode simple
//
// 📥 AJOUTS À IMPLÉMENTER:
// - contains_word(&self, word: &str) -> bool
//   * Case insensitive search
//   * Respecte min_length filter
//   * Return true si word trouvé dans text
//
// 📤 TESTS:
// Tous existants + nouveaux pour contains_word
//
// ⚡ SUCCESS CRITERIA:
// - Compile sans warnings
// - Tous tests passent
// - Utilise iterator chains (pas de collect())

struct TextAnalyzer<'a> {
    text: &'a str,
    min_length: usize,
}

impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str, min_length: usize) -> Self {
        TextAnalyzer { text, min_length }
    }

    fn word_count(&self) -> usize {
        self.text
            .split_whitespace()
            .filter(|word| word.len() >= self.min_length)
            .count()
    }

    fn longest_word(&self) -> Option<&str> {
        self.text
            .split_whitespace()
            .filter(|word| word.len() >= self.min_length)
            .max_by_key(|word| word.len())
    }

    // TODO: Implémenter contains_word
    fn contains_word(&self, target: &str) -> bool {
        self.text
            .split_whitespace()
            .filter(|word| word.len() >= self.min_length && target.len() > self.min_length)
            .any(|e| e.to_lowercase() == target.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let analyzer = TextAnalyzer::new("hello world rust", 4);
        assert_eq!(analyzer.word_count(), 3);
    }

    #[test]
    fn test_longest_word() {
        let analyzer = TextAnalyzer::new("hellos world rust", 1);
        assert_eq!(analyzer.longest_word(), Some("hellos"));
    }

    #[test]
    fn test_contains_word() {
        let analyzer = TextAnalyzer::new("Hello World Rust Programming", 4);

        // Case insensitive
        assert_eq!(analyzer.contains_word("hello"), true);
        assert_eq!(analyzer.contains_word("WORLD"), true);

        // Respecte min_length (4)
        assert_eq!(analyzer.contains_word("hi"), false); // Pas dans text

        // Word not in text
        assert_eq!(analyzer.contains_word("python"), false);

        // Exact match required
        assert_eq!(analyzer.contains_word("program"), false); // "Programming" != "program"
        assert_eq!(analyzer.contains_word("programming"), true);
    }
}
