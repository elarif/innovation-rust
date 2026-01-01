use leptos::prelude::*;
use crate::state::GameSignal;
use innovation_core::{load_all_cards, Color as CardColor};

fn color_class(color: CardColor) -> &'static str {
    match color {
        CardColor::Red => "red",
        CardColor::Blue => "blue",
        CardColor::Green => "green",
        CardColor::Yellow => "yellow",
        CardColor::Purple => "purple",
    }
}

#[component]
pub fn Hand() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    let db = load_all_cards();
    
    let hand_cards = move || {
        game_signal.game.with(|g| {
            let player_idx = g.current_player;
            g.players[player_idx].hand.iter().map(|name| {
                let color = db.get(name).map(|c| c.color).unwrap_or(CardColor::Blue);
                let age = db.get(name).map(|c| c.age).unwrap_or(0);
                (name.clone(), color, age)
            }).collect::<Vec<_>>()
        })
    };
    
    let is_setup = move || game_signal.is_setup_phase();
    
    view! {
        <div class="hand">
            <h3>{move || if is_setup() { "Choisissez une carte à poser" } else { "Votre main" }}</h3>
            <div class="hand-cards">
                <For
                    each=hand_cards
                    key=|(name, _, _)| name.clone()
                    children=move |(name, color, age)| {
                        view! { <HandCard name=name color=color age=age/> }
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn HandCard(name: String, color: CardColor, age: u8) -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    let card_name = name.clone();
    let display_name = name.clone();
    
    let is_selected = {
        let n = name.clone();
        move || {
            game_signal.selected_hand_idx.with(|sel| {
                sel.map(|i| {
                    game_signal.game.with(|g| {
                        g.players[g.current_player].hand.get(i).map(|h| h == &n).unwrap_or(false)
                    })
                }).unwrap_or(false)
            })
        }
    };
    
    let on_click = move |_| {
        if game_signal.is_setup_phase() {
            game_signal.meld(card_name.clone());
        } else {
            // Find index
            let idx = game_signal.game.with(|g| {
                g.players[g.current_player].hand.iter().position(|h| h == &card_name)
            });
            if let Some(i) = idx {
                game_signal.selected_hand_idx.set(Some(i));
            }
        }
    };
    
    let class_str = move || {
        let base = format!("hand-card {}", color_class(color));
        if is_selected() { format!("{} selected", base) } else { base }
    };
    
    view! {
        <div class=class_str on:click=on_click>
            {format!("{} ({})", display_name, age)}
        </div>
    }
}
