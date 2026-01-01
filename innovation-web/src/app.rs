use leptos::prelude::*;
use crate::components::{Header, PlayerBoard, Hand, ActionBar, Log, GameMenu};
use crate::state::GameSignal;
use innovation_core::GameState;

#[component]
pub fn App() -> impl IntoView {
    // Create game state signal with random seed
    let seed = web_sys::js_sys::Date::now() as u64;
    let game = GameSignal::new(seed, 2, 1);
    
    // Provide game context to all children
    provide_context(game);
    
    view! {
        <div class="app">
            <Header/>
            <GameMenu/>
            <div class="game-area">
                <PlayerBoard player_idx=1 is_opponent=true/>
                <PlayerBoard player_idx=0 is_opponent=false/>
            </div>
            <Hand/>
            <Log/>
            <ActionBar/>
        </div>
    }
}
