// main.rs

use std::mem::{size_of, align_of};

// --- DÉFINIS TES STRUCTS ICI ---

// Exemple pour la phase de test
#[repr(C)]
struct PourLeTest {
    a: u8,
    b: u32,
}


fn main() {
    // La fonction main peut servir pour des inspections rapides ou des 'println'
    println!("--- Inspection Manuelle ---");
    println!("Struct PourLeTest: size={}, align={}", size_of::<PourLeTest>(), align_of::<PourLeTest>());
    println!("\nLancez 'cargo test' pour la validation automatisée.");
}


// --- MODULE DE TESTS ---
// C'est ici que la magie opère.
// Ta mission : Remplacer les `_` par les bonnes valeurs pour que `cargo test` passe.

#[cfg(test)]
mod tests {
    use std::mem::{size_of, align_of};

    #[test]
    fn layout_homogene() {
        // Champs de même alignement
        struct Homogene {
            a: u32,
            b: f32,
        }
        assert_eq!(size_of::<Homogene>(), 8);
        assert_eq!(align_of::<Homogene>(), 4);
    }

    #[test]
    fn layout_heterogene_optimise() {
        // Le compilateur va réordonner pour optimiser
        struct Heterogene {
            a: u8,
            b: u64,
            c: u16,
        }
        assert_eq!(size_of::<Heterogene>(), 16); // Probablement 16, pas 24
        assert_eq!(align_of::<Heterogene>(), 8);
    }

    #[test]
     // On force l'ordre, le calcul manuel s'applique
    fn layout_heterogene_repr_c() {
       #[repr(C)] 
       struct HeterogeneC {
            a: u8,
            b: u64,
            c: u16,
        }
        assert_eq!(size_of::<HeterogeneC>(), 24); // Devrait être 24
        assert_eq!(align_of::<HeterogeneC>(), 8);
    }

    #[test]
    fn layout_imbrique() {
        struct Enfant {
            a: u8,
            b: u32,
        }
        struct Parent {
            c: u16,
            d: Enfant,
        }
        assert_eq!(size_of::<Parent>(), 12);
        assert_eq!(align_of::<Parent>(), 4);
    }
    
    #[test]
    fn layout_avec_reference() {
        // L'alignement/taille d'une référence est celui d'un pointeur (usize)
        struct AvecRef {
            a: u8,
            b: &'static str, // ATTENTION FAT POINTER -> adresse memoire x2
            c: bool,
        }
        assert_eq!(size_of::<AvecRef>(), 24);
        assert_eq!(align_of::<AvecRef>(), 8);
    }

#[test]
    fn layout_avec_zst() {
        // Zero-Sized Type: ne prend pas de place
        struct AvecZST {
            a: u32,
            b: (), // C'est un ZST
        }
        assert_eq!(size_of::<AvecZST>(), 4);
        assert_eq!(align_of::<AvecZST>(), 4);
    }
}