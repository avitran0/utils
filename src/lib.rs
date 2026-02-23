pub mod channel;
pub mod future;
pub mod io;
pub mod log;
#[cfg(target_os = "linux")]
pub mod meta;

embed_metadata!(METADATA, ".meta", &[0, 6]);
