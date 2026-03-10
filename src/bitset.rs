use std::{
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Range},
};

const BITS_PER_BYTE: usize = 8;

/// Dynamic `BitSet` type, with an underlying [`Vec<u8>`].
/// Will grow to accomodate set and get requests.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BitSet(Vec<u8>);

impl BitSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity_bits: usize) -> Self {
        Self(Vec::with_capacity(bytes_for_bits(capacity_bits)))
    }

    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self(bytes.into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    pub fn byte_len(&self) -> usize {
        self.0.len()
    }

    pub fn bit_len(&self) -> usize {
        bit_len(self.byte_len())
    }

    pub fn byte_capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn bit_capacity(&self) -> usize {
        bit_len(self.byte_capacity())
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        get(&self.0, index)
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if index >= self.bit_len() {
            self.0.resize(index / BITS_PER_BYTE + 1, 0);
        }

        set(&mut self.0, index, value);
    }

    pub fn set_range(&mut self, range: Range<usize>, value: bool) {
        if range.end > self.bit_len() {
            self.0.resize(bytes_for_bits(range.end), 0);
        }

        set_range(&mut self.0, range, value);
    }

    pub fn clear(&mut self) {
        clear(&mut self.0);
    }

    pub fn not(&self) -> Self {
        Self(bitwise_not(&self.0))
    }

    pub fn count_ones(&self) -> usize {
        count_ones(&self.0)
    }

    pub fn is_zeroed(&self) -> bool {
        is_zeroed(&self.0)
    }

    pub fn iter(&self) -> impl Iterator<Item = bool> {
        iter(&self.0)
    }
}

impl Display for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_bitset(&self.0, f)
    }
}

impl AsRef<[u8]> for BitSet {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl From<Vec<u8>> for BitSet {
    fn from(bytes: Vec<u8>) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<&[u8]> for BitSet {
    fn from(bytes: &[u8]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl<const BYTES: usize> From<[u8; BYTES]> for BitSet {
    fn from(bytes: [u8; BYTES]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<BitSet> for Vec<u8> {
    fn from(bitset: BitSet) -> Self {
        bitset.into_bytes()
    }
}

impl Not for BitSet {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::not(&self)
    }
}

impl Not for &BitSet {
    type Output = BitSet;

    fn not(self) -> Self::Output {
        BitSet::not(self)
    }
}

/// Fixed `BitSet` type, with an underlying `[u8; BYTES]`.
/// Will panic if trying to set indices out of bounds.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixedBitSet<const BYTES: usize>([u8; BYTES]);

impl<const BYTES: usize> FixedBitSet<BYTES> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_bytes(bytes: [u8; BYTES]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; BYTES] {
        &self.0
    }

    pub fn into_bytes(self) -> [u8; BYTES] {
        self.0
    }

    pub const fn byte_len(&self) -> usize {
        BYTES
    }

    pub const fn bit_len(&self) -> usize {
        BYTES * BITS_PER_BYTE
    }

    pub fn set(&mut self, index: usize, value: bool) {
        set(&mut self.0, index, value);
    }

    pub fn set_range(&mut self, range: Range<usize>, value: bool) {
        set_range(&mut self.0, range, value);
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        get(&self.0, index)
    }

    pub fn clear(&mut self) {
        clear(&mut self.0);
    }

    pub fn and(&self, other: &Self) -> Self {
        Self(bitwise_binary_array(&self.0, &other.0, |left, right| {
            left & right
        }))
    }

    pub fn or(&self, other: &Self) -> Self {
        Self(bitwise_binary_array(&self.0, &other.0, |left, right| {
            left | right
        }))
    }

    pub fn xor(&self, other: &Self) -> Self {
        Self(bitwise_binary_array(&self.0, &other.0, |left, right| {
            left ^ right
        }))
    }

    pub fn not(&self) -> Self {
        Self(bitwise_not_array(&self.0))
    }

    pub fn count_ones(&self) -> usize {
        count_ones(&self.0)
    }

    pub fn is_zeroed(&self) -> bool {
        is_zeroed(&self.0)
    }

    pub fn iter(&self) -> impl Iterator<Item = bool> {
        iter(&self.0)
    }
}

impl<const BYTES: usize> Default for FixedBitSet<BYTES> {
    fn default() -> Self {
        Self([0; BYTES])
    }
}

impl<const BYTES: usize> Display for FixedBitSet<BYTES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_bitset(&self.0, f)
    }
}

impl<const BYTES: usize> AsRef<[u8]> for FixedBitSet<BYTES> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const BYTES: usize> AsRef<[u8; BYTES]> for FixedBitSet<BYTES> {
    fn as_ref(&self) -> &[u8; BYTES] {
        self.as_bytes()
    }
}

impl<const BYTES: usize> From<[u8; BYTES]> for FixedBitSet<BYTES> {
    fn from(bytes: [u8; BYTES]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl<const BYTES: usize> From<FixedBitSet<BYTES>> for [u8; BYTES] {
    fn from(bitset: FixedBitSet<BYTES>) -> Self {
        bitset.into_bytes()
    }
}

impl<const BYTES: usize> BitAnd for FixedBitSet<BYTES> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::and(&self, &rhs)
    }
}

impl<const BYTES: usize> BitAnd<&FixedBitSet<BYTES>> for FixedBitSet<BYTES> {
    type Output = Self;

    fn bitand(self, rhs: &Self) -> Self::Output {
        Self::and(&self, rhs)
    }
}

impl<const BYTES: usize> BitAnd<FixedBitSet<BYTES>> for &FixedBitSet<BYTES> {
    type Output = FixedBitSet<BYTES>;

    fn bitand(self, rhs: FixedBitSet<BYTES>) -> Self::Output {
        FixedBitSet::and(self, &rhs)
    }
}

impl<const BYTES: usize> BitAnd for &FixedBitSet<BYTES> {
    type Output = FixedBitSet<BYTES>;

    fn bitand(self, rhs: Self) -> Self::Output {
        FixedBitSet::and(self, rhs)
    }
}

impl<const BYTES: usize> BitAndAssign<&Self> for FixedBitSet<BYTES> {
    fn bitand_assign(&mut self, rhs: &Self) {
        *self = Self::and(self, rhs);
    }
}

impl<const BYTES: usize> BitAndAssign for FixedBitSet<BYTES> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self &= &rhs;
    }
}

impl<const BYTES: usize> BitOr for FixedBitSet<BYTES> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::or(&self, &rhs)
    }
}

impl<const BYTES: usize> BitOr<&FixedBitSet<BYTES>> for FixedBitSet<BYTES> {
    type Output = Self;

    fn bitor(self, rhs: &Self) -> Self::Output {
        Self::or(&self, rhs)
    }
}

impl<const BYTES: usize> BitOr<FixedBitSet<BYTES>> for &FixedBitSet<BYTES> {
    type Output = FixedBitSet<BYTES>;

    fn bitor(self, rhs: FixedBitSet<BYTES>) -> Self::Output {
        FixedBitSet::or(self, &rhs)
    }
}

impl<const BYTES: usize> BitOr for &FixedBitSet<BYTES> {
    type Output = FixedBitSet<BYTES>;

    fn bitor(self, rhs: Self) -> Self::Output {
        FixedBitSet::or(self, rhs)
    }
}

impl<const BYTES: usize> BitOrAssign<&Self> for FixedBitSet<BYTES> {
    fn bitor_assign(&mut self, rhs: &Self) {
        *self = Self::or(self, rhs);
    }
}

impl<const BYTES: usize> BitOrAssign for FixedBitSet<BYTES> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self |= &rhs;
    }
}

impl<const BYTES: usize> BitXor for FixedBitSet<BYTES> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::xor(&self, &rhs)
    }
}

impl<const BYTES: usize> BitXor<&FixedBitSet<BYTES>> for FixedBitSet<BYTES> {
    type Output = Self;

    fn bitxor(self, rhs: &Self) -> Self::Output {
        Self::xor(&self, rhs)
    }
}

impl<const BYTES: usize> BitXor<FixedBitSet<BYTES>> for &FixedBitSet<BYTES> {
    type Output = FixedBitSet<BYTES>;

    fn bitxor(self, rhs: FixedBitSet<BYTES>) -> Self::Output {
        FixedBitSet::xor(self, &rhs)
    }
}

impl<const BYTES: usize> BitXor for &FixedBitSet<BYTES> {
    type Output = FixedBitSet<BYTES>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        FixedBitSet::xor(self, rhs)
    }
}

impl<const BYTES: usize> BitXorAssign<&Self> for FixedBitSet<BYTES> {
    fn bitxor_assign(&mut self, rhs: &Self) {
        *self = Self::xor(self, rhs);
    }
}

impl<const BYTES: usize> BitXorAssign for FixedBitSet<BYTES> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self ^= &rhs;
    }
}

impl<const BYTES: usize> Not for FixedBitSet<BYTES> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::not(&self)
    }
}

impl<const BYTES: usize> Not for &FixedBitSet<BYTES> {
    type Output = FixedBitSet<BYTES>;

    fn not(self) -> Self::Output {
        FixedBitSet::not(self)
    }
}

#[inline]
fn bytes_for_bits(bits: usize) -> usize {
    bits.div_ceil(BITS_PER_BYTE)
}

#[inline]
fn bitset_index(index: usize) -> (usize, usize) {
    (index / BITS_PER_BYTE, index % BITS_PER_BYTE)
}

#[inline]
fn bit_len(byte_len: usize) -> usize {
    byte_len * BITS_PER_BYTE
}

#[inline]
fn bitwise_not(bits: &[u8]) -> Vec<u8> {
    bits.iter().map(|byte| !byte).collect()
}

#[inline]
fn bitwise_binary_array<const BYTES: usize>(
    bits: &[u8; BYTES],
    other: &[u8; BYTES],
    op: impl Fn(u8, u8) -> u8,
) -> [u8; BYTES] {
    let mut out = [0; BYTES];

    for (index, (left, right)) in bits.iter().zip(other.iter()).enumerate() {
        out[index] = op(*left, *right);
    }

    out
}

#[inline]
fn bitwise_not_array<const BYTES: usize>(bits: &[u8; BYTES]) -> [u8; BYTES] {
    let mut out = [0; BYTES];

    for (index, byte) in bits.iter().enumerate() {
        out[index] = !byte;
    }

    out
}

#[inline]
fn get(bits: &[u8], index: usize) -> Option<bool> {
    if index >= bit_len(bits.len()) {
        return None;
    }

    let (byte, bit) = bitset_index(index);

    Some((bits[byte] >> bit) & 1 == 1)
}

#[inline]
fn set(bits: &mut [u8], index: usize, value: bool) {
    assert!(index < bit_len(bits.len()), "Index out of bounds");

    let (byte, bit) = bitset_index(index);

    if value {
        bits[byte] |= 1 << bit;
    } else {
        bits[byte] &= !(1 << bit);
    }
}

#[inline]
fn set_range(bits: &mut [u8], range: Range<usize>, value: bool) {
    if range.is_empty() {
        return;
    }

    assert!(range.end <= bit_len(bits.len()), "Index out of bounds");

    for index in range {
        set(bits, index, value);
    }
}

#[inline]
fn clear(bits: &mut [u8]) {
    bits.fill(0);
}

#[inline]
fn count_ones(bits: &[u8]) -> usize {
    bits.iter().map(|byte| byte.count_ones() as usize).sum()
}

#[inline]
fn is_zeroed(bits: &[u8]) -> bool {
    bits.iter().all(|byte| *byte == 0)
}

#[inline]
fn iter(bits: &[u8]) -> impl Iterator<Item = bool> {
    bits.iter()
        .flat_map(|byte| (0..BITS_PER_BYTE).map(move |bit| (byte >> bit) & 1 == 1))
}

fn display_bitset(bits: &[u8], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[")?;
    for bit in iter(bits) {
        write!(f, "{}", if bit { '1' } else { '0' })?;
    }
    write!(f, "]")
}

#[cfg(test)]
mod test {
    use super::{BitSet, FixedBitSet};

    #[test]
    fn bitset_set_get_roundtrip() {
        let mut bitset = BitSet::new();

        bitset.set(0, true);
        bitset.set(7, true);
        bitset.set(8, true);
        bitset.set(15, false);

        assert_eq!(bitset.get(0), Some(true));
        assert_eq!(bitset.get(7), Some(true));
        assert_eq!(bitset.get(8), Some(true));
        assert_eq!(bitset.get(15), Some(false));
    }

    #[test]
    fn bitset_clear_and_is_zeroed() {
        let mut bitset = BitSet::new();

        assert!(bitset.is_zeroed());

        bitset.set(3, true);
        assert!(!bitset.is_zeroed());

        bitset.clear();
        assert!(bitset.is_zeroed());
        assert_eq!(bitset.bit_len(), 8);
    }

    #[test]
    fn bitset_iter_matches_bits() {
        let mut bitset = BitSet::new();

        bitset.set(0, true);
        bitset.set(3, true);
        bitset.set(8, true);

        let values: Vec<bool> = bitset.iter().collect();

        assert_eq!(values.len(), bitset.bit_len());
        assert!(values[0]);
        assert!(!values[1]);
        assert!(values[3]);
        assert!(values[8]);
    }

    #[test]
    fn bitset_set_range_and_count() {
        let mut bitset = BitSet::new();

        bitset.set_range(2..6, true);
        assert_eq!(bitset.count_ones(), 4);
        assert_eq!(bitset.get(2), Some(true));
        assert_eq!(bitset.get(5), Some(true));
        assert_eq!(bitset.get(6), Some(false));
    }

    #[test]
    fn bitset_capacity_is_reported_in_bits_and_bytes() {
        let bitset = BitSet::with_capacity(9);

        assert_eq!(bitset.byte_capacity(), 2);
        assert_eq!(bitset.bit_capacity(), 16);
        assert!(bitset.is_zeroed());
    }

    #[test]
    fn bitset_from_and_into_bytes_roundtrip() {
        let bitset = BitSet::from_bytes([0b0000_0101, 0b0000_0010]);

        assert_eq!(bitset.as_bytes(), &[0b0000_0101, 0b0000_0010]);
        assert_eq!(bitset.into_bytes(), vec![0b0000_0101, 0b0000_0010]);
    }

    #[test]
    fn bitset_get_out_of_bounds_returns_none() {
        let bitset = BitSet::new();

        assert_eq!(bitset.get(0), None);
    }

    #[test]
    fn bitset_display() {
        let mut bitset = BitSet::new();

        bitset.set(2, true);
        bitset.set(4, true);

        assert_eq!(format!("{bitset}"), "[00101000]");
    }

    #[test]
    fn fixed_bitset_set_get_roundtrip() {
        let mut bitset = FixedBitSet::<4>::new();

        bitset.set(0, true);
        bitset.set(7, true);
        bitset.set(8, true);
        bitset.set(15, false);

        assert_eq!(bitset.get(0), Some(true));
        assert_eq!(bitset.get(7), Some(true));
        assert_eq!(bitset.get(8), Some(true));
        assert_eq!(bitset.get(15), Some(false));
    }

    #[test]
    fn fixed_bitset_clear_and_is_zeroed() {
        let mut bitset = FixedBitSet::<4>::new();

        assert!(bitset.is_zeroed());

        bitset.set(3, true);
        assert!(!bitset.is_zeroed());

        bitset.clear();
        assert!(bitset.is_zeroed());
    }

    #[test]
    fn fixed_bitset_iter_matches_bits() {
        let mut bitset = FixedBitSet::<4>::new();

        bitset.set(0, true);
        bitset.set(3, true);
        bitset.set(8, true);

        let values: Vec<bool> = bitset.iter().collect();

        assert_eq!(values.len(), bitset.bit_len());
        assert!(values[0]);
        assert!(!values[1]);
        assert!(values[3]);
        assert!(values[8]);
    }

    #[test]
    fn fixed_bitset_set_range_and_count() {
        let mut bitset = FixedBitSet::<4>::new();

        bitset.set_range(2..6, true);
        assert_eq!(bitset.count_ones(), 4);
        assert_eq!(bitset.get(2), Some(true));
        assert_eq!(bitset.get(5), Some(true));
        assert_eq!(bitset.get(6), Some(false));
    }

    #[test]
    fn fixed_bitset_display() {
        let mut bitset = FixedBitSet::<1>::new();

        bitset.set(2, true);
        bitset.set(4, true);

        assert_eq!(format!("{bitset}"), "[00101000]");
    }

    #[test]
    fn fixed_bitset_from_and_into_bytes_roundtrip() {
        let bitset = FixedBitSet::<2>::from_bytes([0b0000_0101, 0b0000_0010]);

        assert_eq!(bitset.as_bytes(), &[0b0000_0101, 0b0000_0010]);
        assert_eq!(bitset.into_bytes(), [0b0000_0101, 0b0000_0010]);
    }

    #[test]
    fn bitset_trait_conversions() {
        let bitset = BitSet::from([0b0000_0101, 0b0000_0010]);

        assert_eq!(bitset.as_ref(), &[0b0000_0101, 0b0000_0010]);
        assert_eq!(Vec::<u8>::from(bitset), vec![0b0000_0101, 0b0000_0010]);
    }

    #[test]
    fn fixed_bitset_trait_conversions() {
        let bitset = FixedBitSet::<2>::from([0b0000_0101, 0b0000_0010]);

        assert_eq!(AsRef::<[u8]>::as_ref(&bitset), &[0b0000_0101, 0b0000_0010]);
        assert_eq!(
            AsRef::<[u8; 2]>::as_ref(&bitset),
            &[0b0000_0101, 0b0000_0010]
        );
        assert_eq!(<[u8; 2]>::from(bitset), [0b0000_0101, 0b0000_0010]);
    }

    #[test]
    fn fixed_bitset_bitwise_operations() {
        let left = FixedBitSet::<2>::from([0b0000_1100, 0b1010_1010]);
        let right = FixedBitSet::<2>::from([0b0000_1010, 0b1100_1100]);

        assert_eq!((left & right).as_bytes(), &[0b0000_1000, 0b1000_1000]);
        assert_eq!((left | right).as_bytes(), &[0b0000_1110, 0b1110_1110]);
        assert_eq!((left ^ right).as_bytes(), &[0b0000_0110, 0b0110_0110]);
        assert_eq!((!&left).as_bytes(), &[0b1111_0011, 0b0101_0101]);
    }

    #[test]
    fn fixed_bitset_bitwise_assign_operations() {
        let original = FixedBitSet::<2>::from([0b0000_1100, 0b1010_1010]);
        let other = FixedBitSet::<2>::from([0b0000_1010, 0b1100_1100]);

        let mut anded = original;
        anded &= other;
        assert_eq!(anded.as_bytes(), &[0b0000_1000, 0b1000_1000]);

        let mut ored = original;
        ored |= other;
        assert_eq!(ored.as_bytes(), &[0b0000_1110, 0b1110_1110]);

        let mut xored = original;
        xored ^= other;
        assert_eq!(xored.as_bytes(), &[0b0000_0110, 0b0110_0110]);
    }
}
