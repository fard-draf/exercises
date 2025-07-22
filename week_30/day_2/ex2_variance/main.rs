use std::marker::PhantomData;

// T est covariant sur 'a. C'est le cas le plus courant.
// Un 'long' peut être utilisé là où un 'court' est attendu.
struct Covariant<'a> {
    data: &'a str,
}

// T est contravariant sur 'a.
// Un 'court' peut être utilisé là où un 'long' est attendu.
struct Contravariant<'a> {
    // On utilise PhantomData pour simuler un type qui serait contravariant,
    // comme un type fonction `fn(&'a str)`.
    _marker: PhantomData<fn(&'a str)>,
}

// T est invariant sur 'a.
// Seul 'a peut être utilisé là où 'a est attendu. Aucune substitution.
struct Invariant<'a> {
    // On simule avec PhantomData, comme le ferait un `&'a mut &'a str`.
    _marker: PhantomData<&'a mut &'a str>,
}

fn main() {
    let static_str = "texte avec une lifetime 'static";

    // ----- Bloc de code avec une lifetime plus courte, 'a -----
    {
        let mut string_a = String::from("texte avec une lifetime 'a");
        let lifetime_a_str: &str = &string_a;

        println!("--- Analyse de la Covariance (pour &T) ---");
        let mut cov_static: Covariant<'static> = Covariant { data: static_str };
        let cov_a: Covariant<'_> = Covariant { data: lifetime_a_str };

        // Test 1: Peut-on assigner 'static à 'a ? (long → court)
        // cov_static = cov_a; // <-- Cette ligne va-t-elle compiler ?
        let another_cov_a: Covariant<'_> = cov_static;

        println!("--- Analyse de la Contravariance (pour fn(T)) ---");
        let mut contra_static: Contravariant<'static> = Contravariant { _marker: PhantomData };
        let mut contra_a: Contravariant<'_> = Contravariant { _marker: PhantomData };

        // Test 2: Peut-on assigner 'static à 'a ? (long → court)
        // contra_static = contra_a; // <-- Et celle-ci ?
        contra_a = contra_static;

        println!("--- Analyse de l'Invariance (pour &mut T) ---");
        let mut inv_static: Invariant<'static> = Invariant { _marker: PhantomData };
        let inv_a: Invariant<'_> = Invariant { _marker: PhantomData };

        // Test 3: Peut-on assigner 'static à 'a ? (long → court)
        inv_static = inv_a; // <-- Et finalement, celle-ci ?
    }
}