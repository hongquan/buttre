# Nhật Ký Thay Đổi

Tất cả thay đổi đáng chú ý của buttre được ghi lại tại đây. Định dạng theo [Keep a Changelog](https://keepachangelog.com); phiên bản theo SemVer.

## [Unreleased]

### Cổng chứng thực âm tiết cho transform không liền kề (engine)
- **Sửa lớp lỗi `"data"` → `"dât"`**: nhúng bảng 7.884 âm tiết tiếng Việt có thật (từ điển `ibus-bamboo`, GPLv3) dưới dạng bitset nén (~13 KB). Mark suy luận KHÔNG liền kề (gõ nhanh xen kẽ, dấu thanh chen giữa hai nguyên âm như `reset`/`nasa`, `đ` suy ra từ `d` cuối từ...) chỉ được giữ khi âm tiết ghép ra là âm tiết có thật; nếu không sẽ giáng cấp về đúng ký tự gốc đã gõ. Áp dụng cho cả Telex và VNI; các đường tái dựng prefix trong fallback (toggle dấu thanh/transform) cũng đi qua cùng cổng này để không bị bỏ sót (`dataeee`, `vietess`, `databaaa` không còn rò rỉ dấu phụ âm). VIQR chưa triển khai — cổng tự áp dụng khi preset VIQR hoàn thiện, không cần sửa thêm.
- **Hoàn tác không liền kề** (escape hatch cho va chạm âm tiết có thật): gõ lại đúng phím trigger ngay sau khi nó vừa kích hoạt sẽ hoàn nguyên về ký tự gốc — Telex `cana`+`a` → `cana`; VNI `can6`+`6` → `can6`; `đ` suy luận `dand`+`d` → `dand`. Điều kiện tức thời (immediacy): phím gõ lại phải là phím cuối cùng của chuỗi vừa gõ, nếu không sẽ không hoàn tác (`vietej`+`e` không hoàn tác vì phím cuối lúc đó là dấu nặng `j`).
- **Va chạm âm tiết có thật được chấp nhận theo thiết kế**: cổng chứng thực không thể và không nên phân biệt gõ tiếng Anh với một transform tiếng Việt hợp lệ khi kết quả trùng một từ có thật — `reset` → `rết` (con rết), `cana`/`can6` → `cân`, `dand` → `đan`. Lối thoát duy nhất là hoàn tác không liền kề ở trên, không phải sửa lỗi từng từ.
- Loại bỏ các guard heuristic vá rời rạc nay được cổng chứng thực chung thay thế; giữ lại guard VNI `"ie"` (bảo vệ trạng thái trung gian gõ dấu thanh TRƯỚC transform digit, ví dụ `mieng16` → `miếng`, không bị cổng chứng thực bao trùm vì lúc đó chưa có mark nào được áp dụng) và guard `đ` không dấu (`dad` vẫn giữ nguyên tiếng Anh vì không có nguyên âm theo sau `d` thứ hai).
- Mở rộng golden snapshot Telex/VNI với các từ tiếng Anh mới (`meme`, `photo`, `papa`, `salsa`, `radar`, `banana`, `canal`, `media`, `dad`, `dads`, `nasa`) để chứng minh việc sửa lỗi trên toàn bộ corpus, không chỉ riêng `data`.
- Cập nhật `docs/PIPELINE_ARCHITECTURE.md` với bước cổng chứng thực trong `compose()`.

## [0.7.4-beta] — 2026-06-19

### Gõ nhanh, xóa & sửa từ (Windows hook)
- Sửa lỗi **rớt phím khi gõ nhanh**: đường xử lý phím trong hook dùng `try_write()` và bỏ qua phím khi tranh chấp lock, khiến phím thô lọt lên màn hình còn buffer engine tụt lại → lệch `last_output`. Nay dùng `write()` blocking, chịu poison — không bao giờ bỏ phím.
- Sửa lỗi **nhảy ngược lên dòng/từ trên** khi nhấn Enter (hoặc Tab/mũi tên) rồi gõ tiếp: phím ranh giới từ không reset chắc chắn do read-lock giữ chéo write-lock; nay reset blocking + ép `KEYBOARD_DIRTY` để luôn reset trên ranh giới.
- **Backspace nhận biết grapheme, giữ từ đang gõ**: xóa đúng 1 ký tự hiển thị nhưng vẫn cho phép bỏ dấu/sửa lại từ đang gõ (`việt`→xóa→`việ`→gõ `s`→`viế`), không reset sạch như trước.
- **Cửa sổ nhiều từ (Cách B, như Unikey)**: backspace xuyên dấu cách để sửa/bỏ dấu **1–2 từ trước đó** (`ban cá`→xóa→`ban`→gõ `f`→`bàn`). Window giữ 3 từ gần nhất, từ cũ hơn đóng băng; hard-reset trên Enter/mũi tên/chuột để không lệch khi con trỏ nhảy. Chỉ áp dụng Telex/VNI backend hook (TSF/Nôm giữ đường cũ).
- Chặn `O(n²)`: giới hạn độ dài âm tiết cho recompute (input run-on quá dài → passthrough literal).

### Bộ gõ & chính tả (engine)
- Sửa lỗi bỏ dấu (tone toggle) với từ có phụ âm đầu trùng phím thanh Telex (`seess`→`sês`, `fanss`→`fans`, `sinff`→`sinf`): dùng trailing-run detection đúng theo Unikey/OpenKey.
- Sửa fallback tiếng Anh với từ có nguyên âm lặp xuyên ranh giới phụ âm (`fallback`, `implement`, VNI `color`/`expect`): luật non-adjacent chỉ bắn khi phần trước là một âm tiết tiếng Việt hoàn chỉnh (một nucleus + coda hợp lệ).
- **Bỏ luật `w`→`ư` đầu từ**: từ tiếng Anh bắt đầu bằng `w` (`won`, `with`, `will`, `water`...) gõ tự nhiên; `ư` đầu từ gõ bằng `uw` (`uwng`→`ưng`). `w` chỉ còn là modifier trong `aw`/`ow`/`uw`.
- **Nâng cấp bảng âm vị** (port từ Unikey `VSeqList`/`VCPairList`): bổ sung đầy đủ nuclei (uê, yê, oo loanword, các dạng bare trung gian như `ie`/`uye`...) và ràng buộc nucleus–coda; sửa lỗi cũ từ chối nhầm `iê`+`p/c` (tiếp/biếc).
- **English fallback validation-first**: âm tiết không hợp lệ tiếng Việt sau khi áp dấu/transform → trả literal + chế độ tiếng Anh (`water`→`water`, `wonder`→`wonder`, `result`→`result`).
- **VNI gõ sai thứ tự** (thanh trước transform): `huyen26`→`huyền`, `nguyen64`→`nguyễn`, `quyen26`→`quyền` (thêm dạng bare `uye`).
- **Non-adjacent `đ`** (gõ `d` cuối tạo đ): `datjd`→`đạt`, `datd`→`đat` — chỉ bắn khi âm tiết có coda hoặc dấu thanh (giữ tiếng Anh `dad`→`dad`).
- **`z` xóa dấu** (bỏ dấu) theo chuẩn Telex; `z`/`dz` đầu từ làm phụ âm cho văn phong informal (`dzí dzụ`, `zô`).
- **Kéo dài ký tự** trong văn chương/chat: giữ âm tiết hợp lệ + nối đuôi lặp literal (`khôngggg`, `trờiii`, `ơiii`, `vèoooo`) thay vì fallback cả từ — ưu tiên linh hoạt như Unikey.
- Sửa phiên bản hiển thị trong hộp thoại trợ giúp.

### Tài liệu
- Thêm quy tắc đổi rule gõ tiếng Việt vào `AGENTS.md` (đi qua 7-stage pipeline, thuật toán tổng quát, không hardcode).
- Cập nhật golden snapshot Telex/VNI/Nôm cho các từ bị ảnh hưởng.

## [0.7.1-beta] — 2026-06-14
- Engine — Tái cấu trúc recompute (12 → 7 giai đoạn)
- Thống nhất tất cả bảng dấu thanh và logic vị trí vào `crates/buttre-engine/src/tone/`
- Một pipeline config-driven phục vụ Telex, VNI, VIQR, và Nôm; segment mode (`MarkBased`/`DirectMap`) và validator (`Vietnamese`/`Hmong`/`Custom`/`None`) được chọn qua config, không hardcode.
- Hành vi: VNI `u7o7` các hợp âm compose đúng theo bất kỳ thứ tự nào; English fallback validation-first, undo giữ nguyên transform
- Hiệu năng: ~250 ns–8 µs/lần gõ phím (dưới 1 ms)
- Sửa lỗi bộ cài đặt Windows TSF, macOS FFI và Linux IBus
- Viết lại toàn bộ tài liệu docs/ và README sang tiếng Việt

## [0.6.2-alpha] — 2026-01-13
- Sửa lỗi bỏ digit kiểu "H2O" trong nhập alphanumeric; cải thiện giữ nguyên literal-mark

## [0.6.1-alpha] — 2026-01-10
- Thêm workflow bảo trì tự động bằng agent
- Sửa lỗi desync backspace xuyên từ; mở rộng phát hiện separator

## [0.6.0-alpha] — 2026-01-05
- Mốc kiến trúc core: pipeline 12 giai đoạn, PGO (~1 µs/lần gõ), gõ linh hoạt (permutation), đồng bộ xuyên từ, backend hybrid Hook+TSF, retrofix/undo

## [0.2.0-alpha] — 2025-12-27
- Hiệu năng VNI: bảng dấu thanh được tính sẵn + phát hiện range-based; PGO engine core

## [0.1.0-alpha] — 2025-12-19
- Phát hành đầu tiên. Phương thức: Telex, VNI, Nôm. Nền tảng: Windows (Hook+TSF), Linux (IBus), macOS. Tính năng: English fallback, raw mode, tone toggle, undo
