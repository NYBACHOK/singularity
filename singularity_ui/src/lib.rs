use std::rc::Rc;

use slint::{ModelRc, VecModel};

slint::include_modules!();

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
    env_logger::init();

    let app = setup_app()?;

    app.run()?;

    Ok(())
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).expect("failed to init application");
    if let Err(e) = main() {
        eprintln!("Failed to start app. Reason: {e}")
    }
}
