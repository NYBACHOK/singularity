use mistralrs::Model;

use crate::Builder;

impl Builder {
    pub async fn build_guff_model(
        &self,
        model_id: impl Into<String>,
        model_file: impl AsRef<str>,
    ) -> anyhow::Result<Model> {
        let model = mistralrs::GgufModelBuilder::new(model_id.into(), vec![model_file.as_ref()])
            .with_logging()
            .with_token_source(mistralrs::TokenSource::Path(
                self.cache_dir.display().to_string(),
            ))
            .build()
            .await
            .unwrap();

        Ok(model)
    }
}
