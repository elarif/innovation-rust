
import json

def add_cards():
    filename = "extracted_cards.json"
    cards = []
    
    def add(name, age, color, icons, dogma_icon, dogma_text, is_demand=False):
        # Icons: [TopLeft, BottomLeft, BottomMiddle, BottomRight]
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

    # AGE 1
    # TopLeft, BottomLeft, BottomMiddle, BottomRight
    add("Élevage", 1, "Yellow", ["Castle", "Crown", 1, "Castle"], "Castle", "Mettez en jeu la carte la plus basse de votre Main. Piochez une 1.")
    add("Agriculture", 1, "Yellow", [1, "Leaf", "Leaf", "Leaf"], "Leaf", "Vous pouvez recycler une carte de votre Main. Si vous le faites, piochez une carte d'une valeur supérieure de un à celle que vous avez recyclée et comptabilisez-la.")
    add("Métallurgie", 1, "Red", ["Castle", "Castle", 1, "Castle"], "Castle", "Piochez une 1 et montrez-la. Si elle produit du [Castle], comptabilisez-la et répétez ce Dogme. Sinon, ajoutez-la à votre Main.")
    add("Tissage", 1, "Green", [1, "Crown", "Leaf", "Leaf"], "Leaf", "Mettez en jeu une carte d'une couleur que vous n'avez pas encore dans votre Zone de Jeu. Piochez une 1 pour chaque couleur figurant dans votre Zone de Jeu et ne figurant dans la Zone de Jeu d'aucun autre joueur et comptabilisez-la.")
    add("Outils", 1, "Blue", [1, "Bulb", "Bulb", "Castle"], "Bulb", "Vous pouvez recycler trois cartes de votre Main. Si vous le faites, piochez une 3 et mettez-la en jeu.\nVous pouvez recycler une 3 de votre Main. Si vous le faites, piochez trois 1.")
    add("Écriture", 1, "Blue", [1, "Bulb", "Bulb", "Crown"], "Bulb", "Piochez une 2.")
    add("Mysticisme", 1, "Purple", [1, "Castle", "Castle", "Castle"], "Castle", "Piochez une 1 et montrez-la. Si elle est de la même couleur qu'une autre carte dans votre Zone de Jeu, mettez-la en jeu et piochez une 1. Sinon, ajoutez-la à votre Main.")
    add("Rames", 1, "Red", ["Castle", "Crown", 1, "Castle"], "Castle", "J'exige que vous transfériez une carte qui produit du [Crown] de votre Main vers mon Influence ! Si vous subissez ce Dogme, piochez une 1 !\nSi aucune carte n'a été transférée suite au Dogme de Suprématie, piochez une 1.", is_demand=True)
    
    add("Archerie", 1, "Red", ["Castle", "Castle", 1, "Castle"], "Castle", "J'exige que vous piochiez une 1 ! Puis que vous transfériez la carte la plus élevée de votre Main vers la mienne !", is_demand=True)
    add("Cités-États", 1, "Purple", [1, "Castle", "Crown", "Castle"], "Crown", "J'exige, si vous produisez quatre [Castle] ou plus, que vous transfériez une de vos cartes Actives qui produit du [Castle] vers ma Zone de Jeu ! Si vous subissez ce Dogme, piochez une 1 !", is_demand=True)
    add("Maçonnerie", 1, "Yellow", ["Castle", 1, "Castle", "Castle"], "Castle", "Vous pouvez mettre en jeu autant de cartes qui produisent du [Castle] que vous voulez. Si vous en avez mis quatre en jeu ou plus, dominez le Domaine des TECHNOLOGIES.")
    add("La Roue", 1, "Green", [1, "Castle", "Castle", "Castle"], "Castle", "Piochez deux 1.")
    add("Voiles", 1, "Green", ["Crown", "Crown", 1, "Leaf"], "Crown", "Piochez une 1 et mettez-la en jeu.")
    add("Poterie", 1, "Blue", [1, "Leaf", "Leaf", "Leaf"], "Leaf", "Vous pouvez recycler jusqu'à trois cartes de votre Main. Si vous le faites, piochez une carte d'une valeur égale au nombre de cartes recyclées et comptabilisez-la.\nPiochez une 1.")
    add("Code de lois", 1, "Purple", [1, "Crown", "Crown", "Leaf"], "Crown", "Vous pouvez archiver une carte de votre Main d'une couleur que vous avez déjà en jeu. Si vous le faites, vous pouvez décaler cette couleur à gauche.")

    # AGE 2
    # Transcribing Top-Left icons visually...
    add("Calendrier", 2, "Blue", ["Leaf", "Leaf", "Leaf", "Bulb"], "Leaf", "Si vous avez plus de cartes dans votre Influence que dans votre Main, piochez deux 3.")
    add("Philosophie", 2, "Purple", [2, "Bulb", "Bulb", "Bulb"], "Bulb", "Vous pouvez décaler une de vos couleurs à gauche. Vous pouvez comptabiliser une carte de votre Main.")
    add("Construction de canaux", 2, "Yellow", [2, "Crown", "Leaf", "Crown"], "Crown", "Vous pouvez échanger toutes les cartes ayant la valeur la plus élevée de votre Main avec toutes les cartes ayant la valeur la plus élevée de votre Influence.")
    add("Monothéisme", 2, "Purple", [2, "Castle", "Castle", "Castle"], "Castle", "J'exige que vous transfériez une de vos cartes Actives ayant une couleur que je n'ai pas dans ma Zone de Jeu vers mon Influence ! Si vous subissez ce Dogme, piochez une 1 et archivez-la. Piochez une 1 et archivez-la.", is_demand=True)
    add("Monnaie", 2, "Green", [2, "Leaf", 2, "Crown"], "Crown", "Vous pouvez recycler autant de cartes de votre Main que vous voulez. Si vous le faites, piochez une 2 pour chaque carte recyclée de valeur différente et comptabilisez-la.")
    add("Construction", 2, "Red", ["Castle", 2, "Castle", "Castle"], "Castle", "J'exige que vous transfériez deux cartes de votre Main vers la mienne ! Piochez une 2 ! Si vous êtes le seul joueur avec cinq couleurs en jeu, dominez le Domaine MILITAIRE.", is_demand=True)
    add("Mathématiques", 2, "Blue", ["Bulb", "Bulb", "Crown", "Bulb"], "Bulb", "Vous pouvez recycler une carte de votre Main. Si vous le faites, piochez une carte d'une valeur supérieure de un à celle de la carte recyclée et mettez-la en jeu.")
    add("Cartographie", 2, "Green", ["Crown", "Crown", "Crown", "Castle"], "Crown", "J'exige que vous transfériez une 1 de votre Influence (si elle en contient) vers la mienne ! Si une carte a été transférée suite au Dogme, piochez une 1 et comptabilisez-la.", is_demand=True)
    
    add("Réseau Routier", 2, "Red", ["Castle", "Castle", 2, "Castle"], "Castle", "Mettez en jeu une ou deux cartes de votre Main. Si vous en avez mis deux en jeu, vous pouvez transférer votre carte Active rouge vers la Zone de Jeu d'un autre joueur. Si vous le faites, transférez sa carte Active verte vers votre Zone de Jeu.")
    add("Fermentation", 2, "Yellow", ["Leaf", "Leaf", 2, "Leaf"], "Leaf", "Piochez une 2 pour chaque deux [Leaf] que vous produisez.")

    # AGE 3
    # Viewing images... (Assuming standard TopLeft for now where needed, but trying to be accurate)
    # Checking CCF_000029_page_2_img_0.png
    add("Médecine", 3, "Yellow", [3, "Leaf", "Leaf", 3], "Leaf", "J'exige que vous échangiez la carte la plus élevée de votre Influence contre la carte la plus basse de la mienne !", is_demand=True)
    add("Alchimie", 3, "Blue", [3, "Leaf", "Castle", "Castle"], "Castle", "Piochez une 4 pour chaque trois [Castle] que vous produisez et montrez-la. Si l'une des cartes piochées est rouge, recyclez-les toutes ainsi que toutes les cartes de votre Main. Sinon, ajoutez-les à votre Main. Mettez en jeu une carte de votre Main, puis comptabilisez une carte de votre Main.")
    add("Traduction", 3, "Blue", ["Crown", "Crown", "Crown", "Crown"], "Crown", "Vous pouvez mettre en jeu toutes les cartes de votre Influence. Si toutes vos cartes Actives produisent du [Crown], dominez le Domaine de la DIPLOMATIE.")
    add("Boussole", 3, "Green", [3, "Crown", "Crown", "Leaf"], "Crown", "J'exige que vous transfériez une de vos cartes Actives non-vertes qui produit du [Leaf] vers ma Zone de Jeu ! Puis que vous transfériez une de mes cartes Actives qui ne produit pas du [Leaf] vers votre Zone de Jeu !", is_demand=True)
    add("Optique", 3, "Red", ["Crown", "Crown", "Crown", 3], "Crown", "Piochez une 3 et mettez-la en jeu. Si elle produit du [Crown], piochez une 4 et comptabilisez-la. Sinon, transférez une carte de votre Influence vers celle d'un adversaire qui a moins de points d'Influence que vous.")
    add("Féodalisme", 3, "Purple", [3, "Castle", "Leaf", "Castle"], "Castle", "J'exige que vous transfériez une carte qui produit du [Castle] de votre Main vers la mienne ! Vous pouvez décaler vos cartes jaunes ou mauves à gauche.", is_demand=True)
    add("Éducation", 3, "Purple", ["Bulb", "Bulb", "Bulb", 3], "Bulb", "Vous pouvez recycler la carte la plus élevée de votre Influence. Si vous le faites, piochez une carte d'une valeur supérieure de deux à celle de la carte la plus élevée qui reste dans votre Influence.")
    add("Ingénierie", 3, "Red", ["Castle", 3, "Bulb", "Castle"], "Bulb", "J'exige que vous transfériez toutes vos cartes Actives qui produisent du [Castle] vers mon Influence ! Vous pouvez décaler vos cartes rouges à gauche.", is_demand=True)
    add("Machinerie", 3, "Yellow", [3, "Leaf", 3, "Castle"], "Leaf", "J'exige que vous échangiez toutes les cartes de votre Main contre toutes les cartes ayant la valeur la plus élevée de ma Main ! Comptabilisez une carte de votre Main qui produit du [Castle]. Vous pouvez décaler vos cartes rouges à gauche.", is_demand=True)
    add("Papier", 3, "Green", ["Bulb", "Bulb", "Bulb", "Crown"], "Bulb", "Vous pouvez décaler vos cartes vertes ou bleues à gauche. Piochez une 4 pour chaque couleur décalée à gauche que vous possédez.")

    data = {
        "metadata": {
            "version": "1.3.0",
            "cardCount": len(cards),
            "source": "Manual Digitization with 4 icons"
        },
        "cards": cards
    }
    
    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
        
    print(f"Populated {len(cards)} cards (Age 1-3) with 4 icons.")

if __name__ == "__main__":
    add_cards()
