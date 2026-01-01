use leptos::prelude::*;
use crate::state::GameSignal;
use innovation_core::{load_all_cards, Color as CardColor, model::SplayDirection};

fn color_class(color: CardColor) -> &'static str {
    match color {
        CardColor::Red => "red",
        CardColor::Blue => "blue",
        CardColor::Green => "green",
        CardColor::Yellow => "yellow",
        CardColor::Purple => "purple",
    }
}

fn splay_text(splay: SplayDirection) -> &'static str {
    match splay {
        SplayDirection::Left => "<<<",
        SplayDirection::Right => ">>>",
        SplayDirection::Up => "^^^",
        SplayDirection::None => "",
    }
}

#[component]
pub fn Pile(color: CardColor, player_idx: usize, pile_idx: usize) -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    let db = load_all_cards();
    
    let pile_data = move || {
        game_signal.game.with(|g| {
            let pile = g.players[player_idx].board.get(&color);
            if let Some(p) = pile {
                if let Some(card_name) = p.top() {
                    let card = db.get(card_name);
                    let age = card.map(|c| c.age).unwrap_or(0);
                    return Some((card_name.clone(), age, p.splay, p.cards.len()));
                }
            }
            None
        })
    };
    
    let is_selected = move || {
        game_signal.selected_pile_idx.get() == Some(pile_idx)
    };
    
    let on_click = move |_| {
        if let Some((card_name, _, _, _)) = pile_data() {
            game_signal.selected_pile_idx.set(Some(pile_idx));
        }
    };
    
    let class_str = move || {
        let base = format!("pile {}", color_class(color));
        if is_selected() { format!("{} selected", base) } else { base }
    };
    
    view! {
        <div class=class_str on:click=on_click>
            <div class="pile-header">{format!("{:?}", color)}</div>
            {move || match pile_data() {
                Some((name, age, splay, count)) => view! {
                    <div class="pile-card">{format!("{} ({})", name, age)}</div>
                    <div class="pile-splay">{splay_text(splay)}</div>
                    {if count > 1 {
                        view! { <div class="pile-count">{format!("{} cartes", count)}</div> }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                }.into_any(),
                None => view! { <div class="pile-empty">"Vide"</div> }.into_any(),
            }}
        </div>
    }
}
