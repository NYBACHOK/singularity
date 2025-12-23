use std::path::{Path, PathBuf};

use crate::core::llm::download::ArchiveDownloadError;

#[cfg(target_arch = "aarch64")]
const OLLAMA_LINK: &str = "https://ollama.com/download/ollama-linux-arm64.tgz";
#[cfg(target_arch = "x86_64")]
const OLLAMA_LINK: &str = "https://ollama.com/download/ollama-linux-amd64.tgz";
#[cfg(target_arch = "x86_64")]
const AMD_GPU_ADDON: &str = "https://ollama.com/download/ollama-linux-amd64-rocm.tgz";

const OLLAMA_DOWNLOAD_FILENAME: &str = "ollama.tgz";
const OLLAMA_ROCM_DOWNLOAD_FILENAME: &str = "ollama_rocm.tgz";

pub async fn ollama_download(
    cache_dir: impl AsRef<Path>,
    target_dir: impl AsRef<Path>,
) -> Result<PathBuf, ArchiveDownloadError> {
    let client = reqwest::Client::new();

    let rocm = !crate::core::llm::utils::is_nvidia().await;

    let cache_dir = cache_dir.as_ref();

    let ollama_location = cache_dir.join(OLLAMA_DOWNLOAD_FILENAME);
    super::download_file(&client, OLLAMA_LINK, &ollama_location).await?;
    super::unpack_archive(&ollama_location, target_dir.as_ref())?;

    if rocm {
        let ollama_rocm_location = cache_dir.join(OLLAMA_ROCM_DOWNLOAD_FILENAME);

        super::download_file(&client, AMD_GPU_ADDON, &ollama_rocm_location).await?;
        super::unpack_archive(&ollama_rocm_location, target_dir.as_ref())?;
    }

    Ok(ollama_location)
}
