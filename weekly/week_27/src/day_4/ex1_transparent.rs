// TODO: Ajoute l'attribut nécessaire pour que les tests passent.
#[repr(transparent)]
pub struct Id(u64);

impl Id {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    #[test]
    fn test_zero_cost_memory_layout() {
        println!("size_of: {}", std::mem::size_of::<Id>());
        // Cette assertion est le critère de succès principal.
        // Elle valide que ton type `Id` a exactement la même
        // empreinte mémoire qu'un u64.
        assert_eq!(size_of::<Id>(), size_of::<u64>());
        assert_eq!(align_of::<Id>(), align_of::<u64>());
    }

    #[test]
    fn test_functionality() {
        let id = Id::new(101);
        assert_eq!(id.value(), 101);
    }
    
    // BONUS : Ce test n'est pas requis pour la validation,
    // mais il illustre la sécurité de type.
    // Il suffit de le lire et de le comprendre.
    #[test]
    fn test_type_safety() {
        #[repr(transparent)]
        struct PostId(u64);
        
        let user_id = Id::new(1);
        
        // La ligne suivante ne compilerait pas, et c'est le but !
        // let post_id: PostId = user_id;
        
        // Ceci démontre que même avec un layout identique,
        // le compilateur les traite comme des types distincts.
        assert_eq!(size_of::<Id>(), size_of::<PostId>());
    }
}