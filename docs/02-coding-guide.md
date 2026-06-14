# Hướng Dẫn Code buttre

> Cách viết code phù hợp với quy ước và pattern của buttre

**Cập nhật lần cuối**: 2026-06-14
**Dành cho**: Developer đóng góp cho buttre

---

## Mục Lục

1. [Cài Đặt Dự Án](#cài-đặt-dự-án)
2. [Tổ Chức Code](#tổ-chức-code)
3. [Tiêu Chuẩn Code Rust](#tiêu-chuẩn-code-rust)
4. [Các Pattern Thường Dùng](#các-pattern-thường-dùng)
5. [Hướng Dẫn Kiểm Thử](#hướng-dẫn-kiểm-thử)
6. [Xử Lý Lỗi](#xử-lý-lỗi)
7. [Hướng Dẫn Hiệu Năng](#hướng-dẫn-hiệu-năng)
8. [Cách Thêm Tính Năng Mới](#cách-thêm-tính-năng-mới)

---

## Cài Đặt Dự Án

### Yêu Cầu

- Rust 1.70+
- (Windows) Visual Studio Build Tools
- (macOS) Xcode Command Line Tools
- (Linux) GCC/Clang

### Cấu Trúc Workspace

```
buttre/
├── crates/
│   ├── buttre-engine/    # Pipeline xử lý
│   ├── buttre-core/      # Giao diện độc lập nền tảng
│   ├── buttre-platform/  # Backend nền tảng
│   └── buttre-test/      # Tiện ích kiểm thử
├── docs/                # Tài liệu
├── .agents/             # Tài liệu AI agent
└── .reference/          # Cài đặt tham chiếu
```

### Build

```bash
# Kiểm tra tất cả crate
cargo check

# Build crate cụ thể
cargo build --package buttre-engine

# Build release (đã tối ưu)
cargo build --release

# Chạy test
cargo test

# Chạy test cho crate cụ thể
cargo test --package buttre-engine
```

---

## Tổ Chức Code

### Quy Ước Đặt Tên File

**Module**: `snake_case`
```
pipeline/
├── mod.rs
├── config.rs
├── context.rs
├── executor.rs
└── stages/
    ├── mod.rs
    ├── stage1_normalization.rs
    ├── stage2_gatekeeper.rs
    └── ...
```

**Test**: Cùng file hoặc thư mục `tests/`
```
src/pipeline/executor.rs
src/pipeline/executor_tests.rs  // (nếu quá lớn để inline)

tests/
├── integration_tests.rs
└── test_data/
    ├── telex.txt
    └── vni.txt
```

### Pattern Tổ Chức Module

**Từ `buttre-engine/src/pipeline/mod.rs`**:

```rust
//! Pipeline Module — Pipeline Xử Lý 7 Giai Đoạn
//!
//! Module này cài đặt pipeline config-driven, 7 giai đoạn để xử lý
//! phương thức nhập liệu tiếng Việt (Telex, VNI, v.v.) một cách linh hoạt.
//!
//! ## Kiến Trúc
//!
//! Pipeline gồm 7 giai đoạn:
//! 1. Chuẩn hóa — chuẩn hóa input, đẩy CharInfo vào char_buffer
//! 2. Gatekeeper — định tuyến passthrough tiếng Anh / không phải tiếng Việt
//! 3. Compose — recompute-from-raw: segment → transform → tone → fallback
//! 4. Chính tả — chuẩn hóa form Unicode
//! 5. Học — theo dõi pattern (tương lai)
//! 6. Tra cứu — tra cứu từ điển (Hán Nôm)
//! 7. Đầu ra — diff last_output → syllable_buffer → phát action
//!
//! ## Nguyên Tắc Thiết Kế
//!
//! - **Config-Driven**: Tất cả phương thức nhập được định nghĩa qua cấu hình
//! - **Tăng dần**: Mỗi giai đoạn có thể được kiểm thử độc lập
//! - **Mở rộng**: Dễ thêm giai đoạn mới hoặc sửa giai đoạn hiện có

pub mod config;
pub mod context;
pub mod stage;
pub mod stages;
pub mod executor;
pub mod presets;

// Re-export để tiện dùng
pub use config::{PipelineConfig, ToneMark};
pub use context::{TypingContext, Candidate, CandidateType};
pub use stage::{PipelineStage, StageResult};
pub use executor::PipelineExecutor;
pub use presets::{telex_config, vni_config, viqr_config};
```

**Quy Tắc Pattern**:
- ✅ Dùng `//!` cho tài liệu cấp module
- ✅ Giải thích kiến trúc và nguyên tắc thiết kế
- ✅ Re-export các kiểu thường dùng
- ✅ Tổ chức submodule theo logic

---

## Tiêu Chuẩn Code Rust

### 1. Xử Lý Lỗi

**❌ KHÔNG BAO GIỜ dùng trong library code:**
```rust
// SAI — Sẽ panic trong production
let value = result.unwrap();
let value = option.expect("message");
panic!("error");
```

**✅ LUÔN LUÔN dùng Result/Option:**
```rust
// ĐÚNG — Truyền lỗi
pub fn parse_config(path: &Path) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)?;  // Truyền lỗi
    let config = toml::from_str(&content)?;        // Truyền lỗi
    Ok(config)
}

// ĐÚNG — Xử lý Option
pub fn find_vowel(text: &str) -> Option<char> {
    text.chars().find(|c| is_vowel(*c))
}
```

**Từ `buttre-core/src/keyboard/keyboard.rs`**:

```rust
/// Tạo keyboard mới từ pipeline config
pub(crate) fn new(config: PipelineConfig) -> anyhow::Result<Self> {
    let executor = PipelineExecutor::new(config);

    Ok(Self {
        executor,
        buffer: String::new(),
    })
}

/// Xử lý phím bấm
///
/// Trả về vector các action cần thực hiện. Thường chứa 1-2 action:
/// - Action chính (DoNothing/Commit/Replace/UpdateComposition)
/// - ShowCandidates/HideCandidates tùy chọn cho nhập Nôm
pub fn process(&mut self, key: char) -> anyhow::Result<Vec<Action>> {
    let engine_actions = self.executor.process(key);
    // Chuyển đổi engine action sang action của chúng ta
    // ... (không có unwrap/expect/panic)
}
```

---

### 2. Tiêu Chuẩn Tài Liệu

**Hàm Public** — BẮT BUỘC có tài liệu:

```rust
/// Xử lý phím bấm qua pipeline.
///
/// # Arguments
///
/// * `key` — Ký tự cần xử lý
///
/// # Returns
///
/// Vector các action để thực hiện trên text buffer
///
/// # Example
///
/// ```
/// use buttre_core::{Keyboard, InputMethod};
///
/// let mut keyboard = Keyboard::new(InputMethod::Telex)?;
/// let actions = keyboard.process('a')?;
/// ```
pub fn process(&mut self, key: char) -> anyhow::Result<Vec<Action>> {
    // ...
}
```

---

### 3. Quy Ước Đặt Tên

**Kiểu**: `PascalCase`
```rust
pub struct PipelineExecutor { }
pub enum InputMethodType { }
pub struct TypingContext { }
```

**Hàm/Method**: `snake_case`
```rust
pub fn process_key(&mut self, key: char) -> Vec<Action> { }
pub fn find_main_vowel(text: &str) -> Option<usize> { }
```

**Hằng số**: `SCREAMING_SNAKE_CASE`
```rust
const MAX_BUFFER_SIZE: usize = 32;
const DEFAULT_TONE_STYLE: ToneStyle = ToneStyle::Old;
```

**Hàm Boolean**: `is_`, `has_`, `can_`
```rust
pub fn is_vowel(c: char) -> bool { }
pub fn has_tone_mark(syllable: &Syllable) -> bool { }
pub fn can_apply_transformation(ctx: &Context) -> bool { }
```

**Hàm Chuyển Đổi**: `to_`, `into_`, `as_`, `from_`
```rust
pub fn to_string(&self) -> String { }
pub fn as_bytes(&self) -> &[u8] { }
pub fn from_config(config: Config) -> Self { }
```

**Hàm Có Thể Thất Bại**: `try_`
```rust
pub fn try_parse(input: &str) -> Option<Syllable> { }
pub fn try_apply_tone(ctx: &mut Context) -> Result<(), Error> { }
```

---

### 4. Type Safety

**✅ Dùng newtype cho khái niệm domain:**

```rust
// TỐT — Type-safe
pub struct UserId(u64);
pub struct TonePosition(usize);

impl TonePosition {
    pub fn new(pos: usize) -> Self {
        TonePosition(pos)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

// XẤU — Không có type safety
type UserId = u64;
let id: u64 = 123;  // Có thể là bất cứ thứ gì
```

**✅ Dùng enum thay vì string:**

```rust
// TỐT — Type-safe
pub enum InputMethodType {
    Telex,
    Vni,
    Viqr,
    Nom,
}

// XẤU — Stringly typed
let method = "telex";  // Lỗi gõ sai không được compiler phát hiện
```

---

### 5. Ownership & Borrowing

**Ưu tiên borrow thay vì ownership:**

```rust
// TỐT — Borrow khi có thể
pub fn find_main_vowel(text: &str) -> Option<usize> {
    text.chars().position(|c| is_vowel(c))
}

// ÍT TỐT HƠN — Lấy ownership không cần thiết
pub fn find_main_vowel(text: String) -> Option<usize> {
    text.chars().position(|c| is_vowel(c))
}
```

---

## Các Pattern Thường Dùng

### Pattern 1: Pipeline Stage

**Từ `buttre-engine/src/pipeline/stages/stage4_transform.rs`**:

```rust
use super::super::{PipelineStage, StageResult, TypingContext};

/// Giai Đoạn 4: Biến Đổi
///
/// Áp dụng quy tắc biến đổi (aa→â, aw→ă, dd→đ, v.v.)
pub struct Stage4Transform;

impl PipelineStage for Stage4Transform {
    fn name(&self) -> &'static str {
        "Transform"
    }

    fn process(&self, key: char, ctx: &mut TypingContext) -> StageResult {
        // 1. Kiểm tra xem đây có phải là transformation key không
        // 2. Tìm quy tắc biến đổi
        // 3. Áp dụng biến đổi
        // 4. Cập nhật context
        // 5. Trả về Continue

        StageResult::Continue
    }
}
```

**Quy Tắc Pattern**:
- ✅ Mỗi giai đoạn là một struct riêng cài đặt `PipelineStage`
- ✅ Giai đoạn có `name()` mô tả
- ✅ Method `process()` thuần túy (không có side effect ngoài `ctx`)
- ✅ Trả về `StageResult` để điều khiển luồng

---

### Pattern 2: Action Enum

**Từ `buttre-core/src/action.rs`**:

```rust
/// Các action cần thực hiện trên text buffer
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Không làm gì (ký tự đã được buffer)
    DoNothing,

    /// Commit text vào buffer
    Commit(String),

    /// Thay thế N ký tự cuối bằng text mới
    Replace {
        backspace_count: usize,
        text: String,
    },

    /// Cập nhật composition string (chỉ TSF)
    UpdateComposition {
        text: String,
        cursor: usize,
    },

    /// Xác nhận composition
    ConfirmComposition(String),

    /// Hiện cửa sổ candidate (nhập Nôm)
    ShowCandidates {
        candidates: Vec<Candidate>,
        input: String,
    },

    /// Ẩn cửa sổ candidate
    HideCandidates,
}
```

**Quy Tắc Pattern**:
- ✅ Dùng `#[derive(Debug, Clone, PartialEq)]` cho action enum
- ✅ Tài liệu từng variant
- ✅ Dùng struct-style variant cho dữ liệu phức tạp

---

### Pattern 3: Configuration Struct

**Từ `buttre-engine/src/pipeline/config.rs`**:

```rust
/// Cấu hình cho processing pipeline
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Kiểu phương thức nhập
    pub input_method_type: InputMethodType,

    /// Cấu hình dấu thanh
    pub tone_config: ToneConfig,

    /// Quy tắc biến đổi
    pub transform_rules: Vec<TransformRule>,

    /// Quy tắc áp dụng dấu thanh
    pub tone_rules: Vec<ToneRule>,
}

impl PipelineConfig {
    /// Tạo pipeline config mới
    pub fn new(input_method_type: InputMethodType) -> Self {
        Self {
            input_method_type,
            tone_config: ToneConfig::default(),
            transform_rules: Vec::new(),
            tone_rules: Vec::new(),
        }
    }

    /// Builder pattern: đặt tone config
    pub fn with_tone_config(mut self, config: ToneConfig) -> Self {
        self.tone_config = config;
        self
    }
}
```

**Quy Tắc Pattern**:
- ✅ Dùng builder pattern cho cấu hình phức tạp
- ✅ Cung cấp default hợp lý
- ✅ Để public các field để linh hoạt

---

## Hướng Dẫn Kiểm Thử

### Unit Test

**Từ `buttre-engine/src/pipeline/stages/stage4_transform.rs`**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_aa_to_circumflex() {
        let mut ctx = TypingContext::new();
        ctx.raw_buffer = vec!['a'];
        ctx.current_syllable.text = "a".to_string();

        let stage = Stage4Transform;
        let result = stage.process('a', &mut ctx);

        assert_eq!(result, StageResult::Continue);
        assert_eq!(ctx.current_syllable.text, "â");
    }

    #[test]
    fn test_transform_uo_plus_w_to_horn_pair() {
        let mut ctx = TypingContext::new();
        ctx.raw_buffer = vec!['u', 'o'];
        ctx.current_syllable.text = "uo".to_string();

        let stage = Stage4Transform;
        let result = stage.process('w', &mut ctx);

        assert_eq!(result, StageResult::Continue);
        assert_eq!(ctx.current_syllable.text, "ươ");
    }
}
```

**Đặt Tên Test**: `test_<function>_<scenario>_<expected>`

Tên tốt:
- `test_process_key_valid_input_returns_action`
- `test_apply_tone_empty_buffer_returns_none`
- `test_transform_aa_to_circumflex`

Tên xấu:
- `test_1`
- `test_process`
- `it_works`

---

### Integration Test

**Từ `buttre-engine/tests/flexible_typing_test.rs`**:

```rust
#[test]
fn test_flexible_typing_tuongwf_to_truong() {
    let config = telex_config();
    let mut executor = PipelineExecutor::new(config);

    // Gõ "tuongwf" (không theo thứ tự)
    let keys = ['t', 'u', 'o', 'n', 'g', 'w', 'f'];

    for key in keys {
        executor.process(key);
    }

    let final_output = executor.get_current_output();
    assert_eq!(final_output, "trường");
}
```

---

### File Dữ Liệu Test

**Từ `buttre-test/data/telex.txt`**:

```
# Định dạng: input → expected_output
# Một test mỗi dòng

# Biến đổi cơ bản
aa → â
aw → ă
dd → đ

# Dấu thanh
as → á
af → à

# Từ phức tạp
nguwowif → người
tuongwf → trường
```

---

## Xử Lý Lỗi

### Dùng anyhow Cho Lỗi Application

```rust
use anyhow::{Result, Context};

pub fn load_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Không đọc được config từ {}", path.display()))?;

    let config: Config = toml::from_str(&content)
        .context("Không parse được TOML config")?;

    Ok(config)
}
```

### Dùng thiserror Cho Lỗi Library

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("Phương thức nhập không hợp lệ: {0}")]
    InvalidInputMethod(String),

    #[error("Lỗi cấu hình: {0}")]
    ConfigError(String),

    #[error("Lỗi pipeline nội bộ")]
    InternalError(#[from] std::io::Error),
}
```

---

## Hướng Dẫn Hiệu Năng

### 1. Tránh Allocation Trong Hot Path

```rust
// TỐT — Không allocation
pub fn find_vowel(text: &str) -> Option<usize> {
    text.chars().position(|c| is_vowel(c))
}

// XẤU — Allocation không cần thiết
pub fn find_vowel(text: &str) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();  // Allocation!
    chars.iter().position(|c| is_vowel(*c))
}
```

### 2. Dùng Static Lookup

```rust
// TỐT — Tra cứu O(1)
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref VOWELS: HashSet<char> = {
        ['a', 'à', 'á', 'ả', 'ã', 'ạ',
         'e', 'è', 'é', 'ẻ', 'ẽ', 'ẹ',
         // ... thêm nguyên âm
        ].iter().copied().collect()
    };
}

pub fn is_vowel(c: char) -> bool {
    VOWELS.contains(&c)
}
```

### 3. Dùng #[inline] Cho Hàm Nhỏ

```rust
#[inline]
pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}
```

---

## Cách Thêm Tính Năng Mới

### Ví Dụ: Thêm Phương Thức Nhập Mới

**Bước 1**: Tạo preset config

**File**: `crates/buttre-engine/src/pipeline/presets.rs`

```rust
/// Tạo cấu hình phương thức nhập VIQR
pub fn viqr_config() -> PipelineConfig {
    PipelineConfig {
        input_method_type: InputMethodType::Viqr,
        tone_config: ToneConfig {
            free_marking: true,
            auto_correct_uo: false,
            max_modify_length: 10,
        },
        transform_rules: vec![
            // VIQR dùng phím khác
            TransformRule { pattern: "a^", result: "â", ... },
            TransformRule { pattern: "a+", result: "ă", ... },
            // ... thêm quy tắc
        ],
        tone_rules: vec![
            ToneRule { key: '\'', mark: ToneMark::Acute },
            ToneRule { key: '`', mark: ToneMark::Grave },
            // ... thêm quy tắc
        ],
    }
}
```

**Bước 2**: Thêm variant enum

**File**: `crates/buttre-engine/src/pipeline/config.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethodType {
    Telex,
    Vni,
    Viqr,   // Thêm cái này
    Nom,
}
```

**Bước 3**: Thêm test

**File**: `crates/buttre-engine/tests/viqr_test.rs`

```rust
#[test]
fn test_viqr_circumflex() {
    let config = viqr_config();
    let mut executor = PipelineExecutor::new(config);

    executor.process('a');
    executor.process('^');

    assert_eq!(executor.get_current_output(), "â");
}
```

**Bước 4**: Cập nhật tài liệu

- Thêm vào `docs/01-architecture.md`
- Thêm vào `README.md`

---

## Tóm Tắt

**Tiêu Chuẩn Code buttre**:

✅ **Xử lý lỗi**: Dùng Result/Option, không bao giờ unwrap/expect trong library code
✅ **Tài liệu**: Tài liệu hóa tất cả public API kèm ví dụ
✅ **Đặt tên**: snake_case hàm, PascalCase kiểu, SCREAMING_SNAKE hằng số
✅ **Type Safety**: Dùng newtype và enum, tránh string cho khái niệm domain
✅ **Kiểm thử**: Unit test cho tất cả hàm, integration test cho luồng
✅ **Hiệu năng**: Tránh allocation trong hot path, dùng static lookup
✅ **Pattern**: Tuân theo các pattern đã thiết lập (Pipeline Stage, Action Enum, v.v.)

**Trước Khi Submit PR**:

```bash
# 1. Format code
cargo fmt

# 2. Kiểm tra clippy
cargo clippy --all-targets --all-features

# 3. Chạy test
cargo test --all

# 4. Build release
cargo build --release
```

**Câu hỏi?** Xem code hiện có để tìm pattern, hoặc hỏi qua issues!
