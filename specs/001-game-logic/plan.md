# Implementation Plan: Game Logic Implementation

**Branch**: `001-game-logic` | **Date**: 2025-12-29 | **Spec**: [specs/001-game-logic/spec.md](specs/001-game-logic/spec.md)
**Input**: Feature specification from `/specs/001-game-logic/spec.md`

## Summary

Implement the core game engine for **Innovation** in Rust, enabling full simulation of gameplay loops (Setup, Actions, Dogmas, Victory). The implementation will be purely functional where possible, focusing on a mutable `GameState` that transitions via `Action` inputs, validated by unit and integration tests.

## Technical Context

**Language/Version**: Rust 2021+
**Primary Dependencies**: `serde`, `serde_json` (for serialization), `rand` (for shuffling), `thiserror` (for error handling).
**Storage**: JSON file (`data/cards.json`) for static data; Memory for runtime state.
**Testing**: `cargo test` (Unit), `innovation-cli` (Integration/Verification).
**Target Platform**: CLI / Library (OS Agnostic).
**Project Type**: Standalone Library (`innovation-core`) + CLI Tool.
**Performance Goals**: Instantaneous state transitions (<1ms).
**Constraints**: Must run entirely offline. Logic must be deterministic given a seed (except for initial shuffle, which accepts seed).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Data Integrity**: Uses valid `cards.json` schema.
- **Rust Core Authority**: Logic resides in `innovation-core`.
- **Verification First**: Plan includes unit tests and CLI verification.
- **Agentic Workflow**: Follows Spec-Kit layout.

**Status**: PASSED.

## Project Structure

### Documentation (this feature)

```text
specs/001-game-logic/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
innovation-core/src/
├── lib.rs              # Crate root
├── game_state.rs       # Main state struct
├── player.rs           # Player state
├── actions.rs          # Action Enums and handlers
├── dogma.rs            # Dogma execution engine
└── errors.rs           # Error types

innovation-cli/src/
├── main.rs             # CLI entry point
└── commands/           # CLI subcommands (verify, simulate)
```

**Structure Decision**: Standard Rust library structure. Separating logic (`core`) from interface (`cli`) ensures the engine can be reused (e.g. for a future web backend) without modification.

## Phases

### Phase 0: Outline & Research

1. **Serialization**: Confirm `serde` derivation for complex Enums (nested Dogma effects).
2. **RNG**: Determine easiest consistent seeding strategy for `rand` (likely `StdRng`).

### Phase 1: Design & Contracts

1. **Entities**: Define `GameState`, `Player`, `Card` (already somewhat done, verify completeness).
2. **API**: Define `Action` Enum (Draw, Meld, Achieve, Dogma).
3. **State Management**: Define `GameState::apply(Action) -> Result<(), Error>`.

### Phase 2: Implementation

1. **Core State**: Structs and simple helpers (deck, hand management).
2. **Actions**: Implement `Meld`, `Draw`, `Achieve`.
3. **Icons & Splaying**: Implement dynamic icon counting logic.
4. **Dogma**: Implement the complex execution flow (Demanding, Sharing, Effects).
5. **Victory**: Implement checks.
