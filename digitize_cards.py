
import os
import json
import logging
import cv2
import numpy as np
from pathlib import Path
from paddleocr import PaddleOCR
from PIL import Image

# Suppress PaddleOCR logging
logging.getLogger("ppocr").setLevel(logging.ERROR)

# Configuration
IMAGE_DIR = Path("extracted_cards_images")
OUTPUT_JSON = Path("extracted_cards.json")

# Define schema structure
card_schema = {
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

def sort_files(file_path):
    # Sort heuristics: CCF_0000XX_page_Y
    name = file_path.name
    parts = name.split('_')
    # CCF, 0000XX, page, Y, img, 0.png
    try:
        scan_num = int(parts[1])
        page_num = int(parts[3])
        return (scan_num, page_num)
    except:
        return name

def detect_color(image_path, crop_box=None):
    # Simple color detection: Red, Blue, Green, Yellow, Purple
    # Load image
    img = cv2.imread(str(image_path))
    if img is None:
        return "UNKNOWN"
        
    if crop_box:
        x, y, w, h = crop_box
        img = img[y:y+h, x:x+w]
    
    # Analyze dominant color in HSV
    hsv = cv2.cvtColor(img, cv2.COLOR_BGR2HSV)
    
    # Define ranges
    # Red has two ranges in HSV (0-10 and 170-180)
    lower_red1 = np.array([0, 70, 50])
    upper_red1 = np.array([10, 255, 255])
    lower_red2 = np.array([170, 70, 50])
    upper_red2 = np.array([180, 255, 255])
    
    lower_green = np.array([35, 50, 50])
    upper_green = np.array([85, 255, 255])
    
    lower_blue = np.array([100, 50, 50])
    upper_blue = np.array([130, 255, 255])
    
    lower_yellow = np.array([20, 50, 50])
    upper_yellow = np.array([35, 255, 255])
    
    lower_purple = np.array([130, 50, 50])
    upper_purple = np.array([160, 255, 255])
    
    # Count pixels
    mask_red = cv2.bitwise_or(cv2.inRange(hsv, lower_red1, upper_red1), cv2.inRange(hsv, lower_red2, upper_red2))
    mask_green = cv2.inRange(hsv, lower_green, upper_green)
    mask_blue = cv2.inRange(hsv, lower_blue, upper_blue)
    mask_yellow = cv2.inRange(hsv, lower_yellow, upper_yellow)
    mask_purple = cv2.inRange(hsv, lower_purple, upper_purple)
    
    counts = {
        "RED": np.count_nonzero(mask_red),
        "GREEN": np.count_nonzero(mask_green),
        "BLUE": np.count_nonzero(mask_blue),
        "YELLOW": np.count_nonzero(mask_yellow),
        "PURPLE": np.count_nonzero(mask_purple)
    }
    
    best_color = max(counts, key=counts.get)
    # If counts are very low, might be unknown, but let's default to best guess
    return best_color

def split_cards_from_page(ocr_result, img_width, img_height, image_path):
    # Heuristic: Cards are in a 2x4 grid.
    # Split width in half, height in quarters.
    
    col_width = img_width / 2
    row_height = img_height / 4
    
    cards = []
    
    # Iterate through grid cells
    for row in range(4):
        for col in range(2):
            x1 = int(col * col_width)
            y1 = int(row * row_height)
            x2 = int((col + 1) * col_width)
            y2 = int((row + 1) * row_height)
            
            # Identify text within this box
            card_text_blocks = []
            
            # Simple containment check for OCR text centers
            for line in ocr_result:
                box = line[0]
                text_info = line[1]
                # Box center
                cx = sum([p[0] for p in box]) / 4
                cy = sum([p[1] for p in box]) / 4
                
                if x1 <= cx < x2 and y1 <= cy < y2:
                    card_text_blocks.append((box, text_info))
            
            if not card_text_blocks:
                continue
                
            # Process card data
            # Sort by Y position
            card_text_blocks.sort(key=lambda b: b[0][0][1])
            
            # Name is likely the first block
            if not card_text_blocks:
                continue
                
            name = card_text_blocks[0][1][0]
            
            # Dogma text: Combine the rest
            dogmas = []
            current_dogma = ""
            current_symbol = "UNKNOWN" # Placeholder
            
            # Heuristic for dogma separation? 
            # Look for bullet points or graphic symbols? OCR won't give symbols easily.
            # We'll put all text into one blob for now, or split by lines.
            
            full_text = " ".join([b[1][0] for b in card_text_blocks[1:]])
            
            # Detect Color using the crop area
            # We focus on the "banner" area which is roughly the top 20% of the card
            banner_h = int((y2 - y1) * 0.25)
            # Add margin to avoid border artifacts
            margin_x = 20
            margin_y = 20
            crop_box = (x1 + margin_x, y1 + margin_y, int(col_width) - 2*margin_x, banner_h)
            
            color = detect_color(image_path, crop_box)
            
            if len(name) > 2: # Filter noise
                # Clean name (sometimes OCR catches '1' or icons)
                # Name usually uppercase? 
                
                card_obj = {
                    "name": name,
                    "age": 1, # Placeholder, will update later if possible
                    "color": color,
                    "icons": ["LEAF", "LEAF", "LEAF", "LEAF"], # Placeholder
                    "dogmas": [
                        {
                            "symbol": "LEAF", # Placeholder
                            "text": full_text
                        }
                    ]
                }
                cards.append(card_obj)
    
    return cards

def main():
    if not IMAGE_DIR.exists():
        print("Image directory not found.")
        return

    # Initialize OCR
    ocr = PaddleOCR(use_angle_cls=True, lang='fr', show_log=False)
    
    files = sorted(list(IMAGE_DIR.glob("*.png")), key=sort_files)
    
    all_cards = []
    
    for f in files:
        print(f"Processing {f.name}...")
        
        # Get image size
        img = cv2.imread(str(f))
        if img is None:
            continue
        h, w, _ = img.shape
        
        result = ocr.ocr(str(f), cls=True)
        if not result or result[0] is None:
            continue
            
        page_cards = split_cards_from_page(result[0], w, h, f)
        all_cards.extend(page_cards)
        print(f"  Found {len(page_cards)} cards.")

    # Assign Ages based on file order/assumptions?
    # CCF...28 -> Age 1
    # CCF...29 -> Age 2, 3, 4, 5?
    # This is risky. Let's just output distinct found cards.
    
    # Updating metadata
    card_schema["metadata"]["cardCount"] = len(all_cards)
    card_schema["cards"] = all_cards
    
    with open(OUTPUT_JSON, "w", encoding="utf-8") as f:
        json.dump(card_schema, f, indent=4, ensure_ascii=False)
        
    print(f"Done. Extracted {len(all_cards)} cards to {OUTPUT_JSON}")

if __name__ == "__main__":
    main()
