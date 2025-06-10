pub fn function_pointer1() {
    // ===========================================
    // RÔLE 1: DANS LES TYPES (déclaration)
    // ===========================================

    let x = 42i32;

    // * dans le TYPE = "pointeur vers"
    let ref_pointer: &i32 = &x; // Référence (safe pointer)
    let raw_pointer: *const i32 = &x; // Raw pointer (unsafe)
    let mut_pointer: *mut i32; // Raw pointer mutable

    println!("=== RÔLE 1: Types ===");
    println!("ref_pointer type: &i32");
    println!("raw_pointer type: *const i32");
    println!("mut_pointer type: *mut i32");

    // ===========================================
    // RÔLE 2: DÉRÉFÉRENCEMENT SAFE (opération)
    // ===========================================

    println!("\n=== RÔLE 2: Déréférencement Safe ===");

    // * comme OPÉRATEUR = "va chercher la valeur"
    let value1 = *ref_pointer; // ✅ SAFE - va chercher 42
    println!("*ref_pointer = {}", value1);

    // Même chose avec des structures
    let tuple = (10, 20);
    let tuple_ref = &tuple;
    let (a, b) = *tuple_ref; // ✅ SAFE - destructure le tuple
    println!("*tuple_ref = ({}, {})", a, b);

    // ===========================================
    // RÔLE 3: DÉRÉFÉRENCEMENT UNSAFE (opération)
    // ===========================================

    println!("\n=== RÔLE 3: Déréférencement Unsafe ===");

    // * sur raw pointer = UNSAFE obligatoire
    let value2 = unsafe { *raw_pointer }; // ❌ UNSAFE - va chercher 42
    println!("unsafe {{ *raw_pointer }} = {}", value2);

    // ===========================================
    // COMPARAISON CÔTE À CÔTE
    // ===========================================

    println!("\n=== COMPARAISON ===");

    let data = 100;
    let safe_ref = &data;
    let unsafe_ptr = &data as *const i32;

    // MÊME DONNÉE, MÊME OPÉRATION, SÉCURITÉ DIFFÉRENTE
    let safe_value = *safe_ref; // ✅ Compilateur garantit la sécurité
    let unsafe_value = unsafe { *unsafe_ptr }; // ❌ TOI tu garantis la sécurité

    println!("safe_value = {}", safe_value);
    println!("unsafe_value = {}", unsafe_value);

    // ===========================================
    // QUIZ INTERACTIF
    // ===========================================

    println!("\n=== QUIZ ===");

    let number = 55;
    let ref1 = &number;
    let ref2 = &ref1; // Référence vers une référence !
    let ptr = ref1 as *const i32;

    // Question 1: Que fait chaque déréférencement ?
    println!("number = {}", number);
    println!("*ref1 = {}", *ref1); // ? ca deref ref1 donc ref1 = 55;
    println!("**ref2 = {}", **ref2); // ? ca fait un pointeur ver ref1 = 55;
    // println!("*ptr = {}", *ptr);      // Compile ou pas ? Non car *ptr hors unsafe
    println!("unsafe {{ *ptr }} = {}", unsafe { *ptr }); // ? Oui unsafe precise

    // ===========================================
    // EXERCICE PRATIQUE
    // ===========================================

    println!("\n=== EXERCICE ===");

    // Crée différents types de pointeurs vers la même donnée
    let target = 999;
    let ref1 = &target;
    let ref2 = *ref1;

    // Complète ces déclarations :
    let reference: &i32 = &target; // Référence safe
    let const_ptr: *const i32 = &target; // Raw pointer constant unsafe 
    let mut target_mut = 999;
    let mut_ptr: *mut i32 = &mut target_mut; // Raw pointer mutable unsafe

    // Déréférence chacun correctement :
    println!("Via reference: {}", *reference); // Safe
    println!("Via const_ptr: {}", unsafe { *const_ptr }); // Unsafe
    println!("Via mut_ptr: {}", unsafe { *mut_ptr }); // Unsafe

    // Modification via pointeur mutable
    unsafe {
        *mut_ptr = 777; // Change la valeur originale
    }
    println!("target_mut après modification: {}", target_mut);

    let data = 123;
    let ref1 = &data;
    let ref2 = *ref1;
    let mut data_mut = 5555;
    let ref_mut: *mut i32 = &mut data_mut;
    unsafe { *ref_mut = 55 };
    println!("data: {} -> safe", data);
    println!("ref1: {} -> safe", ref1); // c est une ref datadata
    println!("data: {} -> safe", ref2); // la ref2 prend la value de ref1
    println!("ref1: {} -> safe", ref1); // je comprends pas ici, le deref fait une copie ??
    println!("data still available: {}", data); // il me faut des eclaircissement sur le fond de l operation, au niveau de malloc. D ou vient la valeur owned par la ref ? La ref comporte pourtant juste une adresse, est ce qu un copy est effectif ? 
    println!("data: {} -> unsafe", unsafe { *ref_mut });
}

// ===========================================
// FONCTIONS POUR COMPRENDRE LES SIGNATURES
// ===========================================

// Fonction qui prend une référence (safe)
fn process_reference(data: &i32) -> i32 {
    *data * 2 // Safe dereference
}

// Fonction qui prend un raw pointer (unsafe)
unsafe fn process_raw_pointer(data: *const i32) -> i32 {
    (*data) * 2 // Unsafe dereference
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_types() {
        let value = 42;
        let safe_ref = &value;
        let raw_ptr = &value as *const i32;

        // Test safe dereference
        assert_eq!(*safe_ref, 42);

        // Test unsafe dereference
        unsafe {
            assert_eq!(*raw_ptr, 42);
        }

        // Test fonction calls
        assert_eq!(process_reference(safe_ref), 84);
        unsafe {
            assert_eq!(process_raw_pointer(raw_ptr), 84);
        }
    }
}
