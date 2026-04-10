//! utility types and traits built on top of the standard library.

#[cfg(feature = "bitset")]
pub mod bitset;

#[cfg(feature = "channel")]
pub mod channel;

#[cfg(feature = "future")]
pub mod future;

#[cfg(feature = "io")]
pub mod io;

#[cfg(feature = "log")]
pub mod log;

#[cfg(all(feature = "meta", target_family = "unix"))]
pub mod meta;

#[cfg(all(feature = "path", target_os = "linux"))]
pub mod path;

#[cfg(feature = "sync")]
pub mod sync;

pub const fn is_debug() -> bool {
    cfg!(debug_assertions)
}

pub const fn is_test() -> bool {
    cfg!(test)
}
