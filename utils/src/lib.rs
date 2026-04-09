//! utility types and traits built on top of the standard library.

#[cfg(feature = "bitset")]
pub use bitset;

#[cfg(feature = "channel")]
pub use channel;

#[cfg(feature = "future")]
pub use future;

#[cfg(feature = "io")]
pub use io;

#[cfg(feature = "log")]
pub use log;

#[cfg(all(feature = "meta", target_os = "linux"))]
pub use meta;

#[cfg(all(feature = "path", target_os = "linux"))]
pub use path;

/// re-exports synchronization primitives from `parking_lot`.
#[cfg(feature = "sync")]
pub mod sync {
    pub use parking_lot::{Mutex, RwLock};
}

pub const fn is_debug() -> bool {
    cfg!(debug_assertions)
}

pub const fn is_test() -> bool {
    cfg!(test)
}
