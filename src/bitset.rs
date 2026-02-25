#[derive(Debug, Clone, Default, PartialEq)]
pub struct BitSet(Vec<u8>);

impl BitSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn capacity(&self) -> usize {
        self.0.len() * 8
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if index >= self.capacity() {
            self.0.resize(index / 8 + 1, 0);
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

    pub fn iter(&self) -> impl Iterator<Item = bool> {
        self.0
            .iter()
            .flat_map(|byte| (0..8).map(move |bit| (byte >> bit) & 1 == 1))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixedBitSet<const BYTES: usize>([u8; BYTES]);

impl<const BYTES: usize> FixedBitSet<BYTES> {
    const CAPACITY: usize = BYTES * 8;

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

    pub fn iter(&self) -> impl Iterator<Item = bool> {
        self.0
            .iter()
            .flat_map(|byte| (0..8).map(move |bit| (byte >> bit) & 1 == 1))
    }
}

impl<const BYTES: usize> Default for FixedBitSet<BYTES> {
    fn default() -> Self {
        Self([0; BYTES])
    }
}

fn bitset_index(index: usize) -> (usize, usize) {
    (index / 8, index % 8)
}
