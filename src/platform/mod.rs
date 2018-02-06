#[cfg(target_os = "windows")]
#[path="windows/mod.rs"]
pub mod platform;

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
#[path="linux/mod.rs"]
pub mod platform;

#[cfg(target_os = "macos")]
#[path="macos/mod.rs"]
pub mod platform;