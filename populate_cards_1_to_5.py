
import json

def add_cards():
    filename = "extracted_cards.json"
    with open(filename, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    cards = data["cards"]
    
    def add(name, age, color, icons, dogma_icon, dogma_text, is_demand=False):
        # Icons: Left, Middle, Right.
        # integers in icons list treated as Age icons.
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
    add("Élevage", 1, "Yellow", ["Crown", 1, "Castle"], "Castle", "Mettez en jeu la carte la plus basse de votre Main. Piochez une 1.")
    add("Agriculture", 1, "Yellow", ["Leaf", "Leaf", "Leaf"], "Leaf", "Vous pouvez recycler une carte de votre Main. Si vous le faites, piochez une carte d'une valeur supérieure de un à celle que vous avez recyclée et comptabilisez-la.")
    add("Métallurgie", 1, "Red", ["Castle", 1, "Castle"], "Castle", "Piochez une 1 et montrez-la. Si elle produit du [Castle], comptabilisez-la et répétez ce Dogme. Sinon, ajoutez-la à votre Main.")
    add("Tissage", 1, "Green", ["Crown", "Leaf", "Leaf"], "Leaf", "Mettez en jeu une carte d'une couleur que vous n'avez pas encore dans votre Zone de Jeu. Piochez une 1 pour chaque couleur figurant dans votre Zone de Jeu et ne figurant dans la Zone de Jeu d'aucun autre joueur et comptabilisez-la.")
    add("Outils", 1, "Blue", ["Bulb", "Bulb", "Castle"], "Bulb", "Vous pouvez recycler trois cartes de votre Main. Si vous le faites, piochez une 3 et mettez-la en jeu.\nVous pouvez recycler une 3 de votre Main. Si vous le faites, piochez trois 1.")
    add("Écriture", 1, "Blue", ["Bulb", "Bulb", "Crown"], "Bulb", "Piochez une 2.")
    add("Mysticisme", 1, "Purple", ["Castle", "Castle", "Castle"], "Castle", "Piochez une 1 et montrez-la. Si elle est de la même couleur qu'une autre carte dans votre Zone de Jeu, mettez-la en jeu et piochez une 1. Sinon, ajoutez-la à votre Main.")
    add("Rames", 1, "Red", ["Crown", 1, "Castle"], "Castle", "J'exige que vous transfériez une carte qui produit du [Crown] de votre Main vers mon Influence ! Si vous subissez ce Dogme, piochez une 1 !\nSi aucune carte n'a été transférée suite au Dogme de Suprématie, piochez une 1.", is_demand=True)
    
    add("Archerie", 1, "Red", ["Castle", 1, "Castle"], "Castle", "J'exige que vous piochiez une 1 ! Puis que vous transfériez la carte la plus élevée de votre Main vers la mienne !", is_demand=True)
    add("Cités-États", 1, "Purple", ["Castle", "Crown", "Castle"], "Crown", "J'exige, si vous produisez quatre [Castle] ou plus, que vous transfériez une de vos cartes Actives qui produit du [Castle] vers ma Zone de Jeu ! Si vous subissez ce Dogme, piochez une 1 !", is_demand=True)
    add("Maçonnerie", 1, "Yellow", [1, "Castle", "Castle"], "Castle", "Vous pouvez mettre en jeu autant de cartes qui produisent du [Castle] que vous voulez. Si vous en avez mis quatre en jeu ou plus, dominez le Domaine des TECHNOLOGIES.")
    add("La Roue", 1, "Green", ["Castle", "Castle", "Castle"], "Castle", "Piochez deux 1.")
    add("Voiles", 1, "Green", ["Crown", 1, "Leaf"], "Crown", "Piochez une 1 et mettez-la en jeu.")
    add("Poterie", 1, "Blue", ["Leaf", "Leaf", "Leaf"], "Leaf", "Vous pouvez recycler jusqu'à trois cartes de votre Main. Si vous le faites, piochez une carte d'une valeur égale au nombre de cartes recyclées et comptabilisez-la.\nPiochez une 1.")
    add("Code de lois", 1, "Purple", ["Crown", "Crown", "Leaf"], "Crown", "Vous pouvez archiver une carte de votre Main d'une couleur que vous avez déjà en jeu. Si vous le faites, vous pouvez décaler cette couleur à gauche.")

    # AGE 2
    add("Calendrier", 2, "Blue", ["Leaf", "Leaf", "Bulb"], "Leaf", "Si vous avez plus de cartes dans votre Influence que dans votre Main, piochez deux 3.")
    add("Philosophie", 2, "Purple", ["Bulb", "Bulb", "Bulb"], "Bulb", "Vous pouvez décaler une de vos couleurs à gauche. Vous pouvez comptabiliser une carte de votre Main.")
    add("Construction de canaux", 2, "Yellow", ["Crown", "Leaf", "Crown"], "Crown", "Vous pouvez échanger toutes les cartes ayant la valeur la plus élevée de votre Main avec toutes les cartes ayant la valeur la plus élevée de votre Influence.")
    add("Monothéisme", 2, "Purple", ["Castle", "Castle", "Castle"], "Castle", "J'exige que vous transfériez une de vos cartes Actives ayant une couleur que je n'ai pas dans ma Zone de Jeu vers mon Influence ! Si vous subissez ce Dogme, piochez une 1 et archivez-la. Piochez une 1 et archivez-la.", is_demand=True)
    add("Monnaie", 2, "Green", ["Leaf", 2, "Crown"], "Crown", "Vous pouvez recycler autant de cartes de votre Main que vous voulez. Si vous le faites, piochez une 2 pour chaque carte recyclée de valeur différente et comptabilisez-la.")
    add("Construction", 2, "Red", [2, "Castle", "Castle"], "Castle", "J'exige que vous transfériez deux cartes de votre Main vers la mienne ! Piochez une 2 ! Si vous êtes le seul joueur avec cinq couleurs en jeu, dominez le Domaine MILITAIRE.", is_demand=True)
    add("Mathématiques", 2, "Blue", ["Bulb", "Crown", "Bulb"], "Bulb", "Vous pouvez recycler une carte de votre Main. Si vous le faites, piochez une carte d'une valeur supérieure de un à celle de la carte recyclée et mettez-la en jeu.")
    add("Cartographie", 2, "Green", ["Crown", "Crown", "Castle"], "Crown", "J'exige que vous transfériez une 1 de votre Influence (si elle en contient) vers la mienne ! Si une carte a été transférée suite à ce Dogme, piochez une 1 et comptabilisez-la.", is_demand=True)
    
    add("Réseau Routier", 2, "Red", ["Castle", 2, "Castle"], "Castle", "Mettez en jeu une ou deux cartes de votre Main. Si vous en avez mis deux en jeu, vous pouvez transférer votre carte Active rouge vers la Zone de Jeu d'un autre joueur. Si vous le faites, transférez sa carte Active verte vers votre Zone de Jeu.")
    add("Fermentation", 2, "Yellow", ["Leaf", 2, "Leaf"], "Leaf", "Piochez une 2 pour chaque deux [Leaf] que vous produisez.")

    # AGE 3
    add("Médecine", 3, "Yellow", ["Leaf", "Leaf", 3], "Leaf", "J'exige que vous échangiez la carte la plus élevée de votre Influence contre la carte la plus basse de la mienne !", is_demand=True)
    add("Alchimie", 3, "Blue", ["Leaf", "Castle", "Castle"], "Castle", "Piochez une 4 pour chaque trois [Castle] que vous produisez et montrez-la. Si l'une des cartes piochées est rouge, recyclez-les toutes ainsi que toutes les cartes de votre Main. Sinon, ajoutez-les à votre Main. Mettez en jeu une carte de votre Main, puis comptabilisez une carte de votre Main.")
    add("Traduction", 3, "Blue", ["Crown", "Crown", "Crown"], "Crown", "Vous pouvez mettre en jeu toutes les cartes de votre Influence. Si toutes vos cartes Actives produisent du [Crown], dominez le Domaine de la DIPLOMATIE.")
    add("Boussole", 3, "Green", ["Crown", "Crown", "Leaf"], "Crown", "J'exige que vous transfériez une de vos cartes Actives non-vertes qui produit du [Leaf] vers ma Zone de Jeu ! Puis que vous transfériez une de mes cartes Actives qui ne produit pas du [Leaf] vers votre Zone de Jeu !", is_demand=True)
    add("Optique", 3, "Red", ["Crown", "Crown", 3], "Crown", "Piochez une 3 et mettez-la en jeu. Si elle produit du [Crown], piochez une 4 et comptabilisez-la. Sinon, transférez une carte de votre Influence vers celle d'un adversaire qui a moins de points d'Influence que vous.")
    add("Féodalisme", 3, "Purple", ["Castle", "Leaf", "Castle"], "Castle", "J'exige que vous transfériez une carte qui produit du [Castle] de votre Main vers la mienne ! Vous pouvez décaler vos cartes jaunes ou mauves à gauche.", is_demand=True)
    add("Éducation", 3, "Purple", ["Bulb", "Bulb", 3], "Bulb", "Vous pouvez recycler la carte la plus élevée de votre Influence. Si vous le faites, piochez une carte d'une valeur supérieure de deux à celle de la carte la plus élevée qui reste dans votre Influence.")
    add("Ingénierie", 3, "Red", [3, "Bulb", "Castle"], "Bulb", "J'exige que vous transfériez toutes vos cartes Actives qui produisent du [Castle] vers mon Influence ! Vous pouvez décaler vos cartes rouges à gauche.", is_demand=True)
    add("Machinerie", 3, "Yellow", ["Leaf", 3, "Castle"], "Leaf", "J'exige que vous échangiez toutes les cartes de votre Main contre toutes les cartes ayant la valeur la plus élevée de ma Main ! Comptabilisez une carte de votre Main qui produit du [Castle]. Vous pouvez décaler vos cartes rouges à gauche.", is_demand=True)
    add("Papier", 3, "Green", ["Bulb", "Bulb", "Crown"], "Bulb", "Vous pouvez décaler vos cartes vertes ou bleues à gauche. Piochez une 4 pour chaque couleur décalée à gauche que vous possédez.")

    # AGE 4
    add("Imprimerie", 4, "Blue", ["Bulb", "Bulb", "Crown"], "Bulb", "Vous pouvez recycler une carte de votre Influence. Si vous le faites, piochez une carte d'une valeur supérieure de deux à votre carte Active mauve. Vous pouvez décaler vos cartes bleues à droite.")
    add("Perspective", 4, "Yellow", ["Bulb", "Bulb", "Leaf"], "Bulb", "Vous pouvez recycler une carte de votre Main. Si vous le faites, comptabilisez une carte de votre Main pour chaque deux [Bulb] que vous produisez.")
    add("Droit des sociétés", 4, "Purple", ["Crown", "Crown", "Crown"], "Crown", "J'exige que vous transfériez une de vos cartes Actives non-mauves qui produit du [Crown] vers ma Zone de Jeu ! Si vous subissez ce Dogme, piochez une 4 et mettez-la en jeu ! Vous pouvez décaler vos cartes vertes à droite.", is_demand=True)
    add("Colonialisme", 4, "Red", ["Bulb", "Bulb", "Factory"], "Bulb", "Piochez une 3 et archivez-la. Si elle produit du [Bulb], répétez ce Dogme.")
    add("Navigation", 4, "Green", ["Crown", "Crown", "Crown"], "Crown", "J'exige que vous transfériez une 2 ou une 3 de votre Influence vers la mienne !", is_demand=True)
    add("Réforme", 4, "Purple", ["Leaf", 4, "Leaf"], "Leaf", "Vous pouvez choisir d'archiver une carte de votre Main pour chaque deux [Leaf] que vous produisez. Vous pouvez décaler vos cartes jaunes ou mauves à droite.")
    add("Anatomie", 4, "Yellow", ["Leaf", "Leaf", 4], "Leaf", "J'exige que vous recycliez une carte de votre Influence ! Si vous subissez ce Dogme, recyclez une de vos cartes Actives de la même valeur que la carte recyclée si vous en avez une !", is_demand=True)
    add("Invention", 4, "Green", ["Bulb", "Bulb", "Factory"], "Bulb", "Vous pouvez décaler une de vos couleurs, actuellement décalées à gauche, à droite. Si vous le faites, piochez une 4 et comptabilisez-la. Si vos cinq couleurs sont décalées, dominez le Domaine de la CULTURE.")
    add("Expérimentation", 4, "Blue", ["Bulb", "Bulb", "Bulb"], "Bulb", "Piochez une 5 et mettez-la en jeu.")
    add("Poudre", 4, "Red", ["Factory", "Crown", "Factory"], "Factory", "J'exige que vous transfériez une de vos cartes Actives qui produit du [Factory] vers mon Influence ! Si une carte ou plus a été transférée suite au Dogme de Suprématie, piochez une 2 et comptabilisez-la.", is_demand=True)

    # AGE 5
    add("Physique", 5, "Blue", ["Factory", "Bulb", 5], "Bulb", "Piochez trois 6 et montrez-les. Si deux des cartes piochées ou plus sont de la même couleur, recyclez-les toutes ainsi que toutes les cartes de votre Main. Sinon, ajoutez-les à votre Main.")
    add("Astronomie", 5, "Purple", ["Crown", "Bulb", 5], "Bulb", "Piochez une 6 et montrez-la. Si la carte est verte ou bleue, mettez-la en jeu et répétez ce Dogme. Sinon, ajoutez-la à votre Main. Si toutes vos cartes Actives non-mauves valent 6 ou plus, dominez le Domaine des SCIENCES.")
    add("Machine à Vapeur", 5, "Yellow", ["Factory", "Crown", "Factory"], "Crown", "Piochez deux 4 et archivez-les, puis comptabilisez la carte du dessous de votre pile jaune.")
    add("Chimie", 5, "Blue", ["Bulb", "Factory", 5], "Factory", "Vous pouvez décaler vos cartes bleues à droite. Piochez une carte d'une valeur supérieure de un à votre carte Active la plus élevée et comptabilisez-la, puis recyclez une carte de votre Influence.")
    add("Statistiques", 5, "Yellow", ["Leaf", "Leaf", 5], "Leaf", "J'exige que vous transfériez la carte la plus élevée de votre Influence vers votre Main. Si vous subissez ce Dogme, et que vous n'avez qu'une carte en Main, subissez à nouveau ce Dogme ! Vous pouvez décaler vos cartes jaunes à droite.", is_demand=True)
    add("Théorie de la mesure", 5, "Purple", ["Leaf", "Bulb", 5], "Bulb", "Vous pouvez recycler une carte de votre Main. Si vous le faites, décalez une de vos couleurs à droite et piochez une carte de valeur égale au nombre de cartes de cette couleur que vous avez dans votre Zone de Jeu.")
    add("Le Code des Pirates", 5, "Red", ["Factory", "Crown", 5], "Crown", "J'exige que vous transfériez deux cartes de valeur inférieure ou égale à 4 de votre Influence vers la mienne ! Si une carte ou plus a été transférée suite au Dogme de Suprématie, comptabilisez votre carte Active qui produit du [Crown] ayant la valeur la plus basse.", is_demand=True)
    add("Charbon", 5, "Red", ["Factory", "Factory", 5], "Factory", "Piochez une 5 et archivez-la. Vous pouvez décaler vos cartes rouges à droite. Vous pouvez comptabiliser l'une de vos cartes Actives. Si vous le faites, comptabilisez également la carte qui se trouve en-dessous de cette carte.")
    add("Système Bancaire", 5, "Green", ["Factory", 5, "Crown"], "Crown", "J'exige que vous transfériez une de vos cartes Actives non-vertes qui produit du [Factory] vers ma Zone de Jeu ! Si vous subissez ce Dogme, piochez une 5 et comptabilisez-la ! Vous pouvez décaler vos cartes vertes à droite.", is_demand=True)
    add("Compagnies Marchandes", 5, "Purple", [5, "Bulb", "Crown"], "Crown", "J'exige que vous transfériez une de vos cartes Actives non-mauves qui produit du [Crown] vers ma Zone de Jeu ! Si vous subissez ce Dogme, piochez une 5 !", is_demand=True)

    data["metadata"]["cardCount"] = len(cards)
    
    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
        
    print(f"Populated {len(cards)} cards (Age 1-5).")

if __name__ == "__main__":
    add_cards()
