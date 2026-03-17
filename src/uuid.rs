use std::fmt::{self};

pub struct Uuid(u128);

impl Uuid {
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
    unsafe {
        getrandom(buf.as_mut_ptr().cast(), 16, 2);
    }
    buf
}

#[cfg(target_os = "windows")]
fn rand() -> [u8; 16] {
    unsafe extern "system" {
        fn ProcessPrng(buf: *mut u8, size: usize) -> i32;
    }
    let mut buf = [0; 16];
    unsafe {
        ProcessPrng(buf.as_mut_ptr(), 16);
    }
    buf
}

#[cfg(test)]
mod test {
    use crate::uuid::{Uuid, rand};

    #[test]
    fn test_rng() {
        let value = u128::from_le_bytes(rand());
        assert_ne!(value, 0);
    }

    #[test]
    fn test_display() {
        let uuid = Uuid::from_u128(127);
        assert_eq!(format!("{uuid}"), "00000000-0000-0000-0000-00000000007f");
    }
}
