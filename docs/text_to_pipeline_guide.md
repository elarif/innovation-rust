# Guide de Mapping : Texte vers Pipeline

Ce document illustre comment transformer les descriptions textuelles des cartes en pipelines structurés utilisant les types définis dans le système.

## Table des Matières

1. [Exemples de Cartes Simples](#exemples-de-cartes-simples)
2. [Exemples de Cartes Conditionnelles](#exemples-de-cartes-conditionnelles)
3. [Exemples de Dogmes de Suprématie](#exemples-de-dogmes-de-suprématie)
4. [Exemples de Répétitions](#exemples-de-répétitions)
5. [Exemples de Choix](#exemples-de-choix)
6. [Exemples Complexes](#exemples-complexes)

---

## Exemples de Cartes Simples

### Écriture (Âge 1)

**Texte:** "Piochez une 2."

**Analyse:**
- Action: `Draw`
- Quantité: 1
- Âge: 2
- Joueur cible: Soi-même

**Code Rust:**
```rust
use crate::dogma::actions::DrawAction;
use crate::dogma::selectors::PlayerSelector;

DrawAction::of_age(1, 2, PlayerSelector::Activator)
```

**Type structuré:**
```rust
Effect::Draw {
    action: DrawAction {
        count: 1,
        age: Some(2),
        target_player: PlayerSelector::Activator,
        reveal: false,
    }
}
```

---

### La Roue (Âge 1)

**Texte:** "Piochez deux 1."

**Code Rust:**
```rust
DrawAction::of_age(2, 1, PlayerSelector::Activator)
```

---

### Voiles (Âge 1)

**Texte:** "Piochez une 1 et mettez-la en jeu."

**Analyse:**
- Séquence de 2 actions
- Action 1: Draw (1 carte d'âge 1)
- Action 2: Meld (la carte piochée)

**Code Rust:**
```rust
use crate::dogma::actions::{DrawAction, MeldAction};
use crate::dogma::selectors::{CardSelector, QuantitySelector, CardFilter};
use crate::dogma::conditions::CardReference;

vec![
    Effect::Draw {
        action: DrawAction::of_age(1, 1, PlayerSelector::Activator)
    },
    Effect::Meld {
        action: MeldAction {
            selector: CardSelector {
                quantity: QuantitySelector::Exact(1),
                filter: CardFilter::default(),
                owner: PlayerSelector::Activator,
            },
            target_player: None,
        }
    }
]
```

---

## Exemples de Cartes Conditionnelles

### Agriculture (Âge 1)

**Texte:** "Vous pouvez recycler une carte de votre Main. Si vous le faites, piochez une carte d'une valeur supérieure de un à celle que vous avez recyclée et comptabilisez-la."

**Analyse:**
- Action optionnelle: `Recycle`
- Conditionnelle: Si recyclage réussi
- Séquence de 2 actions:
  - Draw (âge = carte recyclée + 1)
  - Score (la carte piochée)

**Code Rust:**
```rust
use crate::dogma::conditions::{Optional, GameCondition};

Optional::with_success(
    Effect::Recycle {
        action: RecycleAction {
            selector: CardSelector::from_hand(
                QuantitySelector::Exact(1),
                PlayerSelector::Activator
            ),
        }
    },
    Effect::Sequence(vec![
        Effect::Draw {
            action: DrawAction {
                count: 1,
                age: None, // Calculated as LastRecycled.age + 1
                target_player: PlayerSelector::Activator,
                reveal: false,
            }
        },
        Effect::Score {
            action: ScoreAction {
                selector: CardSelector {
                    quantity: QuantitySelector::Exact(1),
                    filter: CardFilter {
                        // Reference to LastDrawn card
                        ..Default::default()
                    },
                    owner: PlayerSelector::Activator,
                },
                target_player: None,
            }
        }
    ])
)
```

---

### Calendrier (Âge 2)

**Texte:** "Si vous avez plus de cartes dans votre Influence que dans votre Main, piochez deux 3."

**Analyse:**
- Conditionnelle: Comparer taille de l'Influence vs Main
- Action si vrai: Draw 2 cartes d'âge 3

**Code Rust:**
```rust
use crate::dogma::conditions::{Conditional, GameCondition};
use crate::dogma::selectors::Comparison;

Conditional::new(
    GameCondition::HandVsScore {
        player: PlayerSelector::Activator,
        hand_comparison: Comparison::LessThan,
    },
    Effect::Draw {
        action: DrawAction::of_age(2, 3, PlayerSelector::Activator)
    }
)
```

---

### Mysticisme (Âge 1)

**Texte:** "Piochez une 1 et montrez-la. Si elle est de la même couleur qu'une autre carte dans votre Zone de Jeu, mettez-la en jeu et piochez une 1. Sinon, ajoutez-la à votre Main."

**Analyse:**
- Draw and reveal
- Condition sur la carte piochée
- Deux branches: meld + draw OR add to hand

**Code Rust:**
```rust
use crate::dogma::conditions::{CardCondition, CardReference};

vec![
    Effect::Draw {
        action: DrawAction::of_age(1, 1, PlayerSelector::Activator).and_reveal()
    },
    Effect::Conditional {
        condition: GameCondition::Card {
            card: CardReference::LastDrawn,
            condition: CardCondition::ColorOnBoard(PlayerSelector::Activator),
        },
        on_true: Box::new(Effect::Sequence(vec![
            Effect::Meld {
                action: MeldAction::to_own_board(
                    // Reference to last drawn card
                )
            },
            Effect::Draw {
                action: DrawAction::of_age(1, 1, PlayerSelector::Activator)
            }
        ])),
        on_false: Some(Box::new(Effect::None)), // Card stays in hand by default
    }
]
```

---

## Exemples de Dogmes de Suprématie

### Archerie (Âge 1)

**Texte:** "J'exige que vous piochiez une 1 ! Puis que vous transfériez la carte la plus élevée de votre Main vers la mienne !"

**Analyse:**
- Dogme de Suprématie (symbole: Castle)
- Cible: Joueurs avec moins de [Castle]
- Séquence: Draw → Transfer

**Code Rust:**
```rust
use crate::dogma::conditions::{DogmaWrapper, DogmaType};
use crate::dogma::actions::TransferAction;
use crate::dogma::selectors::{AgeSelector, Position};

DogmaWrapper {
    dogma_type: DogmaType::Supremacy(Symbol::Castle),
    effects: vec![
        Effect::Draw {
            action: DrawAction::of_age(1, 1, PlayerSelector::CurrentTarget)
        },
        Effect::Transfer {
            action: TransferAction {
                selector: CardSelector::from_hand(
                    QuantitySelector::Exact(1),
                    PlayerSelector::CurrentTarget
                ).with_age(AgeSelector::Max),
                destination: Location::Hand,
                destination_player: PlayerSelector::Activator,
            }
        }
    ],
    activator_bonus: None,
}
```

---

### Rames (Âge 1)

**Texte:** "J'exige que vous transfériez une carte qui produit du [Crown] de votre Main vers mon Influence ! Si vous subissez ce Dogme, piochez une 1 ! Si aucune carte n'a été transférée suite au Dogme de Suprématie, piochez une 1."

**Analyse:**
- Dogme de Suprématie
- Effet sur victime: Transfer + bonus si subit
- Effet sur activateur: Draw si aucun transfert

**Code Rust:**
```rust
use crate::dogma::selectors::SymbolSelector;

DogmaWrapper {
    dogma_type: DogmaType::Supremacy(Symbol::Castle),
    effects: vec![
        Effect::Transfer {
            action: TransferAction {
                selector: CardSelector::from_hand(
                    QuantitySelector::Exact(1),
                    PlayerSelector::CurrentTarget
                ).with_symbol(SymbolSelector::Produces(Symbol::Crown)),
                destination: Location::Score,
                destination_player: PlayerSelector::Activator,
            }
        },
        // Bonus for victim
        Effect::Conditional {
            condition: GameCondition::CardsTransferred { min: 1, max: None },
            on_true: Box::new(Effect::Draw {
                action: DrawAction::of_age(1, 1, PlayerSelector::CurrentTarget)
            }),
            on_false: None,
        }
    ],
    activator_bonus: Some(Box::new(
        Effect::Conditional {
            condition: GameCondition::NoCardsTransferred,
            on_true: Box::new(Effect::Draw {
                action: DrawAction::of_age(1, 1, PlayerSelector::Activator)
            }),
            on_false: None,
        }
    )),
}
```

---

## Exemples de Répétitions

### Métallurgie (Âge 1)

**Texte:** "Piochez une 1 et montrez-la. Si elle produit du [Castle], comptabilisez-la et répétez ce Dogme. Sinon, ajoutez-la à votre Main."

**Analyse:**
- Boucle conditionnelle
- Continue tant que carte piochée a [Castle]
- Arrêt quand carte sans [Castle]

**Code Rust:**
```rust
use crate::dogma::conditions::RepeatWhile;

RepeatWhile {
    effect: Box::new(Effect::Sequence(vec![
        Effect::Draw {
            action: DrawAction::of_age(1, 1, PlayerSelector::Activator).and_reveal()
        },
        Effect::Conditional {
            condition: GameCondition::Card {
                card: CardReference::LastDrawn,
                condition: CardCondition::HasSymbol(Symbol::Castle),
            },
            on_true: Box::new(Effect::Score {
                action: ScoreAction::to_own_score(/* last drawn card */)
            }),
            on_false: Some(Box::new(Effect::None)), // Breaks loop
        }
    ])),
    while_condition: GameCondition::Card {
        card: CardReference::LastDrawn,
        condition: CardCondition::HasSymbol(Symbol::Castle),
    },
    max_iterations: Some(10), // Safety limit
}
```

---

### Colonialisme (Âge 4)

**Texte:** "Piochez une 3 et archivez-la. Si elle produit du [Bulb], répétez ce Dogme."

**Code Rust:**
```rust
RepeatWhile {
    effect: Box::new(Effect::Sequence(vec![
        Effect::Draw {
            action: DrawAction::of_age(1, 3, PlayerSelector::Activator)
        },
        Effect::Archive {
            action: ArchiveAction {
                selector: /* last drawn card */,
                target_color: None,
                target_player: None,
            }
        }
    ])),
    while_condition: GameCondition::Card {
        card: CardReference::LastDrawn,
        condition: CardCondition::HasSymbol(Symbol::Lightbulb),
    },
    max_iterations: Some(10),
}
```

---

## Exemples de Choix

### Évolution (Âge 7)

**Texte:** "Vous pouvez choisir soit de piocher une 8 et de la comptabiliser, puis de recycler une carte de votre Influence ; soit de piocher une carte d'une valeur supérieure de un à la carte la plus élevée de votre Influence."

**Analyse:**
- Choix entre 2 options
- Option 1: Draw+Score+Recycle
- Option 2: Draw (âge relatif)

**Code Rust:**
```rust
use crate::dogma::conditions::Choice;

Choice::pick_one(vec![
    // Option 1
    Effect::Sequence(vec![
        Effect::Draw {
            action: DrawAction::of_age(1, 8, PlayerSelector::Activator)
        },
        Effect::Score {
            action: ScoreAction::to_own_score(/* last drawn */)
        },
        Effect::Recycle {
            action: RecycleAction {
                selector: CardSelector::from_score(
                    QuantitySelector::Exact(1),
                    PlayerSelector::Activator
                ),
            }
        }
    ]),
    // Option 2
    Effect::Draw {
        action: DrawAction {
            count: 1,
            age: None, // Max score age + 1
            target_player: PlayerSelector::Activator,
            reveal: false,
        }
    }
])
```

---

### Scientisme (Âge 8)

**Texte:** "Choisissez deux couleurs, puis piochez une 9 et montrez-la. Si la carte est de l'une des deux couleurs choisies, mettez-la en jeu et vous pouvez décaler sa couleur en haut. Sinon, ajoutez-la à votre Main. Si vous produisez vingt [Bulb] ou plus, vous gagnez."

**Analyse:**
- Action: Choisir 2 couleurs
- Draw and reveal
- Conditionnel basé sur couleur choisie
- Condition de victoire finale

**Code Rust:**
```rust
use crate::dogma::actions::{ChooseAction, ChoiceType, WinAction};
use crate::dogma::selectors::PlayerCondition;

vec![
    Effect::Choose {
        action: ChooseAction {
            choice_type: ChoiceType::Colors(2),
            chooser: PlayerSelector::Activator,
            store_as: Some("chosen_colors".to_string()),
        }
    },
    Effect::Draw {
        action: DrawAction::of_age(1, 9, PlayerSelector::Activator).and_reveal()
    },
    Effect::Conditional {
        condition: GameCondition::Card {
            card: CardReference::LastDrawn,
            condition: CardCondition::ColorInSet("chosen_colors"), // pseudo-code
        },
        on_true: Box::new(Effect::Sequence(vec![
            Effect::Meld { /* ... */ },
            Effect::Optional {
                effect: Effect::Splay { /* ... */ }
            }
        ])),
        on_false: Some(Box::new(Effect::None)),
    },
    Effect::Conditional {
        condition: GameCondition::Player(
            PlayerCondition::ProducesAtLeast(Symbol::Lightbulb, 20)
        ),
        on_true: Box::new(Effect::Win {
            action: WinAction {
                winner: PlayerSelector::Activator,
                condition: Some("20+ Lightbulbs".to_string()),
            }
        }),
        on_false: None,
    }
]
```

---

## Exemples Complexes

### Tissage (Âge 1)

**Texte:** "Mettez en jeu une carte d'une couleur que vous n'avez pas encore dans votre Zone de Jeu. Piochez une 1 pour chaque couleur figurant dans votre Zone de Jeu et ne figurant dans la Zone de Jeu d'aucun autre joueur et comptabilisez-la."

**Analyse:**
- Meld avec filtre de couleur spécial
- ForEach sur couleurs uniques
- Pour chaque: Draw + Score

**Code Rust:**
```rust
use crate::dogma::conditions::{ForEach, Collection};
use crate::dogma::selectors::ColorSelector;

vec![
    Effect::Meld {
        action: MeldAction {
            selector: CardSelector::from_hand(
                QuantitySelector::Exact(1),
                PlayerSelector::Activator
            ).with_color(ColorSelector::NotOnBoard),
            target_player: None,
        }
    },
    Effect::ForEach {
        iteration: ForEach {
            collection: Collection::UniqueColors(PlayerSelector::Activator),
            effect: Box::new(Effect::Sequence(vec![
                Effect::Draw {
                    action: DrawAction::of_age(1, 1, PlayerSelector::Activator)
                },
                Effect::Score {
                    action: ScoreAction::to_own_score(/* last drawn */)
                }
            ])),
        }
    }
]
```

---

### Alchimie (Âge 3)

**Texte:** "Piochez une 4 pour chaque trois [Castle] que vous produisez et montrez-la. Si l'une des cartes piochées est rouge, recyclez-les toutes ainsi que toutes les cartes de votre Main. Sinon, ajoutez-les à votre Main. Mettez en jeu une carte de votre Main, puis comptabilisez une carte de votre Main."

**Analyse:**
- Draw multiple basé sur symboles
- Vérifier si au moins une rouge
- Conditionnel: recycle all OR keep
- Séquence finale: meld + score

**Code Rust:**
```rust
use crate::dogma::selectors::QuantitySelector;

vec![
    Effect::Draw {
        action: DrawAction {
            count: 0, // Calculated: player_symbols(Castle) / 3
            age: Some(4),
            target_player: PlayerSelector::Activator,
            reveal: true,
        }
    },
    Effect::Conditional {
        condition: GameCondition::Card {
            card: CardReference::LastDrawn, // Any of last drawn
            condition: CardCondition::HasColor(Color::Red),
        },
        on_true: Box::new(Effect::Sequence(vec![
            Effect::Recycle {
                selector: /* all drawn cards */
            },
            Effect::Recycle {
                selector: CardSelector::from_hand(
                    QuantitySelector::All,
                    PlayerSelector::Activator
                )
            }
        ])),
        on_false: Some(Box::new(Effect::None)), // Cards stay in hand
    },
    Effect::Meld {
        action: MeldAction::to_own_board(
            CardSelector::from_hand(QuantitySelector::Exact(1), PlayerSelector::Activator)
        )
    },
    Effect::Score {
        action: ScoreAction::to_own_score(
            CardSelector::from_hand(QuantitySelector::Exact(1), PlayerSelector::Activator)
        )
    }
]
```

---

### Machinerie (Âge 3)

**Texte:** "J'exige que vous échangiez toutes les cartes de votre Main contre toutes les cartes ayant la valeur la plus élevée de ma Main ! Comptabilisez une carte de votre Main qui produit du [Castle]. Vous pouvez décaler vos cartes rouges à gauche."

**Analyse:**
- Dogme de Suprématie
- Exchange entre deux ensembles de cartes
- Score avec filtre symbole
- Splay optionnel

**Code Rust:**
```rust
use crate::dogma::actions::{ExchangeAction, SplayAction};

DogmaWrapper {
    dogma_type: DogmaType::Supremacy(Symbol::Leaf),
    effects: vec![
        Effect::Exchange {
            action: ExchangeAction {
                set_a: CardSelector::from_hand(
                    QuantitySelector::All,
                    PlayerSelector::CurrentTarget
                ),
                set_b: CardSelector::from_hand(
                    QuantitySelector::All,
                    PlayerSelector::Activator
                ).with_age(AgeSelector::Max),
            }
        }
    ],
    activator_bonus: Some(Box::new(Effect::Sequence(vec![
        Effect::Score {
            action: ScoreAction::to_own_score(
                CardSelector::from_hand(
                    QuantitySelector::Exact(1),
                    PlayerSelector::Activator
                ).with_symbol(SymbolSelector::Produces(Symbol::Castle))
            )
        },
        Effect::Optional {
            effect: Effect::Splay {
                action: SplayAction::color(
                    PlayerSelector::Activator,
                    Color::Red,
                    SplayDirection::Left
                )
            }
        }
    ]))),
}
```

---

## Patterns de Transformation Communs

### Pattern 1: "Piochez N cartes d'âge X"
```
Texte: "Piochez deux 3"
→ DrawAction::of_age(2, 3, PlayerSelector::Activator)
```

### Pattern 2: "Vous pouvez [action]"
```
Texte: "Vous pouvez recycler une carte"
→ Optional::new(Effect::Recycle { ... })
```

### Pattern 3: "Si [condition], [action]"
```
Texte: "Si elle produit du [Castle], comptabilisez-la"
→ Conditional::new(
    GameCondition::Card { condition: HasSymbol(Castle) },
    Effect::Score { ... }
)
```

### Pattern 4: "J'exige que [action]"
```
Texte: "J'exige que vous transfériez..."
→ DogmaWrapper {
    dogma_type: DogmaType::Supremacy(symbol),
    effects: vec![Effect::Transfer { ... }]
}
```

### Pattern 5: "Pour chaque N [symbole], [action]"
```
Texte: "Piochez une 2 pour chaque deux [Leaf]"
→ DrawAction {
    count: 0, // Calculated: symbols(Leaf) / 2
    age: Some(2),
    ...
}
```

### Pattern 6: "Répétez ce Dogme"
```
Texte: "Si [condition], répétez ce Dogme"
→ RepeatWhile {
    effect: Box::new(/* current effect */),
    while_condition: /* condition */
}
```

### Pattern 7: "Choisissez X"
```
Texte: "Choisissez deux couleurs"
→ ChooseAction {
    choice_type: ChoiceType::Colors(2),
    ...
}
```

---

## Résumé

Ce guide démontre comment transformer systématiquement les descriptions textuelles en pipelines structurés. Les principes clés sont:

1. **Identifier le type d'effet principal** (Draw, Meld, Transfer, etc.)
2. **Analyser les sélecteurs** (qui, quoi, combien)
3. **Déterminer la structure de contrôle** (séquence, conditionnel, boucle, choix)
4. **Gérer les cas spéciaux** (dogmes de suprématie, répétitions, victoires)
5. **Composer les effets** en utilisant les types appropriés

Le système de types Rust garantit que les pipelines construits sont cohérents et peuvent être exécutés de manière déterministe par le moteur de jeu.
