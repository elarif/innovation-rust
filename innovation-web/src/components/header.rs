use leptos::prelude::*;
use crate::state::GameSignal;

#[component]
pub fn Header() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    
    let turn_info = move || {
        let turn = game_signal.turn_number();
        let player = game_signal.current_player();
        let phase = if game_signal.is_setup_phase() { "Setup" } else { "Main" };
        format!("Tour {} | Joueur {} | {}", turn, player, phase)
    };
    
    view! {
        <header class="header">
            <h1>"Innovation"</h1>
            <span class="turn-info">{turn_info}</span>
        </header>
    }
}
