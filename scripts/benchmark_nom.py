import os
import sqlite3
import sys
import time

# Default to buttre_nom.db at the repo root (one level above this script).
_SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
DB_PATH = sys.argv[1] if len(sys.argv) > 1 else os.path.join(_SCRIPT_DIR, "..", "buttre_nom.db")

# Data from Truyen Kieu (Example) and Common Compounds
TEST_CASES = [
    # (Input Keyword, Expected Char/Word, Is_Compound)
    ("tram", "𤾓", False),
    ("nam", "𢆥", False),
    ("trong", "𥪝", False),
    ("coi", "𡐙", False),
    ("nguoi", "𠊛", False),
    ("ta", "些", False),
    ("chu", "𡨸", False),
    ("tai", "才", False),
    ("menh", "命", False),
    ("kheo", "𢫔", False),
    ("la", "羅", False),
    ("ghet", "𢞂", False),
    ("nhau", "饒", False),
    ("troi", "𡗶", False),
    
    # Compounds (High value targets)
    ("nha tho", "茹𰨂", True), # Mined from Rime
    ("co the", "固體", True),
    ("ac cam", "惡感", True),
]

def run_benchmark():
    print(f"Connecting to {DB_PATH}...")
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    
    total = len(TEST_CASES)
    found_top1 = 0
    found_top5 = 0
    not_found = 0
    
    print("\n--- STARTING BENCHMARK ---\n")
    print(f"{'INPUT':<10} | {'EXPECTED':<8} | {'RANK':<5} | {'RESULT'}")
    print("-" * 50)
    
    start_time = time.time()
    
    for keyword, expected, is_compound in TEST_CASES:
        # Simulate query
        query = f"""
            SELECT n.char, n.meaning, n.freq 
            FROM nom_fts f
            JOIN nom_data n ON f.rowid = n.id
            WHERE f.keywords MATCH ?
            ORDER BY n.freq DESC
            LIMIT 10
        """
        rows = c.execute(query, (keyword,)).fetchall()
        
        rank = -1
        found_char = ""
        
        for i, row in enumerate(rows):
            # Check if expected char is present
            # Note: Database might store variations, check strict equality
            if row[0] == expected:
                rank = i + 1
                found_char = row[0]
                break
            
            # Allow fuzzy match for compounds if needed? No, strict for benchmark.
        
        # Analyze result
        status = "FAIL"
        if rank == 1:
            found_top1 += 1
            status = "TOP 1"
        elif rank > 0 and rank <= 5:
            found_top5 += 1
            status = f"TOP {rank}"
        elif rank > 5:
            found_top5 += 1 # Still counted as found, but low rank
            status = f"RANK {rank}"
        else:
            not_found += 1
            status = "NOT FOUND"
            
        print(f"{keyword:<10} | {expected:<8} | {status:<5} | {found_char if rank > 0 else '---'}")

    end_time = time.time()
    duration = end_time - start_time
    
    print("\n--- SUMMARY ---")
    print(f"Total Cases: {total}")
    print(f"Top 1 Hit:   {found_top1} ({found_top1/total*100:.1f}%)")
    print(f"Top 5 Hit:   {found_top5} ({found_top5/total*100:.1f}%)")
    print(f"Not Found:   {not_found} ({not_found/total*100:.1f}%)")
    print(f"Time Taken:  {duration:.4f}s")
    
    conn.close()

if __name__ == "__main__":
    run_benchmark()
