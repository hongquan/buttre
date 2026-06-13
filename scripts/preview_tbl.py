"""Preview the first 10 lines of one or more HanNom .txt dictionary files.

Usage:
    python preview_tbl.py <file1> [file2 ...]
"""
import os
import sys


def preview_file(path: str) -> None:
    print(f"--- Previewing {os.path.basename(path)} ---")
    encodings = ["utf-8", "utf-16", "cp1252", "latin1"]
    content = None
    used_enc = None
    for enc in encodings:
        try:
            with open(path, "r", encoding=enc) as f:
                content = [next(f).strip() for _ in range(10)]
            used_enc = enc
            break
        except Exception:
            pass
    if content:
        print(f"Encoding detected: {used_enc}")
        for line in content:
            print(line)
    else:
        print("Failed to read file with common encodings.")
    print()


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    for path in sys.argv[1:]:
        preview_file(path)
