//! utility types and traits built on top of the standard library.

pub mod bitset;
pub mod channel;
pub mod future;
pub mod io;
pub mod log;
#[cfg(target_os = "linux")]
pub mod meta;
pub mod sync;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod uuid;
