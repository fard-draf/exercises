#![no_std]
use bitflags::bitflags;
use borsh::{BorshDeserialize, BorshSerialize};
use core::mem;
use solana_sdk::pubkey::Pubkey;

// Tes contraintes embedded
const MAX_FLEET_SIZE: usize = 256; // Max sous-marins dans la flotte
const CALLSIGN_BITS: usize = 16; // 65k callsigns possibles
const PUBKEY: [u8; 32] = [1u8; 32];

// #[repr(C, packed)]
bitflags! {
    struct Status: u8 {
        const SURFACED  = 0b00000001;
        const SUBMERGED = 0b00000010;
        const SILENT    = 0b00000011;
    }
}

#[repr(C, packed)]
pub struct CompactSubmarine {
    callsign: u8,
    sub_id: u8,
    depth: u8,
    status: Status,
    // TODO: Design une structure qui stocke:
    // - Un callsign unique (peut être dérivé de la pubkey)
    // - Les données minimales pour reconstruire la pubkey
    // - Le "depth" du sous-marin (0-255 mètres)
    // - Status: surfaced/submerged/silent (2 bits suffisent!)
}

impl CompactSubmarine {
    fn init() -> Self {
        CompactSubmarine {
            callsign: 0,
            sub_id: 0,
            depth: 0,
            status: Status::SURFACED,
        }
    }

    fn add(&mut self, depth: u8) -> Self {
        let callsign = &[self.sub_id, PUBKEY, &[self.depth] ];
    }
}

pub struct SubmarineFleet {
    flotte: [CompactSubmarine; 256],
    arr_pubkey: Pubkey,
    // TODO: Pas de Vec! Array fixe ou mieux
    // Pense "embedded" - mémoire pré-allouée
}

impl SubmarineFleet {
    pub fn new() -> Self {
        let flotte: [CompactSubmarine; MAX_FLEET_SIZE] =
            core::array::from_fn(|_| CompactSubmarine::init());
        let arr_pubkey = Pubkey::new_from_array([0u8; 32]);

        Self { flotte, arr_pubkey }
    }

    // Ajoute un sous-marin à la flotte
    pub fn register_submarine(&mut self, pubkey: &Pubkey, depth: u8) -> Result<u16, &'static str> {
        self.flotte.iter().enumerate().map(|(i, e)| {
            if i == depth && i == 0 {
                i = depth;
                e =  
            }
        })
    }

    // Retrouve une pubkey depuis son callsign
    pub fn decode_callsign(&self, callsign: u16) -> Option<Pubkey> {
        todo!()
    }

    // Bonus: implémente une recherche par "derniers 4 bytes" de la pubkey
    pub fn search_by_tail(&self, tail: u32) -> Option<u16> {
        todo!()
    }
}

// Challenge: Quelle est la taille totale de ta structure pour 256 sous-marins ?

















fn main() {}
