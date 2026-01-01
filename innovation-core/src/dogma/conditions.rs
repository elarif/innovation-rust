/// Conditions and Control Structures
/// 
/// This module defines conditions that can be evaluated during effect execution
/// and control structures that orchestrate effect flow.

use crate::model::Symbol;
use crate::dogma::selectors::{CardSelector, PlayerSelector, PlayerCondition};
use serde::{Deserialize, Serialize};

// ============================================================================
// CONDITIONS
// ============================================================================

/// Conditions on cards
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardCondition {
    /// Card has a specific symbol
    HasSymbol(Symbol),
    /// Card is of a specific color
    HasColor(crate::model::Color),
    /// Card is of a specific age
    HasAge(u8),
    /// Card age matches a comparison
    AgeCompare(crate::dogma::selectors::Comparison, u8),
    /// Card color matches board colors
    ColorOnBoard(PlayerSelector),
    /// Card color not on board
    ColorNotOnBoard(PlayerSelector),
    /// Card is same color as reference
    SameColorAs(CardReference),
    /// Multiple cards have same color
    MultipleHaveSameColor(u8),
    /// Logical combinations
    And(Vec<CardCondition>),
    Or(Vec<CardCondition>),
    Not(Box<CardCondition>),
}

/// Reference to a card in context
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardReference {
    /// Last drawn card
    LastDrawn,
    /// Last melded card
    LastMelded,
    /// Last scored card
    LastScored,
    /// Last recycled card
    LastRecycled,
    /// Last transferred card
    LastTransferred,
    /// Card from named variable
    Named(String),
}

/// Conditions on game state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameCondition {
    /// Number of cards matches criteria
    CardCount {
        selector: CardSelector,
        comparison: crate::dogma::selectors::Comparison,
        value: u8,
    },
    /// Cards were transferred in last action
    CardsTransferred {
        min: u8,
        max: Option<u8>,
    },
    /// No cards were transferred
    NoCardsTransferred,
    /// Player condition
    Player(PlayerCondition),
    /// Card condition on specific card
    Card {
        card: CardReference,
        condition: CardCondition,
    },
    /// Compare player's hand and score pile sizes
    HandVsScore {
        player: PlayerSelector,
        hand_comparison: crate::dogma::selectors::Comparison,
    },
    /// All active cards meet a condition
    AllActiveCards {
        player: PlayerSelector,
        condition: CardCondition,
    },
    /// Check if specific cards are active
    SpecificCardsActive {
        cards: Vec<String>,
    },
    /// Check splay status
    ColorSplayed {
        player: PlayerSelector,
        color: Option<crate::model::Color>,
        direction: crate::model::SplayDirection,
    },
    /// Check number of colors splayed
    ColorsSplayedCount {
        player: PlayerSelector,
        direction: crate::model::SplayDirection,
        comparison: crate::dogma::selectors::Comparison,
        count: u8,
    },
    /// Previous action succeeded
    PreviousActionSucceeded,
    /// Multiple cards were played
    CardsPlayed {
        comparison: crate::dogma::selectors::Comparison,
        count: u8,
    },
    /// Player recycled more than others in this dogma
    RecycledMoreThanOthers(PlayerSelector),
    /// Logical combinations
    And(Vec<GameCondition>),
    Or(Vec<GameCondition>),
    Not(Box<GameCondition>),
}

// ============================================================================
// CONTROL STRUCTURES
// ============================================================================

/// Main effect type (forward declaration - will use from model)
pub type Effect = crate::model::Effect;

/// Optional action - player may choose to execute or not
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Optional {
    /// The effect that may be executed
    pub effect: Box<Effect>,
    /// What happens if player chooses to execute
    pub on_success: Option<Box<Effect>>,
}

/// Conditional execution based on a condition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Conditional {
    /// Condition to evaluate
    pub condition: GameCondition,
    /// Effect if condition is true
    pub on_true: Box<Effect>,
    /// Effect if condition is false
    pub on_false: Option<Box<Effect>>,
}

/// Player choice between multiple options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    /// List of effects to choose from (player picks one)
    pub options: Vec<Effect>,
    /// Minimum number of options to choose
    pub min_choices: u8,
    /// Maximum number of options to choose
    pub max_choices: u8,
}

/// Repeat an effect until a condition is met
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepeatUntil {
    /// Effect to repeat
    pub effect: Box<Effect>,
    /// Condition to check after each iteration
    pub until: GameCondition,
    /// Maximum iterations (safety limit)
    pub max_iterations: Option<u8>,
}

/// Repeat an effect while a condition holds
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepeatWhile {
    /// Effect to repeat
    pub effect: Box<Effect>,
    /// Condition to check before each iteration
    pub while_condition: GameCondition,
    /// Maximum iterations (safety limit)
    pub max_iterations: Option<u8>,
}

/// Iterate over a collection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForEach {
    /// What to iterate over
    pub collection: Collection,
    /// Effect to execute for each element
    pub effect: Box<Effect>,
}

/// Collections that can be iterated
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Collection {
    /// Cards matching a selector
    Cards(CardSelector),
    /// Players matching a condition
    Players(PlayerSelector),
    /// Colors present on a player's board
    ColorsOnBoard(PlayerSelector),
    /// Unique colors (player has but opponents don't)
    UniqueColors(PlayerSelector),
    /// Range of numbers (for counting)
    Range { start: u8, end: u8 },
}

/// Cascade - effect triggers another based on result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cascade {
    /// Initial effect
    pub initial: Box<Effect>,
    /// Condition based on result of initial
    pub condition: GameCondition,
    /// Follow-up effect if condition met
    pub follow_up: Box<Effect>,
}

// ============================================================================
// DOGMA TYPES
// ============================================================================

/// Type of dogma effect sharing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DogmaType {
    /// Supremacy dogma (I demand) - affects players with fewer symbols
    Supremacy(Symbol),
    /// Cooperative dogma - affects players with equal or more symbols
    Cooperative(Symbol),
    /// Non-demand effect (only affects activator)
    NonDemand,
}

/// Wrapper for dogma effects with sharing rules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DogmaWrapper {
    /// Type of dogma (determines who is affected)
    pub dogma_type: DogmaType,
    /// Effects for affected players
    pub effects: Vec<Effect>,
    /// Effects for activator if victims exist (supremacy bonus)
    pub activator_bonus: Option<Box<Effect>>,
}

// ============================================================================
// HELPER CONSTRUCTORS
// ============================================================================

impl Optional {
    /// Create an optional effect with no follow-up
    pub fn new(effect: Effect) -> Self {
        Self {
            effect: Box::new(effect),
            on_success: None,
        }
    }

    /// Create an optional effect with a follow-up
    pub fn with_success(effect: Effect, on_success: Effect) -> Self {
        Self {
            effect: Box::new(effect),
            on_success: Some(Box::new(on_success)),
        }
    }
}

impl Conditional {
    /// Create a conditional with only true branch
    pub fn new(condition: GameCondition, on_true: Effect) -> Self {
        Self {
            condition,
            on_true: Box::new(on_true),
            on_false: None,
        }
    }

    /// Create a conditional with both branches
    pub fn with_else(condition: GameCondition, on_true: Effect, on_false: Effect) -> Self {
        Self {
            condition,
            on_true: Box::new(on_true),
            on_false: Some(Box::new(on_false)),
        }
    }
}

impl Choice {
    /// Create a choice where player picks exactly one option
    pub fn pick_one(options: Vec<Effect>) -> Self {
        Self {
            options,
            min_choices: 1,
            max_choices: 1,
        }
    }

    /// Create a choice where player can pick multiple
    pub fn pick_multiple(options: Vec<Effect>, min: u8, max: u8) -> Self {
        Self {
            options,
            min_choices: min,
            max_choices: max,
        }
    }
}

impl ForEach {
    /// Iterate over cards
    pub fn cards(selector: CardSelector, effect: Effect) -> Self {
        Self {
            collection: Collection::Cards(selector),
            effect: Box::new(effect),
        }
    }

    /// Iterate over players
    pub fn players(selector: PlayerSelector, effect: Effect) -> Self {
        Self {
            collection: Collection::Players(selector),
            effect: Box::new(effect),
        }
    }

    /// Iterate over colors on board
    pub fn colors_on_board(player: PlayerSelector, effect: Effect) -> Self {
        Self {
            collection: Collection::ColorsOnBoard(player),
            effect: Box::new(effect),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optional_constructor() {
        let opt = Optional::new(Effect::None);
        assert!(opt.on_success.is_none());
    }

    #[test]
    fn test_conditional_constructor() {
        let cond = Conditional::new(
            GameCondition::NoCardsTransferred,
            Effect::None,
        );
        assert!(cond.on_false.is_none());
    }

    #[test]
    fn test_choice_constructor() {
        let choice = Choice::pick_one(vec![Effect::None, Effect::None]);
        assert_eq!(choice.min_choices, 1);
        assert_eq!(choice.max_choices, 1);
        assert_eq!(choice.options.len(), 2);
    }
}
