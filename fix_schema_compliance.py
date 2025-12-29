
import json

def fix_compliance():
    filename = "data/cards.json"
    with open(filename, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    cards = data["cards"]
    
    icon_map = {
        "Bulb": "LIGHTBULB",
        "Crown": "CROWN",
        "Leaf": "LEAF",
        "Factory": "FACTORY",
        "Castle": "CASTLE",
        "Clock": "CLOCK"
    }
    
    # Also handle integers (Age icons) - they stay as integers
    
    for card in cards:
        # Fix Color
        if "color" in card:
            card["color"] = card["color"].upper()
            
        # Fix Icons
        new_icons = []
        for icon in card["icons"]:
            if isinstance(icon, str):
                # Normalize PascalCase to match map keys if needed, or just lookup
                # My previous scripts used strict PascalCase e.g. "Bulb"
                if icon in icon_map:
                    new_icons.append(icon_map[icon])
                else:
                    # Fallback or error? Assuming mostly correct PascalCase
                    # Just upper it if not in map (e.g. maybe I missed one?)
                    # But Bulb is special (Lightbulb).
                    print(f"Warning: Unknown icon {icon}")
                    new_icons.append(icon.upper()) 
            else:
                new_icons.append(icon)
        card["icons"] = new_icons
        
        # Fix Dogmas
        for dogma in card["dogmas"]:
            # Fix Symbol
            sym = dogma["symbol"]
            if isinstance(sym, str):
                if sym in icon_map:
                    dogma["symbol"] = icon_map[sym]
                else:
                    dogma["symbol"] = sym.upper()
            
            # Rename isDemand -> isSupremacy
            if "isDemand" in dogma:
                dogma["isSupremacy"] = dogma.pop("isDemand")
            elif "isSupremacy" not in dogma:
                dogma["isSupremacy"] = False # Default

    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
        
    print(f"Fixed {len(cards)} cards.")

if __name__ == "__main__":
    fix_compliance()
