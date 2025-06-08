// Démonstration : Ce qui se passe VRAIMENT en mémoire

pub fn explain_copy() {
    println!("=== MEMORY LAYOUT ANALYSIS ===\n");

    // ===========================================
    // ÉTAPE 1: Création de la donnée originale
    // ===========================================

    let data = 123i32;
    println!("1. let data = 123;");
    println!("   → Alloue 4 bytes sur la stack");
    println!("   → Adresse de data: {:p}", &data);
    println!("   → Valeur: {}\n", data);

    // ===========================================
    // ÉTAPE 2: Création d'une référence
    // ===========================================

    let ref1 = &data;
    println!("2. let ref1 = &data;");
    println!("   → ref1 est un POINTEUR (8 bytes sur 64-bit)");
    println!("   → ref1 contient l'adresse: {:p}", ref1);
    println!("   → ref1 elle-même est stockée à: {:p}", &ref1);
    println!("   → *ref1 donne: {}\n", *ref1);

    // ===========================================
    // ÉTAPE 3: Déréférencement = COPY !
    // ===========================================

    let ref2 = *ref1; // ⚠️ COPY, pas une référence !
    println!("3. let ref2 = *ref1;");
    println!("   → *ref1 LIT la valeur à l'adresse {:p}", ref1);
    println!("   → COPIE cette valeur (123) dans une NOUVELLE variable");
    println!("   → ref2 est stockée à: {:p}", &ref2);
    println!("   → ref2 contient: {} (copie indépendante)", ref2);
    println!("   → data original: {} (inchangé)\n", data);

    // ===========================================
    // ÉTAPE 4: Vraie référence vers référence
    // ===========================================

    let ref3 = &ref1;
    println!("4. let ref3 = &ref1;");
    println!("   → ref3 pointe vers ref1 (pas vers data)");
    println!("   → ref3 contient l'adresse de ref1: {:p}", ref3);
    println!("   → *ref3 donne l'adresse de data: {:p}", *ref3);
    println!("   → **ref3 donne la valeur: {}\n", **ref3);

    // ===========================================
    // ÉTAPE 5: Modification pour comprendre l'indépendance
    // ===========================================

    let mut data_mut = 555;
    let ref_to_mut = &data_mut;
    let copy_of_value = *ref_to_mut; // COPY !

    println!("5. Test d'indépendance:");
    println!("   data_mut avant: {}", data_mut);
    println!("   copy_of_value: {}", copy_of_value);

    // Modification de l'original
    data_mut = 999;

    println!("   data_mut après modification: {}", data_mut);
    println!("   copy_of_value (inchangée): {}", copy_of_value);
    println!("   → PREUVE que copy_of_value est indépendante !\n");

    // ===========================================
    // ÉTAPE 6: Types qui ne peuvent PAS être copiés
    // ===========================================

    let vec_data = vec![1, 2, 3];
    let ref_to_vec = &vec_data;

    // ❌ Ceci ne compilerait PAS :
    // let copy_of_vec = *ref_to_vec;  // Vec ne peut pas être copié !

    println!("6. Types non-Copy:");
    println!("   Vec ne peut pas être copié avec *");
    println!("   → *ref_to_vec donnerait une erreur de compilation");
    println!("   → Seules les références fonctionnent : &vec_data\n");

    // ===========================================
    // ÉTAPE 7: Raw pointers - même principe
    // ===========================================

    let number = 777;
    let raw_ptr = &number as *const i32;

    println!("7. Raw pointers:");
    println!("   number à l'adresse: {:p}", &number);
    println!("   raw_ptr contient: {:p}", raw_ptr);

    unsafe {
        let copied_value = *raw_ptr; // COPY aussi !
        println!("   *raw_ptr copie la valeur: {}", copied_value);
        println!("   → Même principe que les références\n");
    }

    // ===========================================
    // RÉSUMÉ DU MODÈLE MENTAL
    // ===========================================

    println!("=== MODÈLE MENTAL ===");
    println!("• Une référence (&T) = adresse + garanties du compilateur");
    println!("• Un raw pointer (*T) = adresse brute, pas de garanties");
    println!("• Déréférencement (*) = VA CHERCHER la valeur à l'adresse");
    println!("• Pour i32/u32/etc : déréférencement = COPY automatique");
    println!("• La valeur originale reste intacte");
    println!("• Chaque variable a sa propre adresse mémoire");
}

// ===========================================
// FONCTIONS POUR TESTER LA COMPRÉHENSION
// ===========================================

pub fn demonstrate_copy_semantics() {
    println!("\n=== DÉMONSTRATION COPY ===");

    let original = 42;
    let reference = &original;
    let copied = *reference;

    // Affichage des adresses pour prouver que ce sont des variables distinctes
    println!("original stocké à  : {:p}", &original);
    println!("reference stockée à: {:p}", &reference);
    println!("copied stocké à    : {:p}", &copied);

    println!("→ Trois adresses différentes = trois variables distinctes");
    println!("→ copied est une COPIE indépendante de original");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_independence() {
        let mut original = 100;
        let reference = &original;
        let copied = *reference; // COPY

        // Modification de l'original
        original = 200;

        // La copie reste inchangée
        assert_eq!(copied, 100); // ✅ Prouve l'indépendance
        assert_eq!(original, 200);
    }
}
