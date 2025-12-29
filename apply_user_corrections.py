
import json

def apply_corrections():
    filename = "data/cards.json"
    with open(filename, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    cards = data["cards"]
    
    corrections = {
        "Vaccination": "YELLOW",
        "Classification": "GREEN",
        "Évolution": "BLUE",
        "Théorie de la mesure": "GREEN"
    }
    
    print("Applying Correct Colors:")
    for c in cards:
        if c["name"] in corrections:
            print(f"  - {c['name']}: {c['color']} -> {corrections[c['name']]}")
            c["color"] = corrections[c['name']]
            
    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
        
    print("Done.")

if __name__ == "__main__":
    apply_corrections()
