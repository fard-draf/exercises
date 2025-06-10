// Focus: Lifetime bounds et elision rules
// Pas de métaphore - technique pure

struct DataChunk<'a> {
    content: &'a str,
    position: usize,
}

struct Parser<'b> {
    source: &'b str,
    chunks: Vec<DataChunk<'b>>,
}

impl<'b> Parser<'b> {
    fn new(source: &'b str) -> Self {
        Self {
            source,
            chunks: Vec::new(),
        }
    }

    // TODO: Ajouter un chunk - lifetime automatiquement inféré
    fn add_chunk(&mut self, start: usize, end: usize) {
        // Créer DataChunk depuis self.source[start..end]
        // Ton code ici
        let chunks = self.source[start..end].split(" ").collect::<Vec<_>>();
        chunks.iter().enumerate().for_each(|(i, e)| {
            let data = DataChunk {
                content: e,
                position: i,
            };
            self.chunks.push(data);
        });
    }

    // TODO: Retourner le chunk le plus long
    fn longest_chunk(&self) -> Option<&str> {
        // Trouve le chunk avec content.len() maximum
        // Return son content
        // Ton code ici
        let value = self.chunks.iter().max_by_key(|s| s.content.len());
        if let Some(value) = value {
            Some(value.content)
        } else {
            None
        }
    }

    // TODO: Implémenter cette fonction avec lifetime bound explicite
    fn process_external<'ext>(&self, external: &'ext str) -> Vec<&'ext str>
    where
        'ext: 'b, // external doit vivre au moins aussi longtemps que data
    {
        // Retourne vec contenant external si il contient des mots de self.source
        // Sinon vec vide
        let ext_chunks = external.split(" ").collect::<Vec<&str>>();
        if ext_chunks.contains(&self.source) {
            vec![external]
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunks() {
        let data = "hellos world rust";
        let mut parser = Parser::new(data);

        parser.add_chunk(0, 5); // "hello"
        parser.add_chunk(6, 11); // "world"
        parser.add_chunk(12, 16); // "rust"

        assert_eq!(parser.longest_chunk(), Some("hello"));
    }

    #[test]
    fn test_lifetime_bound() {
        let data = "rust";
        let parser = Parser::new(data);
        let external = "I love rust programming";

        let result = parser.process_external(external);
        assert_eq!(result, vec!["I love rust programming"]);

        let external2 = "python is cool";
        let result2 = parser.process_external(external2);
        assert_eq!(result2, Vec::<&str>::new());
    }
}
