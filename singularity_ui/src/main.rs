// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::{ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let ui = MainWindow::new()?;

    ui.set_messages(ModelRc::new(VecModel::from_iter([ChatMessage {
        is_user: false,
        text: "Hello! How can I help you?".into(),
    }])));

    ui.run()?;

    Ok(())
}
