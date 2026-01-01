# Analyse Exhaustive des Éléments du Jeu Innovation

Ce document fournit une liste exhaustive de tous les éléments nécessaires pour transformer la description textuelle d'une carte vers un pipeline de sélecteurs/actions/conditions.

## Table des Matières

1. [Sélecteurs de Cartes](#sélecteurs-de-cartes)
2. [Sélecteurs de Joueurs](#sélecteurs-de-joueurs)
3. [Actions Primitives](#actions-primitives)
4. [Conditions](#conditions)
5. [Structures de Contrôle](#structures-de-contrôle)

---

## 1. Sélecteurs de Cartes

Les sélecteurs de cartes permettent d'identifier et de cibler des cartes spécifiques dans différents contextes du jeu.

### 1.1 Sélection par Localisation

#### Sources de Cartes
- **Main** (`Hand`) - Les cartes dans la main d'un joueur
- **Zone de Jeu** (`Board`) - Les cartes posées devant un joueur
- **Influence** (`Score`) - Les cartes comptabilisées sous le plateau
- **Pile** (`Deck`) - Les piles de cartes à piocher (par période)
- **Archivées** (`Archive`) - Les cartes archivées sous une pile de couleur
- **Dominations** (`Achievements`) - Les cartes de période dominées

#### Positions dans une Localisation
- **Carte Active** - La carte visible sur le dessus d'une pile de couleur
- **Carte la plus haute** (`Top`) - Le dessus d'une pile
- **Carte la plus basse** (`Bottom`) - Le dessous d'une pile
- **Toutes les cartes** (`All`) - L'ensemble des cartes d'une localisation

### 1.2 Sélection par Attributs de Carte

#### Par Couleur
- **Rouge** (`Red`)
- **Jaune** (`Yellow`)
- **Vert** (`Green`)
- **Bleu** (`Blue`)
- **Mauve** (`Purple`)
- **Couleur spécifique** - Une couleur donnée
- **Couleurs présentes dans la Zone de Jeu** - Couleurs déjà jouées
- **Couleurs absentes de la Zone de Jeu** - Couleurs pas encore jouées
- **Couleur non-X** - Toutes les couleurs sauf X

#### Par Valeur (Période)
- **Valeur exacte** (1-10) - Carte d'une période spécifique
- **Valeur la plus élevée** (`MaxAge`) - La carte de plus haute période
- **Valeur la plus basse** (`MinAge`) - La carte de plus basse période
- **Valeur relative** - Supérieure/inférieure de N par rapport à une référence
  - Supérieure de 1
  - Supérieure de 2
  - Inférieure de N

#### Par Symbole/Icône
- **Château** (`Castle`)
- **Couronne** (`Crown`)
- **Feuille** (`Leaf`)
- **Ampoule** (`Lightbulb`)
- **Usine** (`Factory`)
- **Horloge** (`Clock`)
- **Produit du symbole X** - Carte qui a au moins une icône X visible

#### Par Nombre/Quantité
- **Une carte** - Exactement 1 carte
- **N cartes** - Un nombre exact de cartes
- **Jusqu'à N cartes** - Maximum N cartes (choix optionnel)
- **Au moins N cartes** - Minimum N cartes
- **Toutes les cartes** - L'intégralité sans limite
- **La moitié** (arrondie au supérieur/inférieur)
- **Pour chaque N symboles** - Nombre dérivé du comptage d'icônes

### 1.3 Sélection par État

#### État de Décalage
- **Décalée à gauche** (`SplayedLeft`)
- **Décalée à droite** (`SplayedRight`)
- **Décalée en haut** (`SplayedUp`)
- **Non décalée** (`NotSplayed`)

#### État Relationnel
- **Même couleur qu'une autre carte**
- **Couleur différente de toutes les autres**
- **Valeur différente des autres**
- **Carte révélée/montrée** - Carte dont on connaît les attributs

### 1.4 Sélection Composite

- **Cartes Actives qui produisent du X** - Intersection de "Active" et "Produit X"
- **Cartes non-X qui produisent du Y** - Couleur ≠ X ET Produit Y
- **Cartes de valeur X ou Y** - Union de plusieurs critères
- **Carte ayant la valeur la plus élevée de type X** - Max(Age) parmi un sous-ensemble

---

## 2. Sélecteurs de Joueurs

Les sélecteurs de joueurs permettent d'identifier qui est affecté par un effet.

### 2.1 Joueur Unique

- **Vous** (`Self`) - Le joueur qui active le dogme
- **Un adversaire** (`Opponent`) - Un joueur choisi parmi les adversaires
- **Le joueur avec le moins d'Influence** - Joueur ayant le score minimal
- **Le joueur avec le plus d'Influence** - Joueur ayant le score maximal
- **Le joueur avec le plus de symbole X**
- **Le joueur avec le moins de symbole X**
- **Un joueur qui a moins de points d'Influence que vous**

### 2.2 Joueurs Multiples

- **Tous les joueurs** (`All`) - Vous et tous les adversaires
- **Tous les adversaires** (`AllOpponents`) - Tous les joueurs sauf vous
- **Les joueurs qui ont autant ou plus de symbole X que vous** - Dogme Coopératif
- **Les joueurs qui ont moins de symbole X que vous** - Dogme de Suprématie (J'exige)
- **Aucun autre joueur** - Condition d'unicité

### 2.3 Conditions sur les Joueurs

- **Si vous produisez N symboles X ou plus**
- **Si vous êtes le seul joueur avec X couleurs en jeu**
- **Si aucun joueur ne produit plus de X que de Y**
- **Si un joueur produit moins de N symboles X**
- **Si vous avez N cartes X ou plus dans votre Zone de Jeu**

---

## 3. Actions Primitives

Les actions primitives sont les opérations de base qui modifient l'état du jeu.

### 3.1 Actions de Déplacement de Cartes

#### Piocher (`Draw`)
- **Piocher une carte de période N**
- **Piocher N cartes de période X**
- **Piocher jusqu'à ce qu'une condition soit remplie**
- **Piocher de la période la plus élevée disponible** (si pile vide)

#### Mettre en Jeu (`Meld`)
- **Mettre en jeu une carte de la Main**
- **Mettre en jeu une carte de l'Influence**
- **Mettre en jeu dans la Zone de Jeu du joueur**
- **Mettre en jeu dans la pile de sa couleur**

#### Comptabiliser (`Score`)
- **Comptabiliser une carte de la Main**
- **Comptabiliser une carte Active**
- **Comptabiliser une carte piochée**
- **Comptabiliser dans l'Influence du joueur**

#### Archiver (`Archive`)
- **Archiver une carte de la Main**
- **Archiver sous la pile de sa couleur**
- **Archiver au tout en bas de la pile**

#### Recycler (`Recycle`)
- **Recycler une carte de la Main**
- **Recycler une carte de l'Influence**
- **Recycler une carte Active**
- **Remettre sous la pioche de sa période**

#### Transférer (`Transfer`)
- **Transférer de votre Main vers la Main d'un adversaire**
- **Transférer de votre Main vers l'Influence d'un adversaire**
- **Transférer de votre Zone de Jeu vers celle d'un adversaire**
- **Transférer de votre Influence vers celle d'un adversaire**
- **Transférer de la Main d'un adversaire vers la vôtre**
- **Transférer de l'Influence d'un adversaire vers la vôtre**
- **Transférer de la Zone de Jeu d'un adversaire vers la vôtre**

#### Échanger (`Exchange`)
- **Échanger toutes les cartes de la Main avec toutes celles de l'Influence**
- **Échanger des cartes entre deux joueurs**
- **Échanger la carte la plus élevée de X contre la plus basse de Y**

#### Défausser (`Discard`)
- **Défausser toutes les cartes des Mains**
- **Défausser toutes les cartes des Zones de Jeu**
- **Défausser toutes les cartes des Influences**
- **Retirer définitivement du jeu**

### 3.2 Actions de Manipulation de Piles

#### Décaler (`Splay`)
- **Décaler une couleur à gauche** - Révèle les icônes de gauche
- **Décaler une couleur à droite** - Révèle les icônes de droite
- **Décaler une couleur en haut** - Révèle les icônes du haut
- **Décaler une couleur actuellement décalée X vers Y**

#### Changer l'Ordre (`Reorder`)
- **Changer l'ordre des cartes d'une pile de couleur**
- **Placer une carte spécifique au-dessus ou en-dessous**

### 3.3 Actions d'Information

#### Montrer/Révéler (`Reveal`)
- **Montrer une carte**
- **Révéler lors de la pioche**
- **Montrer aux autres joueurs**

### 3.4 Actions de Victoire

#### Dominer (`Achieve`)
- **Dominer une période** - Prendre une carte Période du centre
- **Dominer un Domaine** - Prendre une carte Domaine spécial
  - Domaine des TECHNOLOGIES
  - Domaine MILITAIRE
  - Domaine de la DIPLOMATIE
  - Domaine de la CULTURE
  - Domaine des SCIENCES

#### Gagner (`Win`)
- **Vous gagnez** - Victoire immédiate par effet de carte
- **Le joueur X gagne** - Un autre joueur gagne

### 3.5 Actions Spéciales

#### Exécuter un Dogme (`ExecuteDogma`)
- **Exécuter les Dogmes Coopératifs d'une carte sans les partager**
- **Répéter ce Dogme** - Exécuter à nouveau le même effet
- **Subir ce Dogme** - Être affecté par un Dogme de Suprématie
- **Subir à nouveau ce Dogme** - Répéter l'effet subi

---

## 4. Conditions

Les conditions déterminent si un effet doit être exécuté ou influencent le choix d'actions.

### 4.1 Conditions sur les Cartes

#### Attributs de Carte
- **Si la carte produit du symbole X**
- **Si la carte est de couleur X**
- **Si la carte est de période X**
- **Si la carte est rouge/verte/bleue/jaune/mauve**
- **Si la carte est de la même couleur qu'une autre**
- **Si la carte est d'une couleur non présente**

#### Nombre de Cartes
- **Si vous avez N cartes ou plus**
- **Si vous avez mis N cartes en jeu ou plus**
- **Si N cartes ont été transférées**
- **Si aucune carte n'a été transférée**
- **Si une carte ou plus a été transférée**
- **Si deux cartes ou plus sont de la même couleur**

### 4.2 Conditions sur les Joueurs

#### Ressources/Symboles
- **Si vous produisez N symboles X ou plus**
- **Si vous produisez moins de symboles X que de Y**
- **Si vous produisez quatre [Castle] ou plus**
- **Si vous produisez vingt [Bulb] ou plus**

#### Points d'Influence
- **Si vous avez plus de cartes dans votre Influence que dans votre Main**
- **Si vous avez moins de points d'Influence qu'un adversaire**
- **Si vous n'avez qu'une carte en Main**

#### Zone de Jeu
- **Si toutes vos cartes Actives produisent du X**
- **Si toutes vos cartes Actives non-mauves valent N ou plus**
- **Si vous êtes le seul joueur avec N couleurs en jeu**
- **Si vos N couleurs sont décalées**
- **Si vos cartes X sont décalées à gauche/droite/en haut**

#### Cartes Spécifiques
- **Si ROBOTIQUE et LOGICIEL sont toutes les deux Actives**
- **Si vous avez dix cartes vertes ou plus dans votre Zone de Jeu**

#### Conditions Relationnelles
- **Si aucun joueur ne produit plus de X que de Y**
- **Si un joueur produit moins de N symboles X**
- **Si vous êtes le joueur avec le plus de Dominations**

### 4.3 Conditions Temporelles/Séquentielles

- **Si vous le faites** - Condition de succès d'une action optionnelle
- **Si vous subissez ce Dogme** - Condition d'affectation par un Dogme de Suprématie
- **Si une carte a été transférée suite au Dogme**
- **Si vous en avez recyclé plus que tout autre joueur**

---

## 5. Structures de Contrôle

Les structures de contrôle orchestrent l'exécution des actions et conditions.

### 5.1 Séquence (`Sequence`)

Exécution linéaire d'une série d'actions.

**Syntaxe typique :**
- "Action1. Action2."
- "Action1, puis Action2."

**Exemples :**
- "Piochez une 1. Puis transférez la carte la plus élevée."
- "Mettez en jeu une carte, puis comptabilisez une carte."

### 5.2 Conditionnelle (`Conditional`)

Exécution d'une action basée sur une condition.

**Syntaxe typique :**
- "Si [condition], [action]."
- "Si [condition], [action]. Sinon, [action alternative]."

**Exemples :**
- "Si elle produit du [Castle], comptabilisez-la. Sinon, ajoutez-la à votre Main."
- "Si vous avez plus de cartes en Influence que en Main, piochez deux 3."

### 5.3 Choix Optionnel (`Optional`)

Le joueur peut choisir d'exécuter ou non une action.

**Syntaxe typique :**
- "Vous pouvez [action]."

**Exemples :**
- "Vous pouvez recycler une carte de votre Main."
- "Vous pouvez décaler vos cartes bleues à droite."

### 5.4 Choix Multiple (`Choice`)

Le joueur doit choisir parmi plusieurs options.

**Syntaxe typique :**
- "Choisissez soit [option1] ; soit [option2]."
- "Choisissez [élément]."

**Exemples :**
- "Vous pouvez choisir soit de piocher une 8 et de la comptabiliser ; soit de piocher une carte d'une valeur supérieure."
- "Choisissez deux couleurs."
- "Choisissez une valeur de carte."

### 5.5 Répétition (`Loop`)

Exécution répétée d'une action jusqu'à ce qu'une condition soit remplie.

#### Répétition Conditionnelle
**Syntaxe typique :**
- "Répétez ce Dogme."
- "Si [condition], répétez ce Dogme/cet effet."

**Exemples :**
- "Si elle produit du [Castle], comptabilisez-la et répétez ce Dogme."
- "Si elle produit du [Bulb], répétez ce Dogme."

#### Répétition Basée sur Comptage
**Syntaxe typique :**
- "Pour chaque N [symbole], [action]."
- "Piochez une carte pour chaque X."

**Exemples :**
- "Piochez une 2 pour chaque deux [Leaf] que vous produisez."
- "Piochez une 4 pour chaque trois [Castle] que vous produisez."
- "Piochez une 1 pour chaque couleur unique."

#### Répétition Jusqu'à Échec
**Syntaxe typique :**
- "Piochez jusqu'à ce que [condition ne soit plus vraie]."

**Exemples :**
- "Piochez une 1 et montrez-la. Si elle produit du [Castle], comptabilisez-la et répétez. Sinon, ajoutez-la à votre Main."

### 5.6 Itération sur Collection (`ForEach`)

Exécution d'une action pour chaque élément d'un ensemble.

**Syntaxe typique :**
- "Pour chaque [élément], [action]."
- "Piochez/Faites X pour chaque Y."

**Exemples :**
- "Piochez une 1 pour chaque couleur figurant dans votre Zone de Jeu."
- "Piochez une 4 pour chaque couleur décalée à gauche."
- "Recyclez une carte adverse pour chaque deux [Clock]."

### 5.7 Quantification (`Quantifier`)

Spécification de la portée d'une action.

#### Quantificateurs Numériques
- **Un** / **Une** - Exactement 1
- **Deux** / **Trois** / **N** - Nombre exact
- **Jusqu'à N** - Maximum N (optionnel)
- **Au moins N** - Minimum N
- **Autant que** - Égalité avec une référence
- **Tous** / **Toutes** - L'intégralité
- **La moitié** - Division par 2 (avec arrondi)

#### Quantificateurs Relatifs
- **Pour chaque** - Multiplicateur basé sur comptage
- **Pour chaque N symboles** - Division par N
- **Pour chaque couleur** - Basé sur diversité

### 5.8 Effet en Cascade (`Cascade`)

Un effet déclenche un autre effet basé sur le résultat.

**Syntaxe typique :**
- "Action1. Si [condition sur résultat], Action2."

**Exemples :**
- "Piochez une 3 et mettez-la en jeu. Si elle produit du [Crown], piochez une 4 et comptabilisez-la."
- "Si vous avez mis deux en jeu, vous pouvez transférer... Si vous le faites, transférez..."

### 5.9 Effet Partagé (`Shared`)

Exécution d'un effet pour tous les joueurs éligibles.

**Types :**
- **Dogme Coopératif** - Tous les joueurs avec ≥ symboles que l'activateur
- **Dogme de Suprématie (J'exige)** - Tous les joueurs avec < symboles

**Bonus :**
- "Si au moins un adversaire a profité de votre Dogme Coopératif, vous gagnez une action Piocher gratuite."

### 5.10 Négation/Exception (`Negation`)

Inverse une condition ou exclut certains cas.

**Syntaxe typique :**
- "Non-X" - Tout sauf X
- "Ne produit pas de X"
- "Aucun" / "Aucune"
- "Si aucune carte..."
- "Sauf"

**Exemples :**
- "Cartes non-vertes qui produisent du [Leaf]"
- "Cartes Actives non-mauves"
- "Toutes les cartes sauf une"
- "Si aucune carte n'a été transférée"

---

## Annexes

### A. Exemples de Pipelines Complets

#### Exemple 1 : Écriture (Simple)
**Texte :** "Piochez une 2."

**Pipeline :**
```
Action: Draw
  amount: 1
  age: 2
  target_player: Self
```

#### Exemple 2 : Agriculture (Conditionnel avec Référence)
**Texte :** "Vous pouvez recycler une carte de votre Main. Si vous le faites, piochez une carte d'une valeur supérieure de un à celle que vous avez recyclée et comptabilisez-la."

**Pipeline :**
```
Optional:
  Action: Recycle
    selector: FromHand(count: 1, player: Self)
  OnSuccess:
    Sequence:
      Action: Draw
        amount: 1
        age: RecycledCard.age + 1
      Action: Score
        selector: LastDrawnCard
```

#### Exemple 3 : Métallurgie (Répétition Conditionnelle)
**Texte :** "Piochez une 1 et montrez-la. Si elle produit du [Castle], comptabilisez-la et répétez ce Dogme. Sinon, ajoutez-la à votre Main."

**Pipeline :**
```
Loop:
  Action: Draw
    amount: 1
    age: 1
    reveal: true
  Condition: DrawnCard.hasIcon(Castle)
  OnTrue:
    Sequence:
      Action: Score
        selector: LastDrawnCard
      Action: RepeatLoop
  OnFalse:
    Action: AddToHand
      selector: LastDrawnCard
    Action: BreakLoop
```

#### Exemple 4 : Archerie (Dogme de Suprématie)
**Texte :** "J'exige que vous piochiez une 1 ! Puis que vous transfériez la carte la plus élevée de votre Main vers la mienne !"

**Pipeline :**
```
SupremacyDogma:
  symbol: Castle
  players: OpponentsWithLess(Castle)
  effects:
    Sequence:
      Action: Draw
        amount: 1
        age: 1
        target_player: AffectedPlayer
      Action: Transfer
        selector: FromHand(maxAge: true, player: AffectedPlayer)
        destination: Hand(player: Activator)
```

#### Exemple 5 : Tissage (Multiple Conditions)
**Texte :** "Mettez en jeu une carte d'une couleur que vous n'avez pas encore dans votre Zone de Jeu. Piochez une 1 pour chaque couleur figurant dans votre Zone de Jeu et ne figurant dans la Zone de Jeu d'aucun autre joueur et comptabilisez-la."

**Pipeline :**
```
Sequence:
  Action: Meld
    selector: FromHand(
      filter: ColorNotOnBoard(player: Self),
      count: 1
    )
  ForEach:
    collection: ColorsUniqueToPlayer(Self)
    do:
      Sequence:
        Action: Draw
          amount: 1
          age: 1
        Action: Score
          selector: LastDrawnCard
```

### B. Correspondance Terminologie Française/Anglaise

| Français | Anglais | Note |
|----------|---------|------|
| Piocher | Draw | |
| Mettre en jeu / Découvrir | Meld / Play | |
| Comptabiliser | Score | |
| Archiver | Archive / Tuck | |
| Recycler | Recycle / Return | |
| Transférer | Transfer | |
| Échanger | Exchange / Swap | |
| Décaler | Splay | |
| Dogme | Dogma | |
| J'exige | I demand | Dogme de Suprématie |
| Zone de Jeu | Board | |
| Main | Hand | |
| Influence | Score pile | |
| Carte Active | Top card | |
| Période | Age | |
| Domaine | Achievement (special) | |
| Domination | Achievement | |

### C. Symboles et Icônes

| Français | Notation | Icon Code |
|----------|----------|-----------|
| Château | [Castle] | CASTLE |
| Couronne | [Crown] | CROWN |
| Feuille | [Leaf] | LEAF |
| Ampoule | [Bulb] | LIGHTBULB |
| Usine | [Factory] | FACTORY |
| Horloge | [Clock] | CLOCK |

---

## Résumé

Cette analyse exhaustive couvre :

1. **Sélecteurs de Cartes** : 40+ variations pour cibler des cartes spécifiques
2. **Sélecteurs de Joueurs** : 15+ patterns pour identifier les joueurs affectés
3. **Actions Primitives** : 35+ opérations de base
4. **Conditions** : 45+ types de conditions
5. **Structures de Contrôle** : 10+ patterns de flux d'exécution

Ces éléments constituent la base complète nécessaire pour parser et exécuter n'importe quelle carte du jeu Innovation.
