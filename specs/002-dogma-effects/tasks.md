---
description: "Tasks for Feature 002: Dogma Effects"
---

# Tasks: Feature 002 - Dogma Effects

## Phase 1: Core Architecture (State Machine)
- [x] T027 [P] Define `InputRequest` and `DogmaExecutionState` structs in `executor.rs` or `game_state.rs`
- [x] T028 [P] Update `GameState` to include `pending_input` and `dogma_state` fields
- [x] T029 Implement `Action::ResolveInput` loop in `game_state.rs`
- [x] T030 Refactor `execute_dogma` to initialize `DogmaExecutionState` instead of running immediately

## Phase 2: Primitive Operations
- [x] T031 Implement `tuck_card(pid, card_id)` in `GameState`
- [x] T032 Implement `return_card(pid, card_id)` and `recycle_card` in `GameState`
- [x] T033 Implement `score_card(pid, card_id)` helper
- [x] T034 Implement `exchange_hand_score(pid)` and other common utilities
- [x] T035 Implement `draw_age(pid, age)` helper with proper return value handling

## Phase 3: Card Implementation (Age 1-3)
- [ ] T036 Implement Age 1 Cards (15 cards)
- [ ] T037 Implement Age 2 Cards (10 cards)
- [ ] T038 Implement Age 3 Cards (10 cards)
- [ ] T039 Add Unit Tests for `InputRequest` flow (verify pause/resume)

## Phase 4: Card Implementation (Age 4-10) API Checkpoint
- [ ] T040 Implement Age 4 Cards
- [ ] T041 Implement Age 5 Cards
- [ ] T042 Implement Age 6 Cards
- [ ] T043 Implement Age 7 Cards
- [ ] T044 Implement Age 8 Cards
- [ ] T045 Implement Age 9 Cards
- [ ] T046 Implement Age 10 Cards

## Phase 5: Verification
- [ ] T047 Update CLI `simulate` to handle `pending_input` with random choices
- [ ] T048 Verify full game loop with simulated effects
