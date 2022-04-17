use std::error::Error;

#[derive(Debug)]
pub enum AmenError {
    NoneSelected,
    Internal(Box<dyn Error>),
}

impl std::fmt::Display for AmenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmenError::NoneSelected => write!(f, "NoneSelected"),
            AmenError::Internal(error) => write!(f, "{}", error),
        }
    }
}

impl Error for AmenError {}
