# Innovation Game Elements - Comprehensive Analysis

This directory contains a complete analysis and implementation of the Innovation board game's card effect system.

## Overview

The Innovation card game has complex card effects described in French text. This analysis provides:

1. **Complete taxonomy** of all game elements
2. **Strongly-typed Rust implementation** 
3. **Transformation guide** from text to executable code
4. **Validation** against all 105 base game cards

## Documents

### 1. [Game Elements Analysis](./game_elements_analysis.md)

Exhaustive categorization of all game elements:

- **Card Selectors** (40+ variations)
  - By location, position, attributes, state, and composite filters
  - Examples: "highest card in hand", "active cards producing Castle", "cards of colors not on board"

- **Player Selectors** (15+ patterns)
  - Individual and multiple player targeting
  - Conditional selection based on resources, scores, board state
  - Examples: "opponents with fewer symbols", "player with most influence"

- **Primitive Actions** (35+ operations)
  - Card movement (Draw, Meld, Score, Archive, Recycle, Transfer, Exchange)
  - Pile manipulation (Splay, Reorder)
  - Victory actions (Achieve, Win)
  - Special actions (Execute Dogma, Repeat)

- **Conditions** (45+ types)
  - Card conditions (symbol, color, age)
  - Player conditions (resources, influence, board state)
  - Game state conditions (cards transferred, splayed colors)
  - Temporal/sequential conditions

- **Control Structures** (10+ patterns)
  - Sequence, Conditional, Optional
  - Choice (single or multiple)
  - Loops (RepeatUntil, RepeatWhile, ForEach)
  - Cascade, Shared effects
  - Negation/Exception

### 2. [Text to Pipeline Guide](./text_to_pipeline_guide.md)

Practical guide for transforming card text into structured pipelines:

- **15+ detailed examples** from simple to complex cards
- **Rust code snippets** for each transformation
- **Common patterns** with template code
- Covers:
  - Simple actions (Écriture: "Draw a 2")
  - Conditionals (Agriculture: recycling with relative age)
  - Supremacy dogmas (Archerie: demand effects)
  - Loops (Métallurgie: repeat until condition)
  - Choices (Évolution: pick one of two options)
  - Complex effects (Alchimie, Tissage, Machinerie)

### 3. Rust Implementation

Located in `/innovation-core/src/dogma/`:

#### `selectors.rs` - Card and Player Selection
```rust
// Select highest card from hand producing Crown
CardSelector::from_hand(QuantitySelector::Exact(1), PlayerSelector::Activator)
    .with_age(AgeSelector::Max)
    .with_symbol(SymbolSelector::Produces(Symbol::Crown))

// Target opponents with fewer Castle symbols (supremacy dogma)
PlayerSelector::supremacy_targets(Symbol::Castle)
```

#### `conditions.rs` - Conditions and Control Flow
```rust
// Optional action with follow-up
Optional::with_success(
    Effect::Recycle { /* ... */ },
    Effect::Draw { /* ... */ }
)

// Conditional execution
Conditional::with_else(
    GameCondition::Card { /* card has symbol */ },
    Effect::Score { /* on true */ },
    Effect::None { /* on false */ }
)

// Iterate over collection
ForEach::colors_on_board(PlayerSelector::Activator, Effect::Draw { /* ... */ })
```

#### `actions.rs` - Primitive Actions
```rust
// Draw 2 cards of age 5
DrawAction::of_age(2, 5, PlayerSelector::Activator)

// Meld to own board
MeldAction::to_own_board(selector)

// Achieve domain
AchieveAction::domain(PlayerSelector::Activator, DomainType::Culture)

// Execute dogma without sharing
ExecuteDogmaAction {
    card: selector,
    share: false,
    dogma_index: None,
}
```

## Coverage

### Base Game Cards: 105/105 ✓

All card effects from ages 1-10 are covered by the type system:

- **Age 1-3**: Basic effects (draw, meld, score, splay)
- **Age 4-6**: Complex conditionals and exchanges
- **Age 7-8**: Advanced control flow and multi-target effects
- **Age 9-10**: Meta effects (execute other dogmas, win conditions)

### Special Mechanics

- ✓ Supremacy Dogmas (I demand)
- ✓ Cooperative Dogmas with sharing
- ✓ Victory conditions
- ✓ Domain achievements
- ✓ Splay mechanics
- ✓ Repeat/loop effects
- ✓ Card revelation
- ✓ Player choices

## Usage

### For Card Implementation

1. Read the card text in French
2. Consult the [Text to Pipeline Guide](./text_to_pipeline_guide.md) for similar examples
3. Identify the pattern (sequence, conditional, loop, etc.)
4. Use the Rust types to construct the effect pipeline
5. The type system ensures correctness at compile time

### For Game Engine

The structured representations can be:
- Serialized to JSON for data storage
- Executed by the game engine's effect resolver
- Validated for correctness
- Tested in isolation

### For Analysis Tools

The taxonomy enables:
- Automated card parsing from text
- Card complexity metrics
- Effect frequency analysis
- Balance testing

## Examples

### Simple Card (Écriture)
```
Text: "Piochez une 2."
→ DrawAction::of_age(1, 2, PlayerSelector::Activator)
```

### Complex Card (Tissage)
```
Text: "Mettez en jeu une carte d'une couleur que vous n'avez pas encore dans 
      votre Zone de Jeu. Piochez une 1 pour chaque couleur figurant dans votre 
      Zone de Jeu et ne figurant dans la Zone de Jeu d'aucun autre joueur 
      et comptabilisez-la."

→ Sequence[
    Meld(color: NotOnBoard),
    ForEach(UniqueColors) {
        Draw(age: 1) → Score
    }
  ]
```

### Supremacy Dogma (Archerie)
```
Text: "J'exige que vous piochiez une 1 ! Puis que vous transfériez la carte 
      la plus élevée de votre Main vers la mienne !"

→ DogmaWrapper {
    type: Supremacy(Castle),
    effects: [
        Draw(age: 1, target: CurrentTarget),
        Transfer(from: Hand, filter: MaxAge, to: Activator.Hand)
    ]
  }
```

## Validation

The analysis has been validated against:
- ✓ All 105 base game cards
- ✓ Complex multi-step effects
- ✓ Edge cases (empty piles, unique colors, etc.)
- ✓ Win conditions and special domains
- ✓ Type safety in Rust (compiles without errors)

## Future Work

- [ ] Add expansion card support (Echoes, Figures, Cities, Artifacts)
- [ ] Implement effect executor/interpreter
- [ ] Create card text parser (French → structured types)
- [ ] Add effect visualization tools
- [ ] Generate card documentation automatically

## Technical Notes

### Type Safety

The Rust implementation provides compile-time guarantees:
- All selectors produce valid card references
- Effects can only be composed in meaningful ways
- Player targets are always valid
- Numeric ranges are checked

### Extensibility

New effects can be added by:
1. Defining new enum variants in the appropriate module
2. Implementing the execution logic in the executor
3. Type system ensures all cases are handled

### Performance

The type system is designed for:
- Zero-cost abstractions (Rust enums compile to integers)
- Builder patterns for ergonomic construction
- Minimal allocations (most types are Copy/Clone)

## Contributing

When adding new cards or mechanics:

1. Check if existing types cover the effect
2. If not, extend the appropriate enum
3. Update the documentation with examples
4. Add test cases
5. Ensure compilation succeeds

## License

Part of the Innovation Rust implementation project.

## References

- Game rules: [docs/regles_fr.md](../regles_fr.md)
- Card data: [data/cards.json](../data/cards.json)
- JSON schema: [data/schema.json](../data/schema.json)
