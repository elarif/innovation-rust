use leptos::prelude::*;
use innovation_core::{GameState, actions::Action, load_all_cards, game_state::GamePhase};

#[derive(Clone)]
pub struct GameSignal {
    pub game: RwSignal<GameState>,
    pub selected_hand_idx: RwSignal<Option<usize>>,
    pub selected_pile_idx: RwSignal<Option<usize>>,
    pub action_log: RwSignal<Vec<String>>,
    pub show_full_log: RwSignal<bool>,
    pub seed: RwSignal<u64>,
}

impl GameSignal {
    pub fn new(seed: u64, players: usize, max_age: u8) -> Self {
        let game = GameState::new_with_max_age(seed, players, max_age);
        let initial_log = game.action_log.clone();
        
        Self {
            game: RwSignal::new(game),
            selected_hand_idx: RwSignal::new(None),
            selected_pile_idx: RwSignal::new(None),
            action_log: RwSignal::new(initial_log),
            show_full_log: RwSignal::new(false),
            seed: RwSignal::new(seed),
        }
    }
    
    pub fn new_game(&self) {
        // Generate new random seed
        let new_seed = web_sys::js_sys::Date::now() as u64;
        self.seed.set(new_seed);
        
        let game = GameState::new_with_max_age(new_seed, 2, 1);
        let initial_log = game.action_log.clone();
        
        self.game.set(game);
        self.action_log.set(initial_log);
        self.selected_hand_idx.set(None);
        self.selected_pile_idx.set(None);
        self.show_full_log.set(false);
    }
    
    pub fn abandon_game(&self) {
        self.action_log.update(|l| l.push("=== Partie abandonnée ===".to_string()));
        self.new_game();
    }
    
    pub fn toggle_full_log(&self) {
        self.show_full_log.update(|v| *v = !*v);
    }
    
    pub fn apply_action(&self, action: Action) {
        self.game.update(|g| {
            match g.apply_action(action) {
                Ok(_) => {
                    // Sync logs
                    let logs: Vec<String> = g.action_log.drain(..).collect();
                    self.action_log.update(|l| l.extend(logs));
                }
                Err(e) => {
                    self.action_log.update(|l| l.push(format!("Erreur: {:?}", e)));
                }
            }
        });
    }
    
    pub fn draw(&self) {
        self.apply_action(Action::Draw);
    }
    
    pub fn meld(&self, card_name: String) {
        self.apply_action(Action::Meld(card_name));
        self.selected_hand_idx.set(None);
    }
    
    pub fn dogma(&self, card_name: String) {
        self.apply_action(Action::Dogma(card_name));
        self.selected_pile_idx.set(None);
    }
    
    pub fn is_setup_phase(&self) -> bool {
        self.game.with(|g| g.phase == GamePhase::Setup)
    }
    
    pub fn current_player(&self) -> usize {
        self.game.with(|g| g.current_player)
    }
    
    pub fn turn_number(&self) -> u32 {
        self.game.with(|g| g.turn_number)
    }
}
