use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A specialized `Result` type for prompt bar crate.
pub type Result<T> = std::result::Result<T, PromptBarError>;

/// The main error type of the prompt bar crate.
#[derive(Debug, Error)]
pub enum PromptBarError {
    /// A wrapper around the `tauri::Error` type.
    #[error(transparent)]
    TauriError(#[from] tauri::Error),

    /// A wrapper around the `tauri_plugin_global_shortcut::Error` type.
    #[error(transparent)]
    TauriShortcutsError(#[from] tauri_plugin_global_shortcut::Error),

    /// A wrapper around the `window_vibrancy::Error` type.
    #[error(transparent)]
    WindowVibrancyError(#[from] window_vibrancy::Error),
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates a new `Ok` result.
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> Result<T> {
    Result::Ok(value)
}
