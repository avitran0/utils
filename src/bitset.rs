const BITS_PER_BYTE: usize = 8;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BitSet(Vec<u8>);

impl BitSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity_bits: usize) -> Self {
        Self(Vec::with_capacity(bytes_for_bits(capacity_bits)))
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut inner = Vec::with_capacity(bytes.len());
        inner.copy_from_slice(bytes);
        Self(inner)
    }

    pub fn capacity(&self) -> usize {
        self.0.len() * BITS_PER_BYTE
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if index >= self.capacity() {
            self.0.resize(index / BITS_PER_BYTE + 1, 0);
        }

        let (byte, bit) = bitset_index(index);

        if value {
            self.0[byte] |= 1 << bit;
        } else {
            self.0[byte] &= !(1 << bit);
        }
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < self.capacity(), "Index out of bounds");

        let (byte, bit) = bitset_index(index);

        (self.0[byte] >> bit) & 1 == 1
    }

    pub fn get_checked(&self, index: usize) -> Option<bool> {
        if index >= self.capacity() {
            return None;
        }

        let (byte, bit) = bitset_index(index);

        Some((self.0[byte] >> bit) & 1 == 1)
    }

    pub fn clear(&mut self) {
        for byte in &mut self.0 {
            *byte = 0;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|byte| *byte == 0)
    }

    pub fn iter(&self) -> impl Iterator<Item = bool> {
        self.0
            .iter()
            .flat_map(|byte| (0..BITS_PER_BYTE).map(move |bit| (byte >> bit) & 1 == 1))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixedBitSet<const BYTES: usize>([u8; BYTES]);

impl<const BYTES: usize> FixedBitSet<BYTES> {
    const CAPACITY: usize = BYTES * BITS_PER_BYTE;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < Self::CAPACITY, "Index out of bounds");

        let (byte, bit) = bitset_index(index);

        if value {
            self.0[byte] |= 1 << bit;
        } else {
            self.0[byte] &= !(1 << bit);
        }
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < Self::CAPACITY, "Index out of bounds");

        let (byte, bit) = bitset_index(index);

        (self.0[byte] >> bit) & 1 == 1
    }

    pub fn get_checked(&self, index: usize) -> Option<bool> {
        if index >= Self::CAPACITY {
            return None;
        }

        let (byte, bit) = bitset_index(index);

        Some((self.0[byte] >> bit) & 1 == 1)
    }

    pub fn clear(&mut self) {
        self.0.fill(0);
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|byte| *byte == 0)
    }

    pub fn iter(&self) -> impl Iterator<Item = bool> {
        self.0
            .iter()
            .flat_map(|byte| (0..BITS_PER_BYTE).map(move |bit| (byte >> bit) & 1 == 1))
    }
}

impl<const BYTES: usize> Default for FixedBitSet<BYTES> {
    fn default() -> Self {
        Self([0; BYTES])
    }
}

fn bytes_for_bits(bits: usize) -> usize {
    bits.div_ceil(BITS_PER_BYTE)
}

fn bitset_index(index: usize) -> (usize, usize) {
    (index / BITS_PER_BYTE, index % BITS_PER_BYTE)
}
