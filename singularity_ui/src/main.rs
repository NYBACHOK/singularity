// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, LazyLock};

use singularity_model_builder::TextMessages;
use slint::{ModelRc, ToSharedString, VecModel};

slint::include_modules!();

const MODEL_ID: &str = "Qwen/Qwen3-4B-GGUF";
const MODEL_FILENAME: &str = "Qwen3-4B-Q4_K_M.gguf";

static TOKIO_RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .expect("failed to start runtime")
});

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let ui = MainWindow::new()?;

    let messages = VecModel::from_iter([ChatMessage {
        is_user: false,
        text: "Hello! How can I help you?".into(),
    }]);

    ui.set_messages(ModelRc::new(messages));

    let model = Arc::new(
        TOKIO_RUNTIME
            .block_on(
                singularity_model_builder::Builder::new(dirs::cache_dir().unwrap())
                    .build_guff_model(MODEL_ID, MODEL_FILENAME),
            )
            .unwrap(),
    );

    ui.on_send_clicked({
        let ui = ui.clone_strong();

        {
            let ui = ui.clone_strong();

            move |this| {
                let model = model.clone();
                let ui = ui.clone_strong();
                let _res = slint::spawn_local(async move {
                    dbg!("started prompt");
                    let text = async_compat::Compat::new(model.send_chat_request(
                        TextMessages::new().add_message(
                            singularity_model_builder::TextMessageRole::User,
                            this.to_string(),
                        ),
                    ))
                    .await
                    .unwrap()
                    .choices
                    .pop()
                    .unwrap()
                    .message
                    .content
                    .unwrap();

                    dbg!(&text);

                    let messages = VecModel::from_iter([ChatMessage {
                        is_user: true,
                        text: text.to_shared_string(),
                    }]);

                    ui.set_messages(ModelRc::new(messages));
                });

                dbg!(_res.is_err());
            }
        }
    });

    ui.run()?;

    Ok(())
}
