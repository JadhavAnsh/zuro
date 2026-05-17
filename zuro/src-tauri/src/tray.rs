use crate::windows;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};

pub const TRAY_ICON_ID: &str = "zuro-tray";
const MENU_OPEN_ID: &str = "tray-open";
const MENU_SHOW_PET_ID: &str = "tray-show-pet";
const MENU_HIDE_PET_ID: &str = "tray-hide-pet";
const MENU_QUIT_ID: &str = "tray-quit";

pub fn create_tray_icon<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let open_item = MenuItem::with_id(app, MENU_OPEN_ID, "Open Zuro", true, None::<&str>)?;
    let show_pet_item = MenuItem::with_id(app, MENU_SHOW_PET_ID, "Show Pet", true, None::<&str>)?;
    let hide_pet_item = MenuItem::with_id(app, MENU_HIDE_PET_ID, "Hide Pet", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, MENU_QUIT_ID, "Quit", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(
        app,
        &[
            &open_item,
            &show_pet_item,
            &hide_pet_item,
            &separator,
            &quit_item,
        ],
    )?;

    let icon = app
        .default_window_icon()
        .cloned()
        .ok_or_else(|| tauri::Error::AssetNotFound("default window icon".into()))?;

    TrayIconBuilder::with_id(TRAY_ICON_ID)
        .menu(&menu)
        .icon(icon)
        .tooltip("Zuro Browser")
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event: tauri::menu::MenuEvent| {
            if let Err(error) = handle_menu_event(app, event.id().as_ref()) {
                eprintln!("tray menu action failed: {error}");
            }
        })
        .on_tray_icon_event(|tray: &TrayIcon<R>, event| {
            if let Err(error) = handle_tray_icon_event(tray.app_handle(), event) {
                eprintln!("tray icon action failed: {error}");
            }
        })
        .build(app)?;

    Ok(())
}

fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, menu_id: &str) -> Result<(), String> {
    match menu_id {
        MENU_OPEN_ID => windows::restore_main_window(app).map_err(|error| error.to_string()),
        MENU_SHOW_PET_ID => windows::show_pet_window(app).map_err(|error| error.to_string()),
        MENU_HIDE_PET_ID => windows::hide_pet_window(app)
            .map(|()| {
                windows::mark_tray_only(app);
            })
            .map_err(|error| error.to_string()),
        MENU_QUIT_ID => {
            app.state::<crate::app_state::AppState>().set_quitting(true);
            app.exit(0);
            Ok(())
        }
        _ => Ok(()),
    }
}

fn handle_tray_icon_event<R: Runtime>(
    app: &AppHandle<R>,
    event: TrayIconEvent,
) -> Result<(), String> {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        }
        | TrayIconEvent::DoubleClick {
            button: MouseButton::Left,
            ..
        } => windows::restore_main_window(app).map_err(|error| error.to_string()),
        _ => Ok(()),
    }
}
