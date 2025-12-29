
from pypdf import PdfReader
from pathlib import Path
import sys

# List of files provided by the user
pdf_files = [
    "scans/split/CCF_000028_page_1.pdf",
    "scans/split/CCF_000028_page_2.pdf",
    "scans/split/CCF_000028_page_3.pdf",
    "scans/split/CCF_000029_page_1.pdf",
    "scans/split/CCF_000029_page_2.pdf",
    "scans/split/CCF_000029_page_3.pdf",
    "scans/split/CCF_000029_page_4.pdf",
    "scans/split/CCF_000029_page_5.pdf",
    "scans/split/CCF_000029_page_6.pdf",
    "scans/split/CCF_000030_page_1.pdf",
    "scans/split/CCF_000030_page_2.pdf",
    "scans/split/CCF_000030_page_3.pdf",
    "scans/split/CCF_000030_page_4.pdf",
    "scans/split/CCF_000030_page_5.pdf"
]

def extract_images_from_list():
    output_base = Path("extracted_cards_images")
    output_base.mkdir(exist_ok=True)
    
    total_images = 0
    
    for pdf_path_str in pdf_files:
        pdf_path = Path(pdf_path_str)
        if not pdf_path.exists():
            print(f"Warning: File not found: {pdf_path}")
            continue
            
        try:
            reader = PdfReader(pdf_path)
            # Assuming 1 page per split PDF as verified earlier
            page = reader.pages[0]
            
            # Use filename stem for unique folder/names
            base_name = pdf_path.stem 
            
            for i, image_file_object in enumerate(page.images):
                # Save as PNG for better quality with OCR
                image_name = f"{base_name}_img_{i}.png"
                output_path = output_base / image_name
                
                with open(output_path, "wb") as fp:
                    fp.write(image_file_object.data)
                
                print(f"Extracted: {output_path}")
                total_images += 1
                
        except Exception as e:
            print(f"Error processing {pdf_path}: {e}")

    print(f"Total images extracted: {total_images}")

if __name__ == "__main__":
    extract_images_from_list()
