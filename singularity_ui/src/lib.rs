mod commands;
mod core;
mod error;
use std::rc::Rc;

use slint::{ModelRc, VecModel};

slint::include_modules!();

const APP_ID: &str = env!("APP_ID");

pub fn setup_app() -> Result<App, Box<dyn std::error::Error>> {
    let ui = App::new()?;

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
