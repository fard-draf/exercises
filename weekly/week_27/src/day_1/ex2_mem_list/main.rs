// main.rs
#[derive(Debug)]
struct SimpleList {
    head: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl SimpleList {
    pub fn new() -> Self {
        SimpleList { head: None }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn to_memory_layout_string(&self) -> String {
        // Si la liste est vide, on retourne un message simple.
        if self.head.is_none() {
            return "STACK:\n  list (head) -> None\n".to_string();
        }

        // On commence à construire notre rapport.
        let mut report = String::from("STACK:\n");

        // --- Partie 1: Décrire la tête sur le Stack ---
        // On veut l'adresse vers laquelle le pointeur `head` pointe.
        // self.head est un Option<Box<Node>>. `as_ref()` nous donne un `Option<&Box<Node>>`.
        // `unwrap()` est sûr ici, car on a déjà vérifié que la tête n'est pas `None`.
        let head_pointer: &Box<Node> = self.head.as_ref().unwrap();
        report.push_str(&format!("  list (head) -> HEAP @ {:p}\n\n", head_pointer));

        // --- Partie 2: Décrire les noeuds sur le Heap ---
        report.push_str("HEAP:\n");

        // On initialise notre "curseur" pour qu'il pointe vers la tête de la liste.
        let mut cursor = &self.head;

        // `while let Some(node_box) = cursor` signifie :
        // "Tant que le curseur n'est pas `None`, déballe le `Box<Node>` dans la variable `node_box`
        // et exécute le corps de la boucle."
        while let Some(node_box) = cursor {
            // L'adresse du Box actuel (l'emplacement du wagon)
            let node_address = format!("{:p}", node_box);

            // On regarde vers quoi le `next` de ce wagon pointe.
            let next_address = match &node_box.next {
                Some(next_box) => format!("HEAP @ {:p}", next_box), // Il pointe vers un autre wagon
                None => String::from("None"),                       // C'est le dernier wagon
            };

            // On ajoute la ligne descriptive au rapport.
            let node_description = format!(
                "  @ {} : Node {{ value: {}, next -> {} }}\n",
                node_address, node_box.value, next_address
            );
            report.push_str(&node_description);

            // --- L'ÉTAPE LA PLUS IMPORTANTE ---
            // On déplace le curseur pour qu'il pointe vers le wagon suivant
            // pour la prochaine itération de la boucle.
            cursor = &node_box.next;
        }

        // On retourne le rapport final.
        report
    }
}

fn main() {
    let mut list = SimpleList::new();
    list.push(10);
    list.push(20);

    // Une fois que tout fonctionne :
    println!("{}", list.to_memory_layout_string());
}
