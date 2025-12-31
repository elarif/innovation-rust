use serde::{Serialize, Deserialize};
use crate::model::Color;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputRequest {
    SelectCard {
        player_id: usize,
        source: Vec<String>, // List of card IDs to choose from
        min: usize,
        max: usize,
    },
    SelectColor {
        player_id: usize,
        colors: Vec<Color>,
    },
    Confirm {
        player_id: usize,
        message: String,
    },
}

/// Tracks position within nested effect sequences.
/// Replaces magic number encoding (index * 100 + child_step).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubStep {
    /// Index within a Sequence effect (0-based)
    pub sequence_index: usize,
    /// Step within the current effect (0 = initial, 1+ = waiting for input)
    pub child_step: usize,
}

impl SubStep {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Advance to next effect in sequence, resetting child step
    pub fn advance_sequence(&mut self) {
        self.sequence_index += 1;
        self.child_step = 0;
    }
    
    /// Reset to beginning
    pub fn reset(&mut self) {
        self.sequence_index = 0;
        self.child_step = 0;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DogmaExecutionState {
    pub card_id: String,
    pub dogma_index: usize,
    
    // Players who will execute the effect
    pub eligible_players: Vec<usize>,
    
    // Where we are in the list of eligible players
    // If current_player_index < eligible_players.len(), we are executing for that player
    pub current_player_index: usize,
    
    // For multi-step effects - now using explicit struct
    pub sub_step: SubStep,

    // "I Share" bonus tracking
    pub activator_id: usize,
    pub anyone_shared: bool,
    
    // Context for consecutive effects ("If you do, ...")
    pub context_color: Option<Color>, 
    pub context_value: Option<u8>,
}

impl DogmaExecutionState {
    pub fn new(card_id: String, dogma_index: usize, activator_id: usize, eligible: Vec<usize>) -> Self {
        Self {
            card_id,
            dogma_index,
            eligible_players: eligible,
            current_player_index: 0,
            sub_step: SubStep::new(),
            activator_id,
            anyone_shared: false,
            context_color: None,
            context_value: None,
        }
    }
}
