use leptos::prelude::*;
use crate::state::GameSignal;

#[component]
pub fn ActionBar() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    let is_setup = move || game_signal.is_setup_phase();
    
    view! {
        <div class="action-bar">
            <Show when=move || !is_setup()>
                <DrawButton/>
                <MeldButton/>
                <DogmaButton/>
            </Show>
        </div>
    }
}

#[component]
fn DrawButton() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    
    view! {
        <button class="primary" on:click=move |_| game_signal.draw()>
            "Piocher"
        </button>
    }
}

#[component]
fn MeldButton() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    let has_selection = move || game_signal.selected_hand_idx.get().is_some();
    
    view! {
        <button class="secondary" on:click=move |_| {
            if let Some(idx) = game_signal.selected_hand_idx.get() {
                let name = game_signal.game.with(|g| {
                    g.players[g.current_player].hand.get(idx).cloned()
                });
                if let Some(n) = name {
                    game_signal.meld(n);
                }
            }
        } disabled=move || !has_selection()>
            "Poser"
        </button>
    }
}

#[component]
fn DogmaButton() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    let has_selection = move || game_signal.selected_pile_idx.get().is_some();
    
    let colors = [
        innovation_core::Color::Red,
        innovation_core::Color::Blue,
        innovation_core::Color::Green,
        innovation_core::Color::Yellow,
        innovation_core::Color::Purple,
    ];
    
    view! {
        <button class="secondary" on:click=move |_| {
            if let Some(pile_idx) = game_signal.selected_pile_idx.get() {
                if let Some(color) = colors.get(pile_idx) {
                    let name = game_signal.game.with(|g| {
                        g.players[g.current_player].board.get(color)
                            .and_then(|p| p.top().cloned())
                    });
                    if let Some(n) = name {
                        game_signal.dogma(n);
                    }
                }
            }
        } disabled=move || !has_selection()>
            "Dogme"
        </button>
    }
}
