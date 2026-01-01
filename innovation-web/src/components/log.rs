use leptos::prelude::*;
use crate::state::GameSignal;

#[component]
pub fn Log() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    
    let show_full = move || game_signal.show_full_log.get();
    
    view! {
        <Show when=show_full>
            <FullLogModal/>
        </Show>
        <div class="log">
            {move || {
                let logs = game_signal.action_log.get();
                let last = logs.last().cloned().unwrap_or_else(|| "Bienvenue!".to_string());
                format!(">> {}", last)
            }}
        </div>
    }
}

#[component]
fn FullLogModal() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    
    view! {
        <div class="modal-overlay" on:click=move |_| game_signal.toggle_full_log()>
            <div class="modal full-log-modal" on:click=|e| e.stop_propagation()>
                <div class="modal-header">
                    <h2>"📜 Journal de la partie"</h2>
                    <CloseLogButton/>
                </div>
                <div class="modal-content">
                    <LogEntries/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CloseLogButton() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    view! {
        <button class="close-btn" on:click=move |_| game_signal.toggle_full_log()>"✕"</button>
    }
}

#[component]
fn LogEntries() -> impl IntoView {
    let game_signal = expect_context::<GameSignal>();
    
    view! {
        <div class="log-entries">
            <For
                each=move || game_signal.action_log.get()
                key=|entry| entry.clone()
                children=|entry| {
                    let class = if entry.starts_with("===") {
                        "log-entry section"
                    } else if entry.starts_with("---") {
                        "log-entry subsection"
                    } else if entry.contains("Erreur") {
                        "log-entry error"
                    } else {
                        "log-entry"
                    };
                    view! { <div class=class>{entry}</div> }
                }
            />
        </div>
    }
}
