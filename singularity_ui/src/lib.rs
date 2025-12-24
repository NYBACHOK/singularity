use std::{rc::Rc, sync::LazyLock};

use slint::{ModelRc, VecModel};

use crate::core::llm::llm_download;

mod core;
mod error;

slint::include_modules!();

const APP_ID: &str = env!("APP_ID");

pub static TOKIO_RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .expect("Critical error. Failed to start tokio runtime")
});

pub fn setup_app() -> Result<App, Box<dyn std::error::Error>> {
    let ui = App::new()?;

    let is_ollama_installed = TOKIO_RUNTIME.block_on(core::llm::version())?.is_some();

    ui.set_show_download_warning(is_ollama_installed);

    ui.on_download_accepted({
        let ui = ui.clone_strong();

        move || {
            let _ = slint::spawn_local({
                let ui = ui.clone_strong();

                async move {
                    let _ = async_compat::Compat::new(llm_download())
                        .await
                        .inspect_err(|e| tracing::error!("Failed to download. Reason: {e}"));
                }
            })
            .inspect_err(|e| tracing::error!("Failed to start download. Reason: {e}"));
        }
    });

    let messages = Rc::new(VecModel::from(vec![ChatMessage {
        text: "Hello! How can I help you?".into(),
        is_user: false,
    }]));

    let messages_rc: ModelRc<ChatMessage> = messages.clone().into();
    ui.set_messages(messages_rc);

    ui.on_send_clicked({
        let messages = messages.clone();
        move |text| {
            messages.push(ChatMessage {
                is_user: true,
                text,
            });
        }
    });

    Ok(ui)
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_app()?;

    app.run()?;

    Ok(())
}
