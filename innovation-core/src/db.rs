use crate::model::Card;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize)]
struct CardDbSchema {
    cards: Vec<Card>,
}

static RAW_JSON: &str = include_str!("../../data/cards.json");

pub static ALL_CARDS: Lazy<Vec<Card>> = Lazy::new(|| {
    load_all_cards()
});

pub fn load_all_cards() -> Vec<Card> {
    let schema: CardDbSchema = serde_json::from_str(RAW_JSON).expect("Failed to parse embedded JSON");
    schema.cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_cards() {
        let cards = load_all_cards();
        assert_eq!(cards.len(), 105);
        let first = &cards[0];
        assert_eq!(first.name, "Outils");
    }
}
