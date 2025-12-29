
import os
import sys
from pathlib import Path

try:
    from pypdf import PdfReader, PdfWriter
except ImportError:
    try:
        from PyPDF2 import PdfReader, PdfWriter
    except ImportError:
        print("Error: pypdf or PyPDF2 is required. Please install it via 'pip install pypdf'.")
        sys.exit(1)

def split_pdf(file_path):
    try:
        reader = PdfReader(file_path)
        base_name = Path(file_path).stem
        parent_dir = Path(file_path).parent
        
        # Create a subfolder for pages to keep things tidy? 
        # User said "splittes les pdf dans le dossier docs", sticking to the same folder or a subfolder is safer.
        # Let's put them in the same folder for now, or maybe a 'split' subfolder to avoid clutter.
        # User request: "qu'il n'y ait qu'une seule page par pdf".
        # Let's output to the same directory for visibility, or create a 'split' subdir. 
        # I'll create a 'split' subdir inside 'scans' to avoid messing up the original folder too much strictly.
        output_dir = parent_dir / "split"
        output_dir.mkdir(exist_ok=True)
        
        print(f"Splitting {file_path} ({len(reader.pages)} pages)...")
        
        for i, page in enumerate(reader.pages):
            writer = PdfWriter()
            writer.add_page(page)
            
            output_filename = f"{base_name}_page_{i + 1}.pdf"
            output_path = output_dir / output_filename
            
            with open(output_path, "wb") as out_file:
                writer.write(out_file)
                
        print(f"  -> Created {len(reader.pages)} files in {output_dir}")

    except Exception as e:
        print(f"Error processing {file_path}: {e}")

def main():
    scans_dir = Path("scans")
    if not scans_dir.exists():
        print(f"Directory {scans_dir} not found.")
        return

    files = list(scans_dir.glob("*.pdf"))
    if not files:
        print("No PDF files found in 'scans' directory.")
        return

    print(f"Found {len(files)} PDF files.")
    for pdf_file in files:
        split_pdf(pdf_file)

if __name__ == "__main__":
    main()
