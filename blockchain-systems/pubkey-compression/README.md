# 💚 Submarine Stealth Protocol – Hiding Solana Pubkeys in an Ultra-Compact System

## 🧠 Context

You're the architect of a stealthy underwater communication protocol for Solana transactions. Submarines (your nodes) operate in low-bandwidth environments — similar to embedded systems — where every byte transmitted increases the risk of detection.

Your mission: design a system that **efficiently stores and compresses Solana public keys** using smart encoding techniques.

---

## 🎯 Mission Objectives

Design and implement a system that:

* 🚢 Efficiently stores a fleet of Solana pubkeys (the "submarines")
* 🔐 Uses short *callsigns* (2–4 bytes) to identify each pubkey
* 🛰️ Can retrieve a full pubkey from its callsign
* 🧬 Minimizes memory usage via bit-packing and compact data representation

---

## 🧱 Starter Code

```rust
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// Embedded constraints
const MAX_FLEET_SIZE: usize = 256;  // Max submarines in the fleet
const CALLSIGN_BITS: usize = 16;    // 65k possible callsigns

#[repr(C, packed)]
pub struct CompactSubmarine {
    // TODO:
    // - Unique callsign (possibly derived from pubkey)
    // - Minimal data for pubkey reconstruction
    // - Submarine depth (0–255 meters)
    // - Status: surfaced/submerged/silent (fits in 2 bits!)
}

pub struct SubmarineFleet {
    // TODO: No Vec! Use fixed-size arrays
    // Think embedded – pre-allocated memory
}

impl SubmarineFleet {
    pub fn new() -> Self {
        todo!()
    }

    /// Adds a submarine to the fleet
    pub fn register_submarine(&mut self, pubkey: &Pubkey, depth: u8) -> Result<u16, &'static str> {
        // Returns the assigned callsign
        todo!()
    }

    /// Retrieves a pubkey from its callsign
    pub fn decode_callsign(&self, callsign: u16) -> Option<Pubkey> {
        todo!()
    }

    /// Bonus: Search using the last 4 bytes of a pubkey
    pub fn search_by_tail(&self, tail: u32) -> Option<u16> {
        todo!()
    }
}
```

---

## 💡 Technical Hints

* A Solana pubkey = 32 bytes. But do you really need all 32?
* Program-Derived Addresses (PDAs) often share byte patterns (e.g., prefixes).
* Callsigns can be derived from hashes or byte slices.
* Consider storing only what’s unique or variable: **delta compression**.
* The `status` fits in 2 bits — leaving 6 bits free for other data!

---

## 🧷 Difficulty Levels

| Level             | Description                                                     |
| ----------------- | --------------------------------------------------------------- |
| 🟢 Easy           | Store full pubkeys but optimize everything else (status, depth) |
| 🟡 Medium         | Use 2 + 2 bytes from pubkey as the callsign                     |
| 🔴 Hard           | Store only 16 bytes per pubkey with a reconstruction system     |
| ⚫ Embedded Psycho | Bloom filter + delta compression for lookup and storage         |

---

## 🛠️ What You’ll Learn

* Byte manipulation & bit-packing (`u8`, shifting, masking)
* Memory representation with `#[repr(C, packed)]`
* Fixed-size arrays vs dynamic allocation
* Hashing and fast lookups
* Why Solana pubkeys are 32 bytes (and how to work around it)

---

## ✅ Validation Criteria

Your implementation is considered a success if:

* Total memory usage is **< 6KB** for 256 submarines (vs 8KB+ naïve)
* Callsign lookup is **O(1)** or **O(log n)**
* **No heap allocation** after initialization
* **Works in `no_std`** environments (bonus!)

---

## 🚀 Getting Started

1. Calculate memory usage: `256 pubkeys × 32 bytes = 8192 bytes (8KB)`
2. Ask yourself: *“How can I reduce this by half?”*
3. Start with a working version (even if not optimized)
4. Use `std::mem::size_of` to measure memory
5. Optimize in iterations

---

## 🧪 My Solution 

> I finally opted for using a **Program Derived Address (PDA)**, which only requires 2 bytes instead of 32 — thanks to the combination of seed and bump.
>
> The exercise is still a bit buggy and not over, but that's not the current focus — right now, I'm zeroing in on **byte-level optimization**.

---

