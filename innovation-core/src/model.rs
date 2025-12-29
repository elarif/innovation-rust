use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "RED")]
    Red,
    #[serde(rename = "BLUE")]
    Blue,
    #[serde(rename = "GREEN")]
    Green,
    #[serde(rename = "YELLOW")]
    Yellow,
    #[serde(rename = "PURPLE")]
    Purple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StandardIcon {
    #[serde(rename = "CROWN")]
    Crown,
    #[serde(rename = "LEAF")]
    Leaf,
    #[serde(rename = "FACTORY")]
    Factory,
    #[serde(rename = "LIGHTBULB")]
    Lightbulb,
    #[serde(rename = "CASTLE")]
    Castle,
    #[serde(rename = "CLOCK")]
    Clock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Icon {
    Age(u8),
    Standard(StandardIcon),
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Icon::Age(n) => write!(f, "{}", n),
            Icon::Standard(s) => write!(f, "{:?}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expansion {
    Base,
    Echoes,
    Figures,
    Cities,
    Artifacts,
}

impl Default for Expansion {
    fn default() -> Self {
        Expansion::Base
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DogmaEffect {
    pub symbol: Icon,
    pub text: String,
    #[serde(rename = "isSupremacy", default)]
    pub is_supremacy: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub age: u8,
    pub color: Color,
    pub icons: Vec<Icon>,
    pub dogmas: Vec<DogmaEffect>,
    #[serde(default)]
    pub expansion: Expansion,
}
