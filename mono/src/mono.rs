// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{LogicalPosition, LogicalSize, WebviewBuilder, WebviewUrl, WindowBuilder};

const INIT_SCRIPT: &str = r#"
setTimeout(() => {
    console.log("hello world from js init script");
}, 2000);

window.__MY_CUSTOM_PROPERTY__ = { foo: 'bar' };
"#;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let width = 800.;
            let height = 600.;
            let window = WindowBuilder::new(app, "Mono")
                .inner_size(width, height)
                .build()
                .expect("Failed to create window");

            // First child
            window.add_child(
                WebviewBuilder::new(
                    "First",
                    WebviewUrl::External("http://localhost:5130/".parse()?),
                )
                .auto_resize(),
                LogicalPosition::new(0., 0.),
                LogicalSize::new(width / 2., height),
            )?;

            // Second child
            window.add_child(
                WebviewBuilder::new(
                    "Second",
                    WebviewUrl::External("http://localhost:8081/".parse()?),
                    // WebviewUrl::App("../index.html".into())
                )
                .initialization_script(INIT_SCRIPT)
                .auto_resize(),
                LogicalPosition::new(25., 0.),
                LogicalSize::new(width / 2., height),
            )?;

            Ok(())
        })
        .run(tauri::generate_context!("./tauri.conf.json"))
        .expect("error while running tauri application");
}
