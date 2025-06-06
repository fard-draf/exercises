struct BitVec {
    data: Vec<u8>,
    len: usize,
}

impl BitVec {
    fn new() -> Self {
        BitVec {
            data: Vec::new(),
            len: 0,
        }
    }

    fn push_bit(&mut self, bit: bool) {
        if self.len % 8 == 0 {
            self.data.push(0);
        }

        let byte_index = self.len / 8;
        let bit_index = 7 - (self.len % 8);

        if bit {
            self.data[byte_index] |= 1 << bit_index;
        }

        self.len += 1;
    }
}

fn main() {}
