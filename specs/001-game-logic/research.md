# Research: Game Logic Implementation

**Feature**: Game Logic (001-game-logic)
**Created**: 2025-12-29

## Research Items

### 1. Seeding RNG (StdRng)
**Decision**: Use `rand::rngs::StdRng` with `SeedableRng`.
**Rationale**: Determinism is crucial for replayability and debugging. Tests must be reproducible.
**Alternatives**: `ThreadRng` (non-deterministic), `SmallRng` (faster but less robust, though likely fine for a card game). `StdRng` is standard and sufficiently fast.

### 2. Serde for Enums
**Decision**: Use `#[derive(Serialize, Deserialize)]` on all Core structs.
**Rationale**: `serde_json` is the de facto standard. We need strict typing.
**Notes**: For `DogmaEffect`, we may need custom deserialization if the JSON structure is varied, but current `cards.json` schema is regular enough for automatic derivation.

## Resolved Clarifications
- None required. Tech stack is standard for Rust ecosystem.
