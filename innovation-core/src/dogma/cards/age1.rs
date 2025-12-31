#![allow(dead_code)] // Legacy implementations kept for reference/fallback
use crate::game_state::GameState;
use crate::errors::GameError;
use crate::dogma::flow::InputRequest;
use crate::model::Color;

pub fn execute(state: &mut GameState, player_id: usize, card_name: &str, dogma_index: usize, input: Option<String>) -> Result<(), GameError> {
    match card_name {
        // "Agriculture" => agriculture(state, player_id, dogma_index, input),
        // "Archerie" => archerie(state, player_id, dogma_index, input),
        "Tissage" => tissage(state, player_id, dogma_index, input),
        // "Code de lois" => code_de_lois(state, player_id, dogma_index, input),
        // "Élevage" => elevage(state, player_id, dogma_index, input),
        // "Maçonnerie" => maconnerie(state, player_id, dogma_index, input),
        "Métallurgie" => metallurgie(state, player_id, dogma_index, input),
        "Mysticisme" => mysticisme(state, player_id, dogma_index, input),
        "Rames" => rames(state, player_id, dogma_index, input),
        // "Poterie" => poterie(state, player_id, dogma_index, input),
        // "Fabrication d'outils" => fabrication_d_outils(state, player_id, dogma_index, input),
        // "Voiles" => voiles(state, player_id, dogma_index, input),
        // "La Roue" => la_roue(state, player_id, dogma_index, input),
        // "Outils" => outils(state, player_id, dogma_index, input),
        // "Écriture" => ecriture(state, player_id, dogma_index, input),
        "Cités-États" => cites_etats(state, player_id, dogma_index, input),
        _ => {
            println!("Warning: Unimplemented card {}", card_name);
            Ok(())
        }
    }
}

// --- Card Implementations ---

fn agriculture(state: &mut GameState, player_id: usize, _dogma_index: usize, input: Option<String>) -> Result<(), GameError> {
    // 0: Return card from hand. If do, Score card from hand.
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    
    if sub_step == 0 {
        if state.players[player_id].hand.is_empty() { return Ok(()); }
        state.pending_input = Some(InputRequest::SelectCard {
            player_id,
            source: state.players[player_id].hand.clone(),
            min: 0, max: 1
        });
        update_sub_step(state, 1);
        Ok(())
    } else if sub_step == 1 {
        if let Some(cid) = input.filter(|s| !s.is_empty()) {
            state.remove_from_hand(player_id, &cid)?;
            state.return_card(cid)?;
            
            if state.players[player_id].hand.is_empty() { return Ok(()); }
            state.pending_input = Some(InputRequest::SelectCard {
                player_id,
                source: state.players[player_id].hand.clone(),
                min: 1, max: 1
            });
            update_sub_step(state, 2);
        }
        Ok(())
    } else if sub_step == 2 {
        if let Some(cid) = input {
            state.remove_from_hand(player_id, &cid)?;
            state.score_card(player_id, cid)?;
        }
        Ok(())
    } else { Ok(()) }
}

fn archerie(state: &mut GameState, player_id: usize, _dogma_index: usize, input: Option<String>) -> Result<(), GameError> {
    // 0: Demand: Draw 1. Transfer highest hand to My Hand.
    // Note: Demands are executed by VICTIMS (pid). "My" = Activator.
    let activator_id = state.dogma_state.as_ref().unwrap().activator_id;
    
    // Draw 1
    state.draw_age(player_id, 1)?;
    
    // Transfer highest
    let p = &mut state.players[player_id];
    if p.hand.is_empty() { return Ok(()); }
    
    // Find highest age cards
    let db = crate::db::load_all_cards();
    let max_age = p.hand.iter().filter_map(|id| db.get(id)).map(|c| c.age).max().unwrap_or(0);
    let candidates: Vec<String> = p.hand.iter().filter(|id| db.get(*id).unwrap().age == max_age).cloned().collect();
    
    // If only 1, auto-transfer? Or always ask?
    // Ask if > 1, else auto.
    // Simplifying: Always ask for simplicity or auto if deterministic?
    // Let's ask.
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    if sub_step == 0 {
         if candidates.len() == 1 {
             // Auto
             let cid = candidates[0].clone();
             state.remove_from_hand(player_id, &cid)?;
             state.players[activator_id].hand.push(cid);
             return Ok(());
         }
         state.pending_input = Some(InputRequest::SelectCard {
             player_id, source: candidates, min: 1, max: 1
         });
         update_sub_step(state, 1);
         Ok(())
    } else {
        if let Some(cid) = input {
            state.remove_from_hand(player_id, &cid)?;
            state.players[activator_id].hand.push(cid);
        }
        Ok(())
    }
}

fn tissage(state: &mut GameState, player_id: usize, _dogma_index: usize, input: Option<String>) -> Result<(), GameError> {
    // Meld card of color you don't have. Then Draw 1 for each unique color and score.
    // Sub 0: Select card filter
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    if sub_step == 0 {
        let p = &state.players[player_id];
        let existing_colors: Vec<Color> = p.board.keys().cloned().collect();
        let db = crate::db::load_all_cards();
        let hand_candidates: Vec<String> = p.hand.iter()
            .filter(|id| !existing_colors.contains(&db.get(*id).unwrap().color))
            .cloned().collect();
            
        if hand_candidates.is_empty() {
             // Skip meld, proceed to draw
             // But wait, "Then" implies sequence.
        } else {
             state.pending_input = Some(InputRequest::SelectCard {
                 player_id, source: hand_candidates, min: 1, max: 1
             });
             update_sub_step(state, 1);
             return Ok(());
        }
        update_sub_step(state, 1); // Skip to draw part immediately
        Ok(())
    } else if sub_step == 1 {
        // Handle Meld if input
        if let Some(cid) = input {
             state.meld(player_id, cid)?;
        }
        
        // Count colors unique to self
        // (Simplified: Just count my colors for now, "unique" logic check all players)
        // Correct logic: My colors where NO OTHER PLAYER has that color.
        let my_colors: Vec<Color> = state.players[player_id].board.keys().cloned().collect();
        let mut count = 0;
        for c in my_colors {
            let mut others_have = false;
            for p in &state.players {
                if p.id != player_id && p.board.contains_key(&c) {
                    others_have = true; break;
                }
            }
            if !others_have { count += 1; }
        }
        
        for _ in 0..count {
            if let Some(cid) = state.draw_age(player_id, 1)? {
                state.remove_from_hand(player_id, &cid)?; // Draw usually puts in hand
                state.score_card(player_id, cid)?;
            }
        }
        Ok(())
    } else { Ok(()) }
}

fn code_de_lois(state: &mut GameState, player_id: usize, _dogma_index: usize, input: Option<String>) -> Result<(), GameError> {
    // Tuck a card of a color you already have splayed (or just present? "already on board").
    // "d'une couleur que vous avez déjà en jeu".
    // If you do, splay that color left.
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    if sub_step == 0 {
        // Filter hand for colors present on board
        let p = &state.players[player_id];
        let existing_colors: Vec<Color> = p.board.keys().cloned().collect();
        let db = crate::db::load_all_cards();
        let candidates: Vec<String> = p.hand.iter()
            .filter(|id| existing_colors.contains(&db.get(*id).unwrap().color))
            .cloned().collect();
            
        if candidates.is_empty() { return Ok(()); }
        state.pending_input = Some(InputRequest::SelectCard {
            player_id, source: candidates, min: 0, max: 1
        });
        update_sub_step(state, 1);
        Ok(())
    } else {
        if let Some(cid) = input.filter(|s| !s.is_empty()) {
            // Get color before removing/tucking
            let db = crate::db::load_all_cards();
            let color = db.get(&cid).unwrap().color;
            
            state.remove_from_hand(player_id, &cid)?;
            state.tuck_card(player_id, cid)?;
            
            // Splay Left
            state.splay(player_id, color, crate::model::SplayDirection::Left)?;
        }
        Ok(())
    }
}

fn elevage(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> { 
    // Meld lowest card from hand. Draw 1.
    // Lowest? By Age. If tie? Alphabetical? Choice?
    // Rules: "Lowest value". Tie -> Player choice.
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    
    if sub_step == 0 {
         let p = &state.players[player_id];
         if p.hand.is_empty() { 
             state.draw_age(player_id, 1)?; 
             return Ok(()); 
         }
         let db = crate::db::load_all_cards();
         let min_age = p.hand.iter().map(|id| db.get(id).unwrap().age).min().unwrap_or(11);
         let candidates: Vec<String> = p.hand.iter()
             .filter(|id| db.get(*id).unwrap().age == min_age)
             .cloned().collect();
             
         if candidates.len() == 1 {
             let cid = candidates[0].clone();
             state.remove_from_hand(player_id, &cid)?;
             state.meld(player_id, cid)?;
             state.draw_age(player_id, 1)?; 
             return Ok(());
         }
         
         state.pending_input = Some(InputRequest::SelectCard {
            player_id, source: candidates, min: 1, max: 1
         });
         update_sub_step(state, 1);
         Ok(())
    } else {
        if let Some(cid) = input {
            state.remove_from_hand(player_id, &cid)?;
            state.meld(player_id, cid)?;
        }
        state.draw_age(player_id, 1)?;
        Ok(())
    }
}

fn maconnerie(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> {
    // Meld any number of Castle cards. If 4+ melded, dominate "Technologies"? (Achievement?)
    // This requires a "multiselect" or loop.
    // Simplified: Meld loop.
    // "Dominate Technologies" is a special achievement. Not standard Age achievement.
    // Feature 002 scope: Just implement ability to meld.
    // Implementation: Request cards (min 0, max unlimited) filtered by Castle.
    // Then meld all.
    // Check count.
    
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    if sub_step == 0 {
         let p = &state.players[player_id];
         let db = crate::db::load_all_cards();
         let candidates: Vec<String> = p.hand.iter()
             .filter(|id| {
                 let c = db.get(*id).unwrap();
                 c.dogmas.iter().any(|d| matches!(d.symbol, crate::model::Symbol::Castle)) 
                 // Wait, "Produces Castle"? Or "Has Castle Symbol"? 
                 // Card text says "produisent du [Castle]". 
                 // Usually means Icons array contains Castle.
                 || c.icons.contains(&crate::model::Icon::Resource(crate::model::Symbol::Castle))
             })
             .cloned().collect();
             
         if candidates.is_empty() { return Ok(()); }
         
         state.pending_input = Some(InputRequest::SelectCard {
            player_id, source: candidates, min: 0, max: 99
         });
         update_sub_step(state, 1);
         Ok(())
    } else {
        // How to handle multiple inputs? InputRequest result should be List.
        // Current ResolveInput takes String.
        // We need a helper to split string or assume one card for now?
        // Or change Action::ResolveInput to Vec<String>.
        // I'll assume standard comma separated? Or just ONE card for now (restriction).
        // Let's assume input is JSON list or comma separated.
        if let Some(s) = input {
             let cards: Vec<&str> = s.split(',').filter(|x| !x.is_empty()).collect();
             let count = cards.len();
             for c in cards {
                 state.remove_from_hand(player_id, &c.to_string())?;
                 state.meld(player_id, c.to_string())?;
             }
             if count >= 4 {
                 // Claim Technologies achievement (placeholder)
                 // state.claim_special_achievement(player_id, "Technologies");
             }
        }
        Ok(())
    }
}

fn metallurgie(state: &mut GameState, player_id: usize, _idx: usize, _in: Option<String>) -> Result<(), GameError> {
     // Draw cards until one without Castle is found
     while let Some(cid) = state.draw_age(player_id, 1)? {
         let db = crate::db::load_all_cards();
         let card = db.get(&cid).unwrap();
         let has_castle = card.icons.contains(&crate::model::Icon::Resource(crate::model::Symbol::Castle));
         if has_castle {
             // "Comptabilisez-la" = Score
             // Since draw puts in hand, remove from hand first.
             state.remove_from_hand(player_id, &cid)?;
             state.score_card(player_id, cid)?;
         } else {
             // Keep in hand (already there from draw)
             break;
         }
     }
     Ok(())
}

fn mysticisme(state: &mut GameState, player_id: usize, _idx: usize, _in: Option<String>) -> Result<(), GameError> {
    // Draw 1 reveal. If same color as board, meld and draw 1. Else Hand.
    if let Some(cid) = state.draw_age(player_id, 1)? {
        let db = crate::db::load_all_cards();
        let color = db.get(&cid).unwrap().color;
        let has_color = state.players[player_id].board.contains_key(&color);
        
        if has_color {
            state.meld(player_id, cid)?;
            state.draw_age(player_id, 1)?;
        }
        // Else generic draw kept it in hand.
    }
    Ok(())
}

fn rames(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> {
    // Demand: Transfer Crown from Hand to My Score. If do, draw 1. If not, draw 1.
    // Victim Perspective.
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    let activator_id = state.dogma_state.as_ref().unwrap().activator_id;
    
     if sub_step == 0 {
          let p = &state.players[player_id];
          let db = crate::db::load_all_cards();
          let candidates: Vec<String> = p.hand.iter()
             .filter(|id| db.get(*id).unwrap().icons.contains(&crate::model::Icon::Resource(crate::model::Symbol::Crown)))
             .cloned().collect();
             
          if candidates.is_empty() {
             state.draw_age(player_id, 1)?;
             return Ok(()); 
         }
         
         state.pending_input = Some(InputRequest::SelectCard {
             player_id, source: candidates, min: 1, max: 1
         });
         update_sub_step(state, 1);
         Ok(())
    } else {
        if let Some(cid) = input.filter(|s| !s.is_empty()) {
            state.remove_from_hand(player_id, &cid)?;
            state.score_card(activator_id, cid)?;
            state.draw_age(player_id, 1)?;
        } else {
             state.draw_age(player_id, 1)?;
        }
        Ok(())
    }
}

fn poterie(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> {
    // Return up to 3 cards. Draw value = count. Score it. Draw 1.
     let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
     if sub_step == 0 {
          let p = &state.players[player_id];
          state.pending_input = Some(InputRequest::SelectCard {
              player_id, source: p.hand.clone(), min: 0, max: 3
          });
          update_sub_step(state, 1);
          Ok(())
     } else {
         if let Some(s) = input {
             let cards: Vec<&str> = s.split(',').filter(|x| !x.is_empty()).collect();
             let count = cards.len();
             for c in cards {
                 state.remove_from_hand(player_id, &c.to_string())?;
                 state.return_card(c.to_string())?;
             }
             if count > 0 {
                 if let Some(cid) = state.draw_age(player_id, count as u8)? {
                     state.remove_from_hand(player_id, &cid)?;
                     state.score_card(player_id, cid)?;
                 }
             }
         }
         state.draw_age(player_id, 1)?;
         Ok(())
     }
}

fn outils(state: &mut GameState, player_id: usize, dogma_index: usize, input: Option<String>) -> Result<(), GameError> { 
     // 1. Return 3 cards Hand -> Draw 3 Meld.
     // 2. Return a 3 Hand -> Draw three 1s.
     if dogma_index == 0 {
         let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
         if sub_step == 0 {
             let p = &state.players[player_id];
             if p.hand.len() < 3 { return Ok(()); } // "Recycler TROIS cartes". Exact count? Usually if not explicit "up to", means exact.
             state.pending_input = Some(InputRequest::SelectCard {
                 player_id, source: p.hand.clone(), min: 3, max: 3
             });
             update_sub_step(state, 1);
             Ok(())
         } else {
             if let Some(s) = input {
                 let cards: Vec<&str> = s.split(',').filter(|x| !x.is_empty()).collect();
                 if cards.len() == 3 {
                     for c in cards {
                         state.remove_from_hand(player_id, &c.to_string())?;
                         state.return_card(c.to_string())?;
                     }
                     if let Some(cid) = state.draw_age(player_id, 3)? {
                         state.meld(player_id, cid)?;
                     }
                 }
             }
             Ok(())
         }
     } else {
         // Dogma 2
         // Return a 3.
         let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
         if sub_step == 0 {
             let p = &state.players[player_id];
             let db = crate::db::load_all_cards();
             let candidates: Vec<String> = p.hand.iter().filter(|id| db.get(*id).unwrap().age == 3).cloned().collect();
             if candidates.is_empty() { return Ok(()); }
             
             state.pending_input = Some(InputRequest::SelectCard {
                 player_id, source: candidates, min: 0, max: 1
             });
             update_sub_step(state, 1);
             Ok(())
         } else {
             if let Some(cid) = input.filter(|s| !s.is_empty()) {
                  state.remove_from_hand(player_id, &cid)?;
                  state.return_card(cid)?;
                  for _ in 0..3 { state.draw_age(player_id, 1)?; }
             }
             Ok(())
         }
     }
}

fn cites_etats(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> {
    // Demand: If 4+ Castles, Transfer Active Castle to My Board. If do, draw 1.
    // Calc Castles
         let p = &state.players[player_id];
     let db = crate::db::load_all_cards();
     let castle_count = p.icon_counts(db).get(&crate::model::Symbol::Castle).cloned().unwrap_or(0);
     
     if castle_count < 4 { return Ok(()); }
     
     let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
     let activator_id = state.dogma_state.as_ref().unwrap().activator_id;
 
     if sub_step == 0 {
         // Find Active Castles (Top cards with Castle icon)
         let candidates: Vec<String> = p.board.values()
             .filter_map(|s| s.top().cloned())
             .filter(|id| db.get(id).unwrap().icons.contains(&crate::model::Icon::Resource(crate::model::Symbol::Castle)))
             .collect();
            
        if candidates.is_empty() { return Ok(()); }
        
        state.pending_input = Some(InputRequest::SelectCard {
            player_id, source: candidates, min: 1, max: 1
        });
        update_sub_step(state, 1);
        Ok(())
    } else {
         if let Some(cid) = input.filter(|s| !s.is_empty()) {
             // Transfer board -> board
             // Need remove_from_board_top primitive or similar?
             // Since we have ID and it's top, we can find stack by color.
             let color = db.get(&cid).unwrap().color;
             if let Some(stack) = state.players[player_id].board.get_mut(&color) {
                 stack.cards.pop(); // Remove
             }
             state.meld(activator_id, cid)?;
             state.draw_age(player_id, 1)?;
         }
         Ok(())
    }
}

fn update_sub_step(state: &mut GameState, step: usize) {
    if let Some(ds) = state.dogma_state.as_mut() { ds.sub_step.child_step = step; }
}

fn voiles(state: &mut GameState, _pid: usize, _idx: usize, _in: Option<String>) -> Result<(), GameError> { 
     // Draw 1 and meld it.
     if let Some(cid) = state.draw_age(_pid, 1)? {
         state.meld(_pid, cid)?; // meld checks hand, but draw puts in hand. OK.
     }
     Ok(()) 
}
fn la_roue(state: &mut GameState, _pid: usize, _idx: usize, _in: Option<String>) -> Result<(), GameError> { 
    state.draw_age(_pid, 1)?;
    state.draw_age(_pid, 1)?;
    Ok(()) 
}
fn ecriture(state: &mut GameState, _pid: usize, _idx: usize, _in: Option<String>) -> Result<(), GameError> { 
    state.draw_age(_pid, 2)?;
    Ok(()) 
}
