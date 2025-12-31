use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Card not found: {0}")]
    CardNotFound(String),

    #[error("Invalid action: {0}")]
    InvalidAction(String),
}
