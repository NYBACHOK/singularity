use std::path::{Path, PathBuf};

use crate::{error::BetterIoError, llm::download::ArchiveDownloadError};

#[cfg(target_arch = "aarch64")]
const OLLAMA_LINK: &str = "https://ollama.com/download/ollama-windows-arm64.zip";
#[cfg(target_arch = "x86_64")]
const OLLAMA_LINK: &str = "https://ollama.com/download/ollama-windows-amd64.zip";

#[cfg(target_arch = "x86_64")]
const AMD_GPU_ADDON: &str = "https://ollama.com/download/ollama-windows-amd64-rocm.zip";

const OLLAMA_DOWNLOAD_FILENAME: &str = "ollama.zip";
const OLLAMA_ROCM_DOWNLOAD_FILENAME: &str = "ollama_rocm.zip";

pub async fn ollama_download(
    cache_dir: impl AsRef<Path>,
    target_dir: impl AsRef<Path>,
) -> Result<PathBuf, ArchiveDownloadError> {
    let client = reqwest::Client::new();

    let rocm = !crate::llm::utils::is_nvidia().await;

    let cache_dir = cache_dir.as_ref();

    let ollama_location = cache_dir.join(OLLAMA_DOWNLOAD_FILENAME);
    super::download_file(&client, OLLAMA_LINK, &ollama_location).await?;
    unpack_archive(&ollama_location, target_dir.as_ref())?;

    if rocm {
        let ollama_rocm_location = cache_dir.join(OLLAMA_ROCM_DOWNLOAD_FILENAME);

        super::download_file(&client, AMD_GPU_ADDON, &ollama_rocm_location).await?;
        unpack_archive(&ollama_rocm_location, target_dir.as_ref())?;
    }

    Ok(ollama_location)
}

fn unpack_archive(tar_location: &Path, target_dir: &Path) -> Result<(), ArchiveDownloadError> {
    let file = std::fs::File::open(&tar_location).map_err(|error| BetterIoError {
        location: tar_location.to_path_buf(),
        context: "opening archive descriptor",
        error,
    })?;

    tracing_log::log::info!("Starting unpacking of archive - {}", tar_location.display());

    let mut archive = zip::ZipArchive::new(file)?;

    archive.extract(target_dir)?;

    tracing_log::log::info!("Finished unpacking of archive - {}", tar_location.display());

    Ok(())
}
