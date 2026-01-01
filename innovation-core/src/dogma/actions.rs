/// Primitive Actions
/// 
/// This module defines all primitive actions that can be performed in the game.
/// These are the building blocks for card effects.

use crate::model::{Color, Location, SplayDirection};
use crate::dogma::selectors::{CardSelector, PlayerSelector};
use crate::dogma::conditions::GameCondition;
use serde::{Deserialize, Serialize};

// ============================================================================
// CARD MOVEMENT ACTIONS
// ============================================================================

/// Draw cards from a deck
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrawAction {
    /// How many cards to draw
    pub count: u8,
    /// Age to draw from (None = based on highest active card)
    pub age: Option<u8>,
    /// Who draws the cards
    pub target_player: PlayerSelector,
    /// Whether to reveal the drawn cards
    pub reveal: bool,
}

/// Meld (play) cards to the board
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeldAction {
    /// Which cards to meld
    pub selector: CardSelector,
    /// Where to meld them (target player's board)
    pub target_player: Option<PlayerSelector>,
}

/// Score (comptabiliser) cards
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreAction {
    /// Which cards to score
    pub selector: CardSelector,
    /// Who receives the scored cards
    pub target_player: Option<PlayerSelector>,
}

/// Archive (tuck) cards under a color pile
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveAction {
    /// Which cards to archive
    pub selector: CardSelector,
    /// Target color pile (None = same color as card)
    pub target_color: Option<Color>,
    /// Target player (None = owner of cards)
    pub target_player: Option<PlayerSelector>,
}

/// Recycle cards back to their age deck
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecycleAction {
    /// Which cards to recycle
    pub selector: CardSelector,
}

/// Transfer cards between locations/players
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferAction {
    /// Which cards to transfer
    pub selector: CardSelector,
    /// Destination location
    pub destination: Location,
    /// Destination player
    pub destination_player: PlayerSelector,
}

/// Exchange cards between two sets
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExchangeAction {
    /// First set of cards
    pub set_a: CardSelector,
    /// Second set of cards
    pub set_b: CardSelector,
}

/// Discard cards (remove from game)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscardAction {
    /// Which cards to discard
    pub selector: CardSelector,
}

// ============================================================================
// PILE MANIPULATION ACTIONS
// ============================================================================

/// Splay a color pile in a direction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplayAction {
    /// Which player's pile to splay
    pub player: PlayerSelector,
    /// Color to splay (None = last played color)
    pub color: Option<Color>,
    /// Direction to splay
    pub direction: SplayDirection,
}

/// Reorder cards in a pile
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReorderAction {
    /// Which player's pile
    pub player: PlayerSelector,
    /// Which color pile
    pub color: Color,
    /// New order (list of card indices)
    pub new_order: Vec<usize>,
}

// ============================================================================
// INFORMATION ACTIONS
// ============================================================================

/// Reveal cards to all players
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevealAction {
    /// Which cards to reveal
    pub selector: CardSelector,
}

/// Choose an element (color, age, player, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChooseAction {
    /// What type of element to choose
    pub choice_type: ChoiceType,
    /// Who makes the choice
    pub chooser: PlayerSelector,
    /// Store result in this variable name
    pub store_as: Option<String>,
}

/// Types of choices that can be made
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoiceType {
    /// Choose a color
    Color,
    /// Choose N colors
    Colors(u8),
    /// Choose an age value
    Age,
    /// Choose a card
    Card(CardSelector),
    /// Choose a player
    Player(PlayerSelector),
}

// ============================================================================
// VICTORY ACTIONS
// ============================================================================

/// Achieve a card (dominate)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AchieveAction {
    /// Which player achieves
    pub player: PlayerSelector,
    /// Type of achievement
    pub achievement_type: AchievementType,
}

/// Types of achievements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementType {
    /// Regular age achievement (card from center)
    Age(u8),
    /// Special domain achievement
    Domain(DomainType),
}

/// Special domain types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DomainType {
    Technologies,  // TECHNOLOGIES
    Military,      // MILITAIRE
    Diplomacy,     // DIPLOMATIE
    Culture,       // CULTURE
    Science,       // SCIENCES
}

/// Win the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WinAction {
    /// Which player wins
    pub winner: PlayerSelector,
    /// Condition that triggered the win
    pub condition: Option<String>,
}

// ============================================================================
// SPECIAL ACTIONS
// ============================================================================

/// Execute a dogma effect
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecuteDogmaAction {
    /// Which card's dogma to execute
    pub card: CardSelector,
    /// Whether to share (cooperative) or not
    pub share: bool,
    /// Which dogma index to execute (if card has multiple)
    pub dogma_index: Option<usize>,
}

/// Repeat the current effect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepeatAction {
    /// How many times to repeat
    pub times: u8,
}

/// Store a value in a variable for later use
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoreAction {
    /// What to store
    pub value: StoredValue,
    /// Variable name
    pub name: String,
}

/// Values that can be stored
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StoredValue {
    /// Card reference
    Card(CardSelector),
    /// Number (e.g., count of cards)
    Number(u8),
    /// Color
    Color(Color),
    /// Player
    Player(PlayerSelector),
}

// ============================================================================
// ACTION MODIFIERS
// ============================================================================

/// Modify how an action is executed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModifiedAction {
    /// The base action
    pub action: Box<crate::model::Effect>,
    /// Modifiers to apply
    pub modifiers: Vec<ActionModifier>,
}

/// Action modifiers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionModifier {
    /// Action is optional
    Optional,
    /// Only if condition is met
    OnlyIf(GameCondition),
    /// Repeat N times
    Repeat(u8),
    /// For each element in collection
    ForEach(crate::dogma::conditions::Collection),
}

// ============================================================================
// HELPER CONSTRUCTORS
// ============================================================================

impl DrawAction {
    /// Draw cards based on highest active card age
    pub fn from_highest_active(count: u8, target_player: PlayerSelector) -> Self {
        Self {
            count,
            age: None,
            target_player,
            reveal: false,
        }
    }

    /// Draw cards of specific age
    pub fn of_age(count: u8, age: u8, target_player: PlayerSelector) -> Self {
        Self {
            count,
            age: Some(age),
            target_player,
            reveal: false,
        }
    }

    /// Draw and reveal
    pub fn and_reveal(mut self) -> Self {
        self.reveal = true;
        self
    }
}

impl MeldAction {
    /// Meld to own board
    pub fn to_own_board(selector: CardSelector) -> Self {
        Self {
            selector,
            target_player: None,
        }
    }

    /// Meld to another player's board
    pub fn to_player_board(selector: CardSelector, player: PlayerSelector) -> Self {
        Self {
            selector,
            target_player: Some(player),
        }
    }
}

impl ScoreAction {
    /// Score to own pile
    pub fn to_own_score(selector: CardSelector) -> Self {
        Self {
            selector,
            target_player: None,
        }
    }

    /// Score to another player's pile
    pub fn to_player_score(selector: CardSelector, player: PlayerSelector) -> Self {
        Self {
            selector,
            target_player: Some(player),
        }
    }
}

impl SplayAction {
    /// Splay player's color in direction
    pub fn color(player: PlayerSelector, color: Color, direction: SplayDirection) -> Self {
        Self {
            player,
            color: Some(color),
            direction,
        }
    }

    /// Splay last played color
    pub fn last_color(player: PlayerSelector, direction: SplayDirection) -> Self {
        Self {
            player,
            color: None,
            direction,
        }
    }
}

impl AchieveAction {
    /// Achieve an age card
    pub fn age(player: PlayerSelector, age: u8) -> Self {
        Self {
            player,
            achievement_type: AchievementType::Age(age),
        }
    }

    /// Achieve a domain
    pub fn domain(player: PlayerSelector, domain: DomainType) -> Self {
        Self {
            player,
            achievement_type: AchievementType::Domain(domain),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_action_constructors() {
        let draw = DrawAction::of_age(2, 5, PlayerSelector::Activator);
        assert_eq!(draw.count, 2);
        assert_eq!(draw.age, Some(5));
        assert!(!draw.reveal);

        let draw_reveal = draw.and_reveal();
        assert!(draw_reveal.reveal);
    }

    #[test]
    fn test_achieve_action() {
        let achieve = AchieveAction::domain(
            PlayerSelector::Activator,
            DomainType::Culture,
        );
        match achieve.achievement_type {
            AchievementType::Domain(DomainType::Culture) => (),
            _ => panic!("Wrong achievement type"),
        }
    }
}
