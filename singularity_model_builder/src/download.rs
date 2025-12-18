use std::path::PathBuf;

use hf_hub::api::tokio::ApiError;

pub use hf_hub::api::tokio::Progress;

use crate::Builder;

impl Builder {
    pub async fn download_model(
        &self,
        model_id: impl Into<String>,
        model_file: impl AsRef<str>,
    ) -> Result<PathBuf, ApiError> {
        let api = hf_hub::api::tokio::ApiBuilder::new()
            .with_cache_dir(self.cache_dir.clone())
            .with_progress(false)
            .build()?;

        let repo = api.model(model_id.into());
        let filename = repo.download(model_file.as_ref()).await?;

        Ok(filename)
    }

    pub async fn download_model_with_progress<P: Progress + Clone + Send + Sync + 'static>(
        &self,
        model_id: impl Into<String>,
        model_file: impl AsRef<str>,
        progress: P,
    ) -> Result<PathBuf, ApiError> {
        let api = hf_hub::api::tokio::ApiBuilder::new()
            .with_cache_dir(self.cache_dir.clone())
            .with_progress(true)
            .build()?;

        let repo = api.model(model_id.into());
        let filename = repo
            .download_with_progress(model_file.as_ref(), progress)
            .await?;

        Ok(filename)
    }

    pub fn model_from_cache(
        &self,
        model_id: impl Into<String>,
        model_file: impl AsRef<str>,
    ) -> Option<PathBuf> {
        hf_hub::Cache::new(self.cache_dir.clone())
            .model(model_id.into())
            .get(model_file.as_ref())
    }
}
