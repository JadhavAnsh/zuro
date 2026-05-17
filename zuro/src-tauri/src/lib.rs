mod app_state;
mod commands;
mod error;
mod tray;
mod windows;

use app_state::AppState;
use tauri::{Manager, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            windows::ensure_pet_window(app.handle())?;
            tray::create_tray_icon(app.handle())?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != windows::MAIN_WINDOW_LABEL {
                return;
            }

            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    if window.state::<AppState>().is_quitting() {
                        return;
                    }

                    api.prevent_close();

                    if let Err(error) = windows::hide_to_tray(window.app_handle()) {
                        eprintln!("failed to hide app to tray on close: {error}");
                    }
                }
                WindowEvent::Resized(_) => {
                    if window.state::<AppState>().is_quitting() {
                        return;
                    }

                    if window.is_minimized().unwrap_or(false) {
                        if let Err(error) = windows::hide_to_tray(window.app_handle()) {
                            eprintln!("failed to hide app to tray on minimize: {error}");
                        }
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::restore_main_window,
            commands::show_pet,
            commands::hide_pet,
            commands::quit_from_tray,
            commands::get_visibility_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
