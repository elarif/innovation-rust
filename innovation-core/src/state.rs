use crate::model::{Card, Color};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SplayDirection {
    Left,
    Right,
    Up,
    #[serde(rename = "None")]
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pile {
    pub color: Color,
    pub cards: Vec<Card>,
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Board {
    pub red: Pile,
    pub blue: Pile,
    pub green: Pile,
    pub yellow: Pile,
    pub purple: Pile,
}

impl Board {
    pub fn new() -> Self {
        Self {
            red: Pile::new(Color::Red),
            blue: Pile::new(Color::Blue),
            green: Pile::new(Color::Green),
            yellow: Pile::new(Color::Yellow),
            purple: Pile::new(Color::Purple),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub score_pile: Vec<Card>,
    pub achievements: Vec<Card>,
    pub board: Board,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hand: Vec::new(),
            score_pile: Vec::new(),
            achievements: Vec::new(),
            board: Board::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<Player>,
    /// Supply piles for ages 1 to 10. Index 0 is unused (or could be empty).
    pub supply_piles: Vec<Vec<Card>>,
    pub achievements_available: Vec<Card>,
    pub current_player_index: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerView {
    pub you: Player,
    pub opponents: Vec<OpponentView>,
    pub supply_counts: Vec<usize>,
    pub achievements_available: Vec<Card>,
    pub current_player_index: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpponentView {
    pub name: String,
    pub hand_count: usize,
    pub score_pile_count: usize, // Keeping simple count as per prompt request? Prompt says "Opponent hands needs count", didn't specify score pile. Score pile is usually public in Innovation.
    // "Opponent hands should be represented by a count (usize), not the cards themselves. Draw piles should be represented by a count."
    // Innovation rules: Score pile is public. Achievements are public. Board is public.
    pub score_pile: Vec<Card>,
    pub achievements: Vec<Card>,
    pub board: Board,
}

impl GameState {
    pub fn new(players: Vec<String>) -> Self {
        let mut player_structs = Vec::new();
        for name in players {
            player_structs.push(Player::new(name));
        }
        Self {
            players: player_structs,
            supply_piles: vec![Vec::new(); 11], // 1-10, 0 unused
            achievements_available: Vec::new(),
            current_player_index: 0,
        }
    }

    pub fn view_for_player(&self, player_index: usize) -> PlayerView {
        let you = self.players[player_index].clone();
        let mut opponents = Vec::new();
        for (i, p) in self.players.iter().enumerate() {
            if i != player_index {
                opponents.push(OpponentView {
                    name: p.name.clone(),
                    hand_count: p.hand.len(),
                    score_pile_count: p.score_pile.len(),
                    score_pile: p.score_pile.clone(), // Score pile is public info
                    achievements: p.achievements.clone(),
                    board: p.board.clone(),
                });
            }
        }
        let supply_counts = self.supply_piles.iter().map(|pile| pile.len()).collect();

        PlayerView {
            you,
            opponents,
            supply_counts,
            achievements_available: self.achievements_available.clone(),
            current_player_index: self.current_player_index,
        }
    }
}
