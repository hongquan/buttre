use thiserror::Error;

#[derive(Error, Debug)]
pub enum HotkeyError {
    #[error("Failed to create hotkey manager: {0}")]
    ManagerCreationFailed(String),
    
    #[error("Failed to register hotkey: {0}")]
    RegistrationFailed(String),
    
    #[error("Failed to unregister hotkey: {0}")]
    UnregistrationFailed(String),
    
    #[error("Invalid hotkey configuration: {0}")]
    InvalidConfiguration(String),
}

pub type Result<T> = std::result::Result<T, HotkeyError>;
