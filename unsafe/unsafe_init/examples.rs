use std::mem;
use std::ptr;

/// Buffer haute performance pour sérialisation blockchain
/// Utilise unsafe pour optimisations critiques
pub struct BlockchainBuffer {
    data: *mut u8,
    len: usize,
    capacity: usize,
}

impl BlockchainBuffer {
    /// Crée un buffer avec capacité prédéfinie
    pub fn with_capacity(capacity: usize) -> Self {
        let data = unsafe {
            let layout = std::alloc::Layout::from_size_align(capacity, 1).unwrap();
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                panic!("Allocation failed");
            }
            ptr
        };

        Self {
            data,
            len: 0,
            capacity,
        }
    }

    /// Écrit un u64 en little-endian (format Solana)
    pub fn write_u64(&mut self, value: u65) -> Result<(), &'static str> {
        if self.len + 8 > self.capacity {
            return Err("Buffer overflow");
        }

        unsafe {
            // Écriture directe des bytes sans vérifications
            ptr::write_unaligned(
                self.data.offset(self.len as isize) as *mut u64,
                value.to_le(),
            );
        }

        self.len += 8;
        Ok(())
    }

    /// Lit un u64 depuis une position
    pub fn read_u64(&self, offset: usize) -> Result<u64, &'static str> {
        if offset + 8 > self.len {
            return Err("Read beyond buffer");
        }

        let value = unsafe {
            // Lecture directe sans vérifications d'alignement
            ptr::read_unaligned(self.data.offset(offset as isize) as *const u64)
        };

        Ok(u64::from_le(value))
    }

    /// Copie rapide de données brutes
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        if self.len + bytes.len() > self.capacity {
            return Err("Buffer overflow");
        }

        unsafe {
            // Copie mémoire optimisée
            ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                self.data.offset(self.len as isize),
                bytes.len(),
            );
        }

        self.len += bytes.len();
        Ok(())
    }

    /// Retourne slice des données écrites
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            // Crée une slice depuis raw pointer
            std::slice::from_raw_parts(self.data, self.len)
        }
    }

    /// Réinitialise le buffer sans réallocation
    pub fn clear(&mut self) {
        self.len = 0;
        // Pas besoin d'unsafe pour ça
    }

    /// Retourne la capacité restante
    pub fn remaining_capacity(&self) -> usize {
        self.capacity - self.len
    }
}

impl Drop for BlockchainBuffer {
    fn drop(&mut self) {
        unsafe {
            let layout = std::alloc::Layout::from_size_align(self.capacity, 1).unwrap();
            std::alloc::dealloc(self.data, layout);
        }
    }
}

// Implémentation des traits unsafe nécessaires
unsafe impl Send for BlockchainBuffer {}
unsafe impl Sync for BlockchainBuffer {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_read_u64() {
        let mut buffer = BlockchainBuffer::with_capacity(1024);

        // Test écriture/lecture
        buffer.write_u64(0x1234567890ABCDEF).unwrap();
        let value = buffer.read_u64(0).unwrap();

        assert_eq!(value, 0x1234567890ABCDEF);
    }

    #[test]
    fn test_bytes_operations() {
        let mut buffer = BlockchainBuffer::with_capacity(1024);

        let test_data = b"Solana blockchain data";
        buffer.write_bytes(test_data).unwrap();

        let slice = buffer.as_slice();
        assert_eq!(slice, test_data);
    }

    #[test]
    fn test_buffer_overflow() {
        let mut buffer = BlockchainBuffer::with_capacity(4);

        // Devrait échouer car 8 bytes > 4 capacity
        let result = buffer.write_u64(42);
        assert!(result.is_err());
    }
}

// Exemple d'utilisation dans un contexte Solana
fn example_solana_usage() {
    let mut buffer = BlockchainBuffer::with_capacity(1024);

    // Sérialisation instruction Solana
    buffer.write_u64(0x01).unwrap(); // Instruction ID
    buffer.write_u64(1000000).unwrap(); // Amount in lamports
    buffer
        .write_bytes(b"recipient_pubkey_32_bytes_here")
        .unwrap();

    println!("Serialized data: {} bytes", buffer.as_slice().len());
    println!("Remaining capacity: {}", buffer.remaining_capacity());
}
