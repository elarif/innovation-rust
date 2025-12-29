
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

    # Age 3 (remaining)
    add("Éducation", 3, "Purple", ["Bulb", "Bulb", "Bulb"], "Bulb", "Vous pouvez recycler la carte la plus élevée de votre Influence. Si vous le faites, piochez une carte d'une valeur supérieure de deux à celle de la carte la plus élevée qui reste dans votre Influence.")
    add("Ingénierie", 3, "Red", ["Castle", "Bulb", "Castle"], "Bulb", "J'exige que vous transfériez toutes vos cartes Actives qui produisent du [Castle] vers mon Influence ! Vous pouvez décaler vos cartes rouges à gauche.", is_demand=True)
    add("Machinerie", 3, "Yellow", ["Leaf", "Leaf", "Castle"], "Leaf", "J'exige que vous échangiez toutes les cartes de votre Main contre toutes les cartes ayant la valeur la plus élevée de ma Main ! Comptabilisez une carte de votre Main qui produit du [Castle]. Vous pouvez décaler vos cartes rouges à gauche.", is_demand=True)
    add("Papier", 3, "Green", ["Bulb", "Bulb", "Crown"], "Bulb", "Vous pouvez décaler vos cartes vertes ou bleues à gauche. Piochez une 4 pour chaque couleur décalée à gauche que vous possédez.")

    # Age 4
    add("Imprimerie", 4, "Blue", ["Bulb", "Bulb", "Crown"], "Bulb", "Vous pouvez recycler une carte de votre Influence. Si vous le faites, piochez une carte d'une valeur supérieure de deux à votre carte Active mauve. Vous pouvez décaler vos cartes bleues à droite.")
    add("Perspective", 4, "Yellow", ["Bulb", "Bulb", "Leaf"], "Bulb", "Vous pouvez recycler une carte de votre Main. Si vous le faites, comptabilisez une carte de votre Main pour chaque deux [Bulb] que vous produisez.")
    add("Droit des sociétés", 4, "Purple", ["Crown", "Crown", "Crown"], "Crown", "J'exige que vous transfériez une de vos cartes Actives non-mauves qui produit du [Crown] vers ma Zone de Jeu ! Si vous subissez ce Dogme, piochez une 4 et mettez-la en jeu ! Vous pouvez décaler vos cartes vertes à droite.", is_demand=True)
    add("Colonialisme", 4, "Red", ["Bulb", "Bulb", "Factory"], "Bulb", "Piochez une 3 et archivez-la. Si elle produit du [Bulb], répétez ce Dogme.")
    
    add("Navigation", 4, "Green", ["Crown", "Crown", "Crown"], "Crown", "J'exige que vous transfériez une 2 ou une 3 de votre Influence vers la mienne !", is_demand=True)
    add("Réforme", 4, "Purple", ["Leaf", "Leaf", "Leaf"], "Leaf", "Vous pouvez choisir d'archiver une carte de votre Main pour chaque deux [Leaf] que vous produisez. Vous pouvez décaler vos cartes jaunes ou mauves à droite.")
    add("Anatomie", 4, "Yellow", ["Leaf", "Leaf", "Leaf"], "Leaf", "J'exige que vous recycliez une carte de votre Influence ! Si vous subissez ce Dogme, recyclez une de vos cartes Actives de la même valeur que la carte recyclée si vous en avez une !", is_demand=True)
    add("Invention", 4, "Green", ["Bulb", "Bulb", "Factory"], "Bulb", "Vous pouvez décaler une de vos couleurs, actuellement décalées à gauche, à droite. Si vous le faites, piochez une 4 et comptabilisez-la. Si vos cinq couleurs sont décalées, dominez le Domaine de la CULTURE.")
    add("Expérimentation", 4, "Blue", ["Bulb", "Bulb", "Bulb"], "Bulb", "Piochez une 5 et mettez-la en jeu.")
    add("Poudre", 4, "Red", ["Factory", "Crown", "Factory"], "Factory", "J'exige que vous transfériez une de vos cartes Actives qui produit du [Factory] vers mon Influence ! Si une carte ou plus a été transférée suite au Dogme de Suprématie, piochez une 2 et comptabilisez-la.", is_demand=True)

    # Age 5
    add("Physique", 5, "Blue", ["Factory", "Bulb", "Bulb"], "Bulb", "Piochez trois 6 et montrez-les. Si deux des cartes piochées ou plus sont de la même couleur, recyclez-les toutes ainsi que toutes les cartes de votre Main. Sinon, ajoutez-les à votre Main.")
    add("Astronomie", 5, "Purple", ["Crown", "Bulb", "Bulb"], "Bulb", "Piochez une 6 et montrez-la. Si la carte est verte ou bleue, mettez-la en jeu et répétez ce Dogme. Sinon, ajoutez-la à votre Main. Si toutes vos cartes Actives non-mauves valent 6 ou plus, dominez le Domaine des SCIENCES.")
    
    add("Machine à Vapeur", 5, "Yellow", ["Factory", "Crown", "Factory"], "Crown", "Piochez deux 4 et archivez-les, puis comptabilisez la carte du dessous de votre pile jaune.")
    add("Chimie", 5, "Blue", ["Factory", "Bulb", "Factory"], "Factory", "Vous pouvez décaler vos cartes bleues à droite. Piochez une carte d'une valeur supérieure de un à votre carte Active la plus élevée et comptabilisez-la, puis recyclez une carte de votre Influence.")
    add("Statistiques", 5, "Yellow", ["Leaf", "Bulb", "Leaf"], "Leaf", "J'exige que vous transfériez la carte la plus élevée de votre Influence vers votre Main. Si vous subissez ce Dogme, et que vous n'avez qu'une carte en Main, subissez à nouveau ce Dogme ! Vous pouvez décaler vos cartes jaunes à droite.", is_demand=True)
    add("Théorie de la mesure", 5, "Purple", ["Bulb", "Leaf", "Bulb"], "Bulb", "Vous pouvez recycler une carte de votre Main. Si vous le faites, décalez une de vos couleurs à droite et piochez une carte de valeur égale au nombre de cartes de cette couleur que vous avez dans votre Zone de Jeu.")
    add("Le Code des Pirates", 5, "Red", ["Crown", "Factory", "Crown"], "Crown", "J'exige que vous transfériez deux cartes de valeur inférieure ou égale à 4 de votre Influence vers la mienne ! Si une carte ou plus a été transférée suite au Dogme de Suprématie, comptabilisez votre carte Active qui produit du [Crown] ayant la valeur la plus basse.", is_demand=True)
    add("Charbon", 5, "Red", ["Factory", "Factory", "Factory"], "Factory", "Piochez une 5 et archivez-la. Vous pouvez décaler vos cartes rouges à droite. Vous pouvez comptabiliser l'une de vos cartes Actives. Si vous le faites, comptabilisez également la carte qui se trouve en-dessous de cette carte.")
    add("Système Bancaire", 5, "Green", ["Factory", "Crown", "Crown"], "Crown", "J'exige que vous transfériez une de vos cartes Actives non-vertes qui produit du [Factory] vers ma Zone de Jeu ! Si vous subissez ce Dogme, piochez une 5 et comptabilisez-la ! Vous pouvez décaler vos cartes vertes à droite.", is_demand=True)
    add("Compagnies Marchandes", 5, "Purple", ["Crown", "Bulb", "Crown"], "Crown", "J'exige que vous transfériez une de vos cartes Actives non-mauves qui produit du [Crown] vers ma Zone de Jeu ! Si vous subissez ce Dogme, piochez une 5 !", is_demand=True)

    data["metadata"]["cardCount"] = len(cards)
    
    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
        
    print(f"Added {4 + 10 + 10} cards. Current Count: {len(cards)}")

if __name__ == "__main__":
    add_cards()
