use std::path::Path;

use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

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

use crate::error::BetterIoError;

#[derive(Debug, thiserror::Error)]
pub enum ArchiveDownloadError {
    #[error(transparent)]
    Io(#[from] BetterIoError),
    #[error("Network error. Reason: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Network error. Failed to request file")]
    FailedRequest,
    #[cfg(target_os = "windows")]
    #[error(transparent)]
    Zip(#[from] ::zip::result::ZipError),
}

async fn download_file(
    client: &reqwest::Client,
    url: &str,
    location: &Path,
) -> Result<(), ArchiveDownloadError> {
    // Send HEAD request to get file size
    let head_response = client.head(url).send().await?;

    let total_size = head_response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|val| val.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or(ArchiveDownloadError::FailedRequest)?;

    let is_exists = tokio::fs::try_exists(location)
        .await
        .map_err(|error| BetterIoError {
            location: location.to_path_buf(),
            context: "checking is file exists before download starts",
            error,
        })?;

    if is_exists {
        match tokio::fs::metadata(location).await {
            Ok(meta) => {
                const LOG_MESSAGE: &str =
                    "Existing download with matching size found. Proceeding with unpack";

                #[cfg(target_family = "unix")]
                {
                    use std::os::unix::fs::MetadataExt;

                    if meta.size() == total_size {
                        tracing::warn!("{LOG_MESSAGE}");
                    }
                }

                #[cfg(target_family = "windows")]
                if meta.len() == total_size {
                    tracing::warn!("{LOG_MESSAGE}");
                }

                return Ok(());
            }
            _ => (),
        }
    }

    // Start the actual download
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(ArchiveDownloadError::FailedRequest);
    }

    let mut file = tokio::fs::File::create(location)
        .await
        .map_err(|error| BetterIoError {
            location: location.to_path_buf(),
            context: "creation of file descriptor",
            error,
        })?;

    let mut stream = response.bytes_stream();
    let mut downloaded = 0u64;
    let start_time = tokio::time::Instant::now();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)
            .await
            .map_err(|error| BetterIoError {
                location: location.to_path_buf(),
                context: "saving of downloaded chunk",
                error,
            })?;

        downloaded += chunk.len() as u64;

        // Print progress every 1MB
        if downloaded % (1024 * 1024) == 0 || downloaded == total_size {
            let elapsed = start_time.elapsed().as_secs_f64();
            let speed = downloaded as f64 / elapsed / 1024.0 / 1024.0; // MB/s
            let progress = if total_size > 0 {
                (downloaded as f64 / total_size as f64 * 100.0) as u32
            } else {
                0
            };

            tracing::debug!(
                "Downloaded: {:.1} MB | Progress: {}% | Speed: {:.1} MB/s",
                downloaded as f64 / 1024.0 / 1024.0,
                progress,
                speed
            );
        }
    }

    file.flush().await.map_err(|error| BetterIoError {
        location: location.to_path_buf(),
        context: "flushing file descriptor",
        error,
    })?;

    tracing::info!(
        "Download completed in {:.2} seconds",
        start_time.elapsed().as_secs_f64()
    );

    Ok(())
}

#[cfg(any(target_os = "linux", all(target_os = "macos", target_arch = "x86_64")))]
fn unpack_archive(tar_location: &Path, target_dir: &Path) -> Result<(), ArchiveDownloadError> {
    let file = std::fs::File::open(&tar_location).map_err(|error| BetterIoError {
        location: tar_location.to_path_buf(),
        context: "opening archive descriptor",
        error,
    })?;

    tracing::info!("Starting unpacking of archive - {}", tar_location.display());

    let decoder = flate2::read::MultiGzDecoder::new(file);

    let mut archive = tar::Archive::new(decoder);

    archive.unpack(target_dir).map_err(|error| BetterIoError {
        location: target_dir.to_path_buf(),
        context: "unpacking of archive",
        error,
    })?;

    tracing::info!("Finished unpacking of archive - {}", tar_location.display());

    Ok(())
}
