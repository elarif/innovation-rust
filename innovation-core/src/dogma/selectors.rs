/// Card and Player Selectors
/// 
/// This module defines the type system for selecting cards and players
/// based on the comprehensive game elements analysis.

use crate::model::{Color, Symbol, Location};
use serde::{Deserialize, Serialize};

// ============================================================================
// CARD SELECTORS
// ============================================================================

/// Position within a location (top, bottom, all, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    /// Top card of a pile
    Top,
    /// Bottom card of a pile
    Bottom,
    /// All cards in the location
    All,
    /// Active card (top of a color pile on the board)
    Active,
    /// Card at a specific index
    Index(usize),
}

/// Comparison operators for numeric values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Comparison {
    Equals,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
}

/// Age/value selectors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgeSelector {
    /// Exact age value (1-10)
    Exact(u8),
    /// Maximum age in a set
    Max,
    /// Minimum age in a set
    Min,
    /// Relative to a reference (+1, +2, -1, etc.)
    Relative(i8),
    /// Comparison with a value
    Compare(Comparison, u8),
}

/// Color selectors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorSelector {
    /// Specific color
    Exact(Color),
    /// Any color
    Any,
    /// Colors present on player's board
    OnBoard,
    /// Colors not present on player's board
    NotOnBoard,
    /// Color not matching a specific color
    Not(Color),
    /// Colors unique to player (no opponent has)
    UniqueToPlayer,
    /// Same color as reference card
    SameAsReference,
    /// Different color from all reference cards
    DifferentFromAll,
}

/// Symbol/Icon selectors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolSelector {
    /// Card produces (has visible) a specific symbol
    Produces(Symbol),
    /// Card does not produce a specific symbol
    DoesNotProduce(Symbol),
    /// Card has at least N of a symbol
    ProducesAtLeast(Symbol, u8),
}

/// Quantity selectors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantitySelector {
    /// Exactly N cards
    Exact(u8),
    /// Up to N cards (0 to N, player chooses)
    UpTo(u8),
    /// At least N cards
    AtLeast(u8),
    /// All cards matching criteria
    All,
    /// Half of the cards (with rounding)
    Half(Rounding),
    /// For each N symbols, select 1 card
    PerSymbols(Symbol, u8),
}

/// Rounding direction for division
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rounding {
    Up,
    Down,
}

/// Composite card filter combining multiple criteria
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardFilter {
    /// Filter by location
    pub location: Option<Location>,
    /// Filter by position within location
    pub position: Option<Position>,
    /// Filter by age
    pub age: Option<AgeSelector>,
    /// Filter by color
    pub color: Option<ColorSelector>,
    /// Filter by symbol/icon
    pub symbol: Option<SymbolSelector>,
}

impl Default for CardFilter {
    fn default() -> Self {
        Self {
            location: None,
            position: None,
            age: None,
            color: None,
            symbol: None,
        }
    }
}

/// Complete card selector with quantity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardSelector {
    /// How many cards to select
    pub quantity: QuantitySelector,
    /// Filters to apply
    pub filter: CardFilter,
    /// Reference to player who owns the cards
    pub owner: PlayerSelector,
}

// ============================================================================
// PLAYER SELECTORS
// ============================================================================

/// Player reference types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerSelector {
    /// The player activating the dogma
    Activator,
    /// The player currently being affected (in a loop over players)
    CurrentTarget,
    /// All players including activator
    All,
    /// All opponents (excluding activator)
    AllOpponents,
    /// A single chosen opponent
    ChosenOpponent,
    /// Opponents who meet a condition
    OpponentsWhere(Box<PlayerCondition>),
    /// All players who meet a condition
    PlayersWhere(Box<PlayerCondition>),
}

/// Conditions on players
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerCondition {
    /// Player has fewer of a symbol than the activator
    HasFewerSymbols(Symbol),
    /// Player has equal or more of a symbol than the activator
    HasEqualOrMoreSymbols(Symbol),
    /// Player produces at least N symbols
    ProducesAtLeast(Symbol, u8),
    /// Player has fewer points than the activator
    HasFewerPoints,
    /// Player has most points
    HasMostPoints,
    /// Player has fewest points
    HasFewestPoints,
    /// Player is the only one with N colors on board
    OnlyPlayerWithColors(u8),
    /// Player has N or more cards of type X
    HasCardsCount(Location, u8),
    /// Player is the only one matching a condition
    IsOnly(Box<PlayerCondition>),
    /// Logical AND of conditions
    And(Vec<PlayerCondition>),
    /// Logical OR of conditions
    Or(Vec<PlayerCondition>),
    /// Logical NOT of condition
    Not(Box<PlayerCondition>),
}

// ============================================================================
// HELPER CONSTRUCTORS
// ============================================================================

impl CardSelector {
    /// Create a selector for cards from hand
    pub fn from_hand(quantity: QuantitySelector, owner: PlayerSelector) -> Self {
        Self {
            quantity,
            filter: CardFilter {
                location: Some(Location::Hand),
                ..Default::default()
            },
            owner,
        }
    }

    /// Create a selector for cards from score pile
    pub fn from_score(quantity: QuantitySelector, owner: PlayerSelector) -> Self {
        Self {
            quantity,
            filter: CardFilter {
                location: Some(Location::Score),
                ..Default::default()
            },
            owner,
        }
    }

    /// Create a selector for active cards on board
    pub fn active_cards(owner: PlayerSelector) -> Self {
        Self {
            quantity: QuantitySelector::All,
            filter: CardFilter {
                location: Some(Location::Board),
                position: Some(Position::Active),
                ..Default::default()
            },
            owner,
        }
    }

    /// Add age filter
    pub fn with_age(mut self, age: AgeSelector) -> Self {
        self.filter.age = Some(age);
        self
    }

    /// Add color filter
    pub fn with_color(mut self, color: ColorSelector) -> Self {
        self.filter.color = Some(color);
        self
    }

    /// Add symbol filter
    pub fn with_symbol(mut self, symbol: SymbolSelector) -> Self {
        self.filter.symbol = Some(symbol);
        self
    }
}

impl PlayerSelector {
    /// Create opponent selector based on supremacy dogma
    pub fn supremacy_targets(symbol: Symbol) -> Self {
        Self::OpponentsWhere(Box::new(PlayerCondition::HasFewerSymbols(symbol)))
    }

    /// Create player selector based on cooperative dogma
    pub fn cooperative_targets(symbol: Symbol) -> Self {
        Self::PlayersWhere(Box::new(PlayerCondition::HasEqualOrMoreSymbols(symbol)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_selector_builder() {
        let selector = CardSelector::from_hand(
            QuantitySelector::Exact(1),
            PlayerSelector::Activator,
        )
        .with_age(AgeSelector::Max)
        .with_symbol(SymbolSelector::Produces(Symbol::Crown));

        assert_eq!(selector.filter.location, Some(Location::Hand));
        assert_eq!(selector.filter.age, Some(AgeSelector::Max));
        assert_eq!(
            selector.filter.symbol,
            Some(SymbolSelector::Produces(Symbol::Crown))
        );
    }

    #[test]
    fn test_supremacy_selector() {
        let selector = PlayerSelector::supremacy_targets(Symbol::Castle);
        match selector {
            PlayerSelector::OpponentsWhere(cond) => {
                assert_eq!(*cond, PlayerCondition::HasFewerSymbols(Symbol::Castle));
            }
            _ => panic!("Expected OpponentsWhere"),
        }
    }
}
