# Feature 002: Dogma Effects Implementation

## Goal
Implement the specific game logic for the card effects (Dogmas) of the 105 Base Set cards in `innovation-core`.

## Context
Feature 001 established the `execute_dogma` flow, which iterates over eligible players and handles "Demand" vs "Non-Demand" logic. However, the actual effect execution (`execute_effect`) is a placeholder. This feature will populate that logic.

## Scope
1.  **Effect System Architecture**: Design a robust way to map Card IDs to executable code.
2.  **Primitive Operations**: Implement helper methods in `GameState` for common effects (e.g., `draw_age`, `recycle`, `tuck`, `splay`, `score_card`).
3.  **Card Implementation**: Implement logic for all 105 base cards (or a subset first, e.g., Age 1-5).
    *   *Decision*: We will implement **ALL** Base Set cards in this feature to complete the core engine.

## Functional Requirements
*   **Card Mapping**: System must correctly identify and execute the code corresponding to a `card_id`.
*   **Prompting**: Many effects require user input (e.g., "Choose a card to meld"). The system must support pausing execution to wait for User Input.
*   **Flow Control**: Support conditional logic ("If you do, then...") and loops ("For every...").

## Technical Approach
### 1. Effect Architecture
We will use a **Trait/Enum Dispatch** approach.
*   `DogmaManager`: A registry that maps `card_name` to an implementation function.
*   `GameAction` (Extended): We may need intermediate states for user input.
    *   *Problem*: `GameState` apply_action is currently synchronous. User input breaks this.
    *   *Solution*: **Input Requests**. `execute_dogma` may return a `Result<State, Checkpoint>`. If input is needed, it pauses and stores a `pending_action` in `GameState`.

### 2. Input Handling (The "Pending Action" State)
*   Add `pending_input: Option<InputRequest>` to `GameState`.
*   `InputRequest` Enum: `SelectCard(Hand/Board)`, `ChooseColor`, `Confirm`.
*   When a dogma needs input, it sets `pending_input` and returns.
*   User sends `Action::ResolveInput(choice)`.
*   Game resumes execution.

### 3. Primitives
Implement `GameState` methods:
*   `tuck(pid, card_id)`
*   `return_card(pid, card_id)` (Return to deck)
*   `recycle(pid, card_id)` (Return to bottom of deck)
*   `splay(pid, color, direction)`
*   `exchange_hand_score(pid)`
*   etc.

## User Stories
*   **US2.1 - Effect Primitives**: I can perform atomic game operations (Tuck, Score, Splay) via the `GameState` API.
*   **US2.2 - Input Request System**: The engine can pause to ask the user for a choice and resume correctly.
*   **US2.3 - Age 1-3 Effects**: Implement logic for early ages.
*   **US2.4 - Age 4-10 Effects**: Implement logic for later ages.

## Success Criteria
*   Simulation can play a game where cards actually do things (not just no-ops).
*   CLI `simulate` generates actual board states with splayed piles and scores.
*   Unit tests for complex cards (e.g., `Outils` logic).
