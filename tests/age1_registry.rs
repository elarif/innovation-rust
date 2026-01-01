use innovation_core::game_state::GameState;
use innovation_core::actions::Action;

#[test]
fn test_registry_archerie() {
    // Archerie: Demand (Draw 1, Transfer Highest Hand -> My Hand).
    eprintln!("Test Started: test_registry_archerie");
    let mut game = GameState::new(12345, 2);
    
    // Setup P0 (Activator)
    let p0_id = 0;
    game.players[p0_id].hand.clear();
    game.players[p0_id].hand.push("Archerie".to_string());
    // Assume Meld action works or manually set board
    // We can use apply_action if exposed, or manual manipulation if fields are pub.
    // GameState fields are pub.
    
    // Manually meld Archerie for P0
    let archerie = innovation_core::db::load_all_cards().get("Archerie").unwrap().clone();
    game.players[p0_id].board.entry(archerie.color).or_insert_with(|| innovation_core::player::Pile::new(archerie.color)).cards.push("Archerie".to_string());
    
    // Setup P1 (Victim)
    let p1_id = 1;
    game.players[p1_id].hand.clear();
    game.players[p1_id].hand.push("Agriculture".to_string()); // Age 1
    game.players[p1_id].hand.push("Ingénierie".to_string()); // Age 3
    
    game.current_player = 0;
    
    // Execute
    // Note: Action::Dogma takes string.
    game.apply_action(Action::Dogma("Archerie".to_string())).unwrap();
    
    // Checks:
    // P1 should have drawn a card (Age 1).
    // P1's highest card (Ingénierie, Age 3) should be transferred to P0.
    
    let p0 = &game.players[p0_id];
    let p1 = &game.players[p1_id];
    
    // P0 should have Ingénierie
    assert!(p0.hand.contains(&"Ingénierie".to_string()), "P0 should have received Ingénierie");
    
    // P1 should have drawn a 1. Hand size:
    // Started with 2 (Agri, Ing). 
    // Draw 1 -> 3.
    // Transfer 1 (Ing) -> 2.
    assert_eq!(p1.hand.len(), 2, "P1 should have 2 cards");
    // Should NOT have Ingénierie
    assert!(!p1.hand.contains(&"Ingénierie".to_string()), "P1 should have lost Ingénierie");
}
