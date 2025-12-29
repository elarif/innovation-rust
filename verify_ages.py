
import json
from collections import Counter

def verify():
    with open("extracted_cards.json", "r", encoding="utf-8") as f:
        data = json.load(f)
        
    cards = data["cards"]
    by_age = Counter()
    for c in cards:
        by_age[c["age"]] += 1
        
    print("Card Counts by Age:")
    total = 0
    for age in range(1, 11):
        count = by_age.get(age, 0)
        total += count
        expected = 15 if age == 1 else 10
        status = "OK" if count == expected else f"MISSING {expected - count}"
        print(f"Age {age}: {count} ({status})")
        
    print(f"Total: {total} (Expected: 105)")

if __name__ == "__main__":
    verify()
