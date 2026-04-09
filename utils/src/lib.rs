//! utility types and traits built on top of the standard library.

pub use bitset;
pub use channel;
pub use future;
pub use io;
pub use log;
#[cfg(target_os = "linux")]
pub use meta;
/// re-exports synchronization primitives from `parking_lot`.
pub mod sync {
    pub use parking_lot::{Mutex, RwLock};
}
