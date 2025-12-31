use crate::game_state::GameState;
use crate::errors::GameError;
use crate::dogma::flow::DogmaExecutionState;

pub fn execute_dogma(state: &mut GameState, player_id: usize, card_id: String) -> Result<(), GameError> {
    // 1. Initialize State
    let execution_state = DogmaExecutionState::new(card_id.clone(), 0, player_id, Vec::new());
    state.dogma_state = Some(execution_state);
    
    // 2. Start Execution
    continue_execution(state, None)
}

pub fn continue_execution(state: &mut GameState, mut input: Option<String>) -> Result<(), GameError> {
    // Clear any pending input request as we are now processing it
    if input.is_some() {
        state.pending_input = None;
    }
    
    // Load state
    let mut execution_state = state.dogma_state.clone().ok_or(GameError::InvalidAction("No dogma execution in progress".into()))?;
    
    // Load card
    let card = {
        let db = crate::db::load_all_cards();
        db.get(&execution_state.card_id).ok_or_else(|| GameError::CardNotFound(execution_state.card_id.clone()))?.clone()
    };

    // Loop through dogmas starting from current index
    while execution_state.dogma_index < card.dogmas.len() {
        let dogma = &card.dogmas[execution_state.dogma_index];
        
        // Ensure eligible players are calculated for this dogma
        // We might want to do this only once per dogma index.
        // Simplified: If current_player_index == 0, recalculate eligible.
        if execution_state.current_player_index == 0 {
             let symbol = dogma.symbol;
             
             let counts: Vec<u32> = state.players.iter()
                .map(|p| *p.icon_counts(crate::db::load_all_cards()).get(&symbol).unwrap_or(&0))
                .collect();
             let activator_count = counts[execution_state.activator_id];
             
             let player_count = state.players.len();
             let mut eligible = Vec::new();
             
             // Calc processing order
             for i in 1..=player_count {
                 let pid = (execution_state.activator_id + i) % player_count;
                 
                 let is_eligible = if dogma.is_supremacy {
                     pid != execution_state.activator_id && counts[pid] < activator_count
                 } else {
                     counts[pid] >= activator_count
                 };
                 
                 if is_eligible {
                     eligible.push(pid);
                 }
             }
             execution_state.eligible_players = eligible;
             
             // Save state update
             state.dogma_state = Some(execution_state.clone());
        }
        
    // Loop through eligible players
        while execution_state.current_player_index < execution_state.eligible_players.len() {
            let pid = execution_state.eligible_players[execution_state.current_player_index];
            
            // Execute Effect
            let input_for_this_step = input.take(); 
            
            // Sync state before execution so called function sees correct player/index
            state.dogma_state = Some(execution_state.clone());

            // Hybrid Dispatch: Check Registry first
            if let Some(effects) = crate::dogma::registry::get_effects(&card.name) {
                if execution_state.dogma_index < effects.len() {
                    let effect = &effects[execution_state.dogma_index];
                    execute_effect_tree(state, pid, effect, input_for_this_step)?;
                } else {
                    // Registry has fewer effects than card? Warning or no-op.
                }
            } else {
                // Fallback to legacy
                crate::dogma::cards::execute_effect(state, pid, &card.name, execution_state.dogma_index, input_for_this_step)?;
            }
            
            // If effect requested input, Pause.
            if state.pending_input.is_some() {
                 return Ok(());
            }
            
            // Re-sync execution_state from state.dogma_state to capture any sub_step changes
            // made by execute_effect_tree (critical for Sequence effects)
            if let Some(ds) = state.dogma_state.as_ref() {
                execution_state.sub_step = ds.sub_step.clone();
                execution_state.context_color = ds.context_color;
                execution_state.context_value = ds.context_value;
            }
            
            execution_state.sub_step.reset();
            
            // Advance player
            execution_state.current_player_index += 1;
            state.dogma_state = Some(execution_state.clone());
        }
        
        // Sharing bonus check
        // ...
        
        // Advance dogma
        execution_state.dogma_index += 1;
        execution_state.current_player_index = 0;
        execution_state.eligible_players.clear();
        execution_state.sub_step.reset();
        state.dogma_state = Some(execution_state.clone());
    }
    
    // Finished
    state.dogma_state = None;
    Ok(())
}

fn execute_effect_tree(state: &mut GameState, player_id: usize, effect: &crate::model::Effect, input: Option<String>) -> Result<(), GameError> {
    use crate::model::Effect;
    
    // Extract context value early to avoid borrow checker issues in closures
    let context_value = state.dogma_state.as_ref().and_then(|ds| ds.context_value);
    
    match effect {
        Effect::Conditional { condition, success, failure } => {
             use crate::model::Condition;
             let met = match condition {
                 Condition::ContextValueGreaterThan(v) => {
                     let cv = state.dogma_state.as_ref().and_then(|ds| ds.context_value).unwrap_or(0);
                     cv > *v
                 },
                 Condition::IconCountGreaterThanOrEqual(symbol, count) => {
                     let db = crate::db::load_all_cards();
                     let icon_count = state.players[player_id].icon_counts(db).get(symbol).cloned().unwrap_or(0);
                     icon_count >= *count as u32
                 },
                 Condition::ScoreGreaterThanHand => {
                     let p = &state.players[player_id];
                     p.score_pile.len() > p.hand.len()
                 },
                 Condition::True => true,
             };
             if met {
                 execute_effect_tree(state, player_id, success, input)?;
             } else if let Some(fail) = failure {
                 execute_effect_tree(state, player_id, fail, input)?;
             }
             Ok(())
        },
        Effect::Draw { amount, age } => {
            let target_age = (*age).unwrap_or(1);
            for _ in 0..*amount {
                state.draw_age(player_id, target_age)?;
            }
            Ok(())
        },
        Effect::DrawAndMeld { amount, age } => {
             let target_age = (*age).unwrap_or(1);
             for _ in 0..*amount {
                 if let Some(cid) = state.draw_age(player_id, target_age)? {
                     state.meld(player_id, cid)?;
                 }
             }
             Ok(())
        },
        Effect::DrawAndScore { amount, age } => {
             let target_age = (*age).or(context_value).unwrap_or(1);
             for _ in 0..*amount {
                 if let Some(cid) = state.draw_age(player_id, target_age)? {
                     state.remove_from_hand(player_id, &cid)?;
                     state.score_card(player_id, cid)?;
                 }
             }
             Ok(())
        },
        Effect::Recycle { min, max, source, filters } => {
            execute_recycle(state, player_id, *min as usize, *max as usize, source, filters, input)
        },
        Effect::Score { min, max, filters } => {
            execute_score(state, player_id, *min as usize, *max as usize, filters, input)
        },
        Effect::Meld { source: _, filters, min, max } => {
            execute_meld(state, player_id, *min as usize, *max as usize, filters, input)
        },
        Effect::Tuck { min, max, source: _, filters } => {
             execute_tuck(state, player_id, *min as usize, *max as usize, filters, input)
        },
        Effect::Splay { direction, color } => {
             execute_splay(state, player_id, direction, color)
        },
        Effect::Sequence(effects) => {
             execute_sequence(state, player_id, effects, input)
        },
        Effect::Transfer { source, dest, dest_is_activator, filters, min, max } => {
             execute_transfer(state, player_id, source, dest, *dest_is_activator, filters, *min as usize, *max as usize, input)
        },
        Effect::DrawUntilNoMatch { age, condition, on_match } => {
            execute_draw_until_no_match(state, player_id, *age, condition, on_match)
        },
        Effect::DrawAndCheck { age, condition, on_match, on_fail } => {
            execute_draw_and_check(state, player_id, *age, condition, on_match, on_fail.as_deref())
        },
        Effect::DrawAndScoreForUniqueColors { age } => {
            execute_draw_and_score_unique_colors(state, player_id, *age)
        },
        Effect::DrawForIconPairs { age, symbol, divisor } => {
            execute_draw_for_icon_pairs(state, player_id, *age, symbol, *divisor)
        },
        _ => {
            Ok(())
        }
    }
}

fn execute_sequence(state: &mut GameState, player_id: usize, effects: &[crate::model::Effect], mut input: Option<String>) -> Result<(), GameError> {
    loop {
        // Read current position - sequence_index tracks where we are in THIS Sequence
        let seq_idx = state.dogma_state.as_ref().map(|s| s.sub_step.sequence_index).unwrap_or(0);
        let child = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
        
        if seq_idx >= effects.len() {
            // Finished all effects in this sequence, reset for potential re-entry
            if let Some(ds) = state.dogma_state.as_mut() { ds.sub_step.reset(); }
            return Ok(());
        }
        
        let current_effect = &effects[seq_idx];
        
        // Before calling child, reset child_step appropriately
        if let Some(ds) = state.dogma_state.as_mut() { 
            ds.sub_step.child_step = child; 
        }
        
        // Pass input only if it exists (consumed by take())
        let step_input = input.take(); 
        
        execute_effect_tree(state, player_id, current_effect, step_input)?;
        
        if state.pending_input.is_some() {
            // Paused waiting for input. Save current position.
            let current_child = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
            if let Some(ds) = state.dogma_state.as_mut() { 
                ds.sub_step.sequence_index = seq_idx;
                ds.sub_step.child_step = current_child;
            }
            return Ok(());
        }
        
        // Finished this effect. Advance to next in sequence.
        // Reset child_step since next effect starts fresh.
        if let Some(ds) = state.dogma_state.as_mut() { 
            ds.sub_step.sequence_index = seq_idx + 1;
            ds.sub_step.child_step = 0;
        }
        // Loop continues to next effect immediately
    }
}

#[allow(clippy::too_many_arguments)]
fn execute_transfer(state: &mut GameState, player_id: usize, source: &crate::model::Location, dest: &crate::model::Location, dest_is_activator: bool, filters: &[crate::model::Filter], min: usize, max: usize, input: Option<String>) -> Result<(), GameError> {
    let child_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    let dest_pid = if dest_is_activator {
        state.dogma_state.as_ref().map(|s| s.activator_id).unwrap_or(player_id)
    } else {
        player_id
    };

    if child_step == 0 {
        let p = &state.players[player_id];
        let mut candidates = Vec::new();
        
        match source {
            crate::model::Location::Board => {
                 for pile in p.board.values() {
                     if let Some(top) = pile.top() {
                         candidates.push(top.clone());
                     }
                 }
            },
            crate::model::Location::Hand => {
                candidates = p.hand.clone();
            },
            _ => {} 
        }
        
        let db = crate::db::load_all_cards();
        candidates = apply_filters(candidates, filters, db, p);
        
        if candidates.is_empty() { return Ok(()); }

        let count = candidates.len();
        if count <= max {
             // Auto transfer
             let transferred_count = count;
             if let Some(ds) = state.dogma_state.as_mut() { ds.context_value = Some(transferred_count as u8); }

             for cid in candidates {
                 // Remove
                 match source {
                    crate::model::Location::Board => {
                         let card = db.get(&cid).ok_or_else(|| GameError::CardNotFound(cid.clone()))?;
                         if let Some(pile) = state.players[player_id].board.get_mut(&card.color) {
                             if let Some(top) = pile.top() {
                                 if *top == cid { pile.pop(); }
                             }
                         }
                    },
                    crate::model::Location::Hand => {
                        state.remove_from_hand(player_id, &cid)?;
                    },
                    _ => {}
                 }
                 // Add
                 match dest {
                    crate::model::Location::Score => { state.score_card(dest_pid, cid)?; },
                    crate::model::Location::Hand => { state.players[dest_pid].hand.push(cid); },
                    _ => {}
                 }
             }
             return Ok(());
        }

        // Request Input
        state.pending_input = Some(crate::dogma::flow::InputRequest::SelectCard {
            player_id, 
            source: candidates, 
            min, 
            max
        });
        if let Some(ds) = state.dogma_state.as_mut() { ds.sub_step.child_step = 1; }
        Ok(())
    } else {
         if let Some(s) = input {
             let cards: Vec<&str> = s.split(',').filter(|x| !x.is_empty()).collect();
             let count = cards.len();
             if let Some(ds) = state.dogma_state.as_mut() { ds.context_value = Some(count as u8); }
             
             for c in cards {
                 // Remove
                 match source {
                    crate::model::Location::Board => {
                        let db = crate::db::load_all_cards();
                        let card = db.get(c).ok_or_else(|| GameError::CardNotFound(c.to_string()))?;
                        if let Some(pile) = state.players[player_id].board.get_mut(&card.color) {
                             pile.pop(); 
                        }
                    },
                    crate::model::Location::Hand => {
                        state.remove_from_hand(player_id, &c.to_string())?;
                    },
                    _ => {}
                 }
                 // Add
                 match dest {
                    crate::model::Location::Score => { state.score_card(dest_pid, c.to_string())?; },
                    crate::model::Location::Hand => { state.players[dest_pid].hand.push(c.to_string()); },
                    _ => {}
                 }
             }
         }
         Ok(())
    }
}

fn execute_recycle(state: &mut GameState, player_id: usize, min: usize, max: usize, source: &crate::model::Location, filters: &[crate::model::Filter], input: Option<String>) -> Result<(), GameError> {
    let db = crate::db::load_all_cards();
    let mut p_cards = match source {
        crate::model::Location::Hand => state.players[player_id].hand.clone(),
        crate::model::Location::Score => state.players[player_id].score_pile.clone(),
        _ => Vec::new(),
    };
    
    p_cards = apply_filters(p_cards, filters, db, &state.players[player_id]);

    let child_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);

    if child_step == 0 {
        if p_cards.is_empty() { 
            // Set context_value = 0 so subsequent Conditionals see correct count
            if let Some(ds) = state.dogma_state.as_mut() { ds.context_value = Some(0); }
            return Ok(()); 
        }
        state.pending_input = Some(crate::dogma::flow::InputRequest::SelectCard {
            player_id, 
            source: p_cards, 
            min, 
            max 
        });
        if let Some(ds) = state.dogma_state.as_mut() { ds.sub_step.child_step = 1; }
        Ok(())
    } else {
        if let Some(s) = input {
             let cards: Vec<&str> = s.split(',').filter(|x| !x.is_empty()).collect();
             let count = cards.len();
             if let Some(ds) = state.dogma_state.as_mut() { ds.context_value = Some(count as u8); }

             for c in cards {
                 match source {
                    crate::model::Location::Hand => { state.remove_from_hand(player_id, &c.to_string())?; },
                    crate::model::Location::Score => { 
                        let p = &mut state.players[player_id];
                        if let Some(idx) = p.score_pile.iter().position(|x| x == c) {
                            p.score_pile.remove(idx);
                        }
                    },
                    _ => {}
                 }
                 state.return_card(c.to_string())?;
             }
        }
        Ok(())
    }
}

fn execute_score(state: &mut GameState, player_id: usize, min: usize, max: usize, filters: &[crate::model::Filter], input: Option<String>) -> Result<(), GameError> {
    let child_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    
    if child_step == 0 {
        let p = &state.players[player_id];
        let mut candidates = p.hand.clone();
        let db = crate::db::load_all_cards();
        
        candidates = apply_filters(candidates, filters, db, p);
        
        if candidates.is_empty() { 
            if let Some(ds) = state.dogma_state.as_mut() { ds.context_value = Some(0); }
            return Ok(()); 
        }
        
        state.pending_input = Some(crate::dogma::flow::InputRequest::SelectCard {
            player_id, 
            source: candidates, 
            min, 
            max 
        });
        if let Some(ds) = state.dogma_state.as_mut() { ds.sub_step.child_step = 1; }
        Ok(())
    } else {
         if let Some(s) = input {
             let cards: Vec<&str> = s.split(',').filter(|x| !x.is_empty()).collect();
             let count = cards.len();
             if let Some(ds) = state.dogma_state.as_mut() { ds.context_value = Some(count as u8); }

             for c in cards {
                 state.remove_from_hand(player_id, &c.to_string())?;
                 state.score_card(player_id, c.to_string())?;
             }
         }
         Ok(())
    }
}

fn execute_meld(state: &mut GameState, player_id: usize, min: usize, max: usize, filters: &[crate::model::Filter], input: Option<String>) -> Result<(), GameError> {
    let child_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    if child_step == 0 {
        let p = &state.players[player_id];
        let mut candidates = p.hand.clone();
        let db = crate::db::load_all_cards();
        
        candidates = apply_filters(candidates, filters, db, p);
        
        if candidates.is_empty() { 
            if let Some(ds) = state.dogma_state.as_mut() { ds.context_value = Some(0); }
            return Ok(()); 
        }
        
        if candidates.len() == 1 && min == 1 && max == 1 {
            let cid = candidates[0].clone();
             let card_color = db.get(&cid).ok_or_else(|| GameError::CardNotFound(cid.clone()))?.color;
             if let Some(ds) = state.dogma_state.as_mut() { 
                 ds.context_color = Some(card_color); 
                 ds.context_value = Some(1);
             }
             
            state.meld(player_id, cid)?;
            return Ok(());
        }
        
        state.pending_input = Some(crate::dogma::flow::InputRequest::SelectCard {
            player_id, 
            source: candidates, 
            min, 
            max 
        });
        if let Some(ds) = state.dogma_state.as_mut() { ds.sub_step.child_step = 1; }
        Ok(())
    } else {
         if let Some(s) = input { 
             let cards: Vec<&str> = s.split(',').filter(|x| !x.is_empty()).collect();
             let count = cards.len();
             
             for cid in cards {
                 let db = crate::db::load_all_cards();
                 let card_color = db.get(cid).ok_or_else(|| GameError::CardNotFound(cid.to_string()))?.color;
                 if let Some(ds) = state.dogma_state.as_mut() { 
                     ds.context_color = Some(card_color); 
                     // Since loop updates, keep context_value accumulating or strict count?
                     // Usually dogma says "If you meld X..." so total count matters.
                     // But strictly speaking, context_value is usually set to TOTAL count.
                 }
                 state.meld(player_id, cid.to_string())?;
             }
             // Set final count
             if let Some(ds) = state.dogma_state.as_mut() { ds.context_value = Some(count as u8); }
         }
         Ok(())
    }
}

fn execute_tuck(state: &mut GameState, player_id: usize, min: usize, max: usize, filters: &[crate::model::Filter], input: Option<String>) -> Result<(), GameError> {
     let child_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
     if child_step == 0 {
         let p = &state.players[player_id];
         let mut candidates = p.hand.clone();
         let db = crate::db::load_all_cards();
         
         candidates = apply_filters(candidates, filters, db, p);
         
         if candidates.is_empty() { 
             if let Some(ds) = state.dogma_state.as_mut() { ds.context_value = Some(0); }
             return Ok(()); 
         }
         
         state.pending_input = Some(crate::dogma::flow::InputRequest::SelectCard {
             player_id, 
             source: candidates, 
             min, 
             max 
         });
         if let Some(ds) = state.dogma_state.as_mut() { ds.sub_step.child_step = 1; }
         Ok(())
     } else {
          if let Some(s) = input {
              let cards: Vec<&str> = s.split(',').filter(|x| !x.is_empty()).collect();
              let count = cards.len();
              
              for c in cards {
                  let db = crate::db::load_all_cards();
                  let card_color = db.get(c).ok_or_else(|| GameError::CardNotFound(c.to_string()))?.color;
                  if let Some(ds) = state.dogma_state.as_mut() { ds.context_color = Some(card_color); }
                  state.remove_from_hand(player_id, &c.to_string())?;
                  state.tuck_card(player_id, c.to_string())?;
              }
              if let Some(ds) = state.dogma_state.as_mut() { ds.context_value = Some(count as u8); }
          }
          Ok(())
     }
}

fn execute_splay(state: &mut GameState, player_id: usize, direction: &crate::model::SplayDirection, color: &Option<crate::model::Color>) -> Result<(), GameError> {
    let target_color = match color {
        Some(c) => *c,
        None => {
            state.dogma_state.as_ref().and_then(|ds| ds.context_color)
                .ok_or(GameError::InvalidAction("No context color for splay".into()))?
        }
    };
    state.splay(player_id, target_color, *direction)?;
    Ok(())
}
// Helper for filtering candidates
fn apply_filters(mut candidates: Vec<String>, filters: &[crate::model::Filter], db: &std::collections::HashMap<String, crate::model::Card>, player: &crate::player::Player) -> Vec<String> {
    // eprintln!("Applying filters: {:?} to candidates: {:?}", filters, candidates);
    for f in filters {
        match f {
            crate::model::Filter::ColorsPresentOnBoard => {
                candidates.retain(|id| {
                    if let Some(c) = db.get(id) {
                        player.board.contains_key(&c.color)
                    } else { false }
                });
            },
            crate::model::Filter::MinAge => {
                if candidates.is_empty() { continue; }
                let min_age = candidates.iter().filter_map(|id| db.get(id)).map(|c| c.age).min().unwrap_or(0);
                candidates.retain(|id| db.get(id).map(|c| c.age).unwrap_or(0) == min_age);
            },
            crate::model::Filter::MaxAge => {
                if candidates.is_empty() { continue; }
                let max_age = candidates.iter().filter_map(|id| db.get(id)).map(|c| c.age).max().unwrap_or(0);
                candidates.retain(|id| db.get(id).map(|c| c.age).unwrap_or(0) == max_age);
            },
            crate::model::Filter::Value(v) => {
                 candidates.retain(|id| db.get(id).map(|c| c.age).unwrap_or(0) == *v);
            },
            crate::model::Filter::Color(col) => {
                 candidates.retain(|id| db.get(id).map(|c| c.color).unwrap_or(crate::model::Color::Red) == *col);
            },
            crate::model::Filter::Icon(target_sym) => {
                 candidates.retain(|id| {
                     if let Some(c) = db.get(id) {
                         c.icons.iter().any(|icon| match icon {
                             crate::model::Icon::Resource(s) => s == target_sym,
                             _ => false
                         })
                     } else { false }
                 });
            },
            crate::model::Filter::ColorsNotOnBoard => {
                candidates.retain(|id| {
                    if let Some(c) = db.get(id) {
                        !player.board.contains_key(&c.color)
                    } else { false }
                });
            },
            _ => {} // Implement others as needed
        }
    }
    candidates
}

/// Métallurgie: Draw until no match. Score matching cards, keep last in hand.
fn execute_draw_until_no_match(
    state: &mut GameState, 
    player_id: usize, 
    age: u8,
    condition: &crate::model::DrawnCardCondition,
    on_match: &crate::model::Effect
) -> Result<(), GameError> {
    use crate::model::DrawnCardCondition;
    let db = crate::db::load_all_cards();
    
    loop {
        if let Some(cid) = state.draw_age(player_id, age)? {
            let card = db.get(&cid).ok_or_else(|| GameError::CardNotFound(cid.clone()))?;
            
            let matches = match condition {
                DrawnCardCondition::HasIcon(sym) => {
                    card.icons.iter().any(|icon| match icon {
                        crate::model::Icon::Resource(s) => s == sym,
                        _ => false
                    })
                },
                DrawnCardCondition::ColorOnBoard => {
                    state.players[player_id].board.contains_key(&card.color)
                },
            };
            
            if matches {
                // Execute on_match effect (typically Score)
                // Card is in hand from draw, need to handle based on effect
                match on_match {
                    crate::model::Effect::Score { .. } => {
                        state.remove_from_hand(player_id, &cid)?;
                        state.score_card(player_id, cid)?;
                    },
                    _ => {
                        // Generic: execute effect tree
                        execute_effect_tree(state, player_id, on_match, None)?;
                    }
                }
            } else {
                // Keep in hand and stop
                break;
            }
        } else {
            // No more cards to draw
            break;
        }
    }
    Ok(())
}

/// Mysticisme: Draw 1, check condition, branch
fn execute_draw_and_check(
    state: &mut GameState,
    player_id: usize,
    age: u8,
    condition: &crate::model::DrawnCardCondition,
    on_match: &crate::model::Effect,
    on_fail: Option<&crate::model::Effect>
) -> Result<(), GameError> {
    use crate::model::DrawnCardCondition;
    let db = crate::db::load_all_cards();
    
    if let Some(cid) = state.draw_age(player_id, age)? {
        let card = db.get(&cid).ok_or_else(|| GameError::CardNotFound(cid.clone()))?;
        
        let matches = match condition {
            DrawnCardCondition::HasIcon(sym) => {
                card.icons.iter().any(|icon| match icon {
                    crate::model::Icon::Resource(s) => s == sym,
                    _ => false
                })
            },
            DrawnCardCondition::ColorOnBoard => {
                state.players[player_id].board.contains_key(&card.color)
            },
        };
        
        // Store drawn card id in context for potential use by on_match/on_fail
        if let Some(ds) = state.dogma_state.as_mut() {
            ds.context_color = Some(card.color);
        }
        
        if matches {
            // For Mysticisme: Meld the drawn card and draw another
            match on_match {
                crate::model::Effect::Sequence(effects) => {
                    // First effect is typically Meld (the drawn card)
                    state.meld(player_id, cid)?;
                    // Execute remaining effects
                    for effect in effects.iter().skip(1) {
                        execute_effect_tree(state, player_id, effect, None)?;
                    }
                },
                _ => {
                    execute_effect_tree(state, player_id, on_match, None)?;
                }
            }
        } else if let Some(fail_effect) = on_fail {
            execute_effect_tree(state, player_id, fail_effect, None)?;
        }
        // Else: card stays in hand (default behavior from draw)
    }
    Ok(())
}

/// Tissage: Draw and score N cards where N = unique colors on board
fn execute_draw_and_score_unique_colors(
    state: &mut GameState,
    player_id: usize,
    age: u8
) -> Result<(), GameError> {
    // Count colors that are on my board but no opponent has
    let my_colors: Vec<crate::model::Color> = state.players[player_id].board.keys().cloned().collect();
    let mut unique_count = 0;
    
    for color in my_colors {
        let mut others_have = false;
        for (idx, p) in state.players.iter().enumerate() {
            if idx != player_id && p.board.contains_key(&color) {
                others_have = true;
                break;
            }
        }
        if !others_have {
            unique_count += 1;
        }
    }
    
    // Draw and score that many cards
    for _ in 0..unique_count {
        if let Some(cid) = state.draw_age(player_id, age)? {
            state.remove_from_hand(player_id, &cid)?;
            state.score_card(player_id, cid)?;
        }
    }
    
    Ok(())
}

/// Fermentation: Draw N cards where N = icon_count / divisor
fn execute_draw_for_icon_pairs(
    state: &mut GameState,
    player_id: usize,
    age: u8,
    symbol: &crate::model::Symbol,
    divisor: u8
) -> Result<(), GameError> {
    let db = crate::db::load_all_cards();
    let icon_count = state.players[player_id].icon_counts(db).get(symbol).cloned().unwrap_or(0);
    let draw_count = icon_count / divisor as u32;
    
    for _ in 0..draw_count {
        state.draw_age(player_id, age)?;
    }
    
    Ok(())
}
