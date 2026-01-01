# Implementation Plan - Feature 002: Dogma Effects

## Goal
Implement the dogma execution engine, support user input for complex effects, and implement all 105 Base Set cards.

## Architecture Change: The Input Loop
Innovation cards frequently require choices ("Choose a card to return"). Rust functions cannot easily "pause" in the middle of a loop (iterating players) to wait for I/O without async or a state machine.

### Design Pattern: Stack-Based State Machine
Instead of a single monolithic `execute_dogma` function, we will maintain an `ExecutionStack` in `GameState`.

1.  **ExecutionStack**: A stack of `EffectStep` items.
    *   `EffectStep::Dogma(card_id, player_index)`
    *   `EffectStep::Effect(current_player, instruction)`
    *   `EffectStep::WaitInput(InputType)`
2.  **Flow**:
    *   `Action::Dogma` pushes `EffectStep::Dogma` to stack.
    *   `GameState::process_queue()` pops the top step.
        *   If `Dogma`, it pushes individual `Effect` steps for eligible players.
        *   If `Effect`, it executes the logic.
        *   If the logic needs input, it pushes `WaitInput` and **returns**.
    *   The game is now in `State::WaitingForInput`.
    *   User sends `Action::Input(value)`.
    *   `process_queue()` resumes, using the input for the *previous* step logic.
    *   *Alternative*: Use a callback or simpler "Pending Input" field if the stack is too complex.
    
    *Simplified Approach for Rust*:
    *   `GameState` has `pending_input: Option<InputRequest>`.
    *   `GameState` has `dogma_state: Option<DogmaExecutionState>`.
    *   `DogmaExecutionState` holds:
        *   `card_id`
        *   `current_dogma_index`
        *   `eligible_players: Vec<usize>`
        *   `current_player_index`
    *   When `execute_dogma` runs, it fills `DogmaExecutionState`.
    *   It enters a loop `continue_dogma()`.
    *   If input needed: set `pending_input`, return.
    *   `Action::Input` calls `handle_input()` then `continue_dogma()`.

## Module Structure
*   `innovation-core/src/dogma/`
    *   `mod.rs`: Main entry point.
    *   `ids.rs`: Constants for card references (optional).
    *   `executor.rs`: State machine logic.
    *   `effects/`: Modules for effect logic by Age? Or huge match statement?
        *   Given 105 cards, a single file `cards.rs` with a match on name/ID is manageable but large (approx 2000 lines).
        *   Better: `effects/age1.rs`, `effects/age2.rs`, etc.

## Proposed Changes
### innovation-core
#### [MODIFY] [game_state.rs](file:///c:/Users/elari/workspace/innovation-rust/innovation-core/src/game_state.rs)
*   Add `pending_input` field.
*   Add `dogma_state` field.
*   Add `Action::ResolveInput`.

#### [NEW] [dogma/executor.rs](file:///c:/Users/elari/workspace/innovation-rust/innovation-core/src/dogma/executor.rs)
*   Struct `DogmaState`.
*   Function `step()` that advances the state.

#### [NEW] [dogma/collections.rs](file:///c:/Users/elari/workspace/innovation-rust/innovation-core/src/dogma/collections.rs)
*   Implementation of the 105 card effects.

## Verification Plan
*   **Unit Tests**: Test the State Machine (Pause/Resume).
    *   Trigger an effect requiring input.
    *   Assert `pending_input` is set.
    *   Send Input.
    *   Assert effect completes.
*   **Game Simulation**: Update `simulate` to provide random input when requested.
