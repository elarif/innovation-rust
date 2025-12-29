
from pypdf import PdfReader
import sys
from pathlib import Path

def extract_images(pdf_path):
    try:
        reader = PdfReader(pdf_path)
        page = reader.pages[0]
        
        output_dir = Path(pdf_path).parent / "images"
        output_dir.mkdir(exist_ok=True)
        
        count = 0
        for image_file_object in page.images:
            with open(output_dir / image_file_object.name, "wb") as fp:
                fp.write(image_file_object.data)
                print(f"Extracted image: {output_dir / image_file_object.name}")
                count += 1
        
        if count == 0:
            print("No images found on page.")

    except Exception as e:
        print(f"Error reading PDF: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python extract_images.py <pdf_path>")
        sys.exit(1)
    
    extract_images(sys.argv[1])
