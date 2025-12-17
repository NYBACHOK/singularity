fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(std::collections::HashMap::from([(
            "material".to_string(),
            std::path::Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
                .join("themes/material-1.0/material.slint"),
        )]))
        .with_style("material".into());

    slint_build::compile_with_config("ui/app-window.slint", config).expect("Slint build failed");
}
