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
        self.0.capacity() * BITS_PER_BYTE
    }

    pub fn len(&self) -> usize {
        self.0.len() * BITS_PER_BYTE
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if index >= self.len() {
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
        assert!(index < self.len(), "Index out of bounds");

        let (byte, bit) = bitset_index(index);

        (self.0[byte] >> bit) & 1 == 1
    }

    pub fn get_checked(&self, index: usize) -> Option<bool> {
        if index >= self.len() {
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

#[cfg(test)]
mod tests {
    use super::{BitSet, FixedBitSet};

    #[test]
    fn bitset_set_get_roundtrip() {
        let mut bitset = BitSet::new();

        bitset.set(0, true);
        bitset.set(7, true);
        bitset.set(8, true);
        bitset.set(15, false);

        assert!(bitset.get(0));
        assert!(bitset.get(7));
        assert!(bitset.get(8));
        assert!(!bitset.get(15));
    }

    #[test]
    fn bitset_get_checked_bounds() {
        let mut bitset = BitSet::new();

        assert_eq!(bitset.get_checked(0), None);

        bitset.set(9, true);
        assert_eq!(bitset.get_checked(9), Some(true));
        assert_eq!(bitset.get_checked(10), Some(false));
        assert_eq!(bitset.get_checked(16), None);
    }

    #[test]
    fn bitset_clear_and_is_empty() {
        let mut bitset = BitSet::new();

        assert!(bitset.is_empty());

        bitset.set(3, true);
        assert!(!bitset.is_empty());

        bitset.clear();
        assert!(bitset.is_empty());
    }

    #[test]
    fn bitset_iter_matches_bits() {
        let mut bitset = BitSet::new();

        bitset.set(0, true);
        bitset.set(3, true);
        bitset.set(8, true);

        let values: Vec<bool> = bitset.iter().collect();

        assert_eq!(values.len(), bitset.len());
        assert!(values[0]);
        assert!(!values[1]);
        assert!(values[3]);
        assert!(values[8]);
    }

    #[test]
    fn bitset_with_capacity_is_in_bits() {
        let bitset = BitSet::with_capacity(9);

        assert_eq!(bitset.capacity(), 16);
        assert!(bitset.is_empty());
    }

    #[test]
    fn fixed_bitset_set_get_roundtrip() {
        let mut bitset: FixedBitSet<2> = FixedBitSet::new();

        bitset.set(0, true);
        bitset.set(9, true);

        assert!(bitset.get(0));
        assert!(bitset.get(9));
        assert!(!bitset.get(15));
    }

    #[test]
    fn fixed_bitset_get_checked_bounds() {
        let mut bitset: FixedBitSet<2> = FixedBitSet::new();

        assert_eq!(bitset.get_checked(16), None);

        bitset.set(10, true);
        assert_eq!(bitset.get_checked(10), Some(true));
        assert_eq!(bitset.get_checked(11), Some(false));
    }

    #[test]
    fn fixed_bitset_clear_and_is_empty() {
        let mut bitset: FixedBitSet<1> = FixedBitSet::new();

        assert!(bitset.is_empty());

        bitset.set(4, true);
        assert!(!bitset.is_empty());

        bitset.clear();
        assert!(bitset.is_empty());
    }

    #[test]
    fn fixed_bitset_iter_matches_bits() {
        let mut bitset: FixedBitSet<1> = FixedBitSet::new();

        bitset.set(2, true);
        bitset.set(7, true);

        let values: Vec<bool> = bitset.iter().collect();

        assert_eq!(values.len(), 8);
        assert!(values[2]);
        assert!(values[7]);
        assert!(!values[0]);
    }
}
