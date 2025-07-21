use std::cell::RefCell;

fn main() {
    // Un propriétaire unique (RefCell) qui contient une valeur.
    // Il permet de contourner le borrow checker statique.
    let data = RefCell::new(10);

    println!("Tentative de premier emprunt mutable...");
    // Emprunt mutable N°1 - Tout va bien.
    // La portée de 'emprunt_1' va jusqu'à la fin du bloc 'main'.
    let mut emprunt_1 = data.borrow_mut();
    *emprunt_1 += 1;
    println!("Succès. Valeur: {}", emprunt_1);

    println!("\nTentative de second emprunt mutable alors que le premier est toujours actif...");
    // Le 'panic' va se produire ici. La règle "un seul &mut" est vérifiée à l'exécution.
    let mut emprunt_2 = data.borrow_mut();

    // Ce code ne sera jamais atteint.
    println!("Ce message ne s'affichera jamais.");
    *emprunt_2 += 1;
}