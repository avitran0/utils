//! simple uuid implementation, for linux and windows only

use std::fmt::{self};

pub struct Uuid(u128);

impl Uuid {
    /// returns a randomly generated uuid.
    /// uses `getrandom` on linux, and `ProcessPrng` on windows.
    pub fn v4() -> Self {
        let mut bytes = rand();
        // sets bits for version and clock_seq...
        bytes[6] = (bytes[6] & 0x0F) | 0x40;
        bytes[8] = (bytes[8] & 0x3F) | 0x80;
        Self(u128::from_be_bytes(bytes))
    }

    pub fn from_u128(value: u128) -> Self {
        Self(value)
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(u128::from_be_bytes(bytes))
    }
}

impl std::fmt::Debug for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::LowerHex for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.0.to_be_bytes();
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0],
            b[1],
            b[2],
            b[3],
            b[4],
            b[5],
            b[6],
            b[7],
            b[8],
            b[9],
            b[10],
            b[11],
            b[12],
            b[13],
            b[14],
            b[15]
        )
    }
}

#[cfg(target_os = "linux")]
fn rand() -> [u8; 16] {
    unsafe extern "C" {
        fn getrandom(buf: *mut u8, size: usize, flags: u32) -> isize;
    }

    let mut buf = [0; 16];
    let result = unsafe { getrandom(buf.as_mut_ptr(), buf.len(), 0) };

    if result < 16 {
        let error = std::io::Error::last_os_error();
        panic!("getrandom failed: {error}");
    }

    buf
}

#[cfg(target_os = "windows")]
fn rand() -> [u8; 16] {
    unsafe extern "system" {
        fn ProcessPrng(buf: *mut u8, size: usize) -> i32;
    }

    let mut buf = [0; 16];
    let result = unsafe { ProcessPrng(buf.as_mut_ptr(), buf.len()) };

    if result == 0 {
        let error = std::io::Error::last_os_error();
        panic!("ProcessPrng failed: {error}");
    }

    buf
}

#[cfg(target_os = "macos")]
fn rand() -> [u8; 16] {
    unsafe extern "C" {
        fn getentropy(buf: *mut u8, size: usize) -> i32;
    }

    let mut buf = [0; 16];
    let result = unsafe { getentropy(buf.as_mut_ptr(), buf.len()) };

    if result != 0 {
        let error = std::io::Error::last_os_error();
        panic!("getentropy failed: {error}");
    }

    buf
}

#[cfg(test)]
mod test {
    use crate::uuid::{Uuid, rand};

    #[test]
    fn rng() {
        let first = rand();
        let second = rand();

        assert_ne!(u128::from_be_bytes(first), 0);
        assert_ne!(u128::from_be_bytes(second), 0);
        assert_ne!(first, second);
    }

    #[test]
    fn display() {
        let uuid = Uuid::from_u128(127);
        assert_eq!(format!("{uuid}"), "00000000-0000-0000-0000-00000000007f");
    }

    #[test]
    fn preserve_be_layout() {
        let bytes = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88,
        ];
        let uuid = Uuid::from_bytes(bytes);

        assert_eq!(uuid.0.to_be_bytes(), bytes);
        assert_eq!(format!("{uuid}"), "12345678-9abc-def0-1122-334455667788");
    }

    #[test]
    fn version_variant_bits() {
        for _ in 0..32 {
            let uuid = Uuid::v4();
            let bytes = uuid.0.to_be_bytes();

            assert_eq!(bytes[6] >> 4, 0b0100);
            assert_eq!(bytes[8] >> 6, 0b10);
        }
    }
}
