use std::path::{Path, PathBuf};

#[inline]
pub fn ollama_binary_location(ollama_dir: &Path) -> PathBuf {
    ollama_dir.join("bin/ollama")
}
