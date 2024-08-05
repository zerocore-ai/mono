use cocoa::appkit::{NSMainMenuWindowLevel, NSWindowCollectionBehavior};
use tauri::{App, AppHandle, Emitter, Manager, WebviewWindow, Wry};
use tauri_nspanel::{
    objc_id::ShareId, panel_delegate, raw_nspanel::RawNSPanel, ManagerExt, WebviewWindowExt,
};
use window_vibrancy::NSVisualEffectMaterial;

use crate::Result;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const MAIN_WINDOW_LABEL: &str = "main";

// https://github.com/ryanmcgrath/cacao/blob/7ffe39891cd73984c4bea5e37d463f3398361320/src/appkit/window/enums.rs#L45-L60
const NS_NON_ACTIVATING_PANEL_STYLE_MASK: i32 = 1 << 7;

const WINDOW_DID_BECOME_KEY: &str = "window_did_become_key";

const WINDOW_DID_RESIGN_KEY: &str = "window_did_resign_key";

//--------------------------------------------------------------------------------------------------
// Functions: Setup
//--------------------------------------------------------------------------------------------------

pub(crate) fn setup(app: &mut App) -> Result<()> {
    // Get the main window
    let window = &app.get_webview_window(MAIN_WINDOW_LABEL).unwrap();

    // Hide the app icon in the dock
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    // Make the window spotlight like
    #[cfg(target_os = "macos")]
    make_window_spotlight_like(window)?;

    // Make the window background blurry
    make_window_bg_blurry(window)?;

    // Open the devtools in debug mode
    #[cfg(debug_assertions)]
    window.open_devtools();

    Ok(())
}

//--------------------------------------------------------------------------------------------------
// Functions: *
//--------------------------------------------------------------------------------------------------

pub(crate) fn toggle_visibility(app: &AppHandle) -> Result<()> {
    let ns_panel = app.get_webview_panel(MAIN_WINDOW_LABEL).unwrap();
    if ns_panel.is_visible() {
        ns_panel.order_out(None)
    } else {
        ns_panel.show()
    }

    Ok(())
}

pub(crate) fn show(app: &AppHandle) {
    let ns_panel = app.get_webview_panel(MAIN_WINDOW_LABEL).unwrap();
    ns_panel.show()
}

pub(crate) fn hide(app: &AppHandle) {
    let ns_panel = app.get_webview_panel(MAIN_WINDOW_LABEL).unwrap();
    ns_panel.order_out(None)
}

fn make_window_bg_blurry(window: &WebviewWindow<Wry>) -> Result<()> {
    // Apply the vibrancy effect for macOS
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(window, NSVisualEffectMaterial::HudWindow, None, Some(16.0))?;

    // Apply the blur effect for Windows
    #[cfg(target_os = "windows")]
    window_vibrancy::apply_blur(window, Some((18, 18, 18, 125)))?;

    Ok(())
}

fn make_window_spotlight_like(window: &WebviewWindow<Wry>) -> Result<()> {
    let ns_panel = window.to_panel()?;

    // Set the window level to main menu level
    ns_panel.set_level(NSMainMenuWindowLevel + 3);

    // Set the window style mask to non-activating
    ns_panel.set_style_mask(NS_NON_ACTIVATING_PANEL_STYLE_MASK);

    // Set the window collection behaviour to full screen auxiliary and can join all spaces
    ns_panel.set_collection_behaviour(
        NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces,
    );

    // Set the nspanel delegate
    set_nspanel_delegate(ns_panel, window.app_handle().clone())?;

    Ok(())
}

fn set_nspanel_delegate(ns_panel: ShareId<RawNSPanel>, app: AppHandle) -> Result<()> {
    let delegate = panel_delegate!(PanelDelegate {
        window_did_become_key,
        window_did_resign_key,
    });

    delegate.set_listener(Box::new(move |event| match event.as_str() {
        WINDOW_DID_BECOME_KEY => {
            println!("Window did become key");
            let _ = app.emit(WINDOW_DID_BECOME_KEY, ());
        }
        WINDOW_DID_RESIGN_KEY => {
            println!("Window did resign key");
            let _ = app.emit(WINDOW_DID_RESIGN_KEY, ());
        }
        _ => {}
    }));

    ns_panel.set_delegate(delegate);

    Ok(())
}
