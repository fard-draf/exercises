#![no_std]
#![no_main]
use core::panic::PanicInfo;


// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }



// L'état possible de notre périphérique.
// Le trait `Copy` est dérivé car l'état est une simple énumération,
// facile et peu coûteuse à copier. Nous y reviendrons.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PeripheralState {
    Unclaimed,
    Claimed,
    Error,
}

// Représente un périphérique matériel unique.
// Il a un identifiant et un état.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Peripheral {
    pub id: u8,
    pub state: PeripheralState,
}

// Le gestionnaire qui supervise un ensemble de périphériques.
// La lifetime 'a indique que le PeripheralManager n'est pas propriétaire
// des périphériques, il ne fait que les emprunter pour une durée 'a.
// Il emprunte une tranche mutable, car il devra modifier leur état.
pub struct PeripheralManager<'a> {
    peripherals: &'a mut [Peripheral],
}

impl<'a> PeripheralManager<'a> {
    pub fn claim(&mut self, id: u8) -> Option<&mut Peripheral> {
        self.peripherals
            .iter_mut()
            .filter(|e| e.id == id)
            .find_map(|e| {
                match e.state {
                    PeripheralState::Unclaimed => {
                        e.state = PeripheralState::Claimed;
                        Some(e)
                    }
                    _ => None
                }
                
            })

    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn claim_peripheral_scenarios() {
        // --- SETUP ---
        let mut peripherals_data = [
            Peripheral { id: 0, state: PeripheralState::Unclaimed },
            Peripheral { id: 1, state: PeripheralState::Unclaimed },
            Peripheral { id: 2, state: PeripheralState::Claimed },
        ];
        let mut manager = PeripheralManager { peripherals: &mut peripherals_data };

        // --- SCENARIO 1: Claim OK ---
        {
            let p1 = manager.claim(1);
            assert!(p1.is_some());
        }
        assert_eq!(peripherals_data[1].state, PeripheralState::Claimed);

        // --- SCENARIO 2: Déjà pris ---
        let p2 = manager.claim(2);
        assert!(p2.is_none());

        // // --- SCENARIO 3: Inexistant ---
        // let p99 = manager.claim(99);
        // assert!(p99.is_none());

        // // --- SCENARIO 4: L'ÉPREUVE DE VÉRITÉ ---
        // // On essaie de réclamer deux périphériques en même temps
        // let p0 = manager.claim(0);
        // let p1_again = manager.claim(1); // Le périphérique 1 est maintenant Claimed

        // assert!(p0.is_some());
        // assert!(p1_again.is_none()); // On vérifie que le claim sur p1 échoue bien
    }

    #[test]
    fn claim_two_peripherals_fails() {
        let mut peripherals_data = [
            Peripheral { id: 0, state: PeripheralState::Unclaimed },
            Peripheral { id: 1, state: PeripheralState::Unclaimed },
        ];
        let mut manager = PeripheralManager { peripherals: &mut peripherals_data };

        // Tentative de détenir deux emprunts mutables en même temps
        let _p0 = manager.claim(0);
        let _p1 = manager.claim(1); // <-- Que va dire le compilateur sur cette ligne ?
    }

}