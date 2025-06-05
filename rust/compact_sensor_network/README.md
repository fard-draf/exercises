# 🎯 Practical Exercise: Compact Sensor Network

A focused exercise designed to help you **master bitwise operations** in a realistic context, without getting lost in complex business logic.

## 🧠 Objective

Optimize data transmission from **200 IoT sensors** over **LoRaWAN** by compressing each sensor's data into a compact **8-byte format**, so the entire payload fits into a **51-byte packet**.

---

## 🚧 Technical Constraints

```rust
const MAX_SENSORS: usize = 200;
const PACKET_SIZE: usize = 51; // max size per LoRaWAN packet
```

Each sensor must encode multiple fields into a single `u64`:

* `id`: 0 to 199 → 8 bits
* `temperature`: -40°C to +80°C with 0.5°C precision → 8 bits
* `humidity`: 0–100% → 7 bits
* `battery`: 0–100% → 7 bits
* `is_active`: bool → 1 bit
* `error_count`: 0–15 → 4 bits
* `last_ping`: relative time (in minutes), max range 24h → up to 29 bits

---

## 🔍 Deep Dive: Why This Matters

This exercise forces you to think like a system architect:

* **Bit Layout Strategy**: Plan how to pack data efficiently within limited space.
* **Data-Aware Encoding**: Use the actual data ranges (e.g., humidity, temperature) to decide how many bits are truly needed.
* **Memory Efficiency**: Reduce memory from \~3400 bytes to \~1600 bytes (compression ratio \~2:1), enabling single-packet transmission and saving energy.

---

## 🔧 Core Bitwise Operators to Use

* `<<` (left shift): Position bits at the correct offset
* `>>` (right shift): Extract bits from a position
* `|=` (bitwise OR assignment): Set specific bits
* `&` (bitwise AND): Mask and isolate target bits
* `!` (bitwise NOT): Create bit masks

---

## 🧪 Concrete Example – Temperature Encoding

```rust
// Encode: -40°C to +80°C with 0.5°C precision
// Range = 120°C → 240 steps → fits in 8 bits
fn set_temperature(&mut self, temp_celsius: f32) {
    let encoded = ((temp_celsius + 40.0) * 2.0) as u64;
    self.data |= (encoded & 0xFF) << 8; // store in bits 8–15
}

fn get_temperature(&self) -> f32 {
    let encoded = (self.data >> 8) & 0xFF;
    (encoded as f32 / 2.0) - 40.0
}
```

---

## 📈 Practical Gains

* Use `#[repr(C, packed)]` to **prevent padding**, ensuring exactly 8 bytes per sensor (critical in embedded systems).
* Switch from absolute to **relative timestamps** (e.g., "X minutes ago") to save space.
* Once mastered, consider adding **delta compression**: store only the difference from a reference value—perfect for slowly changing sensor data.

---

## 🛠️ File Overview

The full Rust code contains:

* `VerboseSensor`: the naive structure (17 bytes/sensor)
* `CompactSensor`: the optimized structure (8 bytes/sensor)
* `SensorNetwork`: handles a list of compact sensors
* `bitwise set_* / get_*` methods: to implement manually
* Unit tests to validate correctness

---

## ✅ Tasks

* [ ] Implement all `set_*` and `get_*` methods in `CompactSensor`
* [ ] Use bitwise operations to pack/unpack fields into the `u64`
* [ ] Write unit tests (`#[cfg(test)]`) to verify each field
* [ ] Add at least 2 example sensors and print memory savings


### Starting code

```rust 
use std::mem;

// Contraintes: Un capteur IoT transmet des données via LoRaWAN
// Chaque paquet = 51 bytes max, on veut faire tenir 200 capteurs
const MAX_SENSORS: usize = 200;
const PACKET_SIZE: usize = 51; // bytes max par transmission LoRaWAN

// Données brutes (non optimisées) - INTERDIT d'utiliser ça!
#[allow(dead_code)]
struct VerboseSensor {
    id: u32,              // 4 bytes
    temperature: f32,     // 4 bytes  
    humidity: u8,         // 1 byte
    battery_percent: u8,  // 1 byte
    is_active: bool,      // 1 byte
    error_count: u16,     // 2 bytes
    last_ping: u32,       // 4 bytes (timestamp)
}
// Total: 17 bytes par capteur = 3400 bytes pour 200 capteurs

// VOTRE MISSION: Compacter tout ça dans une structure optimisée
#[repr(C, packed)]
struct CompactSensor {
    // TODO: Utiliser des techniques de bit-packing pour réduire drastiquement
    // Indices:
    // - id: seulement 200 capteurs max -> 8 bits suffisent
    // - température: -40°C à +80°C, précision 0.5°C -> combien de valeurs?
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
    fn new() -> Self {
        CompactSensor { data: 0 }
    }
    
    // TODO: Implémenter ces fonctions avec des opérations bit par bit
    
    fn set_id(&mut self, id: u8) {
        // Utiliser |= et << pour placer l'ID dans les 8 premiers bits
        todo!()
    }
    
    fn get_id(&self) -> u8 {
        // Utiliser >> et & pour extraire l'ID
        todo!()
    }
    
    fn set_temperature(&mut self, temp_celsius: f32) {
        // Encoder température: -40°C à +80°C, pas de 0.5°C
        // Formule: encoded = (temp + 40.0) * 2.0
        // Stocker dans les bits 8-15 (8 bits = 256 valeurs = 128°C de range)
        todo!()
    }
    
    fn get_temperature(&self) -> f32 {
        // Décoder: temp = (encoded / 2.0) - 40.0
        todo!()
    }
    
    fn set_humidity(&mut self, humidity: u8) {
        // Stocker dans les bits 16-22 (7 bits = 0-127, on utilise 0-100)
        todo!()
    }
    
    fn get_humidity(&self) -> u8 {
        todo!()
    }
    
    fn set_battery(&mut self, battery: u8) {
        // Stocker dans les bits 23-29 (7 bits)
        todo!()
    }
    
    fn get_battery(&self) -> u8 {
        todo!()
    }
    
    fn set_active(&mut self, active: bool) {
        // Bit 30
        todo!()
    }
    
    fn get_active(&self) -> bool {
        todo!()
    }
    
    fn set_error_count(&mut self, count: u8) {
        // Bits 31-34 (4 bits = 0-15 erreurs max)
        todo!()
    }
    
    fn get_error_count(&self) -> u8 {
        todo!()
    }
    
    fn set_last_ping_relative(&mut self, minutes_ago: u16) {
        // Bits 35-63 (29 bits = ~537M minutes = ~1000 ans de range)
        // En pratique on veut juste les dernières 24h = 1440 minutes
        todo!()
    }
    
    fn get_last_ping_relative(&self) -> u16 {
        todo!()
    }
}

impl SensorNetwork {
    fn new() -> Self {
        SensorNetwork {
            sensors: [CompactSensor::new(); MAX_SENSORS],
            base_timestamp: 0, // À initialiser avec le timestamp actuel
        }
    }
    
    fn add_sensor(&mut self, 
                  id: u8, 
                  temp: f32, 
                  humidity: u8, 
                  battery: u8, 
                  active: bool,
                  error_count: u8,
                  minutes_ago: u16) -> Result<(), &'static str> {
        
        if id as usize >= MAX_SENSORS {
            return Err("ID trop grand");
        }
        
        let sensor = &mut self.sensors[id as usize];
        
        // TODO: Utiliser vos fonctions set_* pour populer le capteur
        
        Ok(())
    }
    
    fn get_sensor_data(&self, id: u8) -> Option<(f32, u8, u8, bool, u8, u16)> {
        if id as usize >= MAX_SENSORS {
            return None;
        }
        
        let sensor = &self.sensors[id as usize];
        
        // TODO: Utiliser vos fonctions get_* pour récupérer les données
        
        todo!()
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
    println!("Taille structure verbose: {} bytes", mem::size_of::<VerboseSensor>());
    println!("Taille structure compact: {} bytes", mem::size_of::<CompactSensor>());
    println!("Mémoire totale utilisée: {} bytes", network.memory_usage());
    println!("Ratio de compression: {:.2}x", network.compression_ratio());
    
    // Tests basiques
    if let Err(e) = network.add_sensor(0, 23.5, 65, 87, true, 3, 120) {
        println!("Erreur: {}", e);
    }
    
    if let Err(e) = network.add_sensor(1, -10.0, 45, 23, false, 7, 45) {
        println!("Erreur: {}", e);
    }
    
    // TODO: Ajouter vos tests pour vérifier que l'encodage/décodage fonctionne
    
    println!("\n=== TESTS ===");
    // Testez vos fonctions ici!
}

#[cfg(test)]
mod tests {
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
    }
}
```
