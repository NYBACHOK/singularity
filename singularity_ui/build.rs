const DEFAULT_APP_ID: &str = "com.ghuba.singularity.application";

fn main() {
    let config = slint_build::CompilerConfiguration::new();

    slint_build::compile_with_config("ui/app-window.slint", config).expect("Slint build failed");

    let id = std::env::var("APP_ID")
        .ok()
        .unwrap_or(DEFAULT_APP_ID.to_owned());

    println!("cargo:rustc-env=APP_ID={}", id);
}
