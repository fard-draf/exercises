# Learning Bit Operations for Blockchain Development

This repository tracks my journey learning bit manipulation in Rust, with a focus on blockchain/Solana development.

## Why Bit Operations?

In blockchain development, especially on Solana, every byte matters. Understanding how to pack data efficiently is the difference between amateur and production-ready code.

## Exercise Progression

### 1. Binary Operations (`binary_operation/`)
**Goal**: Master the basics
- Learn `<<`, `>>`, `&`, `|`, `^` operators
- Practice hex ↔ binary ↔ decimal conversion
- Understand masking and bit extraction patterns

### 2. Permission System (`bitpacking_permission1/`)
**Goal**: Build a real permission system
- Pack 6 different permissions into a single `u64`
- Add validation rules and error handling
- Create a clean API that hides the complexity

**Result**: Memory usage reduced by ~90% compared to using a `Vec<Permission>`

### 3. Sensor Network (`compact_sensor_network/`)
**Goal**: Optimize IoT data transmission
- **Challenge**: Fit 200 sensors into 51-byte LoRaWAN packets
- **Solution**: Compress each sensor from 17 bytes to 8 bytes
- **Impact**: Total memory: 3400 bytes → 1600 bytes

```rust
// Example: Temperature encoding (-40°C to +80°C, 0.5°C precision)
fn set_temperature(&mut self, temp: f32) {
    let encoded = ((temp + 40.0) * 2.0) as u64;
    self.data |= (encoded & 0xFF) << 8; // Only 8 bits needed!
}
```

### 4. Drone Tracking (`dead_reckoning/`)
**Goal**: Cryptographic proof system
- Program Derived Addresses (PDAs) on Solana
- Recursive cryptographic chain for trajectory validation
- `no_std` compatible for embedded systems

### 5. Submarine Protocol (`submarine_u8/`)
**Goal**: Compress Solana pubkeys
- **Challenge**: Reduce 32-byte pubkeys to minimal storage
- **Approach**: Use PDA patterns (seed + bump = 2 bytes instead of 32)
- **Target**: 256 submarines in under 6KB

## Project Structure

```
exercises/
├── rust/
│   ├── binary_operation/          # Basic bit operations
│   ├── bitpacking_permission1/    # Permission system with validation
│   ├── compact_sensor_network/    # IoT data compression
│   ├── dead_reckoning/           # Cryptographic trajectory tracking
│   ├── submarine_u8/             # Pubkey compression experiments
│   ├── sandbox/                  # Quick tests and experiments
│   └── solana_tokenSPL_simulator/ # SPL Token serialization practice
└── README.md
```

## Key Learning Outcomes

- **Memory Optimization**: Learned to reduce data structures by 50-90%
- **Bit Manipulation**: Comfortable with masks, shifts, and extraction
- **Blockchain Context**: Understanding why these optimizations matter on Solana
- **Production Patterns**: Error handling, validation, and clean APIs

## Running the Examples

Each project is a standard Rust crate:

```bash
cd exercises/rust/binary_operation
cargo run

# Or run tests
cargo test
```

## Current Focus

Working on efficient Solana pubkey compression using Program Derived Addresses. The goal is to store fleet information in minimal memory while maintaining fast lookups.

---

**Note**: This is a learning repository - code may be incomplete or experimental. The focus is on understanding concepts and building practical skills.
