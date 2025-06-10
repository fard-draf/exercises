// Exercice qui FORCE la réflexion

fn extract_and_store(data: &Vec<String>) -> (String, Vec<&str>) {
    //ce move est impossible si on comsomme data. Un &str est un pointer vers une string donc si la string est consommee, on part sur du dangling pointer.
    // on peut jouer avec de l Unsafe ou des struct personnalisee mais la valeur ownee doit forcement etre quelque part de referencee sur la memoire.
    // donc ici clone est necessaire car on ne doit pas consommer la ref data.
    // let first = data[0].clone(); // Clone donc on peut retourner la data tel quel
    let first = &data[0]; // le clone n est que deporte a la fin de la portee, si jamais on est contraint en malloc sur des grandes portees, ca peut etre utile.

    let refs: Vec<&str> = data.into_iter().map(|s| s.as_ref()).collect::<Vec<_>>(); // ❌ Compile pas !
    // (first, refs) // si clone au debut
    (first.to_string(), refs)
}

fn main() {
    let veccy = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let stored = extract_and_store(&veccy);

    println!("{:?}", stored);
}
