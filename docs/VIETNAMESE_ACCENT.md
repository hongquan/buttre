# ĐẶC TẢ VỊ TRÍ DẤU THANH TIẾNG VIỆT buttre (FINAL)

## 1. PHASE 1: BIẾN ĐỔI KÝ TỰ (Mũ, Râu, Trăng)

**Mục tiêu:** Biến đổi các tổ hợp phím (Telex/VNI) thành ký tự tiếng Việt nguyên bản trước khi bỏ dấu.

* **Quy tắc Mũ (Hat):**
  * `a` + `a` → `â`
  * `e` + `e` → `ê`
  * `o` + `o` → `ô`

* **Quy tắc Râu/Trăng (Phím W):**
  * `a` + `w` → `ă`
  * `o` + `w` → `ơ`
  * `u` + `w` → `ư`
  * `uo` + `w` → `ươ` (**Quan trọng**: Biến đổi cả cụm)

* **Phím D:** `d` + `d` → `đ`

---

## 2. PHASE 2: PARSER & CHUẨN HÓA

**Mục tiêu:** Tách từ thành 3 phần: `[Âm Đầu]` + `[Nhân Nguyên Âm]` + `[Âm Cuối]`

1. **TRƯỜNG HỢP ĐẶC BIỆT "QU":**
   * `NẾU` bắt đầu bằng `qu` (không phân biệt hoa/thường):
     * `Âm Đầu` = "qu"
     * `Nhân Nguyên Âm` = Lấy từ ký tự index 2 trở đi.
     * *(Lý do: `u` là âm đệm, không nhận dấu)*

2. **TRƯỜNG HỢP ĐẶC BIỆT "GI":**
   * `NẾU` bắt đầu bằng `gi`:
     * `Kiểm tra ký tự tiếp theo`:
       * Nếu là nguyên âm (`a`, `o`, `u`, `ê`...): `Âm Đầu` = "gi" (bỏ `i`). `Nhân Nguyên Âm` = phần còn lại. *(Ví dụ: già → Core "a")*
       * Nếu là phụ âm hoặc hết từ: `Âm Đầu` = "g". `Nhân Nguyên Âm` = "i" + phần còn lại. *(Ví dụ: gì → Core "i")*

3. **TRƯỜNG HỢP THÔNG THƯỜNG:**
   * Tách phụ âm đầu thường (`b`, `c`, `ch`, `ng`...).
   * Phần còn lại chia làm `Nhân Nguyên Âm` (cụm nguyên âm) và `Âm Cuối` (phụ âm cuối nếu có).

---

## 3. PHASE 3: LOGIC VỊ TRÍ DẤU THANH

Áp dụng theo thứ tự ưu tiên (Priority Queue). Hễ khớp Priority nào thì dừng và bỏ dấu ngay.

**PRIORITY 1: NGUYÊN ÂM BẤT BIẾN**

* **Danh sách:** `ê`, `ô`, `ơ`, `ă`, `â`
* **Logic:** Quét trong `Nhân Nguyên Âm`. Nếu thấy bất kỳ ký tự nào trong danh sách trên → Bỏ dấu ngay vào nó.
* *Ví dụ:* `Huế` (có ê), `Tuấn` (có â), `thuở` (có ơ)

**PRIORITY 1.5: XỬ LÝ CHỮ "Ư"**

* `NẾU` chứa `ư`:
  * Nếu có `ơ` (vần `ươ`): Đã bị bắt ở Priority 1 → Dấu vào `ơ`
  * Nếu KHÔNG có `ơ` (vần `ưa`, `ưi`, `ưu`): Xử lý như vần thường (xuống Priority 3)

**PRIORITY 2: BA NGUYÊN ÂM**

* `NẾU` độ dài `Nhân Nguyên Âm` = 3 (và không chứa Nguyên Âm Bất Biến):
  * `THÌ`: Bỏ dấu vào **nguyên âm thứ 2** (ở giữa)
  * *Ví dụ:* `ngoại` (oai), `khuỷu` (uyu)

**PRIORITY 3: HAI NGUYÊN ÂM**

* `NẾU` độ dài `Nhân Nguyên Âm` = 2:
  * **Trường hợp 3.1: Có Âm Cuối (Vần Khép)**
    * `THÌ`: Luôn bỏ dấu vào **nguyên âm thứ 2**
    * *Ví dụ:* `toán`, `tuân`, `cười`, `huỳnh`

  * **Trường hợp 3.2: Không có Âm Cuối (Vần Mở)**
    * **Sub-case A:** Nhóm `ia`, `ua`, `ưa` → Bỏ dấu **nguyên âm thứ 1**. (*mía, múa, dứa*)
    * **Sub-case B:** Nhóm `oa`, `oe`, `uy` (Vùng Config):
      * `NẾU Config == CLASSIC` (Mặc định): Bỏ dấu **nguyên âm thứ 1** (*hòa, hòe, thủy*)
      * `NẾU Config == MODERN`: Bỏ dấu **nguyên âm thứ 2** (*hoà, hoè, thuỷ*)
    * **Sub-case C:** Các cặp còn lại (`ai`, `oi`...) → Bỏ dấu **nguyên âm thứ 1**. (*cái, bói*)

**PRIORITY 4: MỘT NGUYÊN ÂM**

* `NẾU` độ dài `Nhân Nguyên Âm` = 1 → Bỏ dấu vào chính nó.

---

## Ví Dụ Test Case (Dành Cho Developer)

1. **Input:** `g`, `i`, `a`, `s` (giá)
   * Parser: `Âm Đầu`="gi", `Core`="a"
   * Logic: Priority 4 (Đơn) → `á`
   * Output: **Giá**

2. **Input:** `t`, `u`, `a`, `a`, `n`, `s` (tuấn)
   * Biến đổi: `aa` → `â` → `tuân` + `s`
   * Parser: `Âm Đầu`="t", `Core`="uâ", `Âm Cuối`="n"
   * Logic: Priority 1 (thấy `â`) → Dấu vào `â`
   * Output: **Tuấn**

3. **Input:** `h`, `u`, `y`, `n`, `h`, `f` (huỳnh)
   * Parser: `Âm Đầu`="h", `Core`="uy", `Âm Cuối`="nh"
   * Logic: Priority 3.1 (Có âm cuối) → Dấu vào số 2 (`y`)
   * Output: **Huỳnh**

4. **Input:** `h`, `o`, `a`, `f` (hòa/hoà)
   * Parser: `Core`="oa", không có âm cuối
   * Logic: Priority 3.2.B (phụ thuộc Config)
   * Output: **Hòa** (Classic) hoặc **Hoà** (Modern)
