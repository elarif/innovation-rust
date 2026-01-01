
# PROMPT UTILISATEUR / DOCUMENT DE SPÉCIFICATION (Rust Edition)

**Rôle :** Rust Systems Architect & Game Engine Developer.
**Objectif :** Implémenter le moteur de règles "Innovation" en Rust pur, en maximisant la sécurité du typage et la performance.
**Crate attendue :** `innovation_core` (librairie) + `innovation_cli` (exécutable de test).
**Format de sortie :** Code Rust idiomatique, Tests `cargo test`, et Documentation `rustdoc`.

---

# Innovation Board Game - Rust Core Engine Specification (`spec.kit`)

## 1. Context & Architecture

Le moteur doit être une machine à états finis (FSM) déterministe.

* **Approche :** Functional Core, Imperative Shell.
* **Architecture :** `fn apply(state: GameState, action: Action) -> Result<GameState, GameError>`.
* **Dependencies :** `serde` (JSON parsing), `thiserror` (Error handling).

## 2. Data Structures (Type System)

Utiliser le système de types pour rendre les états invalides impossibles ("Make invalid states unrepresentable").

### 2.1. Primitives & Enums

* **Enums (Coproducts) :**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color { Red, Blue, Green, Yellow, Purple }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Icon { Crown, Leaf, Factory, Lightbulb, Clock, Castle }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SplayDirection { #[default] None, Left, Right, Up }

pub type Age = u8; // 1..=10
pub type CardId = String; // Identifiant unique du JSON

```

---

# ADDENDUM : GESTION DES ICÔNES D'ÂGE (HEXAGONES)

### 2.1. Primitives & Enums (Mise à jour)

Le type `Icon` ne peut pas être un simple Enum de chaînes. Il doit pouvoir désérialiser soit un entier (l'Âge), soit une chaîne (le Symbole). En Rust avec `serde`, l'attribut `#[serde(untagged)]` est parfait pour cela.

```rust
use serde::{Deserialize, Serialize};

/// Représente les symboles standard utilisés par les Dogmes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")] // Pour matcher "CASTLE" du JSON
pub enum Symbol {
    Crown,
    Leaf,
    Factory,
    Lightbulb,
    Clock,
    Castle,
}

/// Représente un slot d'icône sur une carte physique.
/// Peut être un symbole actif ou l'indicateur d'âge (hexagone).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)] // Permet de parser soit un entier (Age), soit une string (Symbol)
pub enum Icon {
    /// L'icône hexagonale indiquant la période (ex: 1, 2... 10)
    Age(u8),
    /// Un symbole de ressource classique
    Resource(Symbol),
}

// Mise à jour de la structure Card pour refléter le parsing
#[derive(Debug, Deserialize)]
pub struct CardData {
    pub name: String,
    pub age: u8,
    pub color: Color,
    pub icons: [Icon; 4], // Le tableau peut contenir des Resource::Castle ou Age(1)
    pub dogmas: Vec<DogmaEffect>,
}

```

### Impact sur la logique de comptage (Impl Blocks)

Lors du comptage des icônes pour résoudre un Dogme, nous cherchons spécifiquement des `Symbol`. Les icônes de type `Icon::Age(_)` ne doivent pas être comptabilisées comme des ressources, mais elles "occupent" bien un index qui peut masquer ou être masqué par le Splay.

**Mise à jour de l'algorithme :**

```rust
impl PlayerState {
    /// Compte le nombre d'occurrences d'un symbole spécifique visible
    pub fn count_symbol(&self, target: Symbol) -> usize {
        self.board.values()
            .map(|pile| pile.count_visible_symbol(target))
            .sum()
    }
}

impl Pile {
    pub fn count_visible_symbol(&self, target: Symbol) -> usize {
        let mut count = 0;
        
        for (index, card_id) in self.cards.iter().enumerate() {
            let card = CARD_REGISTRY.get(card_id).unwrap();
            
            // Déterminer quels slots sont visibles selon le Splay et la position
            let visible_indices = self.get_visible_indices(index, self.cards.len());
            
            for icon_idx in visible_indices {
                // On ne compte QUE si c'est un symbole et qu'il matche la cible
                if let Icon::Resource(s) = card.icons[icon_idx] {
                    if s == target {
                        count += 1;
                    }
                }
                // Si c'est Icon::Age(_), on l'ignore pour le comptage de ressources
            }
        }
        count
    }
}

```

### Checklist de Test Mise à Jour

* [ ] **JSON Parsing Polymorphique :** Vérifier que la carte "Élevage" (Age 1) est parsée correctement avec `icons: [Resource(Castle), Resource(Crown), Age(1), Resource(Castle)]`.
* [ ] **Splay Blocking :** Vérifier un scénario où un décalage révèle l'icône `Age(1)`. Le moteur doit confirmer que cela **n'augmente pas** le compte de couronnes/châteaux (c'est un slot "mort" pour les ressources, mais visible).


### 2.2. Card Representation

Séparer les données statiques (Règles) de l'instance de la carte.

* `CardData` (Static) : Chargé depuis `cards.json`, stocké dans un `lazy_static` ou un `CardRegistry` immuable (Arc). Contient le texte et les `DogmaEffect`.
* `Card` (Instance) : Une simple référence ou un ID wrapper pour manipulation dans les vecteurs.

### 2.3. Player State

* **Struct :**
```rust
pub struct PlayerState {
    pub hand: Vec<CardId>,
    pub score_pile: Vec<CardId>,
    pub board: HashMap<Color, Pile>, // Pile contient les cartes et le Splay
    pub achievements: Vec<AchievementId>,
    pub reference_card: bool, // Pour départager les égalités alphabétiques
}

pub struct Pile {
    pub cards: Vec<CardId>, // Index 0 = Top/Active
    pub splay: SplayDirection,
}

```



### 2.4. Global Game State

* **Ownership :** Le `GameState` possède toutes les ressources.
* **Struct :**
```rust
pub struct GameState {
    pub decks: [Vec<CardId>; 10], // Decks 1 à 10
    pub available_achievements: HashSet<AchievementId>,
    pub players: Vec<PlayerState>,
    pub active_player_index: usize,
    pub actions_remaining: u8,
    pub execution_stack: Vec<PendingEffect>, // Pour gérer la récursion et les méta-effets
    pub winner: Option<usize>,
}

```



## 3. Logic & Algorithms (Impl Blocks)

### 3.1. Icon Counting (Iterator Pattern)

Utiliser les itérateurs Rust pour une évaluation paresseuse et efficace.

* **Function :** `fn count_icons(&self, player_idx: usize, icon: Icon) -> usize`
* **Logic :**
* Iterer sur `self.players[idx].board.values()`.
* Pour chaque `Pile`, `match pile.splay` :
* `None` -> compter uniquement `pile.cards[0]`.
* `Left/Right/Up` -> Utiliser un `match` sur l'index de la carte dans la pile pour savoir combien d'icônes elle expose.


* Utiliser `.fold(0, |acc, count| acc + count)`.



### 3.2. Dogma Resolution (Pattern Matching)

* **Action Enum :**
```rust
pub enum Action {
    Draw,
    Meld(CardId),
    ActivateDogma(CardId),
    Achieve(AchievementId),
    // Réponses aux demandes interactives
    SelectCards(Vec<CardId>), 
    SelectOpponent(usize),
    Confim(bool),
}

```


* **Resolution Flow :**
1. `match action` -> Valider la légalité (ex: `Meld` vérifie si `CardId` est dans `hand`).
2. Si `ActivateDogma` : Pousser les étapes de résolution dans `GameState.execution_stack`.
3. Le moteur dépile (`pop`) la prochaine `PendingEffect`.
* Utiliser un `match` exhaustif sur le type de Dogme.
* Calculer les ensembles `threatened_players` (Suprématie) et `eligible_players` (Coopératif) en utilisant des closures `|p| p.count_icons(...)`.





### 3.3. Atomic Verbs (Mutable Methods)

Implémenter comme méthodes de `GameState` qui mutent `self`.

* `fn draw(&mut self, player_idx: usize, age: Age) -> Option<CardId>` : Gère la logique de pile vide (Age N+1) et la condition de fin de partie (Age 11).
* `fn meld(&mut self, player_idx: usize, card: CardId)` : Gère l'insertion en tête de `Pile` (index 0).
* `fn tuck(&mut self, player_idx: usize, card: CardId)` : Gère l'insertion en queue de `Pile` (`vec.push`).
* `fn splay(&mut self, player_idx: usize, color: Color, dir: SplayDirection)` : Modifie le champ `splay` de la `Pile`.

## 4. Specific Rust Implementation Details

### 4.1. Handling "Effects" (Deserialization)

Ne pas parser le texte naturel. Mapper les IDs du JSON vers un Enum Rust puissant.

* **Enum :**
```rust
#[derive(Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum Effect {
    Draw { count: u8, age_offset: u8 },
    Transfer { source: Location, dest: Location, filter: Filter },
    Conditional { condition: Condition, success: Box<Effect> },
    Repeat { times: u8, effect: Box<Effect> },
    // ...
}

```


* Ceci permet d'utiliser `serde_json` pour charger `cards.json` directement dans des structures exécutables.

### 4.2. Action History & Context

Pour gérer les conditions comme "Si vous avez fait X ce tour-ci" (Age 6, Démocratie), utiliser un champ `turn_context` dans `GameState` :

```rust
pub struct TurnContext {
    pub cards_drawn: u8,
    pub cards_returned: HashMap<usize, u8>, // Map<PlayerIdx, Count>
    // ...
}

```

Ce contexte est reset au début de chaque tour (`start_turn`).

### 4.3. Error Handling

Utiliser `thiserror` pour des erreurs typées.

```rust
#[derive(Error, Debug)]
pub enum GameError {
    #[error("Pile for age {0} is empty and no higher age available")]
    EmptyDecks(Age),
    #[error("Player does not have card {0} in hand")]
    CardNotFound(CardId),
    #[error("Invalid splay direction {0:?} for color {1:?}")]
    InvalidSplay(SplayDirection, Color),
}

```

## 5. Implementation Plan

### Phase 1: Core Types & Serialization

1. Définir `lib.rs` avec les Enums et Structs de base.
2. Implémenter `serde::Deserialize` pour mapper le JSON `cards.json` vers le `CardRegistry`.
3. Test : Charger toutes les 105 cartes et vérifier que les `DogmaEffect` sont bien parsés.

### Phase 2: State Mutation Engine

1. Implémenter `GameState::new()`.
2. Implémenter les verbes atomiques (`draw`, `meld`, `tuck`, `score`) avec des tests unitaires pour chaque méthode.
3. Implémenter l'algorithme `count_icons` et le tester avec différentes configurations de `Splay`.

### Phase 3: The Interaction Loop

1. Implémenter la machine à états pour `Apply(Action)`.
2. Gérer la `execution_stack` pour les Dogmes complexes.
* Si un dogme nécessite un choix utilisateur (ex: "Recyclez une carte"), l'état doit passer à `GameState::WaitingForInput(PlayerIdx, InputRequest)`.
* L'action suivante DOIT être la réponse à cette requête.



### Phase 4: Rules & Win Conditions

1. Implémenter la logique de Domination (`score >= 5 * age`).
2. Implémenter les Dogmes de victoire immédiate (`win_game`).

## 6. Testing & Checklists

### 6.1. Unit Tests (`#[test]`)

* [ ] **Icon Calculation :** Créer un board mocké avec Splay Left et vérifier le compte exact.
* [ ] **Deck Overflow :** `draw(10)` sur un deck vide doit déclencher `GameState.winner`.
* [ ] **Ownership Transfer :** `meld` doit retirer la carte de `hand` et l'ajouter à `board`.

### 6.2. Integration Tests (Scenario)

Utiliser des tests de scénarios complets :

* [ ] **Scenario "Masonry" (Age 1) :**
1. Setup: Main avec 4 cartes Château.
2. Action: Activate Masonry.
3. Assert: Toutes les cartes sont sur le board, l'achievement "Technologies" est dans `achievements`.


* [ ] **Scenario "Fission" (Age 9) :**
1. Setup: Activer Fission, piocher une carte Rouge.
2. Assert: Tous les vecteurs (`hand`, `board`, `score`) de tous les joueurs sont vides (`is_empty()`).



---

# INSTRUCTIONS POUR L'AGENT IA

1. **Safety First :** Utilise `clippy` pour garantir un code idiomatique. Évite `unwrap()` en production ; utilise propagation d'erreurs `?`.
2. **Immutability :** Préfère cloner l'état ou utiliser des références immuables pour les calculs (comptage icônes) et ne muter que lors de l'application de l'action.
3. **Data-Driven :** Le comportement des cartes doit être piloté par les données parsées depuis `cards.json`, pas par des `if/else` hardcodés pour chaque nom de carte (sauf exceptions très complexes comme "AI" ou "Bio-Engineering").
4. **Generics :** Si possible, utilise des Traits pour les Effets (`trait ExecutableEffect`) pour faciliter l'extension.
