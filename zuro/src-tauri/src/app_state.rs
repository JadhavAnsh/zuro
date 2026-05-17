use serde::Serialize;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppVisibilityMode {
    Visible,
    TrayOnly,
    PetVisible,
}

#[derive(Debug, Serialize)]
pub struct PetWindowState {
    pub visible: bool,
}

#[derive(Debug, Serialize)]
pub struct RestoreMainWindowResult {
    pub restored: bool,
}

#[derive(Debug)]
pub struct AppState {
    visibility_mode: Mutex<AppVisibilityMode>,
    is_quitting: AtomicBool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            visibility_mode: Mutex::new(AppVisibilityMode::Visible),
            is_quitting: AtomicBool::new(false),
        }
    }
}

impl AppState {
    pub fn visibility_mode(&self) -> AppVisibilityMode {
        *self
            .visibility_mode
            .lock()
            .expect("app visibility state lock should not be poisoned")
    }

    pub fn set_visibility_mode(&self, mode: AppVisibilityMode) {
        *self
            .visibility_mode
            .lock()
            .expect("app visibility state lock should not be poisoned") = mode;
    }

    pub fn is_quitting(&self) -> bool {
        self.is_quitting.load(Ordering::SeqCst)
    }

    pub fn set_quitting(&self, value: bool) {
        self.is_quitting.store(value, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::{AppState, AppVisibilityMode};

    #[test]
    fn state_starts_visible_and_not_quitting() {
        let state = AppState::default();

        assert_eq!(state.visibility_mode(), AppVisibilityMode::Visible);
        assert!(!state.is_quitting());
    }

    #[test]
    fn state_updates_visibility_and_quit_intent() {
        let state = AppState::default();

        state.set_visibility_mode(AppVisibilityMode::PetVisible);
        state.set_quitting(true);

        assert_eq!(state.visibility_mode(), AppVisibilityMode::PetVisible);
        assert!(state.is_quitting());
    }
}
