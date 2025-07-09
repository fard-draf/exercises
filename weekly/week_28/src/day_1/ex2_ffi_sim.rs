use std::ffi::CString;

// lib.rs
use libc::{__u32, __u64, c_char, c_double, uint64_t};

#[repr(C)]
pub struct StableApiData {
    // TODO: Définissez les champs de la struct ici
    // en respectant les spécifications.
    pub sad_c_double: [libc::c_double; 8], // f64 al8 offset 0
    pub sad_uint64_t: f64,                 // u64 al8 offset 64
    pub sad_int32_t: u32,                  // u32 al4 offset 72
    pub sad_c_char: *const libc::c_char,   // *const al 8 offset 80 padding avant 4
                                           // taille finale = 80 + 8 = 88
}

// TODO: Implémentez une fonction `create_stable_data` qui
// alloue et retourne un pointeur vers une instance de StableApiData.
// Attention à la gestion de la mémoire !

pub fn create_stable_data() -> *mut StableApiData {
    let cstring = CString::new("a1c1e1g1h1j").unwrap();

    let packet = StableApiData {
        sad_c_double: [1.1f64; 8],
        sad_uint64_t: 0.55,
        sad_int32_t: 0xffff,
        sad_c_char: CString::into_raw(cstring),
    };

    let boxed_data = Box::new(packet);

    Box::into_raw(boxed_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_layout_is_stable_and_predictable() {
        // L'ABI C n'a pas de "garantie" de taille exacte entre plateformes,
        // mais sur une plateforme donnée, elle doit être stable.
        // Ce test sert de "snapshot" pour votre architecture.
        // TODO: Calculez la taille attendue manuellement.
        let expected_size = 88; // Exemple, à vous de calculer la vraie valeur !
        assert_eq!(mem::size_of::<StableApiData>(), expected_size);

        // TODO: Vérifiez l'alignement.
        let expected_align = 8;
        assert_eq!(mem::align_of::<StableApiData>(), expected_align);
    }
}
