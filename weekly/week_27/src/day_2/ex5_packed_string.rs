use std::convert::AsRef;
use std::str;

// Ta mission est de remplir les blocs `impl`
// et de faire passer les tests.

#[derive(Debug)]
pub struct PackedString<const N: usize> {
    len: u8,
    buffer: [u8; N],
}

impl<const N: usize> From<&str> for PackedString<N> {
    fn from(s: &str) -> Self {
        // TODO:
        // 1. Vérifier que la longueur de `s` ne dépasse pas N, sinon paniquer.
        // 2. Créer une instance de `PackedString`.
        // 3. Copier les octets de `s` dans le `buffer`.
        // 4. Mettre à jour `len`.
        // 5. Retourner l'instance.
        if s.len() > N {
            panic!();
        }

        let mut instance = PackedString {
            len: s.len() as u8,
            buffer: [0u8; N],
        };
        
       
        
        instance.buffer[..s.len()].copy_from_slice(s.as_bytes());
        


        Self { len: s.len() as u8 , buffer: instance.buffer }
    }
}

impl<const N: usize> AsRef<str> for PackedString<N> {
    fn as_ref(&self) -> &str {
        // TODO:
        // Utiliser `str::from_utf8` sur une slice du `buffer`
        // de la bonne longueur (`self.len`) pour retourner un `&str`.
        // On peut utiliser `unwrap()` car on garantit à la création
        // que les données sont du UTF-8 valide.
        str::from_utf8(&self.buffer[..self.len as usize]).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_et_conversion() {
        let s: PackedString<10> = PackedString::from("hello");
        assert_eq!(s.as_ref(), "hello");
    }

    #[test]
    fn test_taille_memoire() {
        // La taille doit être N (pour le buffer) + 1 (pour len) + padding éventuel
        // Ici N=10, len=1. align_of(u8)=1. Pas de padding. Taille = 11.
        assert_eq!(std::mem::size_of::<PackedString<10>>(), 11);
    }
    
    #[test]
    #[should_panic]
    fn test_creation_trop_longue() {
        // Cette conversion doit paniquer car "this is too long" a plus de 10 octets.
        let _s: PackedString<10> = PackedString::from("this is too long");
    }

    #[test]
    fn test_string_vide() {
        let s: PackedString<5> = PackedString::from("");
        assert_eq!(s.as_ref(), "");
        assert_eq!(s.len, 0);
    }
}