// Pas de `no_std` pour cet exercice, nous utilisons Vec de la bibliothèque standard.

/// Modifie le vecteur en place.
pub fn double_in_place(vec: &mut Vec<i32>) {
    // TODO: Itérer sur les éléments et les multiplier par 2.
    vec.iter_mut().for_each(|e| {*e *= 2;});

}

/// Crée et retourne un nouveau vecteur avec les valeurs doublées.
pub fn doubled_as_new(vec: &Vec<i32>) -> Vec<i32> {
    // TODO: Créer un nouveau Vec, itérer sur le vecteur d'entrée,
    // pusher les valeurs doublées dans le nouveau, et le retourner.
    vec.iter().map(|value| value * 2).collect()
}

fn main() {
    // Scénario pour double_in_place
    let mut vec1 = vec![1, 2, 3];
    println!("Original vec1: {:?}", vec1);
    double_in_place(&mut vec1);
    println!("Vec1 après modification: {:?}", vec1);

    // Scénario pour doubled_as_new
    let vec2 = vec![5, 6, 7];
    println!("\nOriginal vec2: {:?}", vec2);
    let new_vec = doubled_as_new(&vec2);
    println!("Vec2 (inchangé): {:?}", vec2);
    println!("Nouveau vecteur créé: {:?}", new_vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_in_place() {
        // TODO: Écrire le test pour vérifier que la fonction modifie bien le vecteur.
        let mut vec = vec![2,4,6,8,10];
        double_in_place(&mut vec);
        assert_eq!(vec, vec![4,8,12,16,20]);
    }

    #[test]
    fn test_doubled_as_new() {
        // TODO: Écrire le test pour vérifier que la fonction retourne un nouveau
        // vecteur correct et ne modifie pas l'original.
        let vec = vec![2,4,6,8,10];
        let vec_new = doubled_as_new(&vec);
        assert_eq!(vec![4,8,12,16,20], vec_new)        
    }
}