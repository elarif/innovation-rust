use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Draw,
    Meld(String), // Card ID
    Achieve(String), // Card ID (standard achievement age)
    Dogma(String), // Card ID
    ResolveInput(String),
}
