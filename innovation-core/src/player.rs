use crate::model::{Card, Color, Icon, Symbol, SplayDirection};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pile {
    pub color: Color,
    pub cards: Vec<String>, // Card IDs
    pub splay: SplayDirection,
}

impl Pile {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            cards: Vec::new(),
            splay: SplayDirection::None,
        }
    }

    pub fn top(&self) -> Option<&String> {
        self.cards.last()
    }
    
    pub fn pop(&mut self) -> Option<String> {
        self.cards.pop()
    }
    
    pub fn push(&mut self, card_id: String) {
        self.cards.push(card_id);
    }
    
    pub fn visible_icons(&self, db: &HashMap<String, Card>) -> Vec<Icon> {
        let mut icons = Vec::new();
        if self.cards.is_empty() {
            return icons;
        }
        
        let len = self.cards.len();
        // Top card always visible (all 4 icons)
        if let Some(top_card) = self.cards.last().and_then(|id| db.get(id)) {
            icons.extend(top_card.icons.iter().cloned());
        }
        
        // Cards below top
        let indices = match self.splay {
            SplayDirection::None => &[][..],
            SplayDirection::Left => &[2, 3][..],  // Right side (BR, TR) visible
            SplayDirection::Right => &[0, 1][..], // Left side (TL, BL) visible
            SplayDirection::Up => &[1, 2][..],    // Bottom side (BL, BR) visible
        };
        
        if !indices.is_empty() && len > 1 {
            for card_id in self.cards.iter().take(len - 1) {
                if let Some(card) = db.get(card_id) {
                    for &idx in indices {
                        if idx < card.icons.len() {
                            icons.push(card.icons[idx]);
                        }
                    }
                }
            }
        }
        
        icons
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub hand: Vec<String>,
    pub score_pile: Vec<String>,
    pub board: HashMap<Color, Pile>,
    pub achievements: Vec<String>,
}

impl Player {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            hand: Vec::new(),
            score_pile: Vec::new(),
            board: HashMap::new(),
            achievements: Vec::new(),
        }
    }
    
    pub fn calculate_score(&self, cards_db: &HashMap<String, Card>) -> u32 {
        self.score_pile.iter()
            .filter_map(|id| cards_db.get(id))
            .map(|c| c.age as u32)
            .sum()
    }
    
    pub fn icon_counts(&self, db: &HashMap<String, Card>) -> HashMap<Symbol, u32> {
        let mut counts = HashMap::new();
        for pile in self.board.values() {
            for icon in pile.visible_icons(db) {
                if let Icon::Resource(s) = icon {
                    *counts.entry(s).or_insert(0) += 1;
                }
            }
        }
        counts
    }
}
