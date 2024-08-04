use cocoa::{
    appkit::{NSWindow, NSWindowCollectionBehavior},
    base::id,
};
use tauri::{App, AppHandle, Manager, WebviewWindow, Wry};
use window_vibrancy::NSVisualEffectMaterial;

use crate::{AppState, Result};

//--------------------------------------------------------------------------------------------------
// Functions: Setup
//--------------------------------------------------------------------------------------------------

pub(crate) fn setup(app: &mut App) -> Result<()> {
    // Hide the app icon in the dock
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    // Get the main window
    let window = &app.get_webview_window("main").unwrap();

    // Make the window background blurry
    make_window_bg_blurry(window);

    // Make the window NS panel like
    make_window_ns_panel_like(window);

    Ok(())
}

//--------------------------------------------------------------------------------------------------
// Functions: *
//--------------------------------------------------------------------------------------------------

fn make_window_bg_blurry(window: &WebviewWindow<Wry>) {
    // Apply the vibrancy effect for macOS
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(window, NSVisualEffectMaterial::HudWindow, None, Some(16.0))
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // Apply the blur effect for Windows
    #[cfg(target_os = "windows")]
    window_vibrancy::apply_blur(window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
}

fn make_window_ns_panel_like(window: &WebviewWindow<Wry>) {
    let ns_window = window.ns_window().unwrap() as id;
    unsafe {
        // Allows the window to show in full screen space
        ns_window.setCollectionBehavior_(
            NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
        );

        // Allows the window to join all spaces
        ns_window.setCollectionBehavior_(
            NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces,
        );

        // Hides the window when the user switches to another application
        ns_window.setHidesOnDeactivate_(true);

        // Allows the window to be moved by the user
        ns_window.setMovableByWindowBackground_(true);
    }
}

pub(crate) fn toggle_visibility(app: &AppHandle) {
    let state = app.state::<AppState>();
    if state.get_hidden() {
        state.set_hidden(false);
        let _ = app.show();
    } else {
        state.set_hidden(true);
        let _ = app.hide();
    }
}

pub(crate) fn show(app: &AppHandle) {
    let state = app.state::<AppState>();
    state.set_hidden(false);
    let _ = app.show();
}
