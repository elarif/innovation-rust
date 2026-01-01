# Feature Specification: Game Logic Implementation

**Feature Branch**: `001-game-logic`  
**Created**: 2025-12-29  
**Status**: Draft  
**Input**: User description: "game logic implementation"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Core Game Loops (Priority: P1)

As a developer, I want the core game engine to correctly handle the flow of a game from setup to dogmas, so that I can run a complete game simulation.

**Why this priority**: Fundamental to any gameplay. Without turns, actions, and phase management, nothing works.

**Independent Test**: Can be tested by simulating a game loop where players take simplified actions (Draw, Meld) and verifying state transitions.

**Acceptance Scenarios**:

1. **Given** a new game, **When** initialized, **Then** deck is shuffled, players draw hands, and first player is determined by age/card value.
2. **Given** a player's turn, **When** they choose 'Draw', **Then** a card is added to their hand and turn passes (or actions decrement).
3. **Given** a player's turn, **When** they choose 'Meld', **Then** the card moves from hand to board, overriding top card of that color.

---

### User Story 2 - Dogma Execution (Priority: P1)

As a player, I want to execute Dogma effects (Cooperative and Supremacy) so that I can interact with other players and the board state.

**Why this priority**: Innovation's primary mechanic is card effects.

**Independent Test**: Unit tests for specific dogma helper functions (demand, exchange, splay) and integration tests for a full dogma activation.

**Acceptance Scenarios**:

1. **Given** a dogma activation, **When** checking eligibility, **Then** players with strictly fewer/more icons are identified correctly.
2. **Given** a Cooperative dogma, **When** resolved, **Then** all eligible players execute the effect, and if anyone did, the activator gets a free Draw (if applicable rule).
3. **Given** a Supremacy dogma, **When** resolved, **Then** only the activator executes the effect against eligible victims.

---

### User Story 3 - Victory Conditions (Priority: P1)

As a player, I want the game to detect when I win via Achievements or Score, so that the game ends.

**Why this priority**: The game must have a termination condition.

**Independent Test**: Set up board states satisfying victory conditions and assert game end signal.

**Acceptance Scenarios**:

1. **Given** a player claims the required number of Achievements, **When** state is checked, **Then** game ends with that player as winner.
2. **Given** the deck runs out, **When** checking score, **Then** player with highest score wins.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST represent the full Game State (Players, Board, Hand, Score, Achievements, Deck).
- **FR-002**: System MUST implement the 4 core actions: Meld, Draw, Achieve, Dogma.
- **FR-003**: System MUST correctly calculate Icon counts for all players, including Splay directions.
- **FR-004**: System MUST handle "Sharing" bonuses (Draw after sharing in Coop dogma).
- **FR-005**: System MUST enforce Age progression rules (drawing from higher ages if empty).
- **FR-006**: System MUST serialize/deserialize game state for CLI saving/loading.

### Key Entities

- **GameState**: The root object containing all mutable state.
- **Player**: Holds Hand, Score pile, Achievements, and Board (Stacks).
- **Card**: Static data (from `cards.json`).
- **Action**: Enum of possible moves.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The engine passes 100% of the verification integration tests (simulated games).
- **SC-002**: `innovation-cli` can load a state, execute a list of actions, and produce the expected resulting hash/state.
- **SC-003**: All 105 cards can be "attempted" (even if logic is generic) without crashing the engine.
