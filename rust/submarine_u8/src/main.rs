use bitflags::bitflags;
use borsh::{BorshDeserialize, BorshSerialize};
use core::{char::CharTryFromError, mem, str::FromStr};
use solana_sdk::pubkey::{Pubkey, PubkeyError};

// Tes contraintes embedded
const MAX_FLEET_SIZE: usize = 256; // Max sous-marins dans la flotte
const CALLSIGN_BITS: usize = 16; // 65k callsigns possibles
const PUBKEY: Pubkey = Pubkey::new_from_array([
    137, 155, 126, 203, 42, 68, 181, 139, 199, 49, 226, 30, 226, 49, 152, 55, 14, 10, 241, 16, 158,
    243, 222, 29, 200, 31, 11, 169, 112, 218, 2, 106,
]);

bitflags! {
    #[derive(Debug)]
    struct Status: u8 {
        const SURFACED  = 0b00000001;
        const SUBMERGED = 0b00000010;
        const SILENT    = 0b00000011;
    }
}

#[derive(Debug)]
pub struct CompactSubmarine {
    callsign: u8, // seed
    depth: u8,    // bump
    status: Status,
}

impl CompactSubmarine {
    fn init() -> Self {
        CompactSubmarine {
            callsign: 0,
            depth: 0,
            status: Status::SURFACED,
        }
    }

    fn add(depth: u8) -> Self {
        CompactSubmarine {
            callsign: 0,
            depth,
            status: Status::SURFACED,
        }
    }
}

#[derive(Debug)]
pub struct SubmarineFleet {
    flotte: [CompactSubmarine; 256],
}

impl SubmarineFleet {
    pub fn new() -> Self {
        let flotte: [CompactSubmarine; MAX_FLEET_SIZE] =
            core::array::from_fn(|_| CompactSubmarine::init());

        Self { flotte }
    }

    pub fn register_submarine(&mut self, depth: u8) -> Result<(), &'static str> {
        let mut unit = CompactSubmarine::add(depth);
        unit.callsign = depth;
        unit.depth = depth;
        self.flotte[depth as usize] = unit;

        Ok(())
    }

    pub fn decode_callsign(&self, callsign: u8) -> Option<Pubkey> {
        let pubkey = self
            .flotte
            .iter()
            .find(|e| e.callsign == callsign)
            .map(|e| {
                let seed = e.callsign;
                let chosen_bump = e.depth;

                Pubkey::create_program_address(&[&[seed], &[chosen_bump]], &PUBKEY).ok()
            });
        pubkey?
    }
}

fn main() -> Result<(), &'static str> {
    let mut flotte = SubmarineFleet::new();

    let mut count = 0;
    let limit = 255;

    (0..=255).for_each(|e| {
        if let Ok(value) = flotte.register_submarine(e) {
            println!("{:?}", value)
        }
    });

    println!("{:#?}", flotte.flotte);

    let decode = &flotte.decode_callsign(52);
    println!("{:?}", decode);

    let float_size = mem::size_of::<SubmarineFleet>();
    println!("{:?}", float_size);
    Ok(())
}
