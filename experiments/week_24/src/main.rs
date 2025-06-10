// Challenge: Multiple lifetime parameters
// Raw focus: Zero-copy data structures (préparation Solana)

struct Container<'a> {
    id: &'a str,
    contents: Vec<&'a str>,
}

struct CargoManifest<'a, 'b> {
    containers: Vec<Container<'a>>,
    priority_cargo: Vec<&'b str>, // Peut venir d'une source différente
}

impl<'a, 'b> CargoManifest<'a, 'b> {
    fn new(containers: Vec<Container<'a>>) -> Self {
        Self {
            containers,
            priority_cargo: Vec::new(),
        }
    }

    // TODO: Ajouter cargo prioritaire depuis une source externe
    fn add_priority_cargo(&mut self, cargo: &'b str) {
        if !cargo.is_empty() {
            self.priority_cargo.push(cargo);
        }
        // Ton code ici
    }

    // TODO: Retourner (container_ids, priority_items, total_items_count)
    fn get_manifest_summary(&self) -> (Vec<&'a str>, Vec<&'b str>, usize) {
        // Ton code ici - attention aux lifetimes !
        let containers_ids = self.containers.iter().map(|c| c.id).collect::<Vec<_>>();
        let priority_items = self.priority_cargo.to_vec();
        let count = containers_ids.iter().chain(priority_items.iter()).count();

        (containers_ids, priority_items, count)
    }

    //7min45sc

    fn find_container_by_cargo(&self, cargo: &str) -> Option<&Container> {
        self.containers.iter().find(|e| e.contents.contains(&cargo))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_lifetimes() {
        let container_data = vec!["CONT001", "CONT002"];
        let cargo_data = vec!["electronics", "medicine"];

        let containers = vec![
            Container {
                id: container_data[0],
                contents: vec!["laptops", "phones"],
            },
            Container {
                id: container_data[1],
                contents: vec!["pills", "syringes"],
            },
        ];

        let mut manifest = CargoManifest::new(containers);
        manifest.add_priority_cargo(cargo_data[0]);
        manifest.add_priority_cargo(cargo_data[1]);

        let (ids, priority, total) = manifest.get_manifest_summary();

        assert_eq!(ids, vec!["CONT001", "CONT002"]);
        assert_eq!(priority, vec!["electronics", "medicine"]);
        assert_eq!(total, 4); // 2 containers + 2 priority items
    }
}
