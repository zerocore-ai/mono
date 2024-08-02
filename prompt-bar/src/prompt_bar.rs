// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{LogicalPosition, LogicalSize, WebviewBuilder, WebviewUrl, WindowBuilder};
use window_vibrancy::NSVisualEffectMaterial;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let width = 600.;
            let height = 100.;
            let window = WindowBuilder::new(app, "PromptBar")
                .inner_size(width, height)
                .decorations(false)
                .build()
                .expect("Failed to create window");

            window.add_child(
                WebviewBuilder::new("First", WebviewUrl::App(Default::default())).auto_resize(),
                LogicalPosition::new(0., 0.),
                LogicalSize::new(width, height),
            )?;

            #[cfg(target_os = "macos")]
            window_vibrancy::apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            window_vibrancy::apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .run(tauri::generate_context!("./tauri.conf.json"))
        .expect("error while running tauri application");
}
