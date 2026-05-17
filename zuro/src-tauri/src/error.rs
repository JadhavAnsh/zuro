use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("missing window `{0}`")]
    MissingWindow(&'static str),
    #[error("default app icon is not available")]
    MissingDefaultIcon,
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
}

pub type ShellResult<T> = Result<T, ShellError>;
