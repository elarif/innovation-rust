
#[test]
fn test_registry_archerie() {
    // Archerie: Demand (Draw 1, Transfer Highest Hand -> My Hand).
    eprintln!("Test Started: test_registry_archerie");
    let mut game = crate::game_state::GameState::new(12345, 2);
    
    // Setup P0 (Activator)
    let p0_id = 0;
    game.players[p0_id].hand.clear();
    game.players[p0_id].hand.push("Archerie".to_string());
    game.apply_action(crate::actions::Action::Meld("Archerie".to_string())).unwrap();
    
    // Setup P1 (Victim)
    let p1_id = 1;
    game.players[p1_id].hand.clear();
    game.players[p1_id].hand.push("Agriculture".to_string()); // Age 1
    game.players[p1_id].hand.push("Ingénierie".to_string()); // Age 3
    
    game.current_player = 0;
    
    // Ensure P0 has more Crossbow/Red icons? 
    // Archerie is Red. 
    // P1 has 0 icons on board.
    // P0 has Archerie on board. 
    // Is P0 eligible for their own demand?
    // Demands are executed by opponents.
    
    // Execute
    game.apply_action(crate::actions::Action::Dogma("Archerie".to_string())).unwrap();
    
    // Checks:
    // P1 should have drawn a card.
    // P1's highest card (Ingénierie) should be transferred to P0.
    
    let p0 = &game.players[p0_id];
    let p1 = &game.players[p1_id];
    
    // P0 should have Ingénierie
    assert!(p0.hand.contains(&"Ingénierie".to_string()), "P0 should have received Ingénierie");
    
    // P1 should have drawn a 1. Hand size:
    // Started with 2. Lost 1. Gained 1 (Draw).
    // Net 2.
    assert_eq!(p1.hand.len(), 2, "P1 should have 2 cards");
    // Should NOT have Ingénierie
    assert!(!p1.hand.contains(&"Ingénierie".to_string()), "P1 should have lost Ingénierie");
}
