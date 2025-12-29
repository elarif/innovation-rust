
import json

def add_cards():
    filename = "extracted_cards.json"
    with open(filename, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    cards = data["cards"]
    
    # helper to prepend age icon
    def add(name, age, color, icons, dogma_icon, dogma_text, is_demand=False):
        # Prepend age to icons
        new_icons = [age] + icons
        
        card = {
            "name": name,
            "age": age,
            "color": color,
            "icons": new_icons,
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

    # AGE 4 (from populate_cards_1_to_5.py)
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

    # AGE 6 (from populate_cards_6_to_10.py)
    add("Démocratie", 6, "Purple", ["Bulb", "Bulb", 6], "Crown", "Vous pouvez recycler autant de cartes de votre Main que vous voulez. Si vous en avez recyclé plus que tout autre joueur ne l'a fait avant vous pendant ce Dogme, piochez une 8 et comptabilisez-la.")
    add("Système Métrique", 6, "Green", ["Factory", "Crown", "Crown"], "Factory", "Si vos cartes vertes sont décalées à droite, vous pouvez décaler une autre de vos couleurs à droite. Vous pouvez décaler vos cartes vertes à droite.")
    add("Machines-Outils", 6, "Red", ["Factory", 6, "Factory"], "Factory", "Piochez une carte d'une valeur égale à la carte la plus élevée de votre Influence et comptabilisez-la.")
    add("Encyclopédie", 6, "Blue", ["Crown", "Crown", "Crown"], "Crown", "Vous pouvez mettre en jeu toutes les cartes de votre Influence ayant la valeur la plus élevée.")
    add("Conserves", 6, "Yellow", ["Factory", "Leaf", "Factory"], "Factory", "Vous pouvez piocher une 6 et l'archiver. Si vous le faites, comptabilisez toutes vos cartes Actives qui ne produisent pas de [Factory]. Vous pouvez décaler vos cartes jaunes à droite.")
    add("Classification", 6, "Green", ["Bulb", "Bulb", 6], "Bulb", "Montrez une carte de votre Main. Tous les joueurs doivent transférer toutes les cartes de la couleur de cette carte de leur Main vers la vôtre. Puis mettez en jeu toutes les cartes de cette couleur.")
    add("Émancipation", 6, "Purple", ["Bulb", "Factory", 6], "Factory", "J'exige que vous transfériez une carte de votre Main vers mon Influence ! Si vous subissez ce Dogme piochez une 6 ! Vous pouvez décaler vos cartes rouges ou mauves à droite.", is_demand=True)
    add("Théorie de l'Atome", 6, "Blue", ["Bulb", "Bulb", 6], "Bulb", "Vous pouvez décaler vos cartes bleues à droite. Piochez une 7 et mettez-la en jeu.")

    # AGE 6 Missing (from add_missing_cards.py)
    add("Vaccination", 6, "Green", ["Leaf", "Leaf", 6], "Leaf", "J'exige que vous recycliez toutes vos cartes ayant la valeur la plus basse de votre Influence ! Si vous subissez ce Dogme, piochez une 6 et mettez-la en jeu. Si une carte ou plus a été recyclée suite au Dogme de Suprématie, piochez une 7 et mettez-la en jeu.", is_demand=True)
    add("Industrialisation", 6, "Red", ["Factory", "Factory", "Factory"], "Crown", "Piochez une 6 pour chaque couleur présente dans votre Zone de Jeu. Mettez en jeu une carte de votre Main. Si vous avez mis en jeu une 6, répétez cet effet.") 

    # AGE 7
    add("Publications", 7, "Blue", ["Bulb", "Clock", "Bulb"], "Bulb", "Vous pouvez changer l'ordre des cartes d'une de vos couleurs. Vous pouvez décaler vos cartes jaunes ou bleues en haut.")
    add("Explosifs", 7, "Red", ["Factory", "Factory", "Factory"], "Factory", "J'exige que vous transfériez les trois cartes les plus élevées de votre Main vers la mienne ! Si vous en avez transféré au moins une et que vous n'avez plus de cartes en Main, piochez une 7 !", is_demand=True)
    add("Évolution", 7, "Purple", ["Bulb", "Bulb", 7], "Bulb", "Vous pouvez choisir soit de piocher une 8 et de la comptabiliser, puis de recycler une carte de votre Influence ; soit de piocher une carte d'une valeur supérieure de un à la carte la plus élevée de votre Influence.")
    add("Électricité", 7, "Green", ["Factory", 7, "Factory"], "Bulb", "Recyclez toutes vos cartes Actives qui ne produisent pas de [Factory], puis piochez autant de 8 que de cartes recyclées.")
    add("Éclairage", 7, "Purple", ["Leaf", "Clock", "Leaf"], "Leaf", "Vous pouvez archiver jusqu'à trois cartes de votre Main. Si vous le faites, piochez une 7 pour chaque carte de valeur différente que vous avez archivée et comptabilisez-la.")
    add("Chemin de Fer", 7, "Purple", ["Factory", "Clock", 7], "Clock", "Recyclez toutes les cartes de votre Main, puis piochez trois 6. Vous pouvez décaler une de vos couleurs actuellement décalées à droite en haut.")
    add("Santé Publique", 7, "Yellow", ["Leaf", 7, "Leaf"], "Leaf", "J'exige que vous échangiez les deux cartes les plus élevées de votre Main contre la carte la plus basse de la mienne !", is_demand=True)
    add("Moteur à Explosion", 7, "Red", ["Crown", "Factory", 7], "Factory", "J'exige que vous transfériez deux cartes de votre Influence vers la mienne !", is_demand=True)
    add("Bicyclette", 7, "Green", ["Crown", "Clock", 7], "Crown", "Vous pouvez échanger toutes les cartes de votre Main contre toutes les cartes de votre Influence.")
    add("Réfrigération", 7, "Yellow", ["Leaf", "Leaf", "Crown"], "Leaf", "J'exige que vous recycliez la moitié (arrondie à l'inférieur) des cartes que vous avez en Main ! Vous pouvez comptabiliser une carte de votre Main.", is_demand=True)

    # AGE 8
    add("Corporations", 8, "Green", ["Factory", "Factory", "Crown"], "Factory", "J'exige que vous transfériez une de vos cartes Actives non-vertes qui produit du [Factory] vers mon Influence ! Si vous subissez ce Dogme, piochez une 8 et mettez-la en jeu. Piochez une 8 et mettez-la en jeu.", is_demand=True)
    add("Antibiotiques", 8, "Yellow", ["Leaf", "Leaf", 8], "Leaf", "Vous pouvez recycler jusqu'à trois cartes de votre Main. Pour chaque carte recyclée de valeur différente, piochez deux 8.")
    add("Scientisme", 8, "Purple", ["Bulb", "Bulb", 8], "Bulb", "Choisissez deux couleurs, puis piochez une 9 et montrez-la. Si la carte est de l'une des deux couleurs choisies, mettez-la en jeu et vous pouvez décaler sa couleur en haut. Sinon, ajoutez-la à votre Main. Si vous produisez vingt [Bulb] ou plus, vous gagnez.")
    add("Mobilité", 8, "Red", ["Factory", "Clock", "Factory"], "Factory", "J'exige que vous transfériez vos deux cartes Actives non-rouges les plus élevées et qui ne produisent pas de [Factory] vers mon Influence ! Si vous en avez transféré une ou plus, piochez une 8 !", is_demand=True)
    add("Aviation", 8, "Red", [8, "Clock", "Crown"], "Crown", "Si vos cartes rouges sont décalées en haut, vous pouvez décaler une de vos couleurs en haut. Vous pouvez décaler vos cartes rouges en haut.")
    add("Média de Masse", 8, "Green", [8, "Clock", "Bulb"], "Bulb", "Vous pouvez recycler une carte de votre Main. Si vous le faites, choisissez une valeur de carte et tous les joueurs (vous y compris) doivent recycler toutes leurs cartes d'Influence de cette valeur. Vous pouvez décaler vos cartes mauves en haut.")
    add("Fusées", 8, "Blue", ["Clock", "Clock", 8], "Clock", "Recyclez une carte de votre Influence adverse pour chaque deux [Clock] que vous produisez. Vous pouvez choisir des adversaires différents pour chaque carte.")
    add("Théorie Quantique", 8, "Blue", ["Clock", "Clock", 8], "Clock", "Vous pouvez recycler jusqu'à deux cartes de votre Main. Si vous en avez recyclé deux, piochez une 10 puis piochez une 10 et comptabilisez-la.")
    add("Gratte-ciel", 8, "Yellow", ["Factory", "Crown", "Crown"], "Crown", "J'exige que vous transfériez une de vos cartes Actives non-jaunes qui produit du [Crown] vers ma Zone de Jeu ! Si vous subissez ce Dogme, comptabilisez la première carte en-dessous de cette carte et recyclez les autres cartes de cette couleur !", is_demand=True)
    add("Communisme", 8, "Purple", [8, "Leaf", "Leaf"], "Leaf", "Vous pouvez archiver toutes les cartes de votre Main. Si vous avez archivé une carte mauve ou plus, tous les joueurs doivent transférer toutes leurs cartes ayant la valeur la plus basse de leur Main vers la vôtre.")

    # AGE 9
    add("Banlieues Chics", 9, "Yellow", ["Crown", "Leaf", "Leaf"], "Leaf", "Vous pouvez archiver autant de cartes de votre Main que vous voulez. Piochez une 1 pour chaque carte archivée et comptabilisez-la.")
    add("Services", 9, "Purple", ["Leaf", "Leaf", "Leaf"], "Leaf", "J'exige que vous transfériez toutes les cartes ayant la valeur la plus élevée de votre Influence vers ma Main ! Si vous subissez ce Dogme, transférez une de mes cartes Actives qui ne produit pas de [Leaf] vers votre Main !", is_demand=True)
    add("Ordinateurs", 9, "Blue", [9, "Clock", "Factory"], "Clock", "Vous pouvez décaler vos cartes rouges ou vertes en haut. Piochez une 10 et mettez-la en jeu, puis exécutez ses Dogmes Coopératifs sans les partager.")
    add("Écologie", 9, "Yellow", ["Bulb", "Bulb", 9], "Leaf", "Vous pouvez recycler une carte de votre Main. Si vous le faites, comptabilisez une carte de votre Main et piochez deux 10.")
    add("Spécialisation", 9, "Purple", ["Factory", "Leaf", "Factory"], "Factory", "Montrez une carte de votre Main. Transférez la carte Active de vos adversaires ayant la même couleur vers votre Main. Vous pouvez décaler vos cartes jaunes ou bleues en haut.")
    add("Fission", 9, "Red", ["Clock", "Clock", "Clock"], "Clock", "J'exige que vous piochiez une 10 ! Si elle est rouge, défaussez toutes les cartes des Mains, des Zones de Jeu et des Influences de tous les joueurs (vous y compris) ! Recyclez une carte Active autre que FISSION de n'importe quel joueur.", is_demand=True)
    add("Génétique", 9, "Blue", ["Bulb", "Bulb", 9], "Bulb", "Piochez une 10 et mettez-la en jeu. Comptabilisez toutes les cartes qui sont en-dessous.")
    add("Composites", 9, "Red", ["Factory", 9, "Factory"], "Factory", "J'exige que vous transfériez toutes les cartes sauf une de votre Main vers la mienne ! De plus, transférez la carte la plus élevée de votre Influence vers la mienne !", is_demand=True)
    add("Coopération", 9, "Green", ["Crown", "Clock", "Crown"], "Crown", "J'exige que vous piochiez deux 9 et que vous les montriez ! Transférez la carte de mon choix vers ma Zone de Jeu, et mettez l'autre en jeu dans la vôtre ! Si vous avez dix cartes vertes ou plus dans votre Zone de Jeu, vous gagnez.", is_demand=True)
    add("Satellites", 9, "Green", ["Clock", "Clock", "Clock"], "Clock", "Recyclez toutes les cartes de votre Main, puis piochez trois 8. Vous pouvez décaler vos cartes mauves en haut. Mettez en jeu une carte de votre Main, puis exécutez ses Dogmes Coopératifs sans les partager.")

    # AGE 10
    add("Bio-Ingénierie", 10, "Blue", ["Clock", "Clock", 10], "Bulb", "Transférez une carte Active qui produit du [Leaf] de la Zone de Jeu du joueur de votre choix vers votre Influence. Si un joueur produit moins de trois [Leaf], le joueur qui produit le plus de [Leaf] gagne (en cas d'égalité, la partie continue).")
    add("Bases de Données", 10, "Green", ["Clock", "Clock", "Clock"], "Clock", "J'exige que vous recycliez la moitié (arrondie au supérieur) des cartes de votre Influence !", is_demand=True)
    add("Intelligence Artificielle", 10, "Purple", ["Bulb", "Clock", 10], "Bulb", "Piochez une 10 et comptabilisez-la. Si ROBOTIQUE et LOGICIEL sont toutes les deux Actives, même dans des Zones de Jeu différentes, le joueur qui a le moins de points d'Influence gagne.")
    add("Cellules Souches", 10, "Yellow", ["Leaf", "Leaf", "Leaf"], "Leaf", "Vous pouvez comptabiliser toutes les cartes de votre Main.")
    add("Robotique", 10, "Red", ["Factory", "Clock", "Factory"], "Factory", "Comptabilisez votre carte Active verte. Piochez une 10 et mettez-la en jeu, puis exécutez ses Dogmes Coopératifs sans les partager.")
    add("Mondialisation", 10, "Yellow", ["Factory", "Factory", "Factory"], "Factory", "J'exige que vous recycliez une de vos cartes Actives qui produit du [Leaf] ! Piochez une 6 et comptabilisez-la. Si aucun joueur ne produit plus de [Leaf] que de [Factory], le joueur qui a le plus de points d'Influence gagne.", is_demand=True)
    add("Miniaturisation", 10, "Red", ["Bulb", "Clock", "Bulb"], "Bulb", "Vous pouvez recycler une carte de votre Main. Si vous avez recyclé une 10, piochez une 10 pour chaque valeur différente dans votre Influence.")
    add("Domotique", 10, "Green", ["Crown", "Crown", "Crown"], "Crown", "Exécutez les Dogmes Coopératifs d'une autre de vos cartes Actives sans les partager. Si vous êtes le joueur avec le plus de Dominations, vous gagnez.")
    add("Logiciel", 10, "Blue", ["Clock", "Clock", 10], "Clock", "Piochez une 10 et comptabilisez-la. Piochez deux 10 et mettez-les en jeu dans l'ordre, puis exécutez les Dogmes Coopératifs de la seconde carte sans les partager.")
    add("Internet", 10, "Purple", ["Clock", "Clock", "Bulb"], "Clock", "Vous pouvez décaler vos cartes vertes en haut. Piochez une 10 et comptabilisez-la. Piochez une 10 pour chaque deux [Clock] que vous produisez et mettez-la en jeu.")

    data["metadata"]["cardCount"] = len(cards)
    
    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
        
    print(f"Populated {len(cards)} cards (Total with 4 icons).")

if __name__ == "__main__":
    add_cards()
