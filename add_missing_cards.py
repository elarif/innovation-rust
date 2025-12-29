
import json

def add_missing():
    filename = "extracted_cards.json"
    with open(filename, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    cards = data["cards"]
    
    # Check if they exist already to avoid dupes
    names = {c["name"] for c in cards}
    
    def add(name, age, color, icons, dogma_icon, dogma_text, is_demand=False):
        if name in names:
            print(f"Skipping {name}, already exists.")
            return
            
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
        print(f"Added {name}.")

    # AGE 6 Missing
    add("Vaccination", 6, "Green", ["Leaf", "Leaf", 6], "Leaf", "J'exige que vous recycliez toutes vos cartes ayant la valeur la plus basse de votre Influence ! Si vous subissez ce Dogme, piochez une 6 et mettez-la en jeu. Si une carte ou plus a été recyclée suite au Dogme de Suprématie, piochez une 7 et mettez-la en jeu.", is_demand=True)
    add("Industrialisation", 6, "Red", ["Factory", "Factory", "Factory"], "Crown", "Piochez une 6 pour chaque couleur présente dans votre Zone de Jeu. Mettez en jeu une carte de votre Main. Si vous avez mis en jeu une 6, répétez cet effet.") 
    # Note: Industrialization icons are a guess based on Red/Age 6 standard. Assuming standard factories.

    data["metadata"]["cardCount"] = len(cards)
    
    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
        
    print(f"Total cards: {len(cards)}")

if __name__ == "__main__":
    add_missing()
