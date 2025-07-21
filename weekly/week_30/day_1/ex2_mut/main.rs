// Code d'étude
fn main() {
    let mut data = String::from("donnée initiale");

    // Un emprunt mutable est créé. La possession de l'accès est transférée temporairement.
    let ref_mut_1 = &mut data;

    // --- Expérience 1 ---
    // Tentez d'utiliser la variable originale alors qu'un emprunt mutable existe.
    // Décommentez la ligne suivante et essayez de compiler.
    // println!("Accès direct via 'data': {}", data);

    // --- Expérience 2 ---
    // Tentez de créer une seconde référence mutable sur la même donnée.
    // Décommentez la ligne suivante et essayez de compiler.
    // let ref_mut_2 = &mut data;

    // Le seul usage qui sera permis
    ref_mut_1.push_str(" modifiée");
    println!("Usage valide via 'ref_mut_1': {:p}", ref_mut_1);
}

//1.
// Il y a une reference mutable en cour sur le scope lexical. Sa life time se termine ligne 23 lors du drop.
// La ligne 11 essaie de prendre une reference de data. Impossible car ref_mut_1 est encore vivante.
// ref_mut_2 tente de prendre une &mut de data. Idem, ref_mut_1 est encore vivante.
// La regle d'or du la LLVM concernant les refs est: une seule refmut sur le meme lexical scope = no datarace = safety first
//
//2.
// La ligne 22 est autorisee car elle utilise une reference a ref_mut_1, et cela ne represente aucun danger: la data ne peut etre modifiee que par ref_mut_1 -> aucun data race possible.
//
//3.
//Afin qu il y ait un seul conducteur a la fois, pour garantir que le vehicule aille a l endroit prevu par l unique pilote.
