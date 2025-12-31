use crate::model::Card;
use std::collections::HashMap;
use once_cell::sync::Lazy;

// We assume/require the data directory is available or we embed.
// For simplicity in this workspace, let's embed.
// Note: path is relative to the file.
// data/cards.json is at workspace root. 
// innovation-core/src/db.rs -> ../../data/cards.json
const CARDS_JSON: &str = include_str!("../../data/cards.json");

#[derive(serde::Deserialize)]
struct CardsFile {
    cards: Vec<Card>,
}

pub static CARDS: Lazy<HashMap<String, Card>> = Lazy::new(|| {
    let data: CardsFile = serde_json::from_str(CARDS_JSON).expect("Failed to parse cards.json");
    data.cards.into_iter().map(|c| (c.name.clone(), c)).collect()
});

pub fn load_all_cards() -> &'static HashMap<String, Card> {
    &CARDS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Icon, Symbol};

    #[test]
    fn test_card_loading() {
        let cards = load_all_cards();
        assert!(!cards.is_empty(), "Cards should be loaded");
        
        // Check specific card: Élevage (Age 1)
        // json: icons: ["CASTLE", "CROWN", 1, "CASTLE"]
        let elevage = cards.get("Élevage").expect("Élevage should exist");
        assert_eq!(elevage.icons.len(), 4);
        
        // Verify Icon types
        // 0: Resource(Castle)
        match &elevage.icons[0] {
             Icon::Resource(s) => assert_eq!(*s, Symbol::Castle),
             _ => panic!("Expected Castle resource at index 0"),
        }
        
        // 2: Age(1)
        match &elevage.icons[2] {
             Icon::Age(n) => assert_eq!(*n, 1),
             _ => panic!("Expected Age(1) at index 2"),
        }
    }
}
