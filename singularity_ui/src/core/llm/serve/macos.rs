use std::path::{Path, PathBuf};

#[inline]
#[cfg(target_arch = "aarch64")]
pub fn ollama_binary_location(ollama_dir: &Path) -> PathBuf {
    ollama_dir.join("Ollama.app/Contents/Resources/ollama")
}
