#[cfg(any(target_os = "linux", target_os = "windows"))]
mod nvidia;

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub use nvidia::*;
