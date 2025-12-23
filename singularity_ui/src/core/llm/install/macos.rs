use std::path::Path;

use tokio::process::Command;

use crate::error::BetterIoError;

pub async fn ollama_install(ollama_dmg: impl AsRef<Path>) -> Result<(), BetterIoError> {
    let res = Command::new("hdiutil")
        .arg("attach")
        .arg(ollama_dmg.as_ref())
        .spawn()
        .map_err(|e| {
            BetterIoError::new(
                ollama_dmg.as_ref(),
                "mount ollama disk image for installation",
                e,
            )
        })?
        .wait()
        .await;

    match res {
        Ok(status) if status.success() => (),
        _ => {
            return Err(BetterIoError::new(
                ollama_dmg.as_ref(),
                "mount ollama disk image for installation",
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "failed to mount drive image(dmg)",
                ),
            ))
        }
    }

    let res = Command::new("cp")
        .args([
            "-R",
            "/Volumes/Ollama/Ollama.app",
            "/Applications/Ollama.app",
        ])
        .spawn()
        .map_err(|e| {
            BetterIoError::new(
                ollama_dmg.as_ref(),
                "copy ollama app to applications list",
                e,
            )
        })?
        .wait()
        .await;

    match res {
        Ok(status) if status.success() => (),
        _ => {
            return Err(BetterIoError::new(
                ollama_dmg.as_ref(),
                "waiting for copy to finish or other",
                std::io::Error::new(std::io::ErrorKind::Other, "failed to copy"),
            ))
        }
    }

    let _ = Command::new("hdiutil")
        .arg("detach")
        .arg("/Volumes/Ollama")
        .spawn()
        .inspect_err(|e| {
             tracing_log::log::error!("Failed to unmount ollama disk image from system. Manual action needed. Reason: {e}")
        });

    Ok(())
}
