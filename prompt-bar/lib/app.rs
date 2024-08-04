//! The main entry point for the Prompt Bar

use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{App, Manager, RunEvent};

use crate::{error::Result, plugins, tray, window};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The main application struct.
pub struct PromptBar {
    /// The Tauri app instance.
    app: App,
}

/// The state of the Prompt Bar.
#[derive(Default)]
pub struct AppState {
    /// Whether the Prompt Bar is currently hidden.
    pub hidden: AtomicBool,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl PromptBar {
    /// Creates a new instance of the Prompt Bar.
    pub fn new() -> Result<Self> {
        let app = tauri::Builder::default()
            .setup(|app| {
                // Initialize the app state
                app.manage(AppState::default());

                // Set up the plugins, window, and tray
                plugins::setup(app)?;
                window::setup(app)?;
                tray::setup(app)?;

                Ok(())
            })
            .build(tauri::generate_context!("./tauri.conf.json"))?;

        Ok(Self { app })
    }

    /// Runs the Prompt Bar.
    pub fn run(self) {
        self.app.run(move |_app_handle, _event| {
            // Prevent the app from exiting.
            #[cfg(desktop)]
            if let RunEvent::ExitRequested { api, .. } = &_event {
                api.prevent_exit();
            }
        })
    }
}

impl AppState {
    /// Sets the hidden state of the Prompt Bar.
    pub fn set_hidden(&self, hidden: bool) {
        self.hidden.store(hidden, Ordering::SeqCst);
    }

    /// Gets the hidden state of the Prompt Bar.
    pub fn get_hidden(&self) -> bool {
        self.hidden.load(Ordering::SeqCst)
    }
}
