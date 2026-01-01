# Data Model: Game Logic

## Key Entities

### GameState
**Responsibility**: Root verification and state holder.
- `players`: `Vec<Player>`
- `deck`: `Vec<CardId>` (per Age)
- `achievements`: `HashSet<CardId>`
- `current_player`: `usize` (index)
- `turn_number`: `u32`
- `rng`: `StdRng` (internal, not serialized usually, or serialized as seed)

### Player
**Responsibility**: Per-player inventory.
- `id`: `usize`
- `hand`: `Vec<CardId>`
- `score_pile`: `Vec<CardId>`
- `board`: `HashMap<Color, Stack>`
- `achievements`: `Vec<CardId>`

### Stack
**Responsibility**: A pile of cards of one color on a board.
- `cards`: `Vec<CardId>` (Ordered from bottom to top)
- `splay`: `SplayDirection` (None, Left, Right, Up)

### Card (Static)
**Responsibility**: Reference data from `cards.json`.
- `name`: `String`
- `age`: `u8`
- `color`: `Color`
- `icons`: `[Icon; 4]`
- `dogmas`: `Vec<Dogma>`

## API Contracts (Rust Interface)

### Actions
Public Enum `Action`:
- `Draw`
- `Meld(CardId)`
- `Archieve(CardId)` (e.g. for standard achievements)
- `Dogma(CardId)`

### State Methods
- `GameState::new(seed: u64) -> Self`
- `GameState::apply_action(&mut self, action: Action) -> Result<(), GameError>`
- `GameState::calculate_score(&self, player_id: usize) -> u32`
- `GameState::get_icons(&self, player_id: usize) -> HashMap<Icon, u32>`
