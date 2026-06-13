-- buttre Nôm Database Schema (Optimized V6)
-- Location: crates/buttre-core/src/keyboard/nom/schema.sql

-- 1. Bảng Dữ Liệu Gốc (Data Store)
CREATE TABLE IF NOT EXISTS nom_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    char TEXT NOT NULL,          -- Chữ Nôm (𡗶)
    keywords TEXT NOT NULL,      -- Search keys: "troi thien thuong"
    meaning TEXT NOT NULL,       -- Nghĩa hiển thị: "trời"
    freq INTEGER DEFAULT 0,      -- Rank: >1000 (cao), <500 (thấp)
    metadata TEXT                -- JSON: {"unicode": "U+xxxx", "source": "abc"}
);

-- Index phụ cho việc sort và lookup chính xác
CREATE INDEX IF NOT EXISTS idx_keywords ON nom_data(keywords);
CREATE INDEX IF NOT EXISTS idx_freq ON nom_data(freq DESC);

-- 2. Bảng Search Index (FTS5)
-- CHỈ Index cột keywords. Tiết kiệm dung lượng tối đa.
-- Sử dụng Contentless Delete hoặc External Content
CREATE VIRTUAL TABLE IF NOT EXISTS nom_fts USING fts5(
    keywords,
    content='nom_data',
    content_rowid='id'
);

-- 3. Triggers Light-weight (Đồng bộ tối giản)
-- Tự động cập nhật FTS khi bảng gốc thay đổi
CREATE TRIGGER IF NOT EXISTS nom_ai AFTER INSERT ON nom_data BEGIN
    INSERT INTO nom_fts(rowid, keywords) 
    VALUES (new.id, new.keywords);
END;

CREATE TRIGGER IF NOT EXISTS nom_ad AFTER DELETE ON nom_data BEGIN
    DELETE FROM nom_fts WHERE rowid = old.id;
END;

CREATE TRIGGER IF NOT EXISTS nom_au AFTER UPDATE ON nom_data BEGIN
    UPDATE nom_fts SET keywords = new.keywords 
    WHERE rowid = new.id;
END;
