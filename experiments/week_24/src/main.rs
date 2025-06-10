// TEST DIAGNOSTIC LIFETIMES - 15 minutes
// Résous ces 4 exercices progressifs

//=== EXERCICE 1 : Conflict Detection (3 min) ===
// fn broken_append(mut vec: Vec<String>, item: String) -> &str {
//     vec.push(item);
//     &vec.last().unwrap() // ❌ Pourquoi ça compile pas: Parce que dangling pointer si on consomme l owner
// }
// Challenge : Comprendre le lifetime mismatch
// Solution attendue : ?

fn broken_append(vec: &mut Vec<String>, item: String) -> &str {
    vec.push(item);
    &vec.last().unwrap()
}

//=== EXERCICE 2 : Multiple References (4 min) ===
struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn next_token(&mut self) -> Option<&str> {
        // Implémente : retourne prochain token séparé par espaces
        // Challenge : &str doit vivre aussi longtemps que self.input
        // Attention aux slices et lifetime propagation
        let token = self.input.split(" ").collect::<Vec<_>>();

        if let Some(token) = token.get(self.position) {
            self.position += 1;
            Some(&token)
        } else {
            None
        }
    }

    fn peek_ahead(&self, steps: usize) -> Option<&str> {
        // Regarde N tokens en avant sans bouger position
        // Challenge : multiple slices de même source
        let token = self.input.split(" ").collect::<Vec<_>>();
        let look_forward = token.iter().peekable().collect::<Vec<_>>();
        if let Some(peek) = look_forward.get(steps) {
            Some(peek)
        } else {
            None
        }
    }
}

//=== EXERCICE 3 : Lifetime Bounds (4 min) ===
// Pattern Solana AccountInfo simplifié
struct AccountInfo<'a> {
    key: &'a [u8; 32],
    data: &'a mut [u8],
}

fn find_and_modify<'accounts>(
    accounts: &'accounts mut [AccountInfo<'accounts>],
    target_key: &[u8; 32],
    modifier: impl Fn(&mut [u8]) -> Result<(), &'static str>,
) -> Result<&'accounts [u8], &'static str> {
    // 1. Trouve l'account avec target_key
    let focus = accounts.iter_mut().find(|acc| acc.key == target_key);
    if let Some(account) = focus {
        modifier(account.data)?;
        Ok(&account.data)
    } else {
        Err("Unable to find this key")
    }

    // 2. Applique modifier sur ses data
    // 3. Retourne référence vers les data modifiées
    // Challenge : &mut puis & de la même data
}

//=== EXERCICE 4 : Self-Referential Challenge (4 min) ===
struct CircularBuffer<'a> {
    data: &'a mut [u8],
    read_pos: usize,
    write_pos: usize,
}

impl<'a> CircularBuffer<'a> {
    fn write_and_get_readable(&mut self, bytes: &[u8]) -> &[u8] {
        // 1. Écrit bytes dans le buffer à write_pos
        let data_iter = self.data.iter();
        data_iter.enumerate().map(|(i, mut e)| {
            if i == self.write_pos {
                e = &bytes[0];
            }
        });
        // 2. Avance write_pos
        let windows = if self.read_pos <= self.write_pos && self.write_pos < self.data.len() {
            Some(&self.data[self.read_pos..self.write_pos])
        } else {
            None
        };

        windows.unwrap()
        // 3. Retourne slice lisible entre read_pos et write_pos
        // Challenge : emprunter &self.data[read_pos..write_pos]
        //           après avoir modifié self.data et self.write_pos
    }
}

//=== BONUS : Debugging Lifetime Errors ===
// Ces fonctions ont des erreurs subtiles - trouve et explique
fn mystery_error_1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
} // il y a deux parametres de sortie possible donc il faut qu ils partagent la meme life time ou avoir deux types de retour possible
// avec une lifetime adapte a chaque parametre d entree

fn mystery_error_2<'a>(data: &'a [u8]) -> impl Iterator<Item = &'a u8> {
    data.iter().filter(|&&b| b != 0)
}

fn main() {
    let data = [1, 2, 3, 4, 5, 6, 7];
    let mystery = mystery_error_2(&data);
    let result = mystery.collect::<Vec<_>>();
    println!("{:?}", result);
}

// BARÈME :
// 0-1 exercices : Débutant (retour basics)
// 2 exercices : Intermédiaire débutant (planning actuel OK)
// 3 exercices : Intermédiaire confirmé (accélération)
// 4 exercices + bonus : Avancé (skip vers async/Pin patterns)

// Temps limite : 15 minutes
// Focus : COMPRENDRE pourquoi ça compile pas, pas juste faire compiler
