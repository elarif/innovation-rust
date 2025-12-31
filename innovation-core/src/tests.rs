use crate::game_state::GameState;
use crate::actions::Action;

#[test]
fn test_initial_state() {
    let mut game = GameState::new(12345, 2);
    assert_eq!(game.players.len(), 2);
    assert_eq!(game.current_player, 0);
    assert_eq!(game.turn_number, 1);
    
    // Check hands
    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(game.players[1].hand.len(), 2);
}

#[test]
fn test_draw() {
    let mut game = GameState::new(12345, 2);
    let p0_hand_size = game.players[0].hand.len();
    
    game.apply_action(Action::Draw).unwrap();
    
    assert_eq!(game.players[0].hand.len(), p0_hand_size + 1);
}

#[test]
fn test_meld() {
    let mut game = GameState::new(12345, 2);
    
    // Force a specific card into hand for testing or just use one from initial hand
    let card_to_meld = game.players[0].hand[0].clone();
    
    game.apply_action(Action::Meld(card_to_meld.clone())).unwrap();
    
    // Check it's gone from hand
    assert!(!game.players[0].hand.contains(&card_to_meld));
    
    // Check it's on board
    let p = &game.players[0];
    let found_on_board = p.board.values().any(|pile| pile.cards.contains(&card_to_meld));
    assert!(found_on_board);
}

#[test]
fn test_icon_counts() {
    let mut game = GameState::new(12345, 2);
    // TODO: Setup a board with known cards and splay, then assert icon counts
    // For now just basic check
    let counts = game.players[0].icon_counts(crate::db::load_all_cards());
    // Initial hand empty board -> 0 icons
    assert!(counts.is_empty());
}

#[test]
fn test_dogma_execution() {
    let mut game = GameState::new(12345, 2);
    
    // P0 Melds a card
    let card_id = game.players[0].hand[0].clone();
    game.apply_action(Action::Meld(card_id.clone())).unwrap();
    
    // P0 Dogma
    // Should be Ok even if effect is placeholder
    game.apply_action(Action::Dogma(card_id)).unwrap();
}

#[test]
fn test_achieve() {
    let mut game = GameState::new(12345, 2);
    
    // Cheat: Give P0 score
    // Add 5 "Age 1" cards to score pile
    // We need valid IDs. "Élevage" is Age 1.
    // We can just find some age 1 cards
    let db = crate::db::load_all_cards();
    let age1_card = db.values().find(|c| c.age == 1).unwrap();
    
    for _ in 0..5 {
        game.players[0].score_pile.push(age1_card.name.clone());
    }
    
    // Cheat: Meld an Age 1 card (required for Age 1 achievement)
    let meld_card = db.values().find(|c| c.age == 1 && c.color == crate::model::Color::Red).unwrap();
    game.players[0].board.insert(crate::model::Color::Red, {
        let mut p = crate::player::Pile::new(crate::model::Color::Red);
        p.cards.push(meld_card.name.clone());
        p
    });
    
    // Try achieve Age 1
    game.apply_action(Action::Achieve("Age 1".to_string())).unwrap();
    
    assert!(game.achievements.contains("Age 1"));
    assert_eq!(game.players[0].achievements.len(), 1);
}

#[test]
fn test_registry_execution_ecriture() {
    let mut game = crate::game_state::GameState::new(12345, 2);
    
    // Cheat: Give P0 "Écriture" into hand.
    game.players[0].hand.push("Écriture".to_string());
    
    // Meld it.
    game.apply_action(crate::actions::Action::Meld("Écriture".to_string())).unwrap();
    
    let hand_before = game.players[0].hand.len();
    
    // Activate Dogma
    // Ecriture: Draw 2 [Age 2]
    game.apply_action(crate::actions::Action::Dogma("Écriture".to_string())).unwrap();
    
    // Should have drawn 2 cards.
    assert_eq!(game.players[0].hand.len(), hand_before + 2);
    
    // Verify they are Age 2
    let db = crate::db::load_all_cards();
    let new_card_1 = &game.players[0].hand[hand_before];
    let new_card_2 = &game.players[0].hand[hand_before + 1];
    
    assert_eq!(db.get(new_card_1).unwrap().age, 2);
    assert_eq!(db.get(new_card_2).unwrap().age, 2);
}

#[test]
fn test_registry_sequence_outils() {
    let mut game = crate::game_state::GameState::new(12345, 2);
    
    // Setup: P0 has "Outils" in hand and only 1 card to recycle.
    // With only 1 card, context_value = 1 which is NOT > 2, so Conditional fails.
    game.players[0].hand.clear();
    game.players[0].hand.push("Outils".to_string());
    game.players[0].hand.push("Maçonnerie".to_string()); // Age 1, so second Recycle won't find it
    
    // Clear P1 hand
    game.players[1].hand.clear();
    
    game.apply_action(crate::actions::Action::Meld("Outils".to_string())).unwrap();
    
    // Hand now only has Maçonnerie.
    assert_eq!(game.players[0].hand.len(), 1);
    
    // 1. Activate Dogma - Expect Input Request (first Recycle: 0-3 cards)
    game.apply_action(crate::actions::Action::Dogma("Outils".to_string())).unwrap();
    
    // Confirm pending input
    assert!(game.pending_input.is_some());
    
    // 2. Provide Input (Maçonnerie) - recycle 1 card
    game.apply_action(crate::actions::Action::ResolveInput("Maçonnerie".to_string())).unwrap();
    
    // With 1 card recycled, context_value = 1, which is NOT > 2.
    // First Conditional should FAIL (no DrawAndMeld).
    // Second Recycle wants Age 3 cards, but hand is empty now.
    // Both conditionals fail, so no draws happen.
    
    // Verify Maçonnerie gone (Recycled to deck/bottom)
    assert!(!game.players[0].hand.contains(&"Maçonnerie".to_string()));
    
    // No DrawAndMeld happened (conditional failed), hand should be empty
    assert!(game.players[0].hand.is_empty());
    
    // Score pile should be empty (no DrawAndScore in new registry)
    assert_eq!(game.players[0].score_pile.len(), 0);
}

#[test]
fn test_registry_voiles() {
    let mut game = crate::game_state::GameState::new(12345, 2);
    game.players[0].hand.clear();
    game.players[0].hand.push("Voiles".to_string());
    
    // Meld
    game.apply_action(crate::actions::Action::Meld("Voiles".to_string())).unwrap();
    
    // Clear P1
    game.players[1].hand.clear();
    
    // Execute
    // Voiles: Draw 1 and Meld it.
    let board_count_before = game.players[0].board.values().map(|p| p.cards.len()).sum::<usize>();
    
    game.apply_action(crate::actions::Action::Dogma("Voiles".to_string())).unwrap();
    
    let board_count_after = game.players[0].board.values().map(|p| p.cards.len()).sum::<usize>();
    assert_eq!(board_count_after, board_count_before + 1); // +1 from Melded card (Voiles itself is already there)
}

#[test]
fn test_registry_agriculture() {
    let mut game = crate::game_state::GameState::new(12345, 2);
    // Setup P0 with Agriculture + 2 dummy cards (needed for Return and Score)
    game.players[0].hand.clear();
    game.players[0].hand.push("Agriculture".to_string());
    game.players[0].hand.push("Maçonnerie".to_string());
    game.players[0].hand.push("Poterie".to_string()); // 2 extra
    
    game.apply_action(crate::actions::Action::Meld("Agriculture".to_string())).unwrap();
    game.players[1].hand.clear();
    
    // Execute
    // Steps:
    // 1. Recycle (Input required)
    // 2. Score (Input required)
    
    // Trigger Dogma
    game.apply_action(crate::actions::Action::Dogma("Agriculture".to_string())).unwrap();
    
    // 1. Recycle Input
    assert!(game.pending_input.is_some());
    game.apply_action(crate::actions::Action::ResolveInput("Maçonnerie".to_string())).unwrap();
    
    // 2. Score Input
    // Note: Recycle clears pending_input, but next step Score sets it again?
    // Check Sequence logic. Loop advances. Score sees Hand (Poterie left). Requests Input.
    assert!(game.pending_input.is_some());
    game.apply_action(crate::actions::Action::ResolveInput("Poterie".to_string())).unwrap();
    
    // Done
    assert!(game.pending_input.is_none());
    
    // Check State
    // Score pile should have 1 (Poterie)
    assert_eq!(game.players[0].score_pile.len(), 1);
    assert_eq!(game.players[0].score_pile[0], "Poterie");
    
    // Hand empty
    assert!(game.players[0].hand.is_empty());
}

#[test]
fn test_registry_code_de_lois() {
    eprintln!("Test Started: test_registry_code_de_lois");
    let mut game = crate::game_state::GameState::new(12345, 2);
    
    // Setup
    game.players[0].hand.clear();
    eprintln!("Hand Cleared");
    
    // Add Métallurgie (Red) to board
    game.players[0].hand.push("Métallurgie".to_string());
    eprintln!("Pushed Métallurgie");
    game.apply_action(crate::actions::Action::Meld("Métallurgie".to_string())).unwrap();
    eprintln!("Melded Métallurgie");
    
    // Add Code de lois (Purple) to board
    game.players[0].hand.push("Code de lois".to_string());
    game.apply_action(crate::actions::Action::Meld("Code de lois".to_string())).unwrap();
    eprintln!("Melded Code de lois");
    
    // Add Archerie (Red) to hand
    game.players[0].hand.push("Archerie".to_string());
    
    // Current state check
    eprintln!("Checking Red Pile Before");
    let red_pile = game.players[0].board.get(&crate::model::Color::Red).unwrap();
    eprintln!("Red Pile Found");
    use crate::model::SplayDirection;
    assert_eq!(pile_splay(red_pile), SplayDirection::None);
    
    // Execute Dogma
    eprintln!("Executing Dogma Code de lois");
    game.apply_action(crate::actions::Action::Dogma("Code de lois".to_string())).unwrap();
    
    // Should request input (Tuck Red) because Red is on board.
    assert!(game.pending_input.is_some());
    // Resolve input (Archerie)
    eprintln!("Resolving Input Archerie");
    game.apply_action(crate::actions::Action::ResolveInput("Archerie".to_string())).unwrap();
    
    // Check results
    // Hand empty (Archerie tucked)
    assert!(game.players[0].hand.is_empty());
    
    eprintln!("Checking Red Pile After");
    // Red pile should be splayed Left
    let red_pile_after = game.players[0].board.get(&crate::model::Color::Red).unwrap();
    eprintln!("Red Pile After Found. Splay: {:?}", pile_splay(red_pile_after));
    assert_eq!(pile_splay(red_pile_after), SplayDirection::Left);
    
    // Red pile count increased (Outils + Archerie = 2)
    assert_eq!(red_pile_after.cards.len(), 2);
}

#[test]
fn test_registry_elevage() {
    eprintln!("Test Started: test_registry_elevage");
    let mut game = crate::game_state::GameState::new(12345, 2);
    
    // Setup
    game.players[0].hand.clear();
    
    // Add Élevage (Yellow) to board
    game.players[0].hand.push("Élevage".to_string());
    game.apply_action(crate::actions::Action::Meld("Élevage".to_string())).unwrap();
    
    // Add Age 1 card (lowest) and Age 2 card (Calendrier) to hand
    game.players[0].hand.push("Agriculture".to_string()); // Age 1
    game.players[0].hand.push("Calendrier".to_string()); // Age 2
    
    // Verify context
    let p = &game.players[0];
    assert_eq!(p.hand.len(), 2);
    assert!(p.hand.contains(&"Agriculture".to_string()));
    assert!(p.hand.contains(&"Calendrier".to_string()));
    
    // Execute Dogma
    // Execute Dogma
    game.apply_action(crate::actions::Action::Dogma("Élevage".to_string())).unwrap();
    
    // Should AUTO-MELD "Agriculture" because it is uniquely the lowest age (1 vs 2).
    // So no pending input.
    assert!(game.pending_input.is_none());
    
    // Check results
    // Agriculture should be on board (Yellow pile).
    // Élevage is also Yellow, so Agriculture is on top (or tucked? Meld puts on top).
    let yellow_pile = game.players[0].board.get(&crate::model::Color::Yellow).unwrap();
    assert_eq!(yellow_pile.cards.last().map(|s| s.as_str()), Some("Agriculture")); // Top card
    
    // Hand should contain Calendrier + 1 drawn card (Age 1).
    let p = &game.players[0];
    assert_eq!(p.hand.len(), 2);
    assert!(p.hand.contains(&"Calendrier".to_string()));
    // And one other card
}

#[test]
fn test_registry_maconnerie() {
    eprintln!("Test Started: test_registry_maconnerie");
    let mut game = crate::game_state::GameState::new(12345, 2);
    
    // Setup
    game.players[0].hand.clear();
    
    // Add Maçonnerie (Yellow) to board
    game.players[0].hand.push("Maçonnerie".to_string());
    game.apply_action(crate::actions::Action::Meld("Maçonnerie".to_string())).unwrap();
    
    // Add Castles and Non-Castles to hand
    game.players[0].hand.push("Cités-États".to_string()); // Castle
    game.players[0].hand.push("Mysticisme".to_string()); // Castle
    game.players[0].hand.push("Agriculture".to_string()); // Leaf
    
    // Execute Dogma
    game.apply_action(crate::actions::Action::Dogma("Maçonnerie".to_string())).unwrap();
    
    // Should request input because we have candidate Castles and count is variable.
    assert!(game.pending_input.is_some());
    
    if let Some(crate::dogma::flow::InputRequest::SelectCard { source, min, max, .. }) = &game.pending_input {
        assert_eq!(*min, 0);
        assert_eq!(*max, 255);
        // Verify filtering: source should only contain Castles
        assert!(source.contains(&"Cités-États".to_string()));
        assert!(source.contains(&"Mysticisme".to_string()));
        assert!(!source.contains(&"Agriculture".to_string()));
        assert_eq!(source.len(), 2);
    } else {
        panic!("Expected SelectCard input request");
    }
    
    // Select both Castles
    game.apply_action(crate::actions::Action::ResolveInput("Cités-États,Mysticisme".to_string())).unwrap();
    
    // Verify results
    let p = &game.players[0];
    assert!(p.hand.contains(&"Agriculture".to_string()));
    assert!(!p.hand.contains(&"Cités-États".to_string()));
    assert!(!p.hand.contains(&"Mysticisme".to_string()));
    
    // Verify Melds (Purple pile should have 2 cards now? Cités-États and Mysticisme are both Purple)
    let purple_pile = p.board.get(&crate::model::Color::Purple).unwrap();
    assert_eq!(purple_pile.cards.len(), 2);
}

#[test]
fn test_registry_poterie() {
    // Poterie (Green/Leaves): Recycle up to 3 cards from Hand. Draw and Score a card of value = count.
    eprintln!("Test Started: test_registry_poterie");
    let mut game = crate::game_state::GameState::new(12345, 2);
    
    // Setup
    game.players[0].hand.clear();
    
    // Add Poterie
    game.players[0].hand.push("Poterie".to_string());
    game.apply_action(crate::actions::Action::Meld("Poterie".to_string())).unwrap();
    
    // Add 3 cards to hand (Real cards)
    game.players[0].hand.push("La Roue".to_string()); 
    game.players[0].hand.push("Agriculture".to_string());
    game.players[0].hand.push("Métallurgie".to_string());
    game.players[0].hand.push("Tissage".to_string());
    
    // Execute Dogma
    game.apply_action(crate::actions::Action::Dogma("Poterie".to_string())).unwrap();
    
    // Should request input (Recycle 0-3).
    assert!(game.pending_input.is_some());
    
    // Select 2 cards to recycle
    game.apply_action(crate::actions::Action::ResolveInput("Agriculture,Métallurgie".to_string())).unwrap();
    
    // Expected result:
    // Hand: Tissage and La Roue left.
    // Score pile: Should have 1 card of Age 2 (since we recycled 2).
    let p = &game.players[0];
    eprintln!("Hand after Poterie: {:?}", p.hand);
    eprintln!("Score pile after Poterie: {:?}", p.score_pile);
    
    assert!(p.hand.contains(&"Tissage".to_string()));
    assert!(p.hand.contains(&"La Roue".to_string()));
    assert!(!p.hand.contains(&"Agriculture".to_string()));
    assert!(!p.hand.contains(&"Métallurgie".to_string()));
    
    // Score pile check
    assert_eq!(p.score_pile.len(), 1, "Score pile should have 1 card");
    let scored_card_id = &p.score_pile[0];
    let db = crate::db::load_all_cards();
    let card = db.get(scored_card_id).unwrap();
    assert_eq!(card.age, 2, "Scored card should be Age 2");
}

#[test]
fn test_registry_outils() {
    // "Outils" (Tools):
    // 1. Return 3 cards from hand. If do, draw a 3 and meld it.
    // 2. Return a 3 from hand. If do, draw three 1s.
    eprintln!("Test Started: test_registry_outils");
    let mut game = crate::game_state::GameState::new(12345, 2);
    
    // Setup P0
    let p0_id = 0;
    game.players[p0_id].hand.clear();
    game.players[p0_id].hand.push("Outils".to_string());
    game.apply_action(crate::actions::Action::Meld("Outils".to_string())).unwrap();
    
    // Hand Setup: 3 Age 1s + 1 Age 3 (Ingénierie)
    game.players[p0_id].hand.push("Agriculture".to_string());
    game.players[p0_id].hand.push("Archerie".to_string());
    game.players[p0_id].hand.push("Métallurgie".to_string());
    game.players[p0_id].hand.push("Ingénierie".to_string()); // Age 3
    
    game.current_player = 0;
    
    // Execute Dogma
    game.apply_action(crate::actions::Action::Dogma("Outils".to_string())).unwrap();
    
    // Expect Input 1: Select 3 cards to recycle (min 0, max 3). 
    // We select 3 Age 1s.
    assert!(game.pending_input.is_some(), "Should request input for Outils Dogma 1");
    // Assuming CSV input support
    game.apply_action(crate::actions::Action::ResolveInput("Agriculture,Archerie,Métallurgie".to_string())).unwrap();
    
    // Check outcome of Dogma 1:
    // Should have drawn and melded an Age 3 card.
    // P0 Board should have Red (Ingénierie is Red, wait, Ingénierie is in hand).
    // The drawn Age 3 could be anything. Seed is fixed, so deterministic.
    // Let's check hand count. Should have 1 (Ingénierie).
    // Board should have Outils (Blue) + New Card (Color?).
    // Score pile should have 0.
    
    // Expect Input 2: Select a 3 to recycle (min 0, max 1).
    assert!(game.pending_input.is_some(), "Should request input for Outils Dogma 2");
    
    // Select Ingénierie
    game.apply_action(crate::actions::Action::ResolveInput("Ingénierie".to_string())).unwrap();
    
    // Check outcome of Dogma 2:
    // Should draw 3 Age 1s.
    // Hand should now have 3 cards (Age 1s).
    let p0 = &game.players[p0_id];
    assert_eq!(p0.hand.len(), 3, "Should have 3 cards in hand after Dogma 2");
    
    // Verify cards are likely Age 1 (names might vary, assuming deck has them).
    let db = crate::db::load_all_cards();
    for cid in &p0.hand {
        let card = db.get(cid).unwrap();
        assert_eq!(card.age, 1, "Drawn cards should be Age 1");
    }
}

// Helper because pile_splay field access might need wrapping if I messed up imports in test file
fn pile_splay(pile: &crate::player::Pile) -> crate::model::SplayDirection {
    pile.splay
}

#[test]
fn test_registry_rames() {
    // Rames (Demand): Transfer a Crown card from your Hand to my Score. Draw 1.
    // Demand requires activator to have MORE icons than victim.
    eprintln!("Test Started: test_registry_rames");
    let mut game = crate::game_state::GameState::new(12345, 2);
    
    // Setup: P0 has Rames on board. P1 has a Crown card in hand.
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    
    // Meld Rames for P0 (has Castle icons which is the dogma symbol)
    game.players[0].hand.push("Rames".to_string());
    game.apply_action(crate::actions::Action::Meld("Rames".to_string())).unwrap();
    
    // Meld another Castle card for P0 to ensure they have more Castles than P1
    game.players[0].hand.push("Cités-États".to_string());
    game.apply_action(crate::actions::Action::Meld("Cités-États".to_string())).unwrap();
    
    // Verify P0 has Castles
    let db = crate::db::load_all_cards();
    let p0_castles = game.players[0].icon_counts(db).get(&crate::model::Symbol::Castle).cloned().unwrap_or(0);
    eprintln!("P0 Castles: {}", p0_castles);
    assert!(p0_castles > 0, "P0 should have Castle icons");
    
    // Give P1 a card with Crown icon (Élevage has Crown)
    game.players[1].hand.push("Élevage".to_string());
    
    // P0 activates Rames Dogma
    // Since Rames is a Demand (isSupremacy), P1 (victim with fewer Castles) should be affected.
    // With exactly 1 Crown card, P1 auto-transfers it (no input needed).
    game.apply_action(crate::actions::Action::Dogma("Rames".to_string())).unwrap();
    
    // Verify: Élevage should be in P0's score pile (activator)
    assert!(game.players[0].score_pile.contains(&"Élevage".to_string()), "Élevage should be in P0 score pile");
    // P1 should have drawn a card after the transfer
    assert!(!game.players[1].hand.is_empty(), "P1 should have drawn a card");
}