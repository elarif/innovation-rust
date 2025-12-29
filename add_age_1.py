
import json

def add_cards():
    filename = "extracted_cards.json"
    with open(filename, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    cards = data["cards"]
    
    # Helper to add card
    def add(name, age, color, icons, dogma_icon, dogma_text, is_demand=False):
        # specific handling for domains mentioned in text? No, just text.
        # Icons: TopLeft, BottomLeft, BottomRight.
        # Dogma: list of effects. For now simple text string or list.
        # Schema expects "dogmas": [ { "symbol": "...", "text": "...", "isDemand": bool } ]
        
        card = {
            "name": name,
            "age": age,
            "color": color,
            "icons": icons,
            "dogmas": [
                {
                    "symbol": dogma_icon,
                    "text": dogma_text,
                    "isDemand": is_demand
                }
            ],
            "expansion": "Base"
        }
        cards.append(card)

    # PAGE 1
    add("Élevage", 1, "Yellow", ["Castle", "Crown", "Castle"], "Castle", 
        "Mettez en jeu la carte la plus basse de votre Main. Piochez une 1.")
        
    add("Agriculture", 1, "Yellow", ["Leaf", "Leaf", "Leaf"], "Leaf",
        "Vous pouvez recycler une carte de votre Main. Si vous le faites, piochez une carte d'une valeur supérieure de un à celle que vous avez recyclée et comptabilisez-la.")
        
    add("Métallurgie", 1, "Red", ["Castle", "Castle", "Castle"], "Castle",
        "Piochez une 1 et montrez-la. Si elle produit du [Castle], comptabilisez-la et répétez ce Dogme. Sinon, ajoutez-la à votre Main.")
        
    add("Tissage", 1, "Green", ["Leaf", "Crown", "Leaf"], "Leaf",
        "Mettez en jeu une carte d'une couleur que vous n'avez pas encore dans votre Zone de Jeu. Piochez une 1 pour chaque couleur figurant dans votre Zone de Jeu et ne figurant dans la Zone de Jeu d'aucun autre joueur et comptabilisez-la.")
        
    add("Outils", 1, "Blue", ["Bulb", "Bulb", "Castle"], "Bulb",
        "Vous pouvez recycler trois cartes de votre Main. Si vous le faites, piochez une 3 et mettez-la en jeu.\nVous pouvez recycler une 3 de votre Main. Si vous le faites, piochez trois 1.")
        
    add("Écriture", 1, "Blue", ["Bulb", "Bulb", "Crown"], "Bulb",
        "Piochez une 2.")
        
    add("Mysticisme", 1, "Purple", ["Castle", "Castle", "Castle"], "Castle",
        "Piochez une 1 et montrez-la. Si elle est de la même couleur qu'une autre carte dans votre Zone de Jeu, mettez-la en jeu et piochez une 1. Sinon, ajoutez-la à votre Main.")
        
    add("Rames", 1, "Red", ["Castle", "Crown", "Castle"], "Castle",
        "J'exige que vous transfériez une carte qui produit du [Crown] de votre Main vers mon Influence ! Si vous subissez ce Dogme, piochez une 1 !\nSi aucune carte n'a été transférée suite au Dogme de Suprématie, piochez une 1.", is_demand=True)

    # PAGE 2
    add("Archerie", 1, "Red", ["Castle", "Bulb", "Castle"], "Castle",
        "J'exige que vous piochiez une 1 ! Puis que vous transfériez la carte la plus élevée de votre Main vers la mienne !", is_demand=True)
        
    add("Cités-États", 1, "Purple", ["Castle", "Crown", "Castle"], "Crown",
        "J'exige, si vous produisez quatre [Castle] ou plus, que vous transfériez une de vos cartes Actives qui produit du [Castle] vers ma Zone de Jeu ! Si vous subissez ce Dogme, piochez une 1 !", is_demand=True)
        
    add("Maçonnerie", 1, "Yellow", ["Castle", "Castle", "Castle"], "Castle",
        "Vous pouvez mettre en jeu autant de cartes qui produisent du [Castle] que vous voulez. Si vous en avez mis quatre en jeu ou plus, dominez le Domaine des TECHNOLOGIES.")
        
    add("La Roue", 1, "Green", ["Castle", "Castle", "Castle"], "Castle",
        "Piochez deux 1.")
        
    add("Voiles", 1, "Green", ["Crown", "Crown", "Leaf"], "Crown",
        "Piochez une 1 et mettez-la en jeu.")
        
    add("Poterie", 1, "Blue", ["Leaf", "Leaf", "Leaf"], "Leaf",
        "Vous pouvez recycler jusqu'à trois cartes de votre Main. Si vous le faites, piochez une carte d'une valeur égale au nombre de cartes recyclées et comptabilisez-la.\nPiochez une 1.")
        
    add("Code de lois", 1, "Purple", ["Crown", "Crown", "Leaf"], "Crown",
        "Vous pouvez archiver une carte de votre Main d'une couleur que vous avez déjà en jeu. Si vous le faites, vous pouvez décaler cette couleur à gauche.")

    data["metadata"]["cardCount"] = len(cards)
    
    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
        
    print(f"Added {len(cards)} cards (Total Age 1).")

if __name__ == "__main__":
    add_cards()
