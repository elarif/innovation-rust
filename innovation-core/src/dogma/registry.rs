use crate::model::{Effect, Location, Filter, Symbol, SplayDirection};
use std::collections::HashMap;
use std::sync::OnceLock;

// Global Registry
static REGISTRY: OnceLock<HashMap<String, Vec<Effect>>> = OnceLock::new();

pub fn get_effects(card_name: &str) -> Option<&'static Vec<Effect>> {
    REGISTRY.get_or_init(load_registry).get(card_name)
}

fn load_registry() -> HashMap<String, Vec<Effect>> {
    let mut map = HashMap::new();

    // --- Age 1 ---
    
    // Agriculture: Return 1 Hand -> Score Value+1
    // "Vous pouvez recycler une carte de votre Main. Si vous le faites, piochez une carte d'une valeur supérieure de un à celle que vous avez recyclée et comptabilisez-la."
    // Note: The "Draw & Score" part depends on the value of the Returned card.
    // Our primitive `Effect` enum needs to support this dependency or we need a designated "RecycleAndScore" effect?
    // Current `Effect` in model.rs:
    // Recycle { count: u8, source: Location }
    // Conditional? 
    // This is a complex effect. We might need a custom Effect type or enhance the Enum.
    // For now, mapping simplified version or TODO.
    
    // Let's implement what we can.
    
    // Archerie: Demand (Draw 1, Transfer Highest Hand -> My Hand).
    // Archerie: Demand (Draw 1, Transfer Highest Hand -> My Hand).
    map.insert("Archerie".to_string(), vec![
        Effect::Sequence(vec![
            Effect::Draw { amount: 1, age: Some(1) },
            Effect::Transfer {
                source: Location::Hand,
                dest: Location::Hand,
                dest_is_activator: true,
                filters: vec![Filter::MaxAge],
                min: 1,
                max: 1
            }
        ])
    ]);
    
    // For this first pass, I will define the Registry structure and maybe one simple card like "Ecriture" (Draw 2).
    
    map.insert("Écriture".to_string(), vec![
        Effect::Draw { amount: 2, age: Some(2) } 
    ]);

    map.insert("La Roue".to_string(), vec![
        Effect::Draw { amount: 2, age: Some(1) } // "Piochez deux 1."
    ]);


    map.insert("Voiles".to_string(), vec![
        Effect::DrawAndMeld { amount: 1, age: Some(1) }
    ]);

    map.insert("Agriculture".to_string(), vec![
        Effect::Sequence(vec![
            Effect::Recycle { min: 1, max: 1, source: Location::Hand, filters: vec![] },
            Effect::Score { min: 1, max: 1, filters: vec![] }
        ])
    ]);

    map.insert("Code de lois".to_string(), vec![
        Effect::Sequence(vec![
            Effect::Tuck { 
                min: 1, 
                max: 1,
                source: Location::Hand, 
                filters: vec![Filter::ColorsPresentOnBoard] 
            },
            Effect::Conditional {
                condition: crate::model::Condition::ContextValueGreaterThan(0),
                success: Box::new(Effect::Splay { 
                    direction: SplayDirection::Left, 
                    color: None 
                }),
                failure: None
            }
        ])
    ]);

    map.insert("Élevage".to_string(), vec![
        Effect::Sequence(vec![
            Effect::Meld { 
                source: Location::Hand, 
                filters: vec![Filter::MinAge],
                min: 1,
                max: 1
            },
            Effect::Draw { 
                amount: 1, 
                age: Some(1) 
            }
        ])
    ]);

    map.insert("Maçonnerie".to_string(), vec![
        Effect::Sequence(vec![
            Effect::Meld { 
                source: Location::Hand, 
                filters: vec![Filter::Icon(Symbol::Castle)],
                min: 0,
                max: 255
            },
            Effect::Conditional {
                condition: crate::model::Condition::ContextValueGreaterThan(0),
                success: Box::new(Effect::Splay { 
                    direction: SplayDirection::Left, 
                    color: None 
                }),
                failure: None
            }
        ])
    ]);

    map.insert("Poterie".to_string(), vec![
        Effect::Sequence(vec![
            Effect::Recycle { 
                min: 0, 
                max: 3, 
                source: Location::Hand,
                filters: vec![]
            },
            Effect::DrawAndScore { 
                amount: 1, 
                age: None // Depends on context value from Recycle
            }
        ])
    ]);

    // Outils has ONE dogma with TWO parts in sequence (no nested Sequences)
    map.insert("Outils".to_string(), vec![
        Effect::Sequence(vec![
            // Part 1: Recycle 0-3 cards. 
            Effect::Recycle { min: 0, max: 3, source: Location::Hand, filters: vec![] },
            // If recycled 3+, Draw 3 and Meld.
            Effect::Conditional {
                condition: crate::model::Condition::ContextValueGreaterThan(2),
                success: Box::new(Effect::DrawAndMeld { amount: 1, age: Some(3) }),
                failure: None
            },
            // Part 2: Recycle a 3.
            Effect::Recycle { 
                min: 0, max: 1, 
                source: Location::Hand,
                filters: vec![crate::model::Filter::Value(3)]
            },
            // If recycled 1+, Draw three 1s.
            Effect::Conditional {
                condition: crate::model::Condition::ContextValueGreaterThan(0),
                success: Box::new(Effect::Draw { amount: 3, age: Some(1) }),
                failure: None
            }
        ])
    ]);

    // Rames (Demand): Transfer a Crown card from Hand to Activator's Score. Draw 1.
    // Note: Demand effect is executed by victims. dest_is_activator=true means activator gets the card.
    map.insert("Rames".to_string(), vec![
        Effect::Sequence(vec![
            Effect::Transfer {
                source: Location::Hand,
                dest: Location::Score,
                dest_is_activator: true,
                filters: vec![Filter::Icon(Symbol::Crown)],
                min: 1,
                max: 1
            },
            Effect::Draw { amount: 1, age: Some(1) }
        ])
    ]);

    // Cités-États (Demand): If you have 4+ Castles, transfer a Castle from Board to Activator's Board.
    map.insert("Cités-États".to_string(), vec![
        Effect::Conditional {
            condition: crate::model::Condition::IconCountGreaterThanOrEqual(Symbol::Castle, 4),
            success: Box::new(Effect::Sequence(vec![
                Effect::Transfer {
                    source: Location::Board,
                    dest: Location::Board,
                    dest_is_activator: true,
                    filters: vec![Filter::Icon(Symbol::Castle)],
                    min: 1,
                    max: 1
                },
                Effect::Draw { amount: 1, age: Some(1) }
            ])),
            failure: None
        }
    ]);

    // Métallurgie: Draw and reveal 1s until you don't reveal a Castle. Score each Castle.
    map.insert("Métallurgie".to_string(), vec![
        Effect::DrawUntilNoMatch {
            age: 1,
            condition: crate::model::DrawnCardCondition::HasIcon(Symbol::Castle),
            on_match: Box::new(Effect::Score { min: 1, max: 1, filters: vec![] })
        }
    ]);

    // Mysticisme: Draw 1. If same color as any on board, meld and draw 1.
    map.insert("Mysticisme".to_string(), vec![
        Effect::DrawAndCheck {
            age: 1,
            condition: crate::model::DrawnCardCondition::ColorOnBoard,
            on_match: Box::new(Effect::Sequence(vec![
                // Meld is handled specially in executor for DrawAndCheck
                Effect::Draw { amount: 1, age: Some(1) }
            ])),
            on_fail: None
        }
    ]);

    // Tissage: Meld a card of color you don't have. Draw and score for each unique color.
    map.insert("Tissage".to_string(), vec![
        Effect::Sequence(vec![
            Effect::Meld {
                source: Location::Hand,
                filters: vec![Filter::ColorsNotOnBoard],
                min: 0,
                max: 1
            },
            Effect::DrawAndScoreForUniqueColors { age: 1 }
        ])
    ]);

    // --- Age 2 ---
    
    // Calendrier: If score pile > hand, draw 2 Age 3s
    map.insert("Calendrier".to_string(), vec![
        Effect::Conditional {
            condition: crate::model::Condition::ScoreGreaterThanHand,
            success: Box::new(Effect::Draw { amount: 2, age: Some(3) }),
            failure: None
        }
    ]);

    // Fermentation: Draw 1 for every 2 Leaves
    map.insert("Fermentation".to_string(), vec![
        Effect::DrawForIconPairs { age: 2, symbol: Symbol::Leaf, divisor: 2 }
    ]);

    // Cartographie (Demand): Transfer an Age 1 from your Score to my Score. If you do, draw and score a 1.
    map.insert("Cartographie".to_string(), vec![
        Effect::Sequence(vec![
            Effect::Transfer {
                source: Location::Score,
                dest: Location::Score,
                dest_is_activator: true,
                filters: vec![Filter::Value(1)],
                min: 1,
                max: 1
            },
            Effect::Conditional {
                condition: crate::model::Condition::ContextValueGreaterThan(0),
                success: Box::new(Effect::DrawAndScore { amount: 1, age: Some(1) }),
                failure: None
            }
        ])
    ]);

    map
}
