
from paddleocr import PaddleOCR, draw_ocr
import sys
import logging

# Disable extensive logging
logging.getLogger("ppocr").setLevel(logging.ERROR)

def test_ocr(image_path):
    print(f"Testing OCR on {image_path}...")
    # Initialize PaddleOCR
    # lang='fr' for French cards
    ocr = PaddleOCR(use_angle_cls=True, lang='fr', show_log=False)
    
    result = ocr.ocr(image_path, cls=True)
    
    # Result is a list of lists (one per page/image processed)
    for idx in range(len(result)):
        res = result[idx]
        if res is None: # No text found
            print("No text found.")
            continue
            
        for line in res:
            # line structure: [ [ [x1,y1], ... ], ("text", confidence) ]
            box = line[0]
            text, conf = line[1]
            print(f"Text: {text} (Conf: {conf:.2f})")

if __name__ == "__main__":
    test_image = "extracted_cards_images/CCF_000028_page_1_img_0.png"
    test_ocr(test_image)
