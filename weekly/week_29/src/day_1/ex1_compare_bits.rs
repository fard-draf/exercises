// Votre mission : implémentez cette fonction.
pub fn compare_bits(a: u8, b: u8) -> u32 {
    // TODO: Utilisez l'opérateur XOR (^) pour trouver les bits qui diffèrent entre 'a' et 'b'.
    // TODO: Utilisez la méthode .count_ones() sur le résultat pour compter ces différences.
    unimplemented!("Implémentez la logique ici");
}

fn main() {
    println!("Démarrage des tests pour compare_bits...");
    // Vous pouvez ajouter des tests manuels ici si vous le souhaitez.
    let a = 0b1100_1010;
    let b = 0b1010_1011;
    let diff = compare_bits(a, b);
    println!("La distance de Hamming entre {:08b} et {:08b} est de {}", a, b, diff);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_diff() {
        assert_eq!(compare_bits(0b10101010, 0b10101010), 0);
        assert_eq!(compare_bits(0, 0), 0);
        assert_eq!(compare_bits(u8::MAX, u8::MAX), 0);
    }

    #[test]
    fn test_full_diff() {
        assert_eq!(compare_bits(0b00000000, 0b11111111), 8);
        assert_eq!(compare_bits(0b10101010, 0b01010101), 8);
        assert_eq!(compare_bits(0, u8::MAX), 8);
    }
    
    #[test]
    fn test_partial_diff() {
        assert_eq!(compare_bits(0b1100_1010, 0b1010_1011), 3);
        assert_eq!(compare_bits(0b0000_0001, 0b0000_0000), 1);
        assert_eq!(compare_bits(0b1111_0000, 0b0000_1111), 8);
        assert_eq!(compare_bits(0b1001_1001, 0b0110_0110), 8);
    }
}