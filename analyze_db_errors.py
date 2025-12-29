
import json
from collections import Counter

def analyze():
    with open("data/cards.json", "r", encoding="utf-8") as f:
        data = json.load(f)

    cards = data["cards"]
    
    # 1. Check for Duplicate Age Icons
    print("--- Cards with multiple Age icons ---")
    for c in cards:
        age_icons = [i for i in c["icons"] if isinstance(i, int)]
        if len(age_icons) > 1:
            print(f"{c['name']} (Age {c['age']}): {c['icons']}")

    # 2. Check Color Distribution
    print("\n--- Color Distribution per Age ---")
    for age in range(5, 11): # User focused on later ages, but checking 5-10
        age_cards = [c for c in cards if c["age"] == age]
        colors = [c["color"].upper() for c in age_cards]
        cnt = Counter(colors)
        
        imbalance = False
        for color in ["RED", "BLUE", "GREEN", "YELLOW", "PURPLE"]:
            if cnt[color] != 2:
                imbalance = True
                
        if imbalance:
            print(f"Age {age}: {dict(cnt)}")
            for c in age_cards:
                 print(f"  - {c['name']} ({c['color']})")

if __name__ == "__main__":
    analyze()
