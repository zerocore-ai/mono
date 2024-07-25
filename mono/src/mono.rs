// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{LogicalPosition, LogicalSize, WebviewBuilder, WebviewUrl, WindowBuilder};

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
                WebviewBuilder::new("First", WebviewUrl::App(Default::default())).auto_resize(),
                LogicalPosition::new(0., 0.),
                LogicalSize::new(width / 2., height),
            )?;

            // Second child
            window.add_child(
                WebviewBuilder::new(
                    "Second",
                    WebviewUrl::External("https://www.tauri.app".parse()?),
                )
                .auto_resize(),
                LogicalPosition::new(width / 2., 0.),
                LogicalSize::new(width / 2., height),
            )?;

            Ok(())
        })
        .run(tauri::generate_context!("./tauri.conf.json"))
        .expect("error while running tauri application");
}
