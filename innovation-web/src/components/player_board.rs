use leptos::prelude::*;
use crate::state::GameSignal;
use crate::components::Pile;
use innovation_core::{load_all_cards, Color as CardColor};

#[component]
pub fn PlayerBoard(player_idx: usize, is_opponent: bool) -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    let db = load_all_cards();
    
    let player_info = move || {
        game_signal.game.with(|g| {
            let p = &g.players[player_idx];
            let score = p.calculate_score(&db);
            let icons = p.icon_counts(&db);
            let icon_str: String = icons.iter()
                .filter(|(_, &c)| c > 0)
                .map(|(s, c)| format!("{:?}:{}", s, c))
                .collect::<Vec<_>>()
                .join(" ");
            (score, p.hand.len(), icon_str, g.current_player == player_idx)
        })
    };
    
    let colors = vec![CardColor::Red, CardColor::Blue, CardColor::Green, CardColor::Yellow, CardColor::Purple];
    
    view! {
        <div class=move || {
            let (_, _, _, is_current) = player_info();
            if is_current && !is_opponent { "player-board current" } else { "player-board" }
        }>
            <div class="player-info">
                <span>{if is_opponent { "Adversaire" } else { "Vous" }}</span>
                <span>{move || {
                    let (score, hand_len, icons, _) = player_info();
                    format!("Score: {} | Main: {} | {}", score, hand_len, icons)
                }}</span>
            </div>
            <div class="piles">
                {colors.into_iter().enumerate().map(|(i, color)| {
                    view! { <Pile color=color player_idx=player_idx pile_idx=i/> }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
