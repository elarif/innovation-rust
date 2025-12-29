
import json

data = {
    "$schema": "https://innovation-game-engine/schemas/card-schema-v2.json",
    "version": "1.2.0-extracted",
    "metadata": {
        "created": "2025-12-29",
        "source": "Scans Complets IELLO",
        "cardCount": 0,
        "language": "fr"
    },
    "cards": []
}

with open("extracted_cards.json", "w", encoding="utf-8") as f:
    json.dump(data, f, indent=4, ensure_ascii=False)
