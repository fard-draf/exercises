use core::net;
use std::mem;

// Contraintes: Un capteur IoT transmet des données via LoRaWAN
// Chaque paquet = 51 bytes max, on veut faire tenir 200 capteurs
const MAX_SENSORS: usize = 200;
const PACKET_SIZE: usize = 51; // bytes max par transmission LoRaWAN

// Données brutes (non optimisées) - INTERDIT d'utiliser ça!
#[allow(dead_code)]
struct VerboseSensor {
    id: u32,             // 4 bytes
    temperature: f32,    // 4 bytes
    humidity: u8,        // 1 byte
    battery_percent: u8, // 1 byte
    is_active: bool,     // 1 byte
    error_count: u16,    // 2 bytes
    last_ping: u32,      // 4 bytes (timestamp)
}
// Total: 17 bytes par capteur = 3400 bytes pour 200 capteurs

// VOTRE MISSION: Compacter tout ça dans une structure optimisée
#[derive(Debug)]
#[repr(C, packed)]
struct CompactSensor {
    // TODO: Utiliser des techniques de bit-packing pour réduire drastiquement
    // Indices:
    // - id: seulement 200 capteurs max -> 8 bits suffisent // u8
    // - température: -40°C à +80°C, précision 0.5°C -> combien de valeurs? // u8
    // - humidity: 0-100% -> 7 bits suffisent
    // - battery: 0-100% -> 7 bits suffisent
    // - is_active: 1 bit
    // - error_count: max 15 erreurs -> 4 bits
    // - last_ping: timestamp relatif, pas absolu
    data: u64, // Tout doit tenir dans 8 bytes!
}

struct SensorNetwork {
    sensors: [CompactSensor; MAX_SENSORS],
    base_timestamp: u32, // référence pour les timestamps relatifs
}

impl CompactSensor {
    // exctraction
    const ID_MASK: u64 = 0xFF; //  8 bits a 1
    const TEMP_MASK: u64 = 0xFF; // 8 bits a 1
    const HUMIDITY_MASK: u64 = 0x7F; // 7 bits a 1
    const BATTERY_PERCENT_MASK: u64 = 0x7F; // 7 bits a 1
    const IS_ACTIVE_MASK: u64 = 0x01; // 1 bits a 1
    const ERROR_COUNT_MASK: u64 = 0x0F; // 4 bits a 1
    const LAST_PING_MASK: u64 = 0x7FF; // 11 bits a 1 

    // postion des champs dans le u64
    const ID_SHIFT: u32 = 0; // 7 
    const TEMP_SHIFT: u32 = 8; // 8 + 8
    const HUMIDITY_SHIFT: u32 = 16; // 16 + 7
    const BATTERY_PERCENT_SHIFT: u32 = 23; // 23 + 7
    const IS_ACTIVE_SHIFT: u32 = 30; // 30 + 1
    const ERROR_COUNT_SHIFT: u32 = 31; // 31 + 4
    const LAST_PING_SHIFT: u32 = 35; // 34 + 11 -> END SHIFT TO 46

    fn new() -> Self {
        Self { data: 0u64 }
    }

    // TODO: Implémenter ces fonctions avec des opérations bit par bit

    fn set_id(&mut self, id: u8) {
        //nettoyage du receiver
        self.data &= !(Self::ID_MASK << Self::ID_SHIFT);
        //pose de la nouvelle valeur
        self.data |= (id as u64) << Self::ID_SHIFT;
    }

    fn get_id(&self) -> u8 {
        // Utiliser >> et & pour extraire l'ID
        ((self.data >> Self::ID_SHIFT) & Self::ID_MASK) as u8
    }

    fn set_temperature(&mut self, temp_celsius: f32) {
        // Encoder température: -40°C à +80°C, pas de 0.5°C
        // Formule: encoded = (temp + 40.0) * 2.0
        // Stocker dans les bits 8-15 (8 bits = 256 valeurs = 128°C de range)
        self.data &= !(Self::TEMP_MASK << Self::TEMP_SHIFT);
        self.data |= ((((temp_celsius + 40.0) * 2.0) as u64) & Self::ID_MASK) << Self::TEMP_SHIFT;
    }

    fn get_temperature(&self) -> f32 {
        // Décoder: temp = (encoded / 2.0) - 40.0
        // let decoded = ((self.data >> Self::TEMP_SHIFT) & Self::TEMP_MASK) as f32;
        let decoded = ((self.data >> 8) & 0xFF) as f32;
        (decoded / 2.0) - 40.0
    }

    fn set_humidity(&mut self, humidity: u8) {
        self.data &= !(Self::HUMIDITY_MASK) << Self::HUMIDITY_SHIFT;
        self.data |= ((humidity as u64) & Self::HUMIDITY_MASK) << Self::HUMIDITY_SHIFT;
    }

    fn get_humidity(&self) -> u8 {
        ((self.data >> Self::HUMIDITY_SHIFT) & Self::HUMIDITY_MASK) as u8
    }

    fn set_battery(&mut self, battery: u8) {
        self.data &= !(Self::BATTERY_PERCENT_MASK) << Self::BATTERY_PERCENT_SHIFT;
        self.data |= ((battery as u64) & Self::BATTERY_PERCENT_MASK) << Self::BATTERY_PERCENT_SHIFT;
    }

    fn get_battery(&self) -> u8 {
        ((self.data >> Self::BATTERY_PERCENT_SHIFT) & Self::BATTERY_PERCENT_MASK) as u8
    }

    fn set_active(&mut self, active: bool) {
        self.data &= !(Self::IS_ACTIVE_MASK) << Self::IS_ACTIVE_SHIFT;
        self.data |= ((active as u64) & Self::IS_ACTIVE_MASK) << Self::IS_ACTIVE_SHIFT;
    }

    fn get_active(&self) -> bool {
        let data = ((self.data >> Self::IS_ACTIVE_SHIFT) & Self::IS_ACTIVE_MASK) as u8;
        data == 1
    }

    fn set_error_count(&mut self, count: u8) {
        self.data &= !(Self::ERROR_COUNT_MASK) << Self::ERROR_COUNT_SHIFT;
        self.data |= ((count as u64) & Self::ERROR_COUNT_MASK) << Self::ERROR_COUNT_SHIFT;
    }

    fn get_error_count(&self) -> u8 {
        ((self.data >> Self::ERROR_COUNT_SHIFT) & Self::ERROR_COUNT_MASK) as u8
    }

    fn set_last_ping_relative(&mut self, minutes_ago: u16) {
        self.data &= !(Self::LAST_PING_MASK) << Self::LAST_PING_SHIFT;
        self.data |= ((minutes_ago as u64) & Self::LAST_PING_MASK) << Self::LAST_PING_SHIFT;
    }

    fn get_last_ping_relative(&self) -> u16 {
        ((self.data >> Self::LAST_PING_SHIFT) & Self::LAST_PING_MASK) as u16
    }
}

#[allow(clippy::too_many_arguments)]
impl SensorNetwork {
    fn new() -> Self {
        SensorNetwork {
            sensors: core::array::from_fn(|_| CompactSensor::new()),
            base_timestamp: 0, // À initialiser avec le timestamp actuel
        }
    }

    fn add_sensor(
        &mut self,
        id: u8,
        temp: f32,
        humidity: u8,
        battery: u8,
        active: bool,
        error_count: u8,
        minutes_ago: u16,
    ) -> Result<(), &'static str> {
        if id as usize >= MAX_SENSORS {
            return Err("ID trop grand");
        }

        let sensor = &mut self.sensors[id as usize];
        sensor.set_id(id);
        sensor.set_temperature(temp);
        sensor.set_humidity(humidity);
        sensor.set_battery(battery);
        sensor.set_active(active);
        sensor.set_error_count(error_count);
        sensor.set_last_ping_relative(minutes_ago);

        Ok(())
    }

    fn get_sensor_data(&self, id: u8) -> Option<(f32, u8, u8, bool, u8, u16)> {
        if id as usize >= MAX_SENSORS {
            return None;
        }

        let sensor = &self.sensors[id as usize];

        Some((
            sensor.get_temperature(),
            sensor.get_humidity(),
            sensor.get_battery(),
            sensor.get_active(),
            sensor.get_error_count(),
            sensor.get_last_ping_relative(),
        ))
    }

    fn memory_usage(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn compression_ratio(&self) -> f32 {
        let verbose_size = mem::size_of::<VerboseSensor>() * MAX_SENSORS;
        let compact_size = self.memory_usage();
        verbose_size as f32 / compact_size as f32
    }
}

fn main() {
    let mut network = SensorNetwork::new();

    println!("=== SENSOR NETWORK OPTIMIZATION ===");
    println!(
        "Taille structure verbose: {} bytes",
        mem::size_of::<VerboseSensor>()
    );
    println!(
        "Taille structure compact: {} bytes",
        mem::size_of::<CompactSensor>()
    );
    println!("Mémoire totale utilisée: {} bytes", network.memory_usage());
    println!("Ratio de compression: {:.2}x", network.compression_ratio());

    // Tests basiques
    if let Err(e) = network.add_sensor(0, 23.5, 65, 87, true, 3, 120) {
        println!("Erreur: {}", e);
    }

    if let Err(e) = network.add_sensor(1, -10.0, 45, 23, false, 7, 45) {
        println!("Erreur: {}", e);
    }

    if let Err(e) = network.add_sensor(2, 19.5, 95, 97, true, 5, 20) {
        println!("Erreur {:?}", e);
    }

    let result = network.get_sensor_data(0);
    println!("{:#?}", result);
    // TODO: Ajouter vos tests pour vérifier que l'encodage/décodage fonctionne

    println!("\n=== TESTS ===");
    // Testez vos fonctions ici!
}

#[cfg(test)]
mod tests {
    use std::process::id;

    use super::*;

    #[test]
    fn test_sensor_encoding() {
        let mut sensor = CompactSensor::new();

        // TODO: Tests unitaires pour chaque fonction
        // Exemple:
        // sensor.set_id(42);
        // assert_eq!(sensor.get_id(), 42);

        // sensor.set_temperature(23.5);
        // assert_eq!(sensor.get_temperature(), 23.5);
    }

    #[test]
    fn test_bit_boundaries() {
        // TODO: Tester les valeurs limites
        // Exemple: température max/min, humidity 100%, etc.
        let mut network = SensorNetwork::new();
        if let Err(e) = network.add_sensor(0, 30.0, 55, 10, true, 2, 58) {
            println!("Erreur: {}", e);
        }
        let mut sensor = CompactSensor::new();
        sensor.set_id(1);
        sensor.set_temperature(10.5);
        sensor.set_humidity(58);
        sensor.set_battery(100);
        sensor.set_active(false);
        sensor.set_error_count(14);
        sensor.set_last_ping_relative(2000);
        let data = sensor.data;
        println!("Valeur complète: {}", data);
        println!("En binaire: {:064b}", data);
        println!("En hexadécimal: 0x{:016X}", data);

        for shift in (0..64).step_by(8) {
            let mask = 0xFF << shift;
            let extracted = (data & mask) >> shift;
            println!("Bits {}-{}: {}", shift, shift + 7, extracted);
        }

        // println!(" network {:#?}", &network.sensors[0]);
        let id_slice: Vec<u64> = network
            .sensors
            .iter()
            .filter_map(|sensor| {
                let value = (sensor.data & 0xFFFFFFFF);
                if value != 0 { Some(value) } else { None }
            })
            .collect();

        println!("slice {:64b}", id_slice[0]);
        // assert_eq!(id_slice.pop(), Some(55));
    }

    #[test]
    fn test_humidity() {
        let mut sensor = CompactSensor::new();
        let original_hum: u8 = 60;
        sensor.set_humidity(original_hum);
        let retrieved_hum = sensor.get_humidity();
        assert!((original_hum == 60))
    }

    #[test]
    fn test_temperature_precision() {
        let mut sensor = CompactSensor::new();
        let original_temp = 23.5;
        sensor.set_temperature(original_temp);
        let retrievied_temp = sensor.get_temperature();
        assert!((original_temp - retrievied_temp).abs() < 0.1);
    }
}
