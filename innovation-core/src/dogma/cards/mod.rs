pub mod age1;
pub mod age2;

use crate::game_state::GameState;
use crate::errors::GameError;
use crate::db::load_all_cards;

pub fn execute_effect(state: &mut GameState, player_id: usize, card_name: &str, dogma_index: usize, input: Option<String>) -> Result<(), GameError> {
    let db = load_all_cards();
    if let Some(card) = db.get(card_name) {
        if card.age == 1 {
            return age1::execute(state, player_id, card_name, dogma_index, input);
        } else if card.age == 2 {
            return age2::execute(state, player_id, card_name, dogma_index, input);
        }
    }
    
    // Fallback if not found or not age 1/2
    // If not found, maybe it's a test card or error?
    println!("Warning: No execution logic found for {}", card_name);
    Ok(())
}
