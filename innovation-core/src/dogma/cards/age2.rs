#![allow(dead_code)] // Legacy implementations kept for reference/fallback
use crate::game_state::GameState;
use crate::errors::GameError;
use crate::dogma::flow::InputRequest;
use crate::model::Color;
use std::collections::HashSet;

pub fn execute(state: &mut GameState, player_id: usize, card_name: &str, dogma_index: usize, input: Option<String>) -> Result<(), GameError> {
    match card_name {
        "Calendrier" => calendrier(state, player_id, dogma_index, input),
        "Philosophie" => philosophie(state, player_id, dogma_index, input),
        "Construction de canaux" => construction_de_canaux(state, player_id, dogma_index, input),
        "Monothéisme" => monotheisme(state, player_id, dogma_index, input),
        "Monnaie" => monnaie(state, player_id, dogma_index, input),
        "Construction" => construction(state, player_id, dogma_index, input),
        "Mathématiques" => mathematiques(state, player_id, dogma_index, input),
        "Cartographie" => cartographie(state, player_id, dogma_index, input),
        "Réseau Routier" => reseau_routier(state, player_id, dogma_index, input),
        "Fermentation" => fermentation(state, player_id, dogma_index, input),
        _ => {
            println!("Warning: Unimplemented card {}", card_name);
            Ok(())
        }
    }
}

// --- Card Implementations ---

fn calendrier(state: &mut GameState, player_id: usize, _idx: usize, _in: Option<String>) -> Result<(), GameError> {
    // "Si vous avez plus de cartes dans votre Influence que dans votre Main, piochez deux 3."
    let p = &state.players[player_id];
    let score_count = p.score_pile.len();
    let hand_count = p.hand.len();
    
    if score_count > hand_count {
        state.draw_age(player_id, 3)?;
        state.draw_age(player_id, 3)?;
    }
    Ok(())
}

fn philosophie(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> {
    // "Vous pouvez décaler une de vos couleurs à gauche. Vous pouvez comptabiliser une carte de votre Main."
    // 1. Splay Left (Optional)
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    
    if sub_step == 0 {
         let p = &state.players[player_id];
         let candidates: Vec<Color> = p.board.iter()
            .filter(|(_, stack)| stack.cards.len() > 1)
            .map(|(c, _)| *c)
            .collect();
            
         if candidates.is_empty() {
             update_sub_step(state, 2); 
             return Ok(());
         }
         
         state.pending_input = Some(InputRequest::SelectColor {
             player_id, colors: candidates
         });
         update_sub_step(state, 1);
         Ok(())
    } else if sub_step == 1 {
        if let Some(col_str) = input.filter(|s| !s.is_empty()) {
            let color = match col_str.to_uppercase().as_str() {
                "RED" => Some(Color::Red),
                "BLUE" => Some(Color::Blue),
                "GREEN" => Some(Color::Green),
                "YELLOW" => Some(Color::Yellow),
                "PURPLE" => Some(Color::Purple),
                _ => None
            };
            
            if let Some(c) = color {
                 state.splay(player_id, c, crate::model::SplayDirection::Left)?;
            }
        }
        
        // Next: Score 1 card from hand (Optional)
        // Check if hand empty
        if state.players[player_id].hand.is_empty() { return Ok(()); }
        
        state.pending_input = Some(InputRequest::SelectCard {
             player_id, source: state.players[player_id].hand.clone(), min: 0, max: 1
        });
        update_sub_step(state, 2);
        Ok(())
    } else {
        if let Some(cid) = input.filter(|s| !s.is_empty()) {
            state.remove_from_hand(player_id, &cid)?;
            state.score_card(player_id, cid)?;
        }
        Ok(())
    }
}

fn construction_de_canaux(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> {
   // "Vous pouvez échanger toutes les cartes ayant la valeur la plus élevée de votre Main avec toutes les cartes ayant la valeur la plus élevée de votre Influence."
   // Optional.
   
   if let Some(s) = input {
       if s.to_lowercase() == "skip" { return Ok(()); }
   }
   
   let p = &mut state.players[player_id];
   if p.hand.is_empty() && p.score_pile.is_empty() { return Ok(()); }
   
   let db = crate::db::load_all_cards();
   let max_hand = p.hand.iter().filter_map(|id| db.get(id)).map(|c| c.age).max();
   let max_score = p.score_pile.iter().filter_map(|id| db.get(id)).map(|c| c.age).max();
   
   let mut hand_to_move = Vec::new();
   if let Some(mh) = max_hand {
       let mut indexes = Vec::new();
       for (i, cid) in p.hand.iter().enumerate() {
           if db.get(cid).unwrap().age == mh { indexes.push(i); }
       }
       for i in indexes.iter().rev() {
           hand_to_move.push(p.hand.remove(*i));
       }
   }
   
   let mut score_to_move = Vec::new();
    if let Some(ms) = max_score {
       let mut indexes = Vec::new();
       for (i, cid) in p.score_pile.iter().enumerate() {
           if db.get(cid).unwrap().age == ms { indexes.push(i); }
       }
       for i in indexes.iter().rev() {
           score_to_move.push(p.score_pile.remove(*i));
       }
   }
   
   p.hand.extend(score_to_move);
   p.score_pile.extend(hand_to_move);
   Ok(())
}


fn monotheisme(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> {
    // Demand: Transfer Active color that Activator does NOT have.
    // If victim suffers (transfers), victim draws 1 archive, draws 1 archive.
    
    let activator_id = state.dogma_state.as_ref().unwrap().activator_id;
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    
    if sub_step == 0 {
        // Identify colors activator does NOT have
        // Need to borrow activator board separately or carefully.
        // Rust borrow checker issues if we borrow state for both players mutable.
        // We only need to READ activator colors.
        // Clone keys?
        let activator_colors: HashSet<Color> = state.players[activator_id].board.keys().cloned().collect();
        let _db = crate::db::load_all_cards(); 

        // Victim Active cards where color NOT in activator_colors
        let p = &state.players[player_id];
        let candidates: Vec<String> = p.board.iter()
            .filter(|(c, _)| !activator_colors.contains(c))
            .filter_map(|(_, stack)| stack.top().cloned())
            .collect();
            
        if candidates.is_empty() { return Ok(()); }
        
        state.pending_input = Some(InputRequest::SelectCard {
            player_id, source: candidates, min: 1, max: 1
        });
        update_sub_step(state, 1);
        Ok(())
    } else {
        if let Some(cid) = input.filter(|s| !s.is_empty()) {
            // Transfer to Activator Influence
            // "vers mon Influence" -> Score Pile?
            // "Influence" in French translation of Innovation usually means Score Pile.
            // Wait, "Zone de Jeu" = Board. "Influence" = Score Pile. "Main" = Hand.
            // Check text: "vers mon Influence !" -> Score Pile.
            
            // Remove from board
            let db = crate::db::load_all_cards();
            let color = db.get(&cid).unwrap().color;
            if let Some(stack) = state.players[player_id].board.get_mut(&color) {
                stack.cards.pop();
            }
            
            // Add to Activator Score Pile
            state.score_card(activator_id, cid)?;
            
            // Victim bonus: Draw 1 archive x2 (Tuck)
            if let Some(c1) = state.draw_age(player_id, 1)? {
                state.remove_from_hand(player_id, &c1)?;
                state.tuck_card(player_id, c1)?;
            }
            if let Some(c2) = state.draw_age(player_id, 1)? {
                state.remove_from_hand(player_id, &c2)?;
                state.tuck_card(player_id, c2)?;
            }
        }
        Ok(())
    }
}

fn construction(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> {
    // Demand: Transfer 2 cards from Hand to Mine (Activator Hand).
    // Activator Draw 2.
    // If Activator 5 colors, Dominate Military.
    let activator_id = state.dogma_state.as_ref().unwrap().activator_id;
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    
    if sub_step == 0 {
         let p = &state.players[player_id];
         // Need 2 cards. If < 2, transfer all?
         // Rules: "Transfer two cards". If valid, must do.
         let count = p.hand.len();
         let min_req = if count < 2 { count } else { 2 };
         
         if min_req == 0 { 
             // Just Draw for activator? 
             // "If you are the only player with 5 colors...". This part is conditional on Activator state, not transfer.
             // But draw is mandatory.
         } else {
             state.pending_input = Some(InputRequest::SelectCard {
                 player_id, source: p.hand.clone(), min: min_req, max: min_req
             });
             update_sub_step(state, 1);
             return Ok(());
         }
         update_sub_step(state, 1); // Skip to draw
         Ok(())
    } else if sub_step == 1 {
        if let Some(s) = input {
            let cards: Vec<&str> = s.split(',').filter(|x| !x.is_empty()).collect();
            for c in cards {
                state.remove_from_hand(player_id, &c.to_string())?;
                state.players[activator_id].hand.push(c.to_string());
            }
        }
        
        // Activator Draw 2
        state.draw_age(activator_id, 2)?;
        
        // Check Military Domination
        // "Si vous êtes le SEUL joueur avec cinq couleurs en jeu"
        let activator_colors = state.players[activator_id].board.len();
        if activator_colors == 5 {
            let mut others_have_5 = false;
            for p in &state.players {
                if p.id != activator_id && p.board.len() == 5 {
                    others_have_5 = true;
                    break;
                }
            }
            if !others_have_5 {
                state.claim_special_achievement(activator_id, "Domaine Militaire")?;
            }
        }
        Ok(())
    } else { Ok(()) }
}

fn monnaie(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> {
    // Recycle any number. Draw 2 for each different value recycled. Score them.
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    if sub_step == 0 {
         let p = &state.players[player_id];
         state.pending_input = Some(InputRequest::SelectCard {
             player_id, source: p.hand.clone(), min: 0, max: 99
         });
         update_sub_step(state, 1);
         Ok(())
    } else {
        if let Some(s) = input {
            let cards: Vec<&str> = s.split(',').filter(|x| !x.is_empty()).collect();
            let mut values_seen = std::collections::HashSet::new();
            
            let db = crate::db::load_all_cards();
            for c in &cards {
                if let Some(card_data) = db.get(*c) {
                    values_seen.insert(card_data.age);
                }
                state.remove_from_hand(player_id, &c.to_string())?;
                state.return_card(c.to_string())?;
            }
            
            let count = values_seen.len();
            for _ in 0..count {
                if let Some(cid) = state.draw_age(player_id, 2)? {
                    state.remove_from_hand(player_id, &cid)?;
                    state.score_card(player_id, cid)?;
                }
            }
        }
        Ok(())
    }
}

fn mathematiques(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> {
    // Return 1 hand. If done, Draw (Age+1) Meld.
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    if sub_step == 0 {
        let p = &state.players[player_id];
        if p.hand.is_empty() { return Ok(()); } 
         
        state.pending_input = Some(InputRequest::SelectCard {
             player_id, source: p.hand.clone(), min: 0, max: 1
        });
        update_sub_step(state, 1);
        Ok(())
    } else {
        if let Some(cid) = input.filter(|s| !s.is_empty()) {
            // Get age before returning
            let db = crate::db::load_all_cards();
            let age = db.get(&cid).unwrap().age;
            
            state.remove_from_hand(player_id, &cid)?;
            state.return_card(cid)?;
            
            if let Some(new_cid) = state.draw_age(player_id, age + 1)? {
                state.meld(player_id, new_cid)?;
            }
        }
        Ok(())
    }
}


fn cartographie(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> {
    // Demand: Transfer a 1 from Influence to Mine (Score).
    // If transferred, Draw 1 Score.
     let activator_id = state.dogma_state.as_ref().unwrap().activator_id;
     let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
     
     if sub_step == 0 {
         let p = &state.players[player_id];
         let db = crate::db::load_all_cards();
         let candidates: Vec<String> = p.score_pile.iter()
             .filter(|id| db.get(*id).unwrap().age == 1)
             .cloned().collect();
             
         if candidates.is_empty() { return Ok(()); }
         
         state.pending_input = Some(InputRequest::SelectCard {
             player_id, source: candidates, min: 1, max: 1
         });
         update_sub_step(state, 1);
         Ok(())
     } else {
         if let Some(cid) = input.filter(|s| !s.is_empty()) {
             // Transfer (Score -> Activator Score)
             let p = &mut state.players[player_id];
             if let Some(idx) = p.score_pile.iter().position(|x| x == &cid) {
                 p.score_pile.remove(idx);
             }
             state.score_card(activator_id, cid)?;
             
             // Draw 1 Score
             if let Some(c) = state.draw_age(player_id, 1)? {
                 state.remove_from_hand(player_id, &c)?;
                 state.score_card(player_id, c)?;
             }
         }
         Ok(())
     }
}

fn reseau_routier(state: &mut GameState, player_id: usize, _idx: usize, input: Option<String>) -> Result<(), GameError> {
    // Meld 1 or 2 cards.
    // If 2, OPTIONAL transfer Red Active to other player.
    // If transfer, transfer their Green Active to yours.
    let sub_step = state.dogma_state.as_ref().map(|s| s.sub_step.child_step).unwrap_or(0);
    
    if sub_step == 0 {
        let p = &state.players[player_id];
        if p.hand.is_empty() { return Ok(()); }
        state.pending_input = Some(InputRequest::SelectCard {
             player_id, source: p.hand.clone(), min: 1, max: 2
        });
        update_sub_step(state, 1);
        Ok(())
    } else {
        if let Some(s) = input {
            let cards: Vec<&str> = s.split(',').filter(|x| !x.is_empty()).collect();
            let count = cards.len();
            for c in cards {
                state.remove_from_hand(player_id, &c.to_string())?;
                state.meld(player_id, c.to_string())?;
            }
            
            if count == 2 {
                // Optional logic skipped for now based on previous complexity, 
                // but if we were to clean mut sub_step warning we should remove it here too
                // or use it. We are using it in line 307.
                // Optional Transfer Red
                // Need to know if we have Red Active.
                if state.players[player_id].board.contains_key(&Color::Red) {
                    // Logic for transfer?
                    // Simplified: Auto-do or skip?
                    // "You MAY". So skip allowed.
                    // Implementation note: Ignoring optional complex part for now.
                    // Or auto-execute if beneficial? Usually swapping Red for Green might be good or bad.
                    // Need Interaction.
                    // Can use InputRequest::Confirm?
                    // Let's assume Confirm.
                    /*
                    state.pending_input = Some(InputRequest::Confirm {
                        player_id, message: "Swap Red for Green?".into()
                    });
                     update_sub_step(state, 2);
                    */
                }
            }
        }
        Ok(())
    }
}

fn fermentation(state: &mut GameState, player_id: usize, _idx: usize, _in: Option<String>) -> Result<(), GameError> {
    // Draw 2 for every two [Leaf].
    let p = &state.players[player_id];
    let db = crate::db::load_all_cards();
    let leaf_count = p.icon_counts(db).get(&crate::model::Symbol::Leaf).cloned().unwrap_or(0);
    
    let draws = leaf_count / 2;
    for _ in 0..draws {
        state.draw_age(player_id, 2)?;
    }
    Ok(())
}

fn update_sub_step(state: &mut GameState, step: usize) {
    if let Some(ds) = state.dogma_state.as_mut() { ds.sub_step.child_step = step; }
}
