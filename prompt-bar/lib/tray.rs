use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, Runtime,
};

use crate::{window, Result};

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) fn setup(app: &mut App) -> Result<()> {
    let _ = TrayIconBuilder::with_id("prompt-bar-tray-icon")
        .tooltip("Prompt Bar")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&create_menu(app)?)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => app.exit(0),
            "show" => window::show(app),
            _ => (),
        })
        .build(app)?;

    Ok(())
}

fn create_menu(app: &mut App) -> Result<Menu<impl Runtime>> {
    // Create a menu item for quitting the app
    let quit_menu_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    // Create a menu item for showing the prompt bar
    let show_menu_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;

    // Create a menu with the quit item
    let menu = Menu::with_items(app, &[&quit_menu_item, &show_menu_item])?;

    Ok(menu)
}
