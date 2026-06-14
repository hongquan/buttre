# buttre TSF — Hướng Dẫn Kiểm Thử Thủ Công

## Vị Trí Build Output

**Vị trí DLL**: `target/release/buttre_platform.dll`

---

## Cài Đặt & Đăng Ký

### Bước 1: Sao Chép DLL Vào Vị Trí Hệ Thống

```powershell
# Chạy với quyền Administrator
$dllPath = "target\release\buttre_platform.dll"
$systemPath = "$env:ProgramFiles\buttre\buttre_platform.dll"

# Tạo thư mục
New-Item -ItemType Directory -Force -Path "$env:ProgramFiles\buttre"

# Sao chép DLL
Copy-Item $dllPath $systemPath -Force
```

### Bước 2: Đăng Ký COM Server

```powershell
# Chạy với quyền Administrator
regsvr32 "$env:ProgramFiles\buttre\buttre_platform.dll"
```

**Kết quả mong đợi**: "DllRegisterServer in buttre_platform.dll succeeded"

### Bước 3: Kích Hoạt TSF Service

1. Mở **Settings** → **Time & Language** → **Language**
2. Nhấp **Preferred languages** → **Add a language**
3. Tìm kiếm "Vietnamese" → Thêm
4. Nhấp **Vietnamese** → **Options**
5. Trong **Keyboards**, nhấp **Add a keyboard**
6. Tìm **buttre** trong danh sách
7. Chọn **buttre** và nhấp **Add**

---

## Danh Sách Kiểm Thử

### Chức Năng Cơ Bản

#### Kiểm Thử 1: Nhập Telex (Chữ Thường)
1. Mở **Notepad**
2. Chuyển sang nhập buttre (Windows + Space)
3. Gõ: `hoaf`
4. **Mong đợi**: `hoà` (có dấu huyền)

#### Kiểm Thử 2: Nhập Telex (Chữ Hoa)
1. Gõ: `Shift+V` `i` `e` `e` `t`
2. **Mong đợi**: `Việt`

#### Kiểm Thử 3: Từ Phức Tạp
1. Gõ: `t` `o` `a` `n` `f`
2. **Mong đợi**: `toàn`

#### Kiểm Thử 4: Backspace
1. Gõ: `h` `o` `a` `f` → `hoà`
2. Nhấn **Backspace**
3. **Mong đợi**: `hoa` (đã xóa dấu)
4. Nhấn **Backspace** lần nữa
5. **Mong đợi**: `ho`

#### Kiểm Thử 5: Hoàn Thành Bằng Dấu Cách
1. Gõ: `h` `o` `a` `f` → `hoà`
2. Nhấn **Dấu cách**
3. **Mong đợi**: `hoà ` (composition đã hoàn thành, con trỏ sau dấu cách)

#### Kiểm Thử 6: Hoàn Thành Bằng Enter
1. Gõ: `h` `o` `a` `f` → `hoà`
2. Nhấn **Enter**
3. **Mong đợi**: `hoà` trên dòng đầu, con trỏ trên dòng mới

### Kiểm Thử Nâng Cao

#### Kiểm Thử 7: Nhiều Từ
1. Gõ: `t` `i` `e` `e` `n` `g` **Dấu cách** `v` `i` `e` `e` `t`
2. **Mong đợi**: `tiếng việt`

#### Kiểm Thử 8: Thay Đổi Dấu Thanh
1. Gõ: `h` `o` `a` `f` → `hoà`
2. Nhấn `z` (xóa dấu)
3. **Mong đợi**: `hoa`
4. Nhấn `s` (dấu sắc)
5. **Mong đợi**: `hoá`

#### Kiểm Thử 9: Thuộc Tính Hiển Thị
1. Gõ: `h` `o` `a`
2. **Mong đợi**: Text có **gạch chân chấm** trong khi composition đang hoạt động
3. Nhấn **Dấu cách**
4. **Mong đợi**: Gạch chân biến mất (composition đã hoàn thành)

---

## Xử Lý Sự Cố

### Vấn Đề: Đăng Ký DLL Thất Bại

**Triệu chứng**: `regsvr32` trả về lỗi

**Giải pháp**:
1. Chạy PowerShell với quyền Administrator
2. Kiểm tra đường dẫn DLL chính xác
3. Đảm bảo antivirus không chặn
4. Kiểm tra Windows Event Viewer để xem chi tiết

### Vấn Đề: buttre Không Xuất Hiện Trong Danh Sách Bàn Phím

**Triệu chứng**: Không tìm thấy buttre trong tùy chọn bàn phím

**Giải pháp**:
1. Xác minh DLL đã đăng ký: Kiểm tra `HKEY_CLASSES_ROOT\CLSID\{E6B8A6C0-1234-5678-9ABC-DEF012345678}`
2. Khởi động lại Windows Explorer: `taskkill /f /im explorer.exe && start explorer.exe`
3. Khởi động lại máy tính
4. Kiểm tra đăng ký TSF category trong registry

### Vấn Đề: Không Hiển Thị Composition

**Triệu chứng**: Gõ không hiện gì hoặc hiện ký tự trực tiếp

**Giải pháp**:
1. Xác minh buttre đang là phương thức nhập đang hoạt động (kiểm tra language bar)
2. Chuyển đổi phương thức nhập: Windows + Space
3. Kiểm tra logs: `%TEMP%\buttre_tsf.log`
4. Khởi động lại ứng dụng (Notepad, v.v.)

### Vấn Đề: Backspace Không Hoạt Động

**Triệu chứng**: Backspace xóa cả từ thay vì xóa thông minh

**Giải pháp**:
1. Đây là hành vi mong đợi trong một số ứng dụng
2. Thử trong Notepad trước (hỗ trợ TSF tốt nhất)
3. Kiểm tra xem composition có đang hoạt động không (phải có gạch chân)

---

## Hành Vi Mong Đợi

### Các Trạng Thái Composition

1. **Không có Composition**
   - Gõ bình thường
   - Không có gạch chân
   - Đầu ra ký tự trực tiếp

2. **Composition Đang Hoạt Động**
   - Gạch chân chấm dưới text
   - Cập nhật thời gian thực khi gõ
   - Backspace xóa sửa đổi cuối cùng

3. **Đã Hoàn Thành**
   - Gạch chân biến mất
   - Text được commit vào tài liệu
   - Engine reset

### Bảng Phím Telex

| Phím | Đầu Ra | Mô Tả |
|------|--------|-------|
| `aa` | `â` | Mũ |
| `aw` | `ă` | Trăng |
| `dd` | `đ` | D-gạch |
| `ee` | `ê` | Mũ |
| `oo` | `ô` | Mũ |
| `ow` | `ơ` | Râu |
| `uw` | `ư` | Râu |
| `w` (sau nguyên âm) | Thêm râu/trăng | Modifier |
| `f` | Huyền (`) | Dấu thanh |
| `s` | Sắc (´) | Dấu thanh |
| `r` | Hỏi (?) | Dấu thanh |
| `x` | Ngã (~) | Dấu thanh |
| `j` | Nặng (.) | Dấu thanh |
| `z` | Xóa dấu | Undo |

---

## Thông Tin Debug

### Vị Trí Log
Log được ghi vào: `%TEMP%\buttre_tsf.log`

Xem log:
```powershell
Get-Content "$env:TEMP\buttre_tsf.log" -Tail 50 -Wait
```

### Vị Trí Registry

**Đăng Ký COM**:
- `HKEY_CLASSES_ROOT\CLSID\{E6B8A6C0-1234-5678-9ABC-DEF012345678}`

**TSF Categories**:
- `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\CTF\TIP\{E6B8A6C0-1234-5678-9ABC-DEF012345678}`

**Language Profile**:
- `HKEY_CURRENT_USER\Software\Microsoft\CTF\Assemblies\0x0000042a` (tiếng Việt)

---

## Đã Cài Đặt

- ✅ Nhập Telex tiếng Việt
- ✅ Composition thời gian thực
- ✅ Backspace thông minh
- ✅ Chữ hoa/chữ thường (hỗ trợ Shift)
- ✅ Tự động hoàn thành khi Space/Enter
- ✅ Thuộc tính hiển thị (gạch chân chấm)
- ✅ Dấu thanh (f, s, r, x, j, z)
- ✅ Dấu phụ âm (aa, aw, dd, ee, oo, ow, uw, w)

## Chưa Cài Đặt

- ❌ Phương thức nhập VNI (code đã có, chưa có UI để chuyển)
- ❌ UI candidate (không cần thiết cho tiếng Việt)
- ❌ Hỗ trợ Hán Nôm
- ❌ UI cài đặt
- ❌ Cấu hình hotkey
- ❌ Chỉ thị chế độ

---

## Tiêu Chí Thành Công

### Tối Thiểu
- [ ] DLL đăng ký thành công
- [ ] Xuất hiện trong danh sách bàn phím
- [ ] Có thể chuyển sang nhập buttre
- [ ] Telex cơ bản hoạt động (`hoaf` → `hoà`)
- [ ] Backspace hoạt động
- [ ] Dấu cách hoàn thành composition

### Đầy Đủ
- [ ] Tất cả tổ hợp Telex hoạt động
- [ ] Nhập chữ hoa hoạt động
- [ ] Thay đổi dấu thanh hoạt động (z, sau đó s)
- [ ] Nhiều từ hoạt động
- [ ] Hoạt động trong Notepad
- [ ] Hoạt động trong Word
- [ ] Không bị crash

---

## Gỡ Cài Đặt

### Bước 1: Xóa Khỏi Cài Đặt Ngôn Ngữ
1. Settings → Language → Vietnamese → Options
2. Xóa bàn phím **buttre**

### Bước 2: Hủy Đăng Ký DLL
```powershell
# Chạy với quyền Administrator
regsvr32 /u "$env:ProgramFiles\buttre\buttre_platform.dll"
```

### Bước 3: Xóa File
```powershell
Remove-Item "$env:ProgramFiles\buttre" -Recurse -Force
```
