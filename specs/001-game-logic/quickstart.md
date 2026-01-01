# Quickstart: Verifying Game Logic

## Prerequisites
- Rust (Cargo)

## Running Tests
Unit tests cover individual mechanics.
```bash
cargo test -p innovation-core
```

## Simulating a Game
Use the CLI to run a random simulation.
```bash
cargo run -p innovation-cli -- simulate --seed 12345
```

## Verifying Data
Ensure `cards.json` is valid before logic runs.
```bash
cargo run -p innovation-cli -- verify
```
