use serde::{Deserialize, Serialize};
use std::fmt;

pub type CardId = String;
pub type AchievementId = String;

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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Symbol {
    Crown,
    Leaf,
    Factory,
    Lightbulb,
    Castle,
    Clock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Icon {
    Age(u8),
    Resource(Symbol),
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Icon::Age(n) => write!(f, "{}", n),
            Icon::Resource(s) => write!(f, "{:?}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Expansion {
    #[default]
    Base,
    Echoes,
    Figures,
    Cities,
    Artifacts,
}

// --- Logic / Effect System ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Location {
    Hand,
    Score,
    Board, // Implies active/top card usually, or the stack
    Deck,
    Achievements,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplayDirection {
    None,
    Left,
    Right,
    Up,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    Color(Color),
    Value(u8),
    Symbol(Symbol),
    ColorsPresentOnBoard,
    ColorsNotOnBoard,  // For Tissage: filter for colors player doesn't have
    Icon(Symbol),
    MaxAge,
    MinAge,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Condition {
    ContextValueGreaterThan(u8),
    IconCountGreaterThanOrEqual(Symbol, u8),
    ScoreGreaterThanHand,  // Calendrier: If score pile count > hand count
    True,
}

/// Condition to check against a drawn card
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrawnCardCondition {
    HasIcon(Symbol),
    ColorOnBoard,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum Effect {
    Draw { amount: u8, age: Option<u8> }, 
    DrawAndMeld { amount: u8, age: Option<u8> },
    DrawAndScore { amount: u8, age: Option<u8> },
    Return { count: u8, source: Location }, // Return usually implies count? Or min/max? Keep count for Return (rarely optional range?)
    Score { min: u8, max: u8, filters: Vec<Filter> },
    Exchange { source: Location, dest: Location },
    Tuck { min: u8, max: u8, source: Location, filters: Vec<Filter> }, 
    // Splay: default source is Board (implied by color).
    // color: None => Use Context (Last Processed Card Color)
    Splay { direction: SplayDirection, color: Option<Color> }, 
    Recycle { min: u8, max: u8, source: Location, filters: Vec<Filter> },
    Meld { source: Location, filters: Vec<Filter>, min: u8, max: u8 },
    Transfer { 
        source: Location, 
        dest: Location, 
        dest_is_activator: bool, 
        filters: Vec<Filter>,
        min: u8,
        max: u8
    },
    Choice { options: Vec<Effect> },
    Conditional { 
        condition: Condition, 
        success: Box<Effect>,
        failure: Option<Box<Effect>> 
    },
    Sequence(Vec<Effect>),
    /// Draw until a card doesn't match condition. Score matching cards, keep non-matching in hand.
    /// Used by Métallurgie: "Draw and reveal 1s until you don't reveal a Castle. Score each Castle. Keep the last card."
    DrawUntilNoMatch {
        age: u8,
        condition: DrawnCardCondition,
        on_match: Box<Effect>,  // What to do with matching cards (e.g., Score)
    },
    /// Draw one card, check condition, execute different effects based on result.
    /// Used by Mysticisme: "Draw 1. If same color as board, meld and draw 1."
    DrawAndCheck {
        age: u8,
        condition: DrawnCardCondition,
        on_match: Box<Effect>,
        on_fail: Option<Box<Effect>>,
    },
    /// Draw and score N cards where N = count of unique colors on board (colors no opponent has)
    DrawAndScoreForUniqueColors { age: u8 },
    /// Draw N cards where N = icon_count / divisor (for Fermentation: draw 1 per 2 Leaves)
    DrawForIconPairs { age: u8, symbol: Symbol, divisor: u8 },
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DogmaEffect {
    // We try to parse the Symbol. 
    // BUT NOTE: "symbol" in JSON is sometimes missing or null for base effects?
    // Actually schema says it's required and is a ResourceIcon (string).
    // Let's use Icon to be safe, or Symbol if schema guarantees it.
    // Schema says "$ref": "#/definitions/ResourceIcon". So it's a Symbol.
    pub symbol: Symbol, 
    pub text: String,
    #[serde(rename = "isSupremacy", default)]
    pub is_supremacy: bool,
    
    // The structured effect logic. 
    // This will NOT be populated by JSON directly (yet), but by the Registry.
    #[serde(skip)]
    pub effect: Option<Effect>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub name: CardId,
    pub age: u8,
    pub color: Color,
    // Schema says 4 icons fixed, but Vec is fine.
    pub icons: Vec<Icon>,
    pub dogmas: Vec<DogmaEffect>,
    #[serde(default)]
    pub expansion: Expansion,
}
