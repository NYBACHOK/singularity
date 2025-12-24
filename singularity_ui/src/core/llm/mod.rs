use std::{
    path::PathBuf,
    sync::{LazyLock, Mutex, atomic::AtomicBool},
};

use ollama_rs::generation::completion::request::GenerationRequest;

use crate::{
    APP_ID,
    core::llm::{download::ollama_download, serve::ollama_serve},
    error::BetterIoError,
};

pub mod download;
pub mod install;
pub mod serve;
pub mod utils;

const OLLAMA_DATA_DIR: &str = "ollama";
const MODEL_NAME: &str = "gemma3:1b";

static IS_OLLAMA_LOADED: AtomicBool = AtomicBool::new(false);

static OLLAMA_BACKEND: Mutex<Option<tokio::process::Child>> = Mutex::new(Option::None);

static OLLAMA_CLIENT: LazyLock<ollama_rs::Ollama> = LazyLock::new(|| ollama_rs::Ollama::default());

/// Returns string with ollama installed version. None means that ollama probably not installed or missing in $PATH env
async fn ollama_version(binary_dir: Option<PathBuf>) -> Option<String> {
    let ollama_bin = match binary_dir {
        Some(dir) => serve::ollama_binary_location(&dir),
        None => "ollama".into(),
    };

    let res = tokio::process::Command::new(ollama_bin)
        .arg("-v")
        .output()
        .await
        .ok()?;

    if !res.status.success() {
        return None;
    }

    let version = String::from_utf8(res.stdout)
        .ok()
        .map(|this| {
            this.to_lowercase()
                .replace("ollama version is ", "")
                .replace(
                    "warning: could not connect to a running ollama instance",
                    "",
                )
                .replace("warning: client version is ", "")
                .replace("\n", "")
        })
        .unwrap_or("UNKNOWN".to_owned());

    if version.is_empty() {
        return None;
    }

    Some(version)
}

async fn get_or_create_app_dir(root: Option<PathBuf>) -> Result<PathBuf, BetterIoError> {
    let path = root
        .unwrap_or(dirs::data_dir().expect("invalid os"))
        .join(APP_ID);

    if !tokio::fs::try_exists(&path)
        .await
        .map_err(|e| BetterIoError::new(&path, "failed to check application dir", e))?
    {
        tokio::fs::create_dir(&path)
            .await
            .map_err(|e| BetterIoError::new(&path, "failed to create application dir", e))?
    }

    Ok(path)
}

pub async fn version() -> anyhow::Result<Option<String>> {
    let ollama_dir = get_or_create_app_dir(None).await?.join(OLLAMA_DATA_DIR);

    let version = ollama_version(Some(ollama_dir)).await;

    Ok(version)
}

pub async fn llm_download() -> anyhow::Result<()> {
    let cache_dir = get_or_create_app_dir(Some(dirs::cache_dir().expect("invalid os"))).await?;
    let target_dir = get_or_create_app_dir(None).await?.join(OLLAMA_DATA_DIR);

    let _ollama_location = ollama_download(cache_dir, target_dir).await?;

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    install::ollama_install(_ollama_location).await?;

    Ok(())
}

pub async fn llm_download_model() -> anyhow::Result<String> {
    if !IS_OLLAMA_LOADED.load(std::sync::atomic::Ordering::SeqCst) {
        return Err(anyhow::anyhow!("You need to start llm engine first"));
    }

    let msg = OLLAMA_CLIENT
        .pull_model(MODEL_NAME.to_owned(), false)
        .await?
        .message;

    Ok(msg)
}

pub async fn llm_load() -> anyhow::Result<()> {
    let ollama_dir = get_or_create_app_dir(None).await?.join(OLLAMA_DATA_DIR);

    let mut ollama_backend_lock = OLLAMA_BACKEND.lock().expect("POISONED LOCK");

    if ollama_backend_lock.is_none() {
        let child = ollama_serve(ollama_dir)?;

        *ollama_backend_lock = Some(child)
    } else {
        tracing::warn!("Tried to init new ollama instance while old is still running")
    }

    IS_OLLAMA_LOADED.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

pub async fn llm_unload() -> anyhow::Result<()> {
    let mut ollama_backend_lock = OLLAMA_BACKEND.lock().expect("POISONED LOCK");

    let _ = ollama_backend_lock.take();

    IS_OLLAMA_LOADED.store(false, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

pub async fn llm_generate(prompt: String) -> anyhow::Result<String> {
    if !IS_OLLAMA_LOADED.load(std::sync::atomic::Ordering::SeqCst) {
        return Err(anyhow::anyhow!("You need to start llm engine first"));
    }

    let res = OLLAMA_CLIENT
        .generate(GenerationRequest::new(MODEL_NAME.to_owned(), prompt))
        .await?
        .response;

    Ok(res)
}
