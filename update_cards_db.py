
import json
import shutil

def update_main_db():
    src = "extracted_cards.json"
    dst = "data/cards.json"
    
    with open(src, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    count = len(data["cards"])
    print(f"Transferring {count} cards to {dst}...")
    
    with open(dst, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
        
    print("Done.")

if __name__ == "__main__":
    update_main_db()
