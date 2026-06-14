> **Hỗ trợ macOS hiện chỉ dành cho developer.** Chưa có installer cho người dùng cuối.
> Bản release đính kèm `libbuttre_platform.dylib` là artifact dành cho developer (universal binary) để dùng trong custom host app. Xem README bên trong `buttre-*-macos-dylib.zip` để biết hướng dẫn linking và cách bỏ quarantine Gatekeeper. Shell Swift IMK đang được lên kế hoạch nhưng chưa xây dựng.

# 🍎 buttre macOS — Sẵn Sàng Cho Developer!

## 📦 Thông Tin Build

**Phiên bản**: 0.7.0-beta
**Trạng thái**: ✅ Code Hoàn Chỉnh, Sẵn Sàng Build Trên macOS
**Kiến trúc**: IMKit + Rust FFI

---

## 🏗️ Kiến Trúc

```
┌─────────────────────────────┐
│    Ứng Dụng macOS           │
│  (TextEdit, Notes, v.v.)    │
└──────────┬──────────────────┘
           │
    ┌──────▼──────┐
    │  IMKServer  │  (Objective-C)
    └──────┬──────┘
           │
    ┌──────▼──────┐
    │IMKInputCtrl │  (Objective-C)
    └──────┬──────┘
           │ FFI
    ┌──────▼──────┐
    │   Engine    │  (Rust)
    │buttre-engine│
    └─────────────┘
```

---

## 🚀 Hướng Dẫn Build (Trên macOS)

### Yêu Cầu

```bash
# Cài Xcode Command Line Tools
xcode-select --install

# Cài Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Thêm target aarch64 (cho Apple Silicon)
rustup target add aarch64-apple-darwin
```

### Build

```bash
# Cấp quyền thực thi script
chmod +x scripts/build-macos.sh

# Chạy build
./scripts/build-macos.sh
```

**Đầu ra**: `target/release/libbuttre_platform.dylib` (kiến trúc host; dùng `installers/macos/build_dylib.sh <version>` để tạo universal release ZIP)

---

## 📥 Cài Đặt

```bash
# Sao chép vào vị trí hệ thống (yêu cầu sudo)
sudo cp -R build/macos/buttre.app "/Library/Input Methods/"

# Khởi động lại hệ thống Input Method
killall -9 SystemUIServer
```

### Thêm Input Source

1. **System Settings** → **Keyboard** → **Input Sources**
2. Nhấp nút **+**
3. Chọn **Vietnamese**
4. Chọn **buttre**
5. Nhấp **Add**

---

## 🧪 Kiểm Thử

### Test 1: Nhập Cơ Bản
1. Mở **TextEdit**
2. Chuyển sang buttre (Control + Space hoặc phím Globe)
3. Gõ: `hoaf`
4. **Mong đợi**: `hoà` có gạch chân

### Test 2: Chữ Hoa
1. Gõ: `Shift+V` `i` `e` `e` `t`
2. **Mong đợi**: `Việt`

### Test 3: Backspace
1. Gõ: `hoaf` → `hoà`
2. Nhấn **Backspace**
3. **Mong đợi**: `hoa`

### Test 4: Hoàn Thành
1. Gõ: `hoaf` → `hoà`
2. Nhấn **Dấu cách**
3. **Mong đợi**: `hoà ` (gạch chân biến mất)

---

## 📁 Cấu Trúc Dự Án

```
crates/buttre-platform/
├── src/platforms/macos/
│   ├── mod.rs                  # Điểm vào backend macOS
│   └── ffi.rs                  # C ABI expose ra IMKit host
└── Cargo.toml                  # cdylib → libbuttre_platform.dylib

installers/macos/
├── build_dylib.sh              # Universal (arm64 + x86_64) release ZIP
└── ARTIFACT_README.md          # Hướng dẫn tích hợp dylib
```

---

## 🔍 Chi Tiết Cài Đặt

### Hàm FFI

```c
// Tạo engine
void* buttre_engine_new(void);

// Giải phóng engine
void buttre_engine_free(void* engine);

// Xử lý phím
const char* buttre_engine_process_key(void* engine, unsigned short keycode, BOOL shift, BOOL capslock);

// Xử lý backspace
const char* buttre_engine_process_backspace(void* engine);
```

### Luồng Key Event

```
Người dùng gõ 'h'
    ↓
handleEvent: (NSEvent)
    ↓
buttre_engine_process_key(engine, keycode, shift, capslock)
    ↓
Rust: Keyboard::process('h')
    ↓
Trả về: "h"
    ↓
setMarkedText: "h" (có gạch chân)
    ↓
Hiển thị trong ứng dụng
```

---

## ✅ Đã Cài Đặt

- ✅ **IMKServer** — Khởi tạo server
- ✅ **IMKInputController** — Xử lý event
- ✅ **FFI Bridge** — Rust ↔ Objective-C
- ✅ **Engine tiếng Việt** — Xử lý Telex
- ✅ **Composition** — Cập nhật thời gian thực
- ✅ **Backspace** — Xử lý thông minh
- ✅ **Hỗ trợ Shift** — Chữ hoa
- ✅ **Hoàn thành** — Space/Enter xác nhận

---

## ⏳ Chưa Cài Đặt

- ❌ **Chế độ VNI** — Hiện chỉ có Telex
- ❌ **UI Candidate** — Không cần cho tiếng Việt thuần
- ❌ **Hán Nôm** — Dự kiến Giai đoạn 2
- ❌ **UI Cài Đặt** — Panel cấu hình
- ❌ **Icon** — App icon

---

## 🐛 Xử Lý Sự Cố

### Vấn Đề: Build Thất Bại

**Giải pháp**:
- Đảm bảo đã cài Xcode Command Line Tools
- Kiểm tra Rust toolchain: `rustup show`
- Xác minh target: `rustup target list --installed`

### Vấn Đề: Ứng Dụng Không Trong Input Sources

**Giải pháp**:
- Kiểm tra đường dẫn cài đặt: `/Library/Input Methods/buttre.app`
- Xác minh Info.plist đúng
- Khởi động lại: `killall -9 SystemUIServer`
- Khởi động lại macOS

### Vấn Đề: Không Có Composition

**Giải pháp**:
- Kiểm tra Console.app để xem logs (filter: "buttre")
- Xác minh buttre đang là input source được chọn
- Thử trong TextEdit trước (hỗ trợ IMKit tốt nhất)

### Vấn Đề: Cảnh Báo Gatekeeper / Quarantine

**Giải pháp**:
```bash
# Bỏ quarantine cho dylib
xattr -dr com.apple.quarantine libbuttre_platform.dylib

# Hoặc bỏ quarantine cho toàn bộ app
xattr -dr com.apple.quarantine /Library/Input\ Methods/buttre.app
```

---

## 📊 So Sánh: Windows vs macOS

| Khía Cạnh | Windows TSF | macOS IMKit |
|-----------|-------------|-------------|
| **Trạng thái** | ✅ Hoàn chỉnh | ✅ Code Sẵn Sàng |
| **Build** | DLL | App Bundle |
| **Cài đặt** | Registry | /Library/Input Methods/ |
| **API** | COM | Objective-C |
| **Composition** | ITfComposition | setMarkedText |
| **Events** | ITfKeyEventSink | handleEvent |

---

## 🚀 Bước Tiếp Theo

### Ngay Lập Tức
1. **Build trên macOS** — Chạy build script
2. **Kiểm thử** — Xác minh chức năng cơ bản
3. **Debug** — Sửa bất kỳ vấn đề nào

### Giai Đoạn 2
- [ ] Chuyển đổi chế độ VNI
- [ ] Panel cài đặt
- [ ] App icon
- [ ] Localization

### Giai Đoạn 3
- [ ] Hỗ trợ Hán Nôm
- [ ] Cửa sổ candidate
- [ ] Hỗ trợ đa màn hình

---

## 📚 Tài Liệu Tham Khảo

- [Input Method Kit Guide](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/InputMethod/InputMethod.html)
- [IMKInputController](https://developer.apple.com/documentation/inputmethodkit/imkinputcontroller)
- [OpenVanilla](https://github.com/openvanilla/openvanilla) — Tham chiếu

---

**Trạng thái**: Sẵn sàng Build trên macOS
**Tiếp theo**: Build và kiểm thử trên máy macOS
**Timeline**: 1–2 tuần để sẵn sàng production

---

*Được viết bằng ❤️ sử dụng Rust + Objective-C*
