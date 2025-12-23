use std::{path::Path, process::Stdio};

use tokio::process::Child;

use crate::error::BetterIoError;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(target_os = "windows")]
pub use windows::*;

pub fn ollama_serve(ollama_dir: impl AsRef<Path>) -> Result<Child, BetterIoError> {
    let bin = ollama_binary_location(ollama_dir.as_ref());

    tracing::info!("starting ollama binary in - {}", bin.display());

    tokio::process::Command::new(bin)
        .kill_on_drop(true)
        .arg("serve")
        .stderr(Stdio::inherit())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| BetterIoError::new(ollama_dir.as_ref(), "start of ollama binary", e))
}
