// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use prompt_bar::{PromptBar, Result};

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() -> Result<()> {
    PromptBar::new()?.run();
    Ok(())
}
