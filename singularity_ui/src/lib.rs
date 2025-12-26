use std::{rc::Rc, sync::LazyLock};

use slint::{ModelRc, ToSharedString, VecModel};

use crate::core::llm::{llm_download, llm_download_model, llm_generate, llm_load};

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
    let is_ollama_installed = TOKIO_RUNTIME.block_on(core::llm::version())?.is_some();

    let ui = App::new()?;

    ui.set_show_download_warning(is_ollama_installed);
    ui.set_finished_loading(is_ollama_installed);

    ui.on_download_accepted({
        let ui = ui.clone_strong();

        move || {
            let res = slint::spawn_local({
                let ui = ui.clone_strong();

                async move {
                    let _ = async_compat::Compat::new(llm_download())
                        .await
                        .inspect_err(|e| {
                            let _ = slint::quit_event_loop();
                            tracing::error!("Failed to download ollama. Reason: {e}");
                        });

                    let _ = async_compat::Compat::new(llm_load())
                        .await
                        .inspect_err(|e| {
                            let _ = slint::quit_event_loop();
                            tracing::error!("Failed to start ollama. Reason: {e}");
                        });

                    let _ = async_compat::Compat::new(llm_download_model())
                        .await
                        .inspect_err(|e| {
                            let _ = slint::quit_event_loop();
                            tracing::error!("Failed to download model. Reason: {e}");
                        });

                    ui.set_finished_loading(true);
                }
            })
            .inspect_err(|e| tracing::error!("Failed to start download. Reason: {e}"));

            if res.is_err() {
                let _ = slint::quit_event_loop();
            }
        }
    });

    let messages = Rc::new(VecModel::from(Vec::new()));

    let messages_rc: ModelRc<ChatMessage> = messages.clone().into();
    ui.set_messages(messages_rc);

    ui.on_send_clicked({
        let messages = messages.clone();
        move |text| {
            messages.push(ChatMessage {
                is_user: true,
                text: text.clone(),
            });

            let _ = slint::spawn_local({
                let messages = messages.clone();

                async move {
                    let res = async_compat::Compat::new(llm_load()).await;

                    if res.is_err() {
                        let _ = slint::quit_event_loop();
                    }

                    let res = async_compat::Compat::new(llm_generate(text.to_string())).await;

                    let text = match res {
                        Ok(text) => text,
                        Err(e) => {
                            tracing::error!("Failed to generate msg. Reason: {e}");
                            let _ = slint::quit_event_loop();
                            return;
                        }
                    }
                    .to_shared_string();

                    messages.push(ChatMessage {
                        is_user: false,
                        text,
                    });
                }
            })
            .inspect_err(|e| tracing::error!("Failed to generate msg. Reason: {e}"));
        }
    });

    Ok(ui)
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_app()?;

    app.run()?;

    Ok(())
}
