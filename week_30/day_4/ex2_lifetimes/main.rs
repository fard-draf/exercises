fn main() {
    let s = "Un bateau rouge";
    let res = first_word(s);
}

// Code source avec lifetimes élidées

// Fonction 1 : Appliquez la Règle 2
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Fonction 2 : Ici, aucune règle ne s'applique. Pourquoi ?
// Répondez par un commentaire au-dessus de la fonction.
// Aucune regle ne s applique car
// il y a une lifetime commune 'a,
// Il y a plusieurs references en parametre
// Nous ne sommes pas dans l implementation d une struct.
fn longest_word<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        // Cette ligne ne compile pas. Pourquoi ? Corrigez la signature pour que ça compile.
        y
    }
}

struct Document {
    content: String,
}

impl<'a> Document {
    // Fonction 3 : Appliquez la Règle 3
    fn get_content(&'a self, prefix: &str) -> &'a str {
        if self.content.starts_with(prefix) {
            &self.content
        } else {
            "No content found"
        }
    }
}

// Fonction A
// Signature actuelle :
fn slice_data(data: &[u8], start: usize) -> &[u8] {
    &data[start..]
}
// VOTRE ANALYSE : Le système voit un seul verrou en lecture en entrée. Il peut donc logiquement en déduire que le verrou de
// sortie dépend de cet unique verrou. Laquelle de ces annotations est redondante ?
// Il y a une seule reference entree -> Elision rule 2

// Fonction B
struct Inspector<'doc> {
    document: &'doc str,
}
impl<'doc> Inspector<'doc> {
    // Signature actuelle :
    fn first_line(&self) -> &str {
        self.document.lines().next().unwrap_or("")
    }
}
// VOTRE ANALYSE : Le système voit un verrou spécial en entrée : `&self`. Par convention, il suppose que tout verrou
// retourné dépendra de celui-ci. Pourquoi l'annotation 's est-elle superflue ?
// Elision rule n3 -> Dans 99% des cas, nous faisons reference a la lifetime de la struct concernee lorsque nous utilisons
// &self en parametre. Ainsi, au lieu de specifier la lifetime, le borrow checker considere par defaut
// que la lifetime retournee lorsqu une &self entre en jeu est celle de la struct. Ainsi, dans notre cas, specifier
// la lifetime est inutile.
//
// Fonction C
fn min_slice<'a, 'b>(a: &'a [u8], b: &'b [u8]) -> &'a [u8]
where
    'a: 'b;
{
    if a.len() < b.len() {
        a
    } else {
        b
    }
}
// VOTRE ANALYSE : La signature demande à retourner un verrou, mais ne dit pas duquel des verrous d'entrée ('a ou 'b) il hérite.
//  L'arbitre ne peut pas choisir. Si le verrou de 'a est relâché mais que la sortie dépendait de lui, c'est une corruption. ]
// Comment le contrat doit-il être réécrit pour lever cette ambiguïté ?
// Ici, nous avons deux references en parametre avec des lifetimes distinctes.
// La lifetime de retour peut etre lie a l un ou a l autre.
// Nous avons plusieurs solutions.
// - lier 'a et 'b avec where 'a: 'b; a doit durer au moins aussi longtemps que b.
// - mettre tous les parametres sur la meme lifetime.  
