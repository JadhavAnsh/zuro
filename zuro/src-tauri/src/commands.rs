use crate::{
    app_state::{AppState, PetWindowState, RestoreMainWindowResult},
    windows,
};
use tauri::{AppHandle, Manager, Runtime, State};

type CommandResult<T> = Result<T, String>;

#[tauri::command]
pub fn restore_main_window<R: Runtime>(
    app: AppHandle<R>,
) -> CommandResult<RestoreMainWindowResult> {
    windows::restore_main_window(&app)
        .map(|()| RestoreMainWindowResult { restored: true })
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn show_pet<R: Runtime>(app: AppHandle<R>) -> CommandResult<PetWindowState> {
    windows::show_pet_window(&app)
        .map(|()| {
            app.state::<AppState>()
                .set_visibility_mode(crate::app_state::AppVisibilityMode::PetVisible);
            PetWindowState { visible: true }
        })
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn hide_pet<R: Runtime>(app: AppHandle<R>) -> CommandResult<PetWindowState> {
    windows::hide_pet_window(&app)
        .map(|()| {
            windows::mark_tray_only(&app);
            PetWindowState { visible: false }
        })
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn quit_from_tray<R: Runtime>(app: AppHandle<R>) -> CommandResult<()> {
    app.state::<AppState>().set_quitting(true);
    app.exit(0);
    Ok(())
}

#[tauri::command]
pub fn get_visibility_mode(state: State<'_, AppState>) -> crate::app_state::AppVisibilityMode {
    state.visibility_mode()
}
