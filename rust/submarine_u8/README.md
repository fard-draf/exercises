Exercise: "Submarine Stealth Protocol" – Hiding Solana Pubkeys in an Ultra-Compact System

Context:
You are the architect of an underwater communication protocol for Solana transactions. Submarines operate with limited bandwidth (like embedded systems), and every byte transmitted increases the risk of detection. You must create a system that optimally encodes/compresses Solana pubkeys.

Mission:

Build a system that:

Efficiently stores a fleet of Solana pubkeys (the "submarines")

Implements short "callsigns" (2–4 bytes) to identify each pubkey

Can retrieve a full pubkey from its callsign

Minimizes memory usage using bit-packing techniques

Starter Code:

```rust use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// Your embedded constraints
const MAX_FLEET_SIZE: usize = 256;  // Max submarines in the fleet
const CALLSIGN_BITS: usize = 16;    // 65k possible callsigns

#[repr(C, packed)]
pub struct CompactSubmarine {
    // TODO: Design a struct that stores:
    // - A unique callsign (possibly derived from the pubkey)
    // - Minimal data needed to reconstruct the pubkey
    // - Submarine depth (0–255 meters)
    // - Status: surfaced/submerged/silent (2 bits is enough!)
}

pub struct SubmarineFleet {
    // TODO: No Vec! Use a fixed-size array or something better
    // Think embedded – pre-allocated memory
}

impl SubmarineFleet {
    pub fn new() -> Self {
        todo!()
    }

    // Add a submarine to the fleet
    pub fn register_submarine(&mut self, pubkey: &Pubkey, depth: u8) -> Result<u16, &'static str> {
        // Returns the assigned callsign
        todo!()
    }

    // Retrieve a pubkey from its callsign
    pub fn decode_callsign(&self, callsign: u16) -> Option<Pubkey> {
        todo!()
    }

    // Bonus: implement search by the last 4 bytes of the pubkey
    pub fn search_by_tail(&self, tail: u32) -> Option<u16> {
        todo!()
    }
}
```
// Challenge: What is the total size of your structure for 256 submarines?
Technical Hints:

A Solana pubkey = 32 bytes. But do you really need to store all 32?

Pubkeys show patterns. The first few bytes are often similar for program-derived addresses.

A callsign can be a hash of the first/last N bytes.

Think compression: what if you only stored the differences?

Status fits in 2 bits, leaving 6 bits free in one byte...

Difficulty Levels:

🟢 Easy: Store full pubkeys but optimize everything else (status, depth)
🟡 Medium: Use the first 2 + last 2 bytes as the callsign
🔴 Hard: Store only 16 bytes per pubkey with a reconstruction system
⚫ Embedded Psycho: Use a bloom filter for lookups + delta compression between similar pubkeys

What You’ll Learn:

Byte manipulation and bit-packing (u8, shifting, masking)

Memory representation (#[repr(C, packed)])

Fixed arrays vs dynamic allocations

Efficient hashing and lookups

Real-world Solana constraints (why 32 bytes per pubkey?)

Validation Criteria:

Your implementation is considered successful if:

Total size < 6KB for 256 submarines (vs 8KB+ naïvely)

Lookup by callsign is O(1) or O(log n)

Zero heap allocation after initialization

Works in no_std (bonus)

To Get Started:

First calculate: 256 pubkeys × 32 bytes = ? Then ask yourself: "How do I cut that in half?"

Write a version that works first, even if not optimized

Measure with std::mem::size_of

Optimize iteratively

The Twist:
Real submarines change callsigns during missions. Implement rotate_callsign() that changes a submarine’s callsign without losing the associated pubkey. Can you do that without reallocating memory?

---
## Solution 

I finally opted for a PDA, which takes 2 bytes instead of 32 thanks to the seed and bump. The exercise is still buggy, but that's not the focus right now — I'm concentrating on byte optimization for the moment.