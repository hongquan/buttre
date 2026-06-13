//! Vietnamese character decomposition tables for Telex and VNI.
//!
//! Each Vietnamese character maps to (base_ascii, optional_extra_key, optional_tone).
//! The two-char sequences (base + extra) are the transform keys (aa→â, ow→ơ…).
//! Tone is emitted as a suffix key at the end of the syllable.

/// Tone category (language-level, not key-level).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VnTone {
    Sac,   // acute  (sắc)
    Huyen, // grave  (huyền)
    Hoi,   // hook   (hỏi)
    Nga,   // tilde  (ngã)
    Nang,  // dot    (nặng)
}

/// Decompose a Vietnamese char into `(base, telex_extra, tone)`.
/// Plain ASCII returns `(c, None, None)`.
pub fn decompose_telex(ch: char) -> (char, Option<char>, Option<VnTone>) {
    match ch {
        'đ' => ('d', Some('d'), None),   'Đ' => ('D', Some('D'), None),
        'â' => ('a', Some('a'), None),   'ấ' => ('a', Some('a'), Some(VnTone::Sac)),
        'ầ' => ('a', Some('a'), Some(VnTone::Huyen)), 'ẩ' => ('a', Some('a'), Some(VnTone::Hoi)),
        'ẫ' => ('a', Some('a'), Some(VnTone::Nga)),   'ậ' => ('a', Some('a'), Some(VnTone::Nang)),
        'ă' => ('a', Some('w'), None),   'ắ' => ('a', Some('w'), Some(VnTone::Sac)),
        'ằ' => ('a', Some('w'), Some(VnTone::Huyen)), 'ẳ' => ('a', Some('w'), Some(VnTone::Hoi)),
        'ẵ' => ('a', Some('w'), Some(VnTone::Nga)),   'ặ' => ('a', Some('w'), Some(VnTone::Nang)),
        'á' => ('a', None, Some(VnTone::Sac)),  'à' => ('a', None, Some(VnTone::Huyen)),
        'ả' => ('a', None, Some(VnTone::Hoi)),  'ã' => ('a', None, Some(VnTone::Nga)),
        'ạ' => ('a', None, Some(VnTone::Nang)),
        'ê' => ('e', Some('e'), None),   'ế' => ('e', Some('e'), Some(VnTone::Sac)),
        'ề' => ('e', Some('e'), Some(VnTone::Huyen)), 'ể' => ('e', Some('e'), Some(VnTone::Hoi)),
        'ễ' => ('e', Some('e'), Some(VnTone::Nga)),   'ệ' => ('e', Some('e'), Some(VnTone::Nang)),
        'é' => ('e', None, Some(VnTone::Sac)),  'è' => ('e', None, Some(VnTone::Huyen)),
        'ẻ' => ('e', None, Some(VnTone::Hoi)),  'ẽ' => ('e', None, Some(VnTone::Nga)),
        'ẹ' => ('e', None, Some(VnTone::Nang)),
        'í' => ('i', None, Some(VnTone::Sac)),  'ì' => ('i', None, Some(VnTone::Huyen)),
        'ỉ' => ('i', None, Some(VnTone::Hoi)),  'ĩ' => ('i', None, Some(VnTone::Nga)),
        'ị' => ('i', None, Some(VnTone::Nang)),
        'ô' => ('o', Some('o'), None),   'ố' => ('o', Some('o'), Some(VnTone::Sac)),
        'ồ' => ('o', Some('o'), Some(VnTone::Huyen)), 'ổ' => ('o', Some('o'), Some(VnTone::Hoi)),
        'ỗ' => ('o', Some('o'), Some(VnTone::Nga)),   'ộ' => ('o', Some('o'), Some(VnTone::Nang)),
        'ơ' => ('o', Some('w'), None),   'ớ' => ('o', Some('w'), Some(VnTone::Sac)),
        'ờ' => ('o', Some('w'), Some(VnTone::Huyen)), 'ở' => ('o', Some('w'), Some(VnTone::Hoi)),
        'ỡ' => ('o', Some('w'), Some(VnTone::Nga)),   'ợ' => ('o', Some('w'), Some(VnTone::Nang)),
        'ó' => ('o', None, Some(VnTone::Sac)),  'ò' => ('o', None, Some(VnTone::Huyen)),
        'ỏ' => ('o', None, Some(VnTone::Hoi)),  'õ' => ('o', None, Some(VnTone::Nga)),
        'ọ' => ('o', None, Some(VnTone::Nang)),
        'ư' => ('u', Some('w'), None),   'ứ' => ('u', Some('w'), Some(VnTone::Sac)),
        'ừ' => ('u', Some('w'), Some(VnTone::Huyen)), 'ử' => ('u', Some('w'), Some(VnTone::Hoi)),
        'ữ' => ('u', Some('w'), Some(VnTone::Nga)),   'ự' => ('u', Some('w'), Some(VnTone::Nang)),
        'ú' => ('u', None, Some(VnTone::Sac)),  'ù' => ('u', None, Some(VnTone::Huyen)),
        'ủ' => ('u', None, Some(VnTone::Hoi)),  'ũ' => ('u', None, Some(VnTone::Nga)),
        'ụ' => ('u', None, Some(VnTone::Nang)),
        'ý' => ('y', None, Some(VnTone::Sac)),  'ỳ' => ('y', None, Some(VnTone::Huyen)),
        'ỷ' => ('y', None, Some(VnTone::Hoi)),  'ỹ' => ('y', None, Some(VnTone::Nga)),
        'ỵ' => ('y', None, Some(VnTone::Nang)),
        c => (c, None, None),
    }
}

/// Decompose a Vietnamese char into `(base, vni_extra_digit, tone)`.
/// Plain ASCII returns `(c, None, None)`.
pub fn decompose_vni(ch: char) -> (char, Option<char>, Option<VnTone>) {
    match ch {
        'đ' => ('d', Some('9'), None),   'Đ' => ('D', Some('9'), None),
        'â' => ('a', Some('6'), None),   'ấ' => ('a', Some('6'), Some(VnTone::Sac)),
        'ầ' => ('a', Some('6'), Some(VnTone::Huyen)), 'ẩ' => ('a', Some('6'), Some(VnTone::Hoi)),
        'ẫ' => ('a', Some('6'), Some(VnTone::Nga)),   'ậ' => ('a', Some('6'), Some(VnTone::Nang)),
        'ă' => ('a', Some('8'), None),   'ắ' => ('a', Some('8'), Some(VnTone::Sac)),
        'ằ' => ('a', Some('8'), Some(VnTone::Huyen)), 'ẳ' => ('a', Some('8'), Some(VnTone::Hoi)),
        'ẵ' => ('a', Some('8'), Some(VnTone::Nga)),   'ặ' => ('a', Some('8'), Some(VnTone::Nang)),
        'á' => ('a', None, Some(VnTone::Sac)),  'à' => ('a', None, Some(VnTone::Huyen)),
        'ả' => ('a', None, Some(VnTone::Hoi)),  'ã' => ('a', None, Some(VnTone::Nga)),
        'ạ' => ('a', None, Some(VnTone::Nang)),
        'ê' => ('e', Some('6'), None),   'ế' => ('e', Some('6'), Some(VnTone::Sac)),
        'ề' => ('e', Some('6'), Some(VnTone::Huyen)), 'ể' => ('e', Some('6'), Some(VnTone::Hoi)),
        'ễ' => ('e', Some('6'), Some(VnTone::Nga)),   'ệ' => ('e', Some('6'), Some(VnTone::Nang)),
        'é' => ('e', None, Some(VnTone::Sac)),  'è' => ('e', None, Some(VnTone::Huyen)),
        'ẻ' => ('e', None, Some(VnTone::Hoi)),  'ẽ' => ('e', None, Some(VnTone::Nga)),
        'ẹ' => ('e', None, Some(VnTone::Nang)),
        'í' => ('i', None, Some(VnTone::Sac)),  'ì' => ('i', None, Some(VnTone::Huyen)),
        'ỉ' => ('i', None, Some(VnTone::Hoi)),  'ĩ' => ('i', None, Some(VnTone::Nga)),
        'ị' => ('i', None, Some(VnTone::Nang)),
        'ô' => ('o', Some('6'), None),   'ố' => ('o', Some('6'), Some(VnTone::Sac)),
        'ồ' => ('o', Some('6'), Some(VnTone::Huyen)), 'ổ' => ('o', Some('6'), Some(VnTone::Hoi)),
        'ỗ' => ('o', Some('6'), Some(VnTone::Nga)),   'ộ' => ('o', Some('6'), Some(VnTone::Nang)),
        'ơ' => ('o', Some('7'), None),   'ớ' => ('o', Some('7'), Some(VnTone::Sac)),
        'ờ' => ('o', Some('7'), Some(VnTone::Huyen)), 'ở' => ('o', Some('7'), Some(VnTone::Hoi)),
        'ỡ' => ('o', Some('7'), Some(VnTone::Nga)),   'ợ' => ('o', Some('7'), Some(VnTone::Nang)),
        'ó' => ('o', None, Some(VnTone::Sac)),  'ò' => ('o', None, Some(VnTone::Huyen)),
        'ỏ' => ('o', None, Some(VnTone::Hoi)),  'õ' => ('o', None, Some(VnTone::Nga)),
        'ọ' => ('o', None, Some(VnTone::Nang)),
        'ư' => ('u', Some('7'), None),   'ứ' => ('u', Some('7'), Some(VnTone::Sac)),
        'ừ' => ('u', Some('7'), Some(VnTone::Huyen)), 'ử' => ('u', Some('7'), Some(VnTone::Hoi)),
        'ữ' => ('u', Some('7'), Some(VnTone::Nga)),   'ự' => ('u', Some('7'), Some(VnTone::Nang)),
        'ú' => ('u', None, Some(VnTone::Sac)),  'ù' => ('u', None, Some(VnTone::Huyen)),
        'ủ' => ('u', None, Some(VnTone::Hoi)),  'ũ' => ('u', None, Some(VnTone::Nga)),
        'ụ' => ('u', None, Some(VnTone::Nang)),
        'ý' => ('y', None, Some(VnTone::Sac)),  'ỳ' => ('y', None, Some(VnTone::Huyen)),
        'ỷ' => ('y', None, Some(VnTone::Hoi)),  'ỹ' => ('y', None, Some(VnTone::Nga)),
        'ỵ' => ('y', None, Some(VnTone::Nang)),
        c => (c, None, None),
    }
}

pub fn telex_tone_key(tone: VnTone) -> char {
    match tone { VnTone::Sac=>'s', VnTone::Huyen=>'f', VnTone::Hoi=>'r', VnTone::Nga=>'x', VnTone::Nang=>'j' }
}

pub fn vni_tone_key(tone: VnTone) -> char {
    match tone { VnTone::Sac=>'1', VnTone::Huyen=>'2', VnTone::Hoi=>'3', VnTone::Nga=>'4', VnTone::Nang=>'5' }
}
