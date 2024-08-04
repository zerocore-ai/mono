use tauri::App;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::{window, Result};

//--------------------------------------------------------------------------------------------------
// Functions: Setup
//--------------------------------------------------------------------------------------------------

pub(crate) fn setup(app: &mut App) -> Result<()> {
    global_shortcuts_plugin(app)?;
    Ok(())
}

//--------------------------------------------------------------------------------------------------
// Functions: *
//--------------------------------------------------------------------------------------------------

fn global_shortcuts_plugin(app: &mut App) -> Result<()> {
    // Register the global shortcut plugin
    app.handle()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())?;

    // Get the global shortcut instance
    let shortcut = app.global_shortcut();

    // Register a global shortcut
    shortcut.on_shortcut("ctrl+space", |app, _, event| {
        if event.state == ShortcutState::Pressed {
            window::toggle_visibility(app);
        }
    })?;

    Ok(())
}
