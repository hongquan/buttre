import csv
import sqlite3
import re
import os

# Reference data lives in the .reference/ directory adjacent to the repo root.
# Clone the three sources there before running:
#   .reference/rime-han-nom-data/  → https://github.com/aaabbb/rime-han-nom-data
#   .reference/hannom-dictionaries/ → internal / shared drive
#   .reference/weasel-hannom-keyboard/ → https://github.com/aaabbb/weasel-hannom-keyboard
import os as _os, sys as _sys
_SCRIPT_DIR = _os.path.dirname(_os.path.abspath(__file__))
_REPO_ROOT = _os.path.abspath(_os.path.join(_SCRIPT_DIR, ".."))
_REF = _os.path.join(_REPO_ROOT, ".reference")

STANDARD_CSV_PATH = _os.path.join(_REF, "rime-han-nom-data", "raw-data", "standard-list.csv")
COMBINED_CSV_PATH = _os.path.join(_REF, "rime-han-nom-data", "raw-data", "combined-list.csv")
DICT_PATH         = _os.path.join(_REF, "hannom-dictionaries", "All-Nom-Viet.txt")
COMPOUND_TXT_PATH = _os.path.join(_REF, "hannom-dictionaries", "TuPhuc-HanQNgu-LST.txt")
RIME_YAML_PATH    = _os.path.join(_REF, "weasel-hannom-keyboard", "newhannom.dict.yaml")
DB_PATH           = _sys.argv[1] if len(_sys.argv) > 1 else _os.path.join(_REPO_ROOT, "buttre_nom.db")

def load_nom_dictionary_strict(path):
    print(f"Loading dictionary from {path}...")
    nom_map = {}
    try:
        with open(path, 'r', encoding='utf-16', errors='replace') as f:
            for line in f:
                line = line.strip()
                if not line or line.startswith('#'): continue
                parts = line.split()
                if len(parts) < 2: continue
                code_str = parts[0]
                reading = " ".join(parts[1:]) 
                if code_str.startswith('U+'):
                    try:
                        code_point = int(code_str[2:], 16)
                        char = chr(code_point)
                        if char not in nom_map: nom_map[char] = []
                        nom_map[char].append(reading.strip().lower())
                    except ValueError: pass
    except Exception as e:
        print(f"Error loading dictionary: {e}")
    print(f"Loaded {len(nom_map)} valid characters from dictionary.")
    return nom_map

def remove_tone_marks(text):
    mapping = {
        'à': 'a', 'á': 'a', 'ả': 'a', 'ã': 'a', 'ạ': 'a',
        'ă': 'a', 'ằ': 'a', 'ắ': 'a', 'ẳ': 'a', 'ẵ': 'a', 'ặ': 'a',
        'â': 'a', 'ầ': 'a', 'ấ': 'a', 'ẩ': 'a', 'ẫ': 'a', 'ậ': 'a',
        'è': 'e', 'é': 'e', 'ẻ': 'e', 'ẽ': 'e', 'ẹ': 'e',
        'ê': 'e', 'ề': 'e', 'ế': 'e', 'ể': 'e', 'ễ': 'e', 'ệ': 'e',
        'ì': 'i', 'í': 'i', 'ỉ': 'i', 'ĩ': 'i', 'ị': 'i',
        'ò': 'o', 'ó': 'o', 'ỏ': 'o', 'õ': 'o', 'ọ': 'o',
        'ô': 'o', 'ồ': 'o', 'ố': 'o', 'ổ': 'o', 'ỗ': 'o', 'ộ': 'o',
        'ơ': 'o', 'ờ': 'o', 'ớ': 'o', 'ở': 'o', 'ỡ': 'o', 'ợ': 'o',
        'ù': 'u', 'ú': 'u', 'ủ': 'u', 'ũ': 'u', 'ụ': 'u',
        'ư': 'u', 'ừ': 'u', 'ứ': 'u', 'ử': 'u', 'ữ': 'u', 'ự': 'u',
        'ỳ': 'y', 'ý': 'y', 'ỷ': 'y', 'ỹ': 'y', 'ỵ': 'y',
        'đ': 'd',
    }
    result = []
    for char in text: result.append(mapping.get(char, char))
    return "".join(result)

def simple_telex_to_vietnamese(telex):
    words = telex.split()
    converted_words = []
    for word in words:
        s = word.lower()
        s = s.replace('uwow', 'ươ').replace('uow', 'ươ')
        s = s.replace('aw', 'ă').replace('aa', 'â')
        s = s.replace('dd', 'đ')
        s = s.replace('ee', 'ê').replace('oo', 'ô').replace('ow', 'ơ')
        s = s.replace('uw', 'ư')
        tone_map = {'s': 0, 'f': 1, 'r': 2, 'x': 3, 'j': 4}
        tone = None
        if s:
            if s[-1] == 'z': s = s[:-1]; tone = None 
            elif s[-1] in tone_map: tone = tone_map[s[-1]]; s = s[:-1]
        if tone is not None:
            vowel_tones = {
                'a': ['á', 'à', 'ả', 'ã', 'ạ'], 'ă': ['ắ', 'ằ', 'ẳ', 'ẵ', 'ặ'], 'â': ['ấ', 'ầ', 'ẩ', 'ẫ', 'ậ'],
                'e': ['é', 'è', 'ẻ', 'ẽ', 'ẹ'], 'ê': ['ế', 'ề', 'ể', 'ễ', 'ệ'], 'i': ['í', 'ì', 'ỉ', 'ĩ', 'ị'],
                'o': ['ó', 'ò', 'ỏ', 'õ', 'ọ'], 'ô': ['ố', 'ồ', 'ổ', 'ỗ', 'ộ'], 'ơ': ['ớ', 'ờ', 'ở', 'ỡ', 'ợ'],
                'u': ['ú', 'ù', 'ủ', 'ũ', 'ụ'], 'ư': ['ứ', 'ừ', 'ử', 'ữ', 'ự'], 'y': ['ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ'],
                'ươ': ['ướ', 'ườ', 'ưở', 'ưỡ', 'ượ']
            }
            chars = list(s)
            if 'ươ' in s: s = s.replace('ươ', vowel_tones['ươ'][tone], 1)
            else:
                priority_vowels = ['ơ', 'ê', 'ô', 'ă', 'â', 'ư']
                target_vowel = None
                for pv in priority_vowels: 
                    if pv in s: target_vowel = pv; break
                if target_vowel is None:
                    for c in chars: 
                        if c in vowel_tones: target_vowel = c; break
                if target_vowel and target_vowel in vowel_tones:
                    toned = vowel_tones[target_vowel][tone]
                    s = s.replace(target_vowel, toned, 1)
        converted_words.append(s)
    return " ".join(converted_words)

def find_primary_reading(telex, all_readings):
    if not all_readings: return None
    telex_converted = simple_telex_to_vietnamese(telex)
    for reading in all_readings:
        if reading.lower() == telex_converted.lower(): return reading
    return all_readings[0]

def setup_database(db_path):
    if os.path.exists(db_path):
        try: os.remove(db_path)
        except OSError: pass
    conn = sqlite3.connect(db_path)
    c = conn.cursor()
    c.execute("""
        CREATE TABLE nom_data (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            char TEXT NOT NULL,
            keywords TEXT NOT NULL, 
            meaning TEXT NOT NULL,
            freq INTEGER DEFAULT 0,
            metadata TEXT
        );
    """)
    c.execute("CREATE INDEX idx_keywords ON nom_data(keywords);")
    c.execute("CREATE INDEX idx_freq ON nom_data(freq DESC);")
    c.execute("""
        CREATE VIRTUAL TABLE nom_fts USING fts5(
            keywords,
            content='nom_data',
            content_rowid='id'
        );
    """)
    c.execute("""CREATE TRIGGER nom_ai AFTER INSERT ON nom_data BEGIN INSERT INTO nom_fts(rowid, keywords) VALUES (new.id, new.keywords); END;""")
    c.execute("""CREATE TRIGGER nom_ad AFTER DELETE ON nom_data BEGIN DELETE FROM nom_fts WHERE rowid = old.id; END;""")
    c.execute("""CREATE TRIGGER nom_au AFTER UPDATE ON nom_data BEGIN UPDATE nom_fts SET keywords = new.keywords WHERE rowid = new.id; END;""")
    conn.commit()
    return conn

def get_unicode_hex_string(text):
    if not text: return ""
    return " ".join([f"U+{ord(c):04X}" for c in text])

# GLOBAL TRACKER
existing_chars = set()

def insert_entry(c, char, meaning, freq, metadata_type="imported"):
    if char in existing_chars:
        return False
        
    unsigned = remove_tone_marks(meaning)
    keywords = f"{meaning} {unsigned}"
    u_hex = get_unicode_hex_string(char)
    metadata = f'{{"u": "{u_hex}", "type": "{metadata_type}"}}'
    
    c.execute("INSERT INTO nom_data (char, keywords, meaning, freq, metadata) VALUES (?, ?, ?, ?, ?)",
                (char, keywords, meaning, freq, metadata))
    existing_chars.add(char)
    return True

def process_csv(csv_path, nom_map, conn, base_freq_single=20000, base_freq_compound=5000):
    print(f"Processing CSV from {csv_path}...")
    c = conn.cursor()
    imported = 0
    skipped = 0
    
    with open(csv_path, 'r', encoding='utf-8') as f:
        reader = csv.reader(f)
        for row in reader:
            if len(row) < 3: continue
            
            char_text = row[0].strip()
            if not char_text or char_text in existing_chars: continue
                
            telex = row[1].strip()
            rank_str = row[2].strip()
            is_compound = len(char_text) > 1
            try: rank = int(rank_str)
            except ValueError: rank = 9999
            
            if is_compound:
                # Compound
                vietnamese_text = simple_telex_to_vietnamese(telex)
                freq = max(1, base_freq_compound - rank) 
                insert_entry(c, char_text, vietnamese_text, freq, "compound")
                imported += 1
            else:
                # Single
                if char_text not in nom_map:
                    skipped += 1
                    continue
                all_readings = nom_map[char_text]
                primary = find_primary_reading(telex, all_readings) 
                
                # Manual entry for single to include specialized keywords/metadata
                kw_set = set()
                for r in all_readings:
                    kw_set.add(r.lower())
                    kw_set.add(remove_tone_marks(r))
                keywords = " ".join(kw_set)
                u_hex = get_unicode_hex_string(char_text)
                alt = [r for r in all_readings if r != primary]
                metadata = f'{{"u": "{u_hex}", "alt": {alt}}}'
                freq = max(10000, base_freq_single - rank)
                
                c.execute("INSERT INTO nom_data (char, keywords, meaning, freq, metadata) VALUES (?, ?, ?, ?, ?)",
                          (char_text, keywords, primary, freq, metadata))
                existing_chars.add(char_text)
                imported += 1

    conn.commit()
    print(f"  Finished CSV: +{imported} entries. Skipped {skipped}.")

def process_text_compound(txt_path, conn, base_freq=3000):
    print(f"Processing Compound List from {txt_path}...")
    c = conn.cursor()
    count = 0
    try:
        with open(txt_path, 'r', encoding='utf-16', errors='replace') as f:
            for line in f:
                line = line.strip()
                if not line or line.startswith('#'): continue
                parts = line.split()
                if len(parts) < 2: continue
                char_text = parts[0]
                meaning = " ".join(parts[1:])
                
                if insert_entry(c, char_text, meaning, base_freq, "han_compound"):
                    count += 1
        conn.commit()
        print(f"  Finished Text List: +{count} compounds.")
    except Exception as e:
        print(f"Error processing compound list: {e}")

def process_rime_yaml(yaml_path, conn, base_freq=4000):
    """
    Mines compound words from Rime YAML comments.
    Format example: 𰨂      thoⓥ... nhà thờ, thờ cúng...
    """
    print(f"Mining Rime Dictionary from {yaml_path}...")
    c = conn.cursor()
    count = 0
    
    # Regex to capture Nôm/Viet pairs like "茹𰨂 nhà thờ" inside the description
    # Since Nôm chars are distinct from Latin, we can regex:
    # ([NomChars]+) \s+ ([VietChars]+)
    # But python regex for unicode ranges is tricky.
    # Simple heuristic: Split by comma, look for "Nom Viet" patterns.
    
    try:
        with open(yaml_path, 'r', encoding='utf-8') as f:
            for line in f:
                if 'ⓥ' not in line: continue
                
                # Extract part after ⓥ
                # Example: 𰨂      thoⓥ 茹𰨂 nhà thờ, 𰨂供 thờ cúng
                # Note: The separators might be special spaces usually found in Rime dicts (U+2005)
                
                parts = line.split('ⓥ')
                if len(parts) < 2: continue
                desc = parts[1]
                
                # Split by comma or pipe
                phrases = re.split(r'[,|]', desc)
                
                for phrase in phrases:
                    phrase = phrase.strip()
                    # Expect "NomWord VietWord"
                    # Split by space
                    tokens = phrase.split()
                    if len(tokens) < 2: continue
                    
                    # Assume first token is Nom, Rest is Viet
                    # Validate first token: has > 128 ord
                    # A robust check: are all chars in token[0] "High Unicode"?
                    
                    nom_cand = tokens[0]
                    viet_cand = " ".join(tokens[1:])
                    
                    is_nom = all(ord(ch) > 0x2E80 for ch in nom_cand) # CJK range start approx
                    
                    if is_nom and len(nom_cand) > 1: # Only mine COMPOUNDS (len > 1)
                        # We found a compound!
                        if insert_entry(c, nom_cand, viet_cand, base_freq, "mined_rime"):
                            count += 1
                            
        conn.commit()
        print(f"  Mined Rime List: +{count} new compounds.")
        
    except Exception as e:
        print(f"Error processing Rime YAML: {e}")


def main():
    if not os.path.exists(DICT_PATH):
        print(f"Dictionary file not found at {DICT_PATH}")
        return

    nom_map = load_nom_dictionary_strict(DICT_PATH)
    conn = setup_database(DB_PATH)
    
    # 1. Standard List (Priority) -> Coverage & Freq
    process_csv(STANDARD_CSV_PATH, nom_map, conn, base_freq_single=20000, base_freq_compound=8000)
    
    # 2. Combined List (Extension)
    process_csv(COMBINED_CSV_PATH, nom_map, conn, base_freq_single=15000, base_freq_compound=4000)
    
    # 3. Rime Mining (Nôm Thuần Việt Compounds - High Value) -> Tier 3A (Above Han-Viet)
    if os.path.exists(RIME_YAML_PATH):
        process_rime_yaml(RIME_YAML_PATH, conn, base_freq=3500)
        
    # 4. Han-Viet Compounds (Lower Priority) -> Tier 3B
    if os.path.exists(COMPOUND_TXT_PATH):
        process_text_compound(COMPOUND_TXT_PATH, conn, base_freq=3000)
    
    conn.close()
    print(f"\nFinal DB built with Total {len(existing_chars)} entries.")

if __name__ == "__main__":
    main()
