// #[repr(C)]
struct EventPayload {
    timestamps: u128, // 16
    paylaod_hash: u64, // 8
    event_type: u16, // 2
    is_critical: bool, // 1
    source_id: u8, // 1
 }

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_memory() {
        println!("Size_of {}", std::mem::size_of::<EventPayload>());
        println!("Align_of {}", std::mem::align_of::<EventPayload>());

    }
}

// Mon raisonnement pour 32 bytes :
// Alignement de la struct : [Quel est l'alignement de la struct et pourquoi ?]
//
// Champ          | Taille | Offset | Padding Avant
// ------------------------------------------------
// timestamps     | 16     | 0      | 0
// paylaod_hash   | 8      | 16     | ...
// event_type     | 2      | 24     | ...
// is_critical    | 1      | 26     | ...
// source_id      | 1      | 27     | ...
// ------------------------------------------------
// Total des champs + padding interne : 28
// Padding final pour respecter l'alignement : 4
//
// Taille totale finale : 32