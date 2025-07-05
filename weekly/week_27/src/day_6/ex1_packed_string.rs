use std::mem;

// Définissez ici la capacité maximale de votre chaîne.
// Essayez de faire en sorte que la taille totale de la struct soit "ronde" (ex: 24 octets).
const PACKED_STRING_CAPACITY: usize = 23;

// Votre struct. Pensez à l'ordre des champs pour optimiser le padding !
#[derive(Debug, PartialEq, PartialOrd)]
pub struct PackedString {
    data: [u8; PACKED_STRING_CAPACITY],
    len: u8,
}

#[derive(Debug, PartialEq)]
pub enum PackedStringError {
    CapacityExceeded,
}

impl PackedString {

    pub fn from_str(s: &str) -> Result<Self, PackedStringError> {
        if s.len() >= 24 {
            return  Err(PackedStringError::CapacityExceeded);
        }
        let mut data = [0u8; PACKED_STRING_CAPACITY];
        data[..s.len()].copy_from_slice(s.as_bytes());
        let len = s.len() as u8;

        Ok(Self { data, len })
    }


    pub fn as_str(&self) -> &str {

        unsafe {            
        std::str::from_utf8_unchecked(&self.data[..self.len as usize])
        }
    }
}

// Question bonus pour après l'implémentation :
// Quelle est la taille de PackedString ? Pourquoi ?
// La taille de la struct PackedString est de 24 bytes car -> 1 byte pour la len qui est < a 255, 23 bytes pour le tableau, aligne sur 1 car u8.

fn print_layout() {
    println!("Size of PackedString: {} bytes", mem::size_of::<PackedString>());
    println!("Align of PackedString: {} bytes", mem::align_of::<PackedString>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_succes() {
        let s = "hello";
        let packed = PackedString::from_str(s).unwrap();
        assert_eq!(packed.as_str(), "hello");
        assert_eq!(packed.len, 5);
    }

    #[test]
    fn test_empty_string() {
        let s = "";
        let packed = PackedString::from_str(s).unwrap();
        assert_eq!(packed.as_str(), "");
        assert_eq!(packed.len, 0);
    }

    #[test]
    fn test_max_capacity() {
        let s = "a".repeat(PACKED_STRING_CAPACITY);
        let packed = PackedString::from_str(&s).unwrap();
        assert_eq!(packed.as_str(), s);
        assert_eq!(packed.len as usize, PACKED_STRING_CAPACITY);
    }

    #[test]
    fn test_over_capacity() {
        let s = "a".repeat(PACKED_STRING_CAPACITY + 1);
        let result = PackedString::from_str(&s);
        assert_eq!(result, Err(PackedStringError::CapacityExceeded));
    }

    #[test]
    fn test_memory_layout() {

        assert_eq!(mem::size_of::<PackedString>(), 24);
        print_layout();
    }
}