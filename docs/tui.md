Voici la spécification de design pour l'interface TUI d'Innovation.

---

# Innovation TUI - Design & Architecture (Ratatui)

## 1. Concept Visuel (Mockup ASCII)

L'écran est divisé en blocs (`Layout`). Nous utiliserons les capacités de couleur du terminal pour distinguer les 5 couleurs du jeu (Rouge, Bleu, Vert, Jaune, Magenta pour Violet).

```text
+------------------------------------------------------------------------------+
| INNOVATION TUI v0.1 | Tour: 4 | Joueur Actif: TOI                            |
+------------------------------------------------------------------------------+
| [ADVERSAIRE]  Main: 4 | Score: 12 (3 cartes) | 🏰2 🌿3 🏭0 💡4 🕰️1 👑0     |
|                                                                              |
|  [R] Archery     [B] Tools       [G] ---         [Y] Agri.       [P] ---     |
|  Age: 1          Age: 1          (Vide)          Age: 1          (Vide)      |
|  Icon: 🏰🏰      Icon: 💡💡                      Icon: 🌿🌿                  |
|                                                                              |
+------------------------------------------------------------------------------+
| [CENTRE]                                                                     |
| Pioches: [1] [2] [3] [4] [5] [6] [7] [8] [9] [10]  |  Dominations dispo: 4   |
| Actions: [>>> LOG: J2 a pioché une carte Age 2...                        ]   |
+------------------------------------------------------------------------------+
| [TOI]         Score: 15 (4 cartes) | 🏰5 🌿4 🏭2 💡6 🕰️0 👑1  (ATT vs ADV) |
|                                                                              |
|  [ROUGE] ^^^     [BLEU] >>>      [VERT]          [JAUNE]         [VIOLET]    |
|  +-------------+ +-------------+ +-------------+ +-------------+ +---------+ |
|  | Oars (1)    | | Maths (2)   | | Sailing (1) | | ---         | | ---     | |
|  | 🏰 👑       | | 💡 💡       | | 👑 👑       | |             | |         | |
|  | ...         | | ...         | | ...         | |             | |         | |
|  | I Demand... | | May return..| | Draw 1...   | |             | |         | |
|  +-------------+ +-------------+ +-------------+ +-------------+ +---------+ |
|   Total: 🏰3      Total: 💡4      Total: 👑2                                 |
+------------------------------------------------------------------------------+
| [MAIN] (Utiliser <TAB> pour focus)                                           |
| > 1. Code of Laws (1) [P] 👑👑🌿 - "You may tuck..."                         |
|   2. Fermentation (2) [Y] 🌿🌿🌿 - "Draw a 2 for every..."                   |
|   3. Gunpowder (4)    [R] 🏭👑🏭 - "I Demand..."                             |
+------------------------------------------------------------------------------+
| [KEYBINDS] ?: Aide | q: Quitter | ESPACE: Activer | m: Meld | d: Draw        |
+------------------------------------------------------------------------------+

```

## 2. Structure du Layout (Ratatui)

Nous utiliserons `Layout::default().direction(Direction::Vertical)` pour découper l'écran principal.

1. **Header (3%) :** Titre et état global.
2. **OpponentView (20%) :** Une vue simplifiée. On n'a pas besoin de lire le texte de ses cartes, juste voir le Top, l'Age et ses Icônes totaux.
3. **MidSection (10%) :**
* Split Horizontal : Pioches (Decks) à gauche, Log à droite.


4. **PlayerBoard (40%) :** La zone la plus importante.
* Split Horizontal en 5 colonnes (`Constraint::Percentage(20)`) pour les 5 piles de couleur.


5. **HandView (20%) :** Liste défilante (`List` widget) des cartes en main.
6. **Footer (7%) :** Barre de statut et raccourcis.

## 3. Représentation des Composants Complexes

### 3.1. Les Piles et le Splay (Décalage)

En TUI, on ne peut pas dessiner des cartes qui se chevauchent pixel par pixel.
**Solution :** Utiliser le `Block` title et des symboles pour indiquer le décalage.

* **Splay Left :** Afficher `<<<` au dessus de la pile.
* **Splay Right :** Afficher `>>>`.
* **Splay Up :** Afficher `^^^`.
* **Contenu du bloc :**
1. Ligne 1 : Nom de la carte Active + (Age).
2. Ligne 2 : Les icônes de la carte Active.
3. Ligne 3-5 : Résumé textuel du Dogme (tronqué).
4. **Bas du bloc :** Le total des icônes de cette pile (calculé par le moteur), ex: `Total: 🏰4`. C'est l'info cruciale pour le joueur.



### 3.2. Les Couleurs

Utiliser `ratatui::style::Color`.

* Cartes Rouges : `Color::Red` (ou `LightRed` pour le texte).
* Cartes Bleues : `Color::Blue`.
* Cartes Vertes : `Color::Green`.
* Cartes Jaunes : `Color::Yellow`.
* Cartes Violettes : `Color::Magenta`.

### 3.3. Les Icônes

Utiliser des caractères Unicode si le terminal le supporte, sinon du fallback ASCII.

* Crown: `👑` ou `(K)`
* Leaf: `🌿` ou `(L)`
* Factory: `🏭` ou `(F)`
* Bulb: `💡` ou `(B)`
* Clock: `🕰️` ou `(C)`
* Castle: `🏰` ou `(T)` (Tower)

## 4. Interaction & UX (Mode Clavier)

Pas de souris nécessaire. Navigation rapide de type "Vim" ou Flèches.

* **Focus Switch (`Tab`) :** Permet de basculer le curseur entre la **Zone de Jeu** (pour activer des dogmes) et la **Main** (pour poser des cartes).
* **Sélection (`←` / `→`) :**
* Dans la *Zone de Jeu* : Passe de la pile Rouge à la pile Bleue, etc. La pile sélectionnée a une bordure `Style::default().fg(Color::White).add_modifier(Modifier::BOLD)`.
* Dans la *Main* : Monte/Descend dans la liste.


* **Actions Contextuelles :**
* `Space` : Action par défaut.
* Sur une Pile : **Activer le Dogme** (Ouvre une popup de résolution si nécessaire).
* Sur une Carte en Main : **Meld** (Poser).


* `d` : **Draw** (Action globale, toujours dispo si c'est votre tour).
* `a` : **Achieve** (Ouvre la liste des dominations pour en choisir une).
* `i` : **Inspect** (Ouvre une popup modale avec le texte complet de la carte sélectionnée).



## 5. Implémentation Technique (Snippet Rust)

Voici à quoi ressemblerait la boucle de rendu principale avec `ratatui`.

```rust
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Terminal, Frame,
};

struct App {
    // État de l'UI
    selected_panel: Panel, // Board ou Hand
    selected_col_idx: usize, // 0..4 (Piles de couleur)
    selected_hand_idx: usize,
    // Référence au moteur de jeu (défini précédemment)
    game_state: GameState, 
}

enum Panel { Board, Hand }

fn ui(f: &mut Frame, app: &App) {
    // 1. Découpage vertical principal
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(10), // Opponent
            Constraint::Length(3),  // Mid (Decks)
            Constraint::Min(10),    // Player Board (Flexible)
            Constraint::Length(8),  // Hand
            Constraint::Length(1),  // Footer
        ].as_ref())
        .split(f.size());

    // --- RENDER HEADER ---
    let header = Paragraph::new("INNOVATION TUI")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // --- RENDER PLAYER BOARD (5 Colonnes) ---
    let pile_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), Constraint::Percentage(20),
            Constraint::Percentage(20), Constraint::Percentage(20), Constraint::Percentage(20),
        ].as_ref())
        .split(chunks[3]);

    let colors = [Color::Red, Color::Blue, Color::Green, Color::Yellow, Color::Magenta];
    
    // Boucle sur les 5 piles du joueur
    for i in 0..5 {
        let is_selected = app.selected_panel == Panel::Board && app.selected_col_idx == i;
        
        let border_style = if is_selected {
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(colors[i])
        };

        // Récupération simulée de la carte active
        let pile_text = format!("Top: CardName\nIcons: 🏰 🏰\nSplay: NONE");
        
        let pile_block = Paragraph::new(pile_text)
            .block(Block::default().title(format!(" Pile {} ", i)).borders(Borders::ALL).border_style(border_style));
            
        f.render_widget(pile_block, pile_chunks[i]);
    }

    // --- RENDER HAND ---
    let hand_block = Block::default().title(" Hand (TAB to focus) ").borders(Borders::ALL);
    let items = vec![
        ListItem::new("1. Agriculture (1) - Y"),
        ListItem::new("2. Tools (1) - B"),
    ];
    let list = List::new(items)
        .block(hand_block)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    
    // Si le panneau Hand est actif, on passe l'état de sélection au widget
    let mut list_state = ratatui::widgets::ListState::default();
    if app.selected_panel == Panel::Hand {
        list_state.select(Some(app.selected_hand_idx));
    }
    
    f.render_stateful_widget(list, chunks[4], &mut list_state);
}

```

## 6. Avantages de cette approche

1. **Densité :** On peut afficher beaucoup plus d'infos au cm² qu'une interface web aérée.
2. **Vitesse :** Une fois les raccourcis mémorisés (`d` pour Draw, `m` pour Meld), le jeu est ultra fluide.
3. **Portabilité :** Fonctionne via SSH, sur des petites machines, etc.

C'est une interface idéale pour tester le moteur de jeu rapidement avant de construire une GUI complexe.