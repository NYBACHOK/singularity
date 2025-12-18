// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::rc::Rc;

use slint::{ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let ui = MainWindow::new()?;

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

    ui.run()?;

    Ok(())
}
