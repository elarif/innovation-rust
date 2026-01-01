// use crate::model::Card;
use crate::player::Player;
use crate::actions::Action;
use crate::db::load_all_cards;
use crate::errors::GameError;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use std::collections::{HashSet, HashMap};

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GamePhase {
    Setup,
    Main,
    GameOver,
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    #[serde(skip, default = "default_rng")]
    pub rng: StdRng,
    pub players: Vec<Player>,
    pub deck: HashMap<u8, Vec<String>>, // Age -> Card IDs
    pub achievements: HashSet<String>,
    pub current_player: usize,
    pub turn_number: u32,
    pub winners: Vec<usize>,
    
    pub phase: GamePhase,
    pub initial_melds: HashMap<usize, String>, // Track initial melds during Setup
    pub actions_taken: u8, // Track actions taken in current turn
    
    pub pending_input: Option<crate::dogma::flow::InputRequest>,
    pub dogma_state: Option<crate::dogma::flow::DogmaExecutionState>,
    pub action_log: Vec<String>, // Log of recent actions for display
}

impl GameState {
    pub fn new(seed: u64, player_count: usize) -> Self {
        Self::new_with_max_age(seed, player_count, 10) // Default: all ages
    }
    
    pub fn new_with_max_age(seed: u64, player_count: usize, max_age: u8) -> Self {
        use rand::SeedableRng;
        let mut rng = StdRng::seed_from_u64(seed);
        
        // Load Cards
        let cards_db = load_all_cards();
        let mut deck: HashMap<u8, Vec<String>> = HashMap::new();
        
        for card in cards_db.values() {
            // Filter by max_age
            if card.age <= max_age {
                deck.entry(card.age).or_default().push(card.name.clone());
            }
        }
        
        // Shuffle
        for pile in deck.values_mut() {
            pile.shuffle(&mut rng);
        }
        
        // Create Players
        let mut players = Vec::with_capacity(player_count);
        for i in 0..player_count {
            players.push(Player::new(i));
        }
        
        // Logging for initial draws
        let mut action_log = Vec::new();
        action_log.push("=== Début de la partie ===".to_string());
        
        // Deal Initial Hands (2 cards of Age 1)
        for (i, p) in players.iter_mut().enumerate() {
            if let Some(pile) = deck.get_mut(&1) {
                for _ in 0..2 {
                    if let Some(card_id) = pile.pop() {
                        action_log.push(format!("P{} pioche '{}' (Age 1)", i, card_id));
                        p.hand.push(card_id);
                    }
                }
            }
        }
        
        action_log.push("--- Phase de Setup: chaque joueur pose une carte ---".to_string());
        
        Self {
            rng,
            players,
            deck,
            achievements: HashSet::new(),
            current_player: 0, 
            turn_number: 1,
            winners: Vec::new(),
            pending_input: None,
            dogma_state: None,
            phase: if cfg!(test) { GamePhase::Main } else { GamePhase::Setup },
            initial_melds: HashMap::new(),
            actions_taken: 0,
            action_log,
        }
    }
    
    pub fn apply_action(&mut self, action: Action) -> Result<(), GameError> {
        // Handle Setup Phase
        if self.phase == GamePhase::Setup {
            match action {
                Action::Meld(card_id) => {
                    // 1. Verify card in hand
                    let p = &mut self.players[self.current_player];
                    if !p.hand.contains(&card_id) {
                        return Err(GameError::InvalidAction("Card not in hand".into()));
                    }
                    
                    // Log setup meld
                    self.action_log.push(format!("P{} choisit de poser '{}'", self.current_player, card_id));
                    
                    // 2. Meld logic (reuse generic meld or inline)
                    self.meld(self.current_player, card_id.clone())?;
                    
                    // 3. Track
                    self.initial_melds.insert(self.current_player, card_id);
                    
                    // 4. Advance
                    self.current_player += 1;
                    
                    // 5. Check if all players ready
                    if self.current_player >= self.players.len() {
                        // Determine First Player
                        // "First alphabetically"
                        let mut first_player = 0;
                        let mut first_card = self.initial_melds.get(&0).unwrap().clone();
                        
                        for (pid, cid) in &self.initial_melds {
                             if *cid < first_card {
                                 first_card = cid.clone();
                                 first_player = *pid;
                             } else if *cid == first_card && *pid < first_player {
                                 // Tie-breaker? Usually not possible with unique names.
                                 // Or maybe "closer to A"? A < B is true.
                                 // So min string is correct.
                             }
                        }
                        
                        // Log first player determination
                        self.action_log.push(format!("--- P{} commence (carte '{}' première alphabétiquement) ---", first_player, first_card));
                        self.action_log.push(format!("=== Tour {} - P{} ===", self.turn_number, first_player));
                        
                        self.current_player = first_player;
                        self.phase = GamePhase::Main;
                        self.actions_taken = 0;
                    }
                    
                    return Ok(());
                },
                _ => {
                    return Err(GameError::InvalidAction("Only Meld is allowed during Setup".into()));
                }
            }
        }
        
        // Main Phase
        match action {
            Action::Draw => {
                 self.draw(self.current_player)?;
                 self.finish_action()?;
                 Ok(())
            },
            Action::Meld(card_id) => {
                 self.action_log.push(format!("P{} pose '{}'", self.current_player, card_id));
                 self.meld(self.current_player, card_id)?;
                 self.finish_action()?;
                 Ok(())
            },
            Action::Achieve(target) => {
                 self.achieve(self.current_player, target)?;
                 self.finish_action()?;
                 Ok(())
            },
            Action::Dogma(card_id) => {
                crate::dogma::executor::execute_dogma(self, self.current_player, card_id)?;
                // Only count as action if dogma completed (no pending input)
                if self.pending_input.is_none() {
                    self.finish_action()?;
                }
                Ok(())
            },
            Action::ResolveInput(input) => {
                crate::dogma::executor::continue_execution(self, Some(input))?;
                // Count action when dogma fully resolves
                if self.pending_input.is_none() && self.dogma_state.is_none() {
                    self.finish_action()?;
                }
                Ok(())
            },
        }
    }
    
    fn finish_action(&mut self) -> Result<(), GameError> {
        self.actions_taken += 1;
        
        // Action limits per rules:
        // - 2 players: 1st player gets 1 action on first turn
        // - 4 players: 1st AND 2nd player get 1 action on first turn
        // - After turn 1, everyone gets 2 actions
        let actions_limit = if self.turn_number == 1 {
            let first_player_id = self.initial_melds.iter()
                .min_by_key(|(_, card)| card.clone())
                .map(|(pid, _)| *pid)
                .unwrap_or(0);
            
            let player_count = self.players.len();
            let current_pos = (self.current_player + player_count - first_player_id) % player_count;
            
            // 4 players: positions 0 and 1 get 1 action
            // 2-3 players: position 0 gets 1 action
            if player_count == 4 && current_pos <= 1 {
                1
            } else if current_pos == 0 {
                1
            } else {
                2
            }
        } else {
            2
        };
        
        if self.actions_taken >= actions_limit {
            self.actions_taken = 0;
            let prev_player = self.current_player;
            self.current_player = (self.current_player + 1) % self.players.len();
            
            let first_player = self.initial_melds.iter()
                .min_by_key(|(_, card)| card.clone())
                .map(|(pid, _)| *pid)
                .unwrap_or(0);
            
            if self.current_player == first_player { // Full round
                 self.turn_number += 1;
                 self.action_log.push(format!("=== Tour {} ===", self.turn_number));
            }
            self.action_log.push(format!("--- Au tour de P{} ---", self.current_player));
        }
        Ok(())
    }
    
    pub fn draw(&mut self, player_id: usize) -> Result<(), GameError> {
        // Simple draw logic: Max(TopCardAge, 1)
        let mut age_to_draw = 1; 
        if let Some(p) = self.players.get(player_id) {
            // Find max top card age
            for pile in p.board.values() {
                if let Some(top_id) = pile.top() {
                    if let Some(card) = load_all_cards().get(top_id) {
                        if card.age > age_to_draw {
                            age_to_draw = card.age;
                        }
                    }
                }
            }
        }
        
        // Try drawing from Age, then Age+1, ...
        for age in age_to_draw..=11 {
            if age > 10 {
                // Deck depletion -> Game End
                self.end_game_by_score();
                return Ok(());
            }
            
            if let Some(deck_pile) = self.deck.get_mut(&age) {
                if let Some(card_id) = deck_pile.pop() {
                    self.action_log.push(format!("P{} pioche '{}' (Age {})", player_id, card_id, age));
                    if let Some(p) = self.players.get_mut(player_id) {
                        p.hand.push(card_id);
                    }
                    return Ok(());
                }
            }
        }
        Ok(())
    }
    
    pub fn meld(&mut self, player_id: usize, card_id: String) -> Result<(), GameError> {
        let p = &mut self.players[player_id];
        if let Some(idx) = p.hand.iter().position(|c| c == &card_id) {
            let card_id = p.hand.remove(idx);
            let cards_db = load_all_cards();
            let card = cards_db.get(&card_id).ok_or_else(|| GameError::CardNotFound(card_id.clone()))?;
            
            p.board.entry(card.color).or_insert_with(|| crate::player::Pile::new(card.color))
                .cards.push(card_id);
            Ok(())
        } else {
            Err(GameError::InvalidAction("Card not in hand".into()))
        }
    }

    pub fn achieve(&mut self, player_id: usize, target: String) -> Result<(), GameError> {
        // Parse target. Ex: "Age 1", "Age 2"...
        if let Some(age_str) = target.strip_prefix("Age ") {
            if let Ok(age) = age_str.parse::<u8>() {
                // Check if already claimed
                let ach_id = format!("Age {}", age);
                if self.achievements.contains(&ach_id) {
                    return Err(GameError::InvalidAction("Achievement already claimed".into()));
                }
                // Check eligibility
                let p = &self.players[player_id];
                let score = p.calculate_score(load_all_cards());
                let required_score = (age as u32) * 5;
                if score < required_score {
                    return Err(GameError::InvalidAction("Insufficient score".into()));
                }
                
                // Check top card >= Age
                let max_top_age = p.board.values().filter_map(|s| s.top())
                    .filter_map(|id| load_all_cards().get(id))
                    .map(|c| c.age)
                    .max()
                    .unwrap_or(0);
                    
                if max_top_age < age {
                    return Err(GameError::InvalidAction("No top card of sufficient age".into()));
                }
                
                // Claim
                self.achievements.insert(ach_id.clone());
                self.players[player_id].achievements.push(ach_id);
                
                self.check_victory();
                return Ok(());
            }
        }
        Err(GameError::InvalidAction("Invalid achievement target".into()))
    }
    
    pub fn check_victory(&mut self) {
        let achievements_needed = match self.players.len() {
            2 => 6,
            3 => 5,
            4 => 4,
            _ => 6,
        };
        
        for p in &self.players {
            if p.achievements.len() >= achievements_needed {
                self.winners = vec![p.id];
                return;
            }
        }
    }
    
    pub fn end_game_by_score(&mut self) {
        let mut max_score = 0;
        let mut winners = Vec::new();
        let db = load_all_cards();
        
        for p in &self.players {
            let score = p.calculate_score(db); // Note: Should include Achievements count as tiebreaker?
            // Rules: Highest score wins. Tiebreaker: Most achievements.
            if score > max_score {
                max_score = score;
                winners = vec![p.id];
            } else if score == max_score {
                winners.push(p.id);
            }
        }
        // Apply tiebreaker (Achievements) if multiple winners
        if winners.len() > 1 {
            let mut max_ach = 0;
            let mut final_winners = Vec::new();
            for &pid in &winners {
                let count = self.players[pid].achievements.len();
                if count > max_ach {
                    max_ach = count;
                    final_winners = vec![pid];
                } else if count == max_ach {
                    final_winners.push(pid);
                }
            }
            self.winners = final_winners;
        } else {
            self.winners = winners;
        }
    }

    // --- Primitives for Dogma Effects ---

    pub fn draw_age(&mut self, player_id: usize, mut age: u8) -> Result<Option<String>, GameError> {
        if age < 1 { return Ok(None); }
        // Cap age at 10 (or 11? Rules say if draw 11 -> end game)
        // Innovation rules: If you must draw a card of value 11 or higher, the game ends.
        
        while age <= 10 {
            if let Some(pile) = self.deck.get_mut(&age) {
                if let Some(card_id) = pile.pop() {
                    if let Some(p) = self.players.get_mut(player_id) {
                        p.hand.push(card_id.clone());
                    }
                    return Ok(Some(card_id));
                }
            }
            age += 1;
        }
        
        // If age > 10 (Deck Depleted through drawing), trigger End Game
        self.end_game_by_score();
        Ok(None)
    }

    pub fn tuck_card(&mut self, player_id: usize, card_id: String) -> Result<(), GameError> {
        let card = load_all_cards().get(&card_id).ok_or_else(|| GameError::CardNotFound(card_id.clone()))?;
        let color = card.color;
        
        if let Some(p) = self.players.get_mut(player_id) {
            let pile = p.board.entry(color).or_insert_with(|| crate::player::Pile::new(color));
            pile.cards.insert(0, card_id); // Tuck = Insert at bottom (index 0)
        }
        Ok(())
    }

    pub fn score_card(&mut self, player_id: usize, card_id: String) -> Result<(), GameError> {
        if let Some(p) = self.players.get_mut(player_id) {
            p.score_pile.push(card_id);
        }
        Ok(())
    }

    pub fn return_card(&mut self, card_id: String) -> Result<(), GameError> {
        let card = load_all_cards().get(&card_id).ok_or_else(|| GameError::CardNotFound(card_id.clone()))?;
        let age = card.age;
        
        // Return to bottom of deck
        self.deck.entry(age).or_default().insert(0, card_id);
        Ok(())
    }
    
    // Helper to remove from hand (since many effects do "Return a card from Hand")
    pub fn remove_from_hand(&mut self, player_id: usize, card_id: &String) -> Result<String, GameError> {
        let p = self.players.get_mut(player_id).ok_or(GameError::InvalidAction("Invalid Player".into()))?;
        if let Some(idx) = p.hand.iter().position(|c| c == card_id) {
            Ok(p.hand.remove(idx))
        } else {
            Err(GameError::InvalidAction("Card not in hand".into()))
        }
    }
    pub fn splay(&mut self, player_id: usize, color: crate::model::Color, direction: crate::model::SplayDirection) -> Result<(), GameError> {
        if let Some(p) = self.players.get_mut(player_id) {
            if let Some(pile) = p.board.get_mut(&color) {
                if pile.cards.len() > 1 {
                    pile.splay = direction;
                }
            }
        }
        Ok(())
    }

    pub fn exchange_hand_score(&mut self, player_id: usize) -> Result<(), GameError> {
        if let Some(p) = self.players.get_mut(player_id) {
            std::mem::swap(&mut p.hand, &mut p.score_pile);
        }
        Ok(())
    }

    pub fn claim_special_achievement(&mut self, player_id: usize, achievement_name: &str) -> Result<(), GameError> {
        let ach_id = achievement_name.to_string();
        if self.achievements.contains(&ach_id) {
             return Ok(());
        }
        
        self.achievements.insert(ach_id.clone());
        if let Some(p) = self.players.get_mut(player_id) {
            p.achievements.push(ach_id);
        }
        self.check_victory();
        Ok(())
    }
}

fn default_rng() -> StdRng {
    use rand::SeedableRng;
    StdRng::seed_from_u64(0)
}
