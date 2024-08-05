//! The main entry point for the Prompt Bar

use tauri::{App, Manager, RunEvent};

use crate::{cmd, error::Result, plugins, tray, window};

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
pub struct AppState {}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl PromptBar {
    /// Creates a new instance of the Prompt Bar.
    pub fn new() -> Result<Self> {
        let builder = tauri::Builder::default();

        // Set plugins
        #[cfg(target_os = "macos")]
        let builder = builder.plugin(tauri_nspanel::init());
        let builder = builder.plugin(tauri_plugin_window_state::Builder::default().build());
        let builder = builder.plugin(tauri_plugin_global_shortcut::Builder::new().build());

        // Set invoke handlers
        let builder = builder.invoke_handler(tauri::generate_handler![cmd::hide_window]);

        // Setup
        let builder = builder.setup(|app| {
            // Initialize the app state
            app.manage(AppState::default());

            // Set up the plugins, window, and tray
            plugins::setup(app)?;
            window::setup(app)?;
            tray::setup(app)?;

            Ok(())
        });

        // Build app
        let app = builder.build(tauri::generate_context!("./tauri.conf.json"))?;

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
