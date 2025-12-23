use std::path::{Path, PathBuf};

use crate::llm::download::ArchiveDownloadError;

#[cfg(target_arch = "aarch64")]
const OLLAMA_LINK: &str = "https://ollama.com/download/Ollama.dmg";
#[cfg(target_arch = "x86_64")]
const OLLAMA_LINK: &str = "https://ollama.com/download/ollama-darwin.tgz";

#[cfg(target_arch = "aarch64")]
const OLLAMA_DOWNLOAD_FILENAME: &str = "ollama.dmg";
#[cfg(target_arch = "x86_64")]
const OLLAMA_DOWNLOAD_FILENAME: &str = "ollama.tgz";

pub async fn ollama_download(
    cache_dir: impl AsRef<Path>,
    _target_dir: impl AsRef<Path>,
) -> Result<PathBuf, ArchiveDownloadError> {
    let client = reqwest::Client::new();

    let cache_dir = cache_dir.as_ref();

    let ollama_location = cache_dir.join(OLLAMA_DOWNLOAD_FILENAME);
    super::download_file(&client, OLLAMA_LINK, &ollama_location).await?;

    #[cfg(target_arch = "x86_64")]
    {
        const TAR_UNPACK_DIR: &str = "ollama";

        let unpack_dir = _target_dir.as_ref().join(TAR_UNPACK_DIR);
        super::unpack_archive(&ollama_location, &unpack_dir)?;

        return Ok(unpack_dir.join("ollama"));
    }

    Ok(ollama_location)
}
