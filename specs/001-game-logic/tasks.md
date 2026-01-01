---
description: "Task list for Game Logic Implementation"
---

# Tasks: Game Logic Implementation

**Input**: Design documents from `/specs/001-game-logic/`
**Prerequisites**: plan.md, spec.md, data-model.md
**Tests**: Included as required for verification.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Parallelizable
- **[Story]**: US1, US2, US3

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization.

- [x] T001 Add `serde`, `serde_json`, `rand` dependencies to `innovation-core/Cargo.toml`
- [x] T002 [P] Create `errors.rs` in `innovation-core/src/` defining `GameError` enum
- [x] T003 Create `game_state.rs` skeleton in `innovation-core/src/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data structures required by all logic.

- [x] T004 [P] Define `Card` and `Dogma` structs in `innovation-core/src/lib.rs` (or dedicated file) matching `cards.json`
- [x] T005 [P] Implement `Player` struct in `innovation-core/src/player.rs` (Hand, Score, Board, Achievements)
- [x] T006 [P] Implement `Stack` struct in `innovation-core/src/player.rs` with `Splay` logic
- [x] T007 Define `Action` Enum in `innovation-core/src/actions.rs`
- [x] T008 Update `GameState` in `innovation-core/src/game_state.rs` to hold `deck`, `players`, `achievements`

**Checkpoint**: Core data structures exist.

---

## Phase 3: User Story 1 - Core Game Loops (Priority: P1)

**Goal**: Setup game, turns, basic actions (Meld, Draw).

- [x] T009 [P] [US1] Implement `GameState::new(seed)` to load cards and setup initial state (deal hands) in `game_state.rs`
- [x] T010 [P] [US1] Implement `Action::Draw` handler in `game_state.rs` (handle age progression)
- [x] T011 [P] [US1] Implement `Action::Meld` handler in `game_state.rs`
- [x] T012 [P] [US1] Integrate `Action` handler into `GameState::apply_action` in `game_state.rs`
- [x] T013 [US1] Add unit tests for Setup, Draw, and Meld in `innovation-core/src/lib.rs` (or tests module)

**Checkpoint**: Can play a game without dogmas or winning.

---

## Phase 4: User Story 2 - Dogma Execution (Priority: P1)

**Goal**: Activation of Card Effects.

- [x] T014 [US2] Implement Icon counting logic in `player.rs` (ignoring splay for now, or full impl)
- [x] T015 [P] [US2] Implement `Dogma` execution engine scaffold in `innovation-core/src/dogma.rs`
- [x] T016 [US2] Implement "Demand" vs "Non-Demand" logic flow (identify eligible players) in `dogma.rs`
- [x] T017 [US2] Implement "Share Bonus" logic (Draw after sharing) in `dogma.rs`
- [x] T018 [US2] Implement `Action::Dogma` handler in `game_state.rs` linking to `dogma.rs`
- [x] T019 [US2] Add unit tests for Icon counting and basic Dogma flow in `dogma.rs`

**Checkpoint**: Dogmas execute.

---

## Phase 5: User Story 3 - Victory Conditions (Priority: P1)

**Goal**: Winning the game.

- [x] T020 [P] [US3] Implement `Action::Achieve` handler in `game_state.rs` checking eligibility
- [x] T021 [P] [US3] Implement `check_victory()` in `game_state.rs` (Score vs Deck runout, Achievements count)
- [x] T022 [US3] Expose victory state in `GameState` (Winner ID) in `game_state.rs`
- [x] T023 [US3] Add unit tests for Victory conditions

**Checkpoint**: Game has an end state.

---

## Phase 6: Polish & Integration

- [x] T024 [P] [Checks] Ensure `GameState` (and children) derive `Serialize`, `Deserialize`
- [x] T025 [P] [Checks] Implement `simulate` command in `innovation-cli` (Setup game, random moves, check victory)
- [x] T026 [Checks] Update `verify` command in `innovation-cli` to use `load_all_cards`
