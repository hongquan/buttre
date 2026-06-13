"""Preview the first few rows of a HanNom tbl_dictionary.txt export.

Usage:
    python analyze_mdb_export.py <path/to/tbl_dictionary.txt>
"""
import csv
import sys


def analyze_large_file(path: str) -> None:
    print(f"Analyzing {path}...")
    try:
        with open(path, "r", encoding="utf-16", errors="replace") as f:
            header = next(f).strip().split("\t")
            print(f"Header columns: {header}")
            for i in range(5):
                line = next(f).strip()
                if not line:
                    continue
                parts = line.split("\t")
                print(f"\nRow {i + 1}:")
                for col_idx, val in enumerate(parts):
                    col_name = header[col_idx] if col_idx < len(header) else f"Col_{col_idx}"
                    disp_val = (val[:50] + "...") if len(val) > 50 else val
                    print(f"  {col_name}: {disp_val}")
    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    analyze_large_file(sys.argv[1])
