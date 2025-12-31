use innovation_core::{load_all_cards, Color, GameState, actions::Action};
use std::collections::HashMap;
use clap::{Parser, Subcommand};
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};

#[derive(Parser)]
#[command(name = "innovation-cli")]
#[command(about = "CLI for Innovation Board Game Engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify database integrity and stats
    Verify,
    /// Run a game simulation
    Simulate {
        #[arg(short, long, default_value_t = 12345)]
        seed: u64,
        #[arg(short, long, default_value_t = 2)]
        players: usize,
        #[arg(short = 'a', long, default_value_t = 10)]
        max_age: u8,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Verify => verify_db(),
        Commands::Simulate { seed, players, max_age } => simulate_game(*seed, *players, *max_age),
    }
}

fn verify_db() {
    let cards = load_all_cards();
    println!("Total cards loaded: {}", cards.len());

    let mut by_age: HashMap<u8, HashMap<Color, usize>> = HashMap::new();

    for card in cards.values() {
        by_age
            .entry(card.age)
            .or_default()
            .entry(card.color)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    let mut ages: Vec<_> = by_age.keys().cloned().collect();
    ages.sort();

    println!("\nDistribution by Age and Color:");
    for age in ages {
        println!("Age {}:", age);
        let colors = &by_age[&age];
        let mut color_names: Vec<_> = colors.keys().collect();
        color_names.sort_by_key(|c| format!("{:?}", c)); 

        for color in color_names {
            println!("  {:?}: {}", color, colors[color]);
        }
        let total_age: usize = colors.values().sum();
        println!("  Total: {}", total_age);
    }
}

// Custom helper for rustyline with dynamic completions
struct GameHelper {
    commands: Vec<String>,
    cards: Vec<String>,
}

impl GameHelper {
    fn new() -> Self {
        Self {
            commands: vec![
                "meld".to_string(),
                "draw".to_string(), 
                "dogma".to_string(),
                "achieve".to_string(),
                "quit".to_string(),
            ],
            cards: Vec::new(),
        }
    }
    
    fn update_cards(&mut self, hand: &[String], board_cards: Vec<String>) {
        self.cards = hand.iter().chain(board_cards.iter()).cloned().collect();
    }
}

impl Completer for GameHelper {
    type Candidate = Pair;
    
    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> rustyline::Result<(usize, Vec<Pair>)> {
        let line_to_cursor = &line[..pos];
        let words: Vec<&str> = line_to_cursor.split_whitespace().collect();
        
        if words.is_empty() || (words.len() == 1 && !line_to_cursor.ends_with(' ')) {
            // Complete command
            let prefix = words.first().unwrap_or(&"");
            let matches: Vec<Pair> = self.commands.iter()
                .filter(|c| c.starts_with(prefix))
                .map(|c| Pair { display: c.clone(), replacement: c.clone() })
                .collect();
            let start = line_to_cursor.rfind(' ').map(|i| i + 1).unwrap_or(0);
            Ok((start, matches))
        } else {
            // Complete card name (for meld, dogma commands)
            let cmd = words[0];
            if cmd == "meld" || cmd == "dogma" {
                let prefix = if words.len() > 1 && !line_to_cursor.ends_with(' ') {
                    words[1]
                } else {
                    ""
                };
                let matches: Vec<Pair> = self.cards.iter()
                    .filter(|c| c.to_lowercase().starts_with(&prefix.to_lowercase()))
                    .map(|c| Pair { display: c.clone(), replacement: c.clone() })
                    .collect();
                let start = line_to_cursor.rfind(' ').map(|i| i + 1).unwrap_or(0);
                Ok((start, matches))
            } else {
                Ok((pos, vec![]))
            }
        }
    }
}

impl Hinter for GameHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for GameHelper {}
impl Validator for GameHelper {}
impl Helper for GameHelper {}

fn simulate_game(seed: u64, player_count: usize, max_age: u8) {
    println!("Starting simulation with seed {}, {} players, max age {}...", seed, player_count, max_age);
    println!("Tip: Use TAB for auto-completion!");
    let mut game = GameState::new_with_max_age(seed, player_count, max_age);
    
    let mut helper = GameHelper::new();
    let mut rl: Editor<GameHelper, rustyline::history::DefaultHistory> = Editor::new().unwrap();
    rl.set_helper(Some(helper));

    loop {
        if !game.winners.is_empty() {
            println!("Game Over! Winners: {:?}", game.winners);
            return;
        }

        // Update completer with current cards
        if let Some(h) = rl.helper_mut() {
            let p = &game.players[game.current_player];
            let board_cards: Vec<String> = p.board.values()
                .filter_map(|pile| pile.top().cloned())
                .collect();
            h.update_cards(&p.hand, board_cards);
        }

        // Check for Pending Input
        if let Some(req) = &game.pending_input {
            println!("\n*** INPUT REQUIRED ***");
            match req {
                innovation_core::dogma::flow::InputRequest::SelectCard { player_id, source, min, max } => {
                     println!("Player {} must select between {} and {} cards from: {:?}", player_id, min, max, source);
                     if let Some(h) = rl.helper_mut() {
                         h.cards = source.clone();
                     }
                },
                innovation_core::dogma::flow::InputRequest::SelectColor { player_id, colors } => {
                     println!("Player {} must select a color from: {:?}", player_id, colors);
                },
                _ => println!("Other input: {:?}", req)
            }
            
            match rl.readline("Select> ") {
                Ok(line) => {
                    let trimmed = line.trim().to_string();
                    let input = if trimmed == "auto" {
                        match req {
                            innovation_core::dogma::flow::InputRequest::SelectCard { source, .. } => {
                                if !source.is_empty() { source[0].clone() } else { String::new() }
                            }
                            innovation_core::dogma::flow::InputRequest::SelectColor { colors, .. } => {
                                if !colors.is_empty() { format!("{:?}", colors[0]) } else { String::new() }
                            }
                            _ => String::new()
                        }
                    } else {
                        trimmed
                    };
                    
                    if let Err(e) = game.apply_action(Action::ResolveInput(input)) {
                        println!("Error applying input: {:?}", e);
                    }
                },
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => return,
                Err(e) => { println!("Error: {:?}", e); continue; }
            }
            continue;
        }

        // Main Turn Loop
        if game.phase == innovation_core::game_state::GamePhase::Setup {
            println!("\n--- SETUP PHASE: Player {} ---", game.current_player);
            let p = &game.players[game.current_player];
            println!("Hand: {:?}", p.hand);
            println!("You must Meld one card from your hand to start.");
        } else {
            println!("\n--- Player {} Turn (Action {}/{}) ---", 
                game.current_player, 
                game.actions_taken + 1,
                if game.turn_number == 1 { 1 } else { 2 }
            );
            let p = &game.players[game.current_player];
            println!("Hand: {:?}", p.hand);
            println!("Board: {:?}", p.board.keys());
            let db = load_all_cards();
            println!("Score: {} points ({:?})", p.calculate_score(&db), p.score_pile);
            println!("Commands: meld <card>, draw, dogma <card>, achieve <age>, quit");
        }

        match rl.readline("> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(&line);
                let cmd: Vec<&str> = line.trim().split_whitespace().collect();
                
                if cmd.is_empty() { continue; }
                
                let res = match cmd[0] {
                    "meld" => {
                        if cmd.len() > 1 { game.apply_action(Action::Meld(cmd[1].to_string())) }
                        else { println!("Usage: meld <card_id>"); continue; }
                    },
                    "draw" => game.apply_action(Action::Draw),
                    "dogma" => {
                        if cmd.len() > 1 { game.apply_action(Action::Dogma(cmd[1].to_string())) }
                        else { println!("Usage: dogma <card_id>"); continue; }
                    },
                    "achieve" => {
                        println!("Achieve not implemented yet");
                        Ok(())
                    },
                    "exit" | "quit" => return,
                    _ => { println!("Unknown command. Try: meld, draw, dogma, achieve, quit"); continue; }
                };
                
                if let Err(e) = res {
                    println!("Action Error: {:?}", e);
                } else {
                    println!("OK.");
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => return,
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
