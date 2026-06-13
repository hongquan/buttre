# buttre VIETNAMESE ACCENT PLACEMENT SPECIFICATION (FINAL)

## 1. PHASE 1: CHARACTER TRANSFORMATION (Mũ, Râu, Trăng)

**Mục tiêu:** Biến đổi các tổ hợp phím (Telex/VNI) thành ký tự tiếng Việt nguyên bản trước khi bỏ dấu.

* **Hat Rules (Mũ):**
* `a` + `a`  `â`
* `e` + `e`  `ê`
* `o` + `o`  `ô`


* **Breve/Horn Rules (Râu/Trăng - W Key):**
* `a` + `w`  `ă`
* `o` + `w`  `ơ`
* `u` + `w`  `ư`
* `uo` + `w`  `ươ` (Quan trọng: Biến cả cụm).


* **D Key:** `d` + `d`  `đ`.

---

## 2. PHASE 2: PARSER & NORMALIZATION

**Mục tiêu:** Tách từ thành 3 phần: `[Initial]` + `[Vowel_Core]` + `[Final]`

1. **SPECIAL CASE "QU":**
* `IF` starts with `qu` (case-insensitive):
* `Initial` = "qu"
* `Vowel_Core` = Lấy từ ký tự index 2 trở đi.
* *(Lý do: `u` là đệm, không nhận dấu).*




2. **SPECIAL CASE "GI":**
* `IF` starts with `gi`:
* `Check Next Char`:
* Nếu là nguyên âm (`a`, `o`, `u`, `ê`...): `Initial` = "gi" (Bỏ `i`). `Vowel_Core` = phần còn lại. *(Ví dụ: già -> Core "a")*.
* Nếu là phụ âm hoặc hết từ: `Initial` = "g". `Vowel_Core` = "i" + phần còn lại. *(Ví dụ: gì -> Core "i")*.






3. **NORMAL CASE:**
* Tách phụ âm đầu thường (`b`, `c`, `ch`, `ng`...).
* Phần còn lại chia làm `Vowel_Core` (Cụm nguyên âm) và `Final` (Phụ âm cuối nếu có).



---

## 3. PHASE 3: ANCHOR LOGIC (Quy tắc bỏ dấu)

Áp dụng theo thứ tự ưu tiên (Priority Queue). Hễ khớp Priority nào thì dừng và bỏ dấu ngay.

**PRIORITY 1: THE "ABSOLUTE VOWELS" (Nhóm Bất Biến)**

* **Danh sách:** `ê`, `ô`, `ơ`, `ă`, `â`.
* **Logic:** Quét trong `Vowel_Core`. Nếu thấy bất kỳ ký tự nào trong danh sách trên  Bỏ dấu ngay vào nó.
* *Ví dụ:* `Huế` (có ê), `Tuấn` (có â), `thuở` (có ơ).



**PRIORITY 1.5: HANDLING "Ư" (Xử lý chữ Ư)**

* `IF` contains `ư`:
* Nếu có `ơ` (vần `ươ`): Đã bị bắt ở Priority 1  Dấu vào `ơ`.
* Nếu KHÔNG có `ơ` (vần `ưa`, `ưi`, `ưu`): Xử lý như vần thường (xuống Priority 3).



**PRIORITY 2: THREE VOWELS (Vần 3 âm)**

* `IF` `Vowel_Core` length = 3 (và không chứa Absolute Vowels):
* `THEN`: Bỏ dấu vào **nguyên âm thứ 2** (giữa).
* *Ví dụ:* `ngoại` (oai), `khuỷu` (uyu).



**PRIORITY 3: TWO VOWELS (Vần 2 âm)**

* `IF` `Vowel_Core` length = 2:
* **Case 3.1: Có Phụ âm cuối (Closed Syllable)**
* `THEN`: Luôn bỏ dấu vào **nguyên âm thứ 2**.
* *Ví dụ:* `toán`, `tuân`, `cười`, `huỳnh`.


* **Case 3.2: Không có Phụ âm cuối (Open Syllable)**
* **Sub-case A:** Nhóm `ia`, `ua`, `ưa`  Bỏ dấu **nguyên âm thứ 1**. (*mía, múa, dứa*).
* **Sub-case B:** Nhóm `oa`, `oe`, `uy` (Vùng Config):
* `IF Config == CLASSIC` (Mặc định): Bỏ dấu **nguyên âm thứ 1** (*hòa, hòe, thủy*).
* `IF Config == MODERN`: Bỏ dấu **nguyên âm thứ 2** (*hoà, hoè, thuỷ*).


* **Sub-case C:** Các cặp còn lại (`ai`, `oi`...)  Bỏ dấu **nguyên âm thứ 1**. (*cái, bói*).





**PRIORITY 4: SINGLE VOWEL**

* `IF` `Vowel_Core` length = 1  Bỏ dấu vào chính nó.

---

### Ví dụ Test Cases (Cho Developer)

1. **Input:** `g`, `i`, `a`, `s` (giá)
* Parser: `Initial`="gi", `Core`="a".
* Logic: Priority 4 (Single) -> `á`.
* Output: **Giá**.


2. **Input:** `t`, `u`, `a`, `a`, `n`, `s` (tuấn)
* Transform: `aa` -> `â` -> `tuân` + `s`.
* Parser: `Initial`="t", `Core`="uâ", `Final`="n".
* Logic: Priority 1 (thấy `â`) -> Dấu vào `â`.
* Output: **Tuấn**.


3. **Input:** `h`, `u`, `y`, `n`, `h`, `f` (huỳnh)
* Parser: `Initial`="h", `Core`="uy", `Final`="nh".
* Logic: Priority 3.1 (Có Final) -> Dấu vào số 2 (`y`).
* Output: **Huỳnh**.


4. **Input:** `h`, `o`, `a`, `f` (hòa/hoà)
* Parser: `Core`="oa", No Final.
* Logic: Priority 3.2.B (Config).
* Output: **Hòa** (Classic) hoặc **Hoà** (Modern).
