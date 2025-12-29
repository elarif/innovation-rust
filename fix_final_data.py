
import json

def fix_data():
    filename = "data/cards.json"
    with open(filename, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    cards = data["cards"]
    
    # Heuristic: Map dogma symbols to strings for replacement
    # (Assuming they are already upper caused by previous fix, but be safe)
    
    for c in cards:
        # 1. Specific Color Fixes
        if c["name"] == "Évolution":
            c["color"] = "BLUE"
            print("Fixed Évolution color to BLUE")
            
        if c["name"] == "Classification":
            c["color"] = "YELLOW"
            print("Fixed Classification color to YELLOW")
            
        # 2. Duplicate Icon Fix
        # Identify indices of integer icons
        age_indices = [i for i, icon in enumerate(c["icons"]) if isinstance(icon, int)]
        
        if len(age_indices) > 1:
            # We want to keep exactly one age icon, preferably at index 0.
            # But the 4-icon logic we verified says TopLeft (index 0) = Age.
            # So keep index 0. Replace others.
            
            # Get replacement symbol (Dogma Symbol)
            replacement = c["dogmas"][0]["symbol"]
            
            print(f"Fixing icons for {c['name']} {c['icons']} -> ", end="")
            
            for idx in age_indices:
                if idx == 0:
                    continue # Keep TopLeft Age
                else:
                    c["icons"][idx] = replacement
            
            print(f"{c['icons']}")

    # 3. Ensure Uppercase (Just in case user reverted something manually)
    icon_map = {
        "Bulb": "LIGHTBULB", "Crown": "CROWN", "Leaf": "LEAF", 
        "Factory": "FACTORY", "Castle": "CASTLE", "Clock": "CLOCK",
        "Red": "RED", "Blue": "BLUE", "Green": "GREEN", "Yellow": "YELLOW", "Purple": "PURPLE"
    }

    for c in cards:
        if c["color"].title() in icon_map: # Check if loose match
             # Force UPPER
             c["color"] = c["color"].upper()
             
        new_icons = []
        for icon in c["icons"]:
            if isinstance(icon, str):
                # normalize
                if icon.title() in icon_map:
                    new_icons.append(icon_map[icon.title()])
                else:
                    new_icons.append(icon.upper())
            else:
                new_icons.append(icon)
        c["icons"] = new_icons

    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
        
    print(f"Processed {len(cards)} cards.")

if __name__ == "__main__":
    fix_data()
