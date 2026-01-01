use leptos::prelude::*;
use crate::state::GameSignal;

#[component]
pub fn GameMenu() -> impl IntoView {
    view! {
        <div class="game-menu">
            <NewGameButton/>
            <FullLogButton/>
            <AbandonButton/>
        </div>
    }
}

#[component]
fn NewGameButton() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    view! {
        <button class="menu-btn" on:click=move |_| game_signal.new_game()>
            "🔄 Nouvelle Partie"
        </button>
    }
}

#[component]
fn FullLogButton() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    view! {
        <button class="menu-btn" on:click=move |_| game_signal.toggle_full_log()>
            "📜 Journal Complet"
        </button>
    }
}

#[component]
fn AbandonButton() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    view! {
        <button class="menu-btn danger" on:click=move |_| game_signal.abandon_game()>
            "🏳️ Abandonner"
        </button>
    }
}
