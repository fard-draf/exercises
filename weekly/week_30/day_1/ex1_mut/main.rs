// main.rs

pub struct Uart {
    pub data: u8,
}

pub struct I2c {
    pub data: u8,
}

pub enum PeripheralId {
    Uart,
    I2c,
}

pub struct PeripheralManager {
    pub uart: Uart,
    pub i2c: I2c,
}

impl PeripheralManager {
    pub fn new() -> Self {
        Self {
            uart: Uart { data: 10 },
            i2c: I2c { data: 20 },
        }
    }

    /// TODO: Implémentez cette méthode.
    /// Elle doit retourner une référence mutable à la donnée
    /// du périphérique spécifié par `id`.
    pub fn get_mut(&mut self, id: PeripheralId) -> &mut u8 {
        match id {
            PeripheralId::I2c => {
                &mut self.i2c.data
            }
            PeripheralId::Uart => {
                &mut self.uart.data
            }
        }
    }
}

fn main() {
    let mut manager = PeripheralManager::new();

    // TODO: Tentez d'obtenir deux références mutables simultanément.
    // Cette section DOIT produire une erreur de compilation.
    // Analysez l'erreur. Pourquoi le compilateur refuse-t-il ?
    // Même si `uart` et `i2c` sont des champs distincts,
    // pourquoi l'emprunt de `&mut self` bloque-t-il tout ?
    
    // Le compilateur refuse car nous sommes en train d'utiliser une reference de PeripheralManager en mutable. Donc une seule ref mut dans le scope. De plus i2c_data demande une ref mut egalement -> Double erreur: 2 ref mut dans le meme scope. 

    // let uart_data = manager.get_mut(PeripheralId::Uart);
    // let i2c_data = manager.get_mut(PeripheralId::I2c);

    let uart_data = &mut manager.uart.data;
    let i2c_data = &mut manager.i2c.data;

    *uart_data += 1;
    *i2c_data += 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic] // Ce test est conçu pour échouer à la compilation, pas à l'exécution.
                    // C'est une illustration, pas un test fonctionnel classique.
    fn test_double_borrow_fails_compilation() {
        let mut manager = PeripheralManager::new();
        let _uart_data = manager.get_mut(PeripheralId::Uart);
        let _i2c_data = manager.get_mut(PeripheralId::I2c);
    }
}