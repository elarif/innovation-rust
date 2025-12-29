
import json

def add_cards():
    filename = "extracted_cards.json"
    with open(filename, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    cards = data["cards"]
    
    def add(name, age, color, icons, dogma_icon, dogma_text, is_demand=False):
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

    # PAGE 3 (Age 2)
    add("Calendrier", 2, "Blue", ["Leaf", "Leaf", "Bulb"], "Leaf", "Si vous avez plus de cartes dans votre Influence que dans votre Main, piochez deux 3.")
    add("Philosophie", 2, "Purple", ["Bulb", "Bulb", "Bulb"], "Bulb", "Vous pouvez décaler une de vos couleurs à gauche. Vous pouvez comptabiliser une carte de votre Main.")
    add("Construction de canaux", 2, "Yellow", ["Crown", "Leaf", "Crown"], "Crown", "Vous pouvez échanger toutes les cartes ayant la valeur la plus élevée de votre Main avec toutes les cartes ayant la valeur la plus élevée de votre Influence.")
    add("Monothéisme", 2, "Purple", ["Castle", "Castle", "Castle"], "Castle", "J'exige que vous transfériez une de vos cartes Actives ayant une couleur que je n'ai pas dans ma Zone de Jeu vers mon Influence ! Si vous subissez ce Dogme, piochez une 1 et archivez-la. Piochez une 1 et archivez-la.", is_demand=True)
    add("Monnaie", 2, "Green", ["Leaf", "Crown", "Crown"], "Crown", "Vous pouvez recycler autant de cartes de votre Main que vous voulez. Si vous le faites, piochez une 2 pour chaque carte recyclée de valeur différente et comptabilisez-la.")
    add("Construction", 2, "Red", ["Castle", "Castle", "Castle"], "Castle", "J'exige que vous transfériez deux cartes de votre Main vers la mienne ! Piochez une 2 ! Si vous êtes le seul joueur avec cinq couleurs en jeu, dominez le Domaine MILITAIRE.", is_demand=True)
    add("Mathématiques", 2, "Blue", ["Bulb", "Crown", "Bulb"], "Bulb", "Vous pouvez recycler une carte de votre Main. Si vous le faites, piochez une carte d'une valeur supérieure de un à celle de la carte recyclée et mettez-la en jeu.")
    add("Cartographie", 2, "Green", ["Crown", "Crown", "Castle"], "Crown", "J'exige que vous transfériez une 1 de votre Influence (si elle en contient) vers la mienne ! Si une carte a été transférée suite à ce Dogme, piochez une 1 et comptabilisez-la.", is_demand=True)

    # PAGE 4 (Age 2/3)
    add("Réseau Routier", 2, "Red", ["Castle", "Castle", "Castle"], "Castle", "Mettez en jeu une ou deux cartes de votre Main. Si vous en avez mis deux en jeu, vous pouvez transférer votre carte Active rouge vers la Zone de Jeu d'un autre joueur. Si vous le faites, transférez sa carte Active verte vers votre Zone de Jeu.")
    add("Fermentation", 2, "Yellow", ["Leaf", "Leaf", "Castle"], "Leaf", "Piochez une 2 pour chaque deux [Leaf] que vous produisez.")
    
    add("Médecine", 3, "Yellow", ["Crown", "Leaf", "Leaf"], "Leaf", "J'exige que vous échangiez la carte la plus élevée de votre Influence contre la carte la plus basse de la mienne !", is_demand=True)
    add("Alchimie", 3, "Blue", ["Castle", "Castle", "Castle"], "Castle", "Piochez une 4 pour chaque trois [Castle] que vous produisez et montrez-la. Si l'une des cartes piochées est rouge, recyclez-les toutes ainsi que toutes les cartes de votre Main. Sinon, ajoutez-les à votre Main. Mettez en jeu une carte de votre Main, puis comptabilisez une carte de votre Main.")
    add("Traduction", 3, "Blue", ["Crown", "Crown", "Crown"], "Crown", "Vous pouvez mettre en jeu toutes les cartes de votre Influence. Si toutes vos cartes Actives produisent du [Crown], dominez le Domaine de la DIPLOMATIE.")
    add("Boussole", 3, "Green", ["Crown", "Crown", "Leaf"], "Crown", "J'exige que vous transfériez une de vos cartes Actives non-vertes qui produit du [Leaf] vers ma Zone de Jeu ! Puis que vous transfériez une de mes cartes Actives qui ne produit pas du [Leaf] vers votre Zone de Jeu !", is_demand=True)
    add("Optique", 3, "Red", ["Crown", "Crown", "Crown"], "Crown", "Piochez une 3 et mettez-la en jeu. Si elle produit du [Crown], piochez une 4 et comptabilisez-la. Sinon, transférez une carte de votre Influence vers celle d'un adversaire qui a moins de points d'Influence que vous.")
    add("Féodalisme", 3, "Purple", ["Castle", "Leaf", "Castle"], "Castle", "J'exige que vous transfériez une carte qui produit du [Castle] de votre Main vers la mienne ! Vous pouvez décaler vos cartes jaunes ou mauves à gauche.", is_demand=True)

    data["metadata"]["cardCount"] = len(cards)
    
    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
        
    print(f"Added {10 + 8} cards (Total Age 2-3). Current Count: {len(cards)}")

if __name__ == "__main__":
    add_cards()
