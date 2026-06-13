import os
import sqlite3
import sys

_SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
DB_PATH = sys.argv[1] if len(sys.argv) > 1 else os.path.join(_SCRIPT_DIR, "..", "buttre_nom.db")

conn = sqlite3.connect(DB_PATH)
c = conn.cursor()

def search(keyword):
    print(f"\n--- Search: '{keyword}' ---")
    rows = c.execute("""
        SELECT n.char, n.meaning, n.freq, n.keywords 
        FROM nom_fts f
        JOIN nom_data n ON f.rowid = n.id
        WHERE f.keywords MATCH ?
        ORDER BY n.freq DESC
        LIMIT 5
    """, (keyword,)).fetchall()
    
    if not rows:
        print("No results found.")
    for row in rows:
        print(f"  {row[0]} ({row[1]}) - Freq: {row[2]}")
        print(f"    Keywords: {row[3]}")

search("coi")  # Expect cõi
search("chu")  # Expect chữ
search("la")   # Expect là

conn.close()
