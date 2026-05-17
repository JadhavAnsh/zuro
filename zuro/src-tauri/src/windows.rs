use crate::{
    app_state::{AppState, AppVisibilityMode},
    error::{ShellError, ShellResult},
};
use tauri::{
    AppHandle, LogicalPosition, Manager, Monitor, Position, Runtime, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder,
};

pub const MAIN_WINDOW_LABEL: &str = "main";
pub const PET_WINDOW_LABEL: &str = "pet";

const PET_ROUTE: &str = "index.html#/pet";
const PET_WIDTH: f64 = 176.0;
const PET_HEIGHT: f64 = 176.0;
const PET_MARGIN: f64 = 24.0;

pub fn ensure_pet_window<R: Runtime>(app: &AppHandle<R>) -> ShellResult<WebviewWindow<R>> {
    if let Some(window) = app.get_webview_window(PET_WINDOW_LABEL) {
        return Ok(window);
    }

    let icon = app
        .default_window_icon()
        .cloned()
        .ok_or(ShellError::MissingDefaultIcon)?;

    let window =
        WebviewWindowBuilder::new(app, PET_WINDOW_LABEL, WebviewUrl::App(PET_ROUTE.into()))
            .title("Zuro Pet")
            .inner_size(PET_WIDTH, PET_HEIGHT)
            .min_inner_size(PET_WIDTH, PET_HEIGHT)
            .max_inner_size(PET_WIDTH, PET_HEIGHT)
            .position(PET_MARGIN, PET_MARGIN)
            .visible(false)
            .focused(false)
            .resizable(false)
            .maximizable(false)
            .minimizable(false)
            .closable(false)
            .decorations(false)
            .transparent(true)
            .shadow(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .icon(icon)?
            .build()?;

    let _ = window.set_skip_taskbar(true);

    Ok(window)
}

pub fn hide_to_tray<R: Runtime>(app: &AppHandle<R>) -> ShellResult<()> {
    let main = get_main_window(app)?;

    if main.is_visible().unwrap_or(false) || main.is_minimized().unwrap_or(false) {
        let _ = main.unminimize();
        main.hide()?;
    }

    show_pet_window(app)?;
    app.state::<AppState>()
        .set_visibility_mode(AppVisibilityMode::PetVisible);

    Ok(())
}

pub fn restore_main_window<R: Runtime>(app: &AppHandle<R>) -> ShellResult<()> {
    let main = get_main_window(app)?;

    hide_pet_window(app)?;
    let _ = main.unminimize();
    main.show()?;
    main.set_focus()?;

    app.state::<AppState>()
        .set_visibility_mode(AppVisibilityMode::Visible);

    Ok(())
}

pub fn show_pet_window<R: Runtime>(app: &AppHandle<R>) -> ShellResult<()> {
    let pet = ensure_pet_window(app)?;
    let main = get_main_window(app)?;

    position_pet_window(&pet, &main)?;
    pet.show()?;
    pet.set_focus()?;

    Ok(())
}

pub fn hide_pet_window<R: Runtime>(app: &AppHandle<R>) -> ShellResult<()> {
    if let Some(pet) = app.get_webview_window(PET_WINDOW_LABEL) {
        pet.hide()?;
    }

    Ok(())
}

pub fn mark_tray_only<R: Runtime>(app: &AppHandle<R>) {
    app.state::<AppState>()
        .set_visibility_mode(AppVisibilityMode::TrayOnly);
}

pub fn get_main_window<R: Runtime>(app: &AppHandle<R>) -> ShellResult<WebviewWindow<R>> {
    app.get_webview_window(MAIN_WINDOW_LABEL)
        .ok_or(ShellError::MissingWindow(MAIN_WINDOW_LABEL))
}

fn position_pet_window<R: Runtime>(
    pet: &WebviewWindow<R>,
    main: &WebviewWindow<R>,
) -> ShellResult<()> {
    let monitor = preferred_monitor(main)?;
    let work_area = monitor.work_area();

    let x = f64::from(work_area.position.x + work_area.size.width as i32) - PET_WIDTH - PET_MARGIN;
    let y =
        f64::from(work_area.position.y + work_area.size.height as i32) - PET_HEIGHT - PET_MARGIN;

    pet.set_position(Position::Logical(LogicalPosition::new(
        x.max(0.0),
        y.max(0.0),
    )))?;

    Ok(())
}

fn preferred_monitor<R: Runtime>(window: &WebviewWindow<R>) -> ShellResult<Monitor> {
    window
        .current_monitor()?
        .or_else(|| window.primary_monitor().ok().flatten())
        .ok_or(ShellError::MissingWindow(MAIN_WINDOW_LABEL))
}
