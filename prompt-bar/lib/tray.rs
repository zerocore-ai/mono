use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, Runtime,
};

use crate::{window, Result};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const TRAY_ICON_ID: &str = "prompt-bar-tray-icon";
const TRAY_ICON_TOOLTIP: &str = "Prompt Bar";
const QUIT_MENU_ID: &str = "quit";
const SHOW_MENU_ID: &str = "show";

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) fn setup(app: &mut App) -> Result<()> {
    let _ = TrayIconBuilder::with_id(TRAY_ICON_ID)
        .tooltip(TRAY_ICON_TOOLTIP)
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&create_menu(app)?)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            QUIT_MENU_ID => app.exit(0),
            SHOW_MENU_ID => window::show(app),
            _ => (),
        })
        .build(app)?;

    Ok(())
}

fn create_menu(app: &mut App) -> Result<Menu<impl Runtime>> {
    // Create a menu item for quitting the app
    let quit_menu_item = MenuItem::with_id(app, QUIT_MENU_ID, "Quit", true, None::<&str>)?;

    // Create a menu item for showing the prompt bar
    let show_menu_item = MenuItem::with_id(app, SHOW_MENU_ID, "Show", true, None::<&str>)?;

    // Create a menu with the quit item
    let menu = Menu::with_items(app, &[&quit_menu_item, &show_menu_item])?;

    Ok(menu)
}
