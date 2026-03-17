pub struct Uuid(u128);

impl Uuid {
    pub fn v4() -> Self {
        let bytes = rand();
        Self(u128::from_le_bytes(bytes))
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
    use crate::uuid::rand;

    #[test]
    fn test_rng() {
        let value = u128::from_le_bytes(rand());
        assert_ne!(value, 0);
    }
}
