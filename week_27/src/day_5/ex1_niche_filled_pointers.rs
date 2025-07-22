use std::mem;
use std::ptr::NonNull;

/// Converts an `Option<NonNull<T>>` into a raw `*const T`.
///
/// This function should be a zero-cost abstraction. It leverages the fact
/// that `Option<NonNull<T>>` has the same memory representation as `*const T`,
/// where `None` is represented by a null pointer.
pub fn option_nonnull_to_raw<T>(opt: Option<NonNull<T>>) -> *const T {

    unsafe {
    std::mem::transmute(opt)
    }    
    
}

// Le NonNull<T> garantie que le pointeur n'est aps nul
// L'option sur un pointeur utilise une optimisation de niche, l'adresse memoire 0 etant libre, elle y loge le variant None -> aucun bit flag n'est necessaire, l'adresse memoire est intacte, meme wrappee dans un Some.
// On peut donc utiliser le unsafe a bon escient, le std::mem::transmute permet de transferer l'adresse memoire du wrapper opt en un *const T.


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_is_identical() {
        // Ce test est la clé de l'exercice. S'il passe, tu as la preuve
        // de l'optimisation de niche.
        assert_eq!(
            mem::size_of::<Option<NonNull<u8>>>(),
            mem::size_of::<*const u8>()
        );
        assert_eq!(
            mem::align_of::<Option<NonNull<u8>>>(),
            mem::align_of::<*const u8>()
        );
    }

    #[test]
    fn test_some_value_is_converted_correctly() {
        let mut data = 1337;
        let pointer = NonNull::new(&mut data).unwrap();
        
        // Convertis l'Option<NonNull> en pointeur brut
        let raw_pointer = option_nonnull_to_raw(Some(pointer));

        // Vérifie que le pointeur n'est pas nul et pointe vers la bonne donnée
        unsafe {
            assert!(!raw_pointer.is_null());
            assert_eq!(*raw_pointer, 1337);
        }
    }

    #[test]
    fn test_none_is_converted_to_null() {
        // Convertis None en pointeur brut
        let raw_pointer = option_nonnull_to_raw::<i32>(None);

        // Vérifie que le pointeur est bien nul
        assert!(raw_pointer.is_null());
    }
}