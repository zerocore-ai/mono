use tauri::App;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::{window, Result};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const GLOBAL_SHORTCUT_CTRL_SPACE: &str = "ctrl+space";

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
    // Get the global shortcut instance
    let shortcut = app.global_shortcut();

    // Register a global shortcut
    shortcut.on_shortcut(GLOBAL_SHORTCUT_CTRL_SPACE, |app, _, event| {
        if event.state == ShortcutState::Pressed {
            window::toggle_visibility(app).unwrap();
        }
    })?;

    Ok(())
}
