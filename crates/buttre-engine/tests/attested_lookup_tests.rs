//! Tests for `pipeline::validation::{is_attested, is_shape_attested,
//! is_attested_overlay}` — the embedded attested-Vietnamese-syllable bitset
//! lookup, plus its Phase 5 user-attested-overlay OR-check.

use std::collections::HashSet;

use buttre_engine::pipeline::validation::{
    bit_index, decompose_ids, is_attested, is_attested_overlay, is_shape_attested,
};

#[test]
fn attested_true_for_common_words() {
    for w in ["việt", "đông", "cân", "dật", "gì", "gìn"] {
        assert!(is_attested(w), "{w} should be attested");
    }
}

#[test]
fn attested_false_for_non_words() {
    for w in ["dât", "mêm", "phôt", "fâllb"] {
        assert!(!is_attested(w), "{w} should NOT be attested");
    }
}

#[test]
fn shape_attested_ignores_tone() {
    // "nhât" itself carries no tone, but "nhất" (nh + â + t + sắc) is
    // attested — the shape (onset nh, nucleus â, coda t) must match under
    // ANY tone.
    assert!(
        is_shape_attested("nhât"),
        "nhât shape should be attested via nhất"
    );
    assert!(
        is_shape_attested("nhất"),
        "nhất itself must also match its own shape"
    );
}

#[test]
fn shape_attested_false_for_unattested_shape() {
    assert!(!is_shape_attested("fâllb"));
}

#[test]
fn attested_old_and_new_tone_placement_agree() {
    // "hoà" (old placement) and "hòa" (new placement) are the same syllable;
    // both must decompose to the same (onset, nucleus, coda, tone) tuple.
    assert!(is_attested("hoà"));
    assert!(is_attested("hòa"));
    assert_eq!(is_attested("hoà"), is_attested("hòa"));
}

#[test]
fn attested_handles_nfd_and_uppercase() {
    use unicode_normalization::UnicodeNormalization;

    let nfc = "việt";
    let nfd: String = nfc.nfd().collect();
    assert_ne!(
        nfc.as_bytes(),
        nfd.as_bytes(),
        "test fixture must actually be NFD-decomposed"
    );

    assert!(is_attested(nfc));
    assert!(
        is_attested(&nfd),
        "NFD input must decompose the same as NFC"
    );
    assert!(is_attested("VIỆT"), "uppercase input must be handled");
    assert!(is_attested("Việt"), "title-case input must be handled");
}

#[test]
fn attested_empty_and_garbage_input_fail_open() {
    // Fail-open: unparseable input returns false, never panics.
    assert!(!is_attested(""));
    assert!(!is_attested("123"));
    assert!(!is_attested("!!!"));
    assert!(!is_shape_attested(""));
    assert!(!is_shape_attested("123"));
}

#[test]
fn attested_rejects_conflicting_double_tone() {
    // Two different tone diacritics mashed together is not a real syllable;
    // decompose_ids must fail closed (return false) rather than pick one.
    assert!(!is_attested("việtầ"));
}

#[test]
fn attested_true_for_reembedded_k_coda_place_names() {
    // P6: the former "k-coda" skip category (data/attested-syllables.txt
    // header) is resolved — these 9 dict entries now decompose and are
    // embedded in the bitset, same as any other attested syllable.
    for w in ["đắk", "đăk", "lắk", "lăk", "măk", "ắk", "ăk", "búk", "úk"] {
        assert!(
            is_attested(w),
            "{w} should now be attested (P6 coda-k re-embed)"
        );
    }
}

#[test]
fn attested_false_for_unattested_k_coda_shapes() {
    // No dict evidence exists for these — the per-nucleus gate in
    // `is_valid_combination` keeps them off the bitset AND off the shape
    // check; a blanket "k" coda allowance would wrongly relax this.
    for w in ["đik", "đok", "đek"] {
        assert!(!is_attested(w), "{w} should NOT be attested");
        assert!(
            !is_shape_attested(w),
            "{w} should NOT be shape-attested either"
        );
    }
}

// ── Phase 5: user-attested overlay OR-check ──────────────────────────────────

#[test]
fn overlay_none_is_byte_identical_to_is_attested() {
    for w in ["việt", "dât", "mêm", "fâllb", ""] {
        assert_eq!(
            is_attested_overlay(w, None),
            is_attested(w),
            "{w}: None overlay must match is_attested exactly"
        );
    }
}

#[test]
fn overlay_empty_set_is_byte_identical_to_is_attested() {
    let empty = HashSet::new();
    for w in ["việt", "dât", "mêm", "fâllb"] {
        assert_eq!(
            is_attested_overlay(w, Some(&empty)),
            is_attested(w),
            "{w}: an empty overlay must match is_attested exactly"
        );
    }
}

#[test]
fn overlay_rescues_an_unattested_but_decomposable_syllable() {
    // "dât" is decomposable but NOT in the static bitset (see
    // `attested_false_for_non_words`). Adding its bit to the overlay must
    // flip `is_attested_overlay` to `true`, while leaving the STATIC
    // `is_attested` result unaffected (the overlay never mutates the
    // embedded bitset — it is data ORed in at the call site).
    assert!(!is_attested("dât"));
    let (o, n, c, t) = decompose_ids("dât").expect("'dât' must be a decomposable shape");
    let mut overlay = HashSet::new();
    overlay.insert(bit_index(o, n, c, t) as u32);

    assert!(
        is_attested_overlay("dât", Some(&overlay)),
        "overlay bit must rescue an unattested-but-learned syllable"
    );
    assert!(
        !is_attested("dât"),
        "the static table itself must be unaffected by the overlay"
    );
}

#[test]
fn overlay_does_not_rescue_an_unrelated_syllable() {
    // The overlay is bit-exact: adding "dât"'s bit must not accidentally
    // rescue a completely different unattested syllable.
    let (o, n, c, t) = decompose_ids("dât").expect("'dât' must be a decomposable shape");
    let mut overlay = HashSet::new();
    overlay.insert(bit_index(o, n, c, t) as u32);

    assert!(!is_attested("mêm"));
    assert!(
        !is_attested_overlay("mêm", Some(&overlay)),
        "an unrelated overlay bit must not rescue a different syllable"
    );
}

#[test]
fn overlay_fails_open_on_unparseable_input_just_like_is_attested() {
    let mut overlay = HashSet::new();
    overlay.insert(0u32); // arbitrary bit — irrelevant since decompose fails first
    assert!(!is_attested_overlay("!!!", Some(&overlay)));
    assert!(!is_attested_overlay("", Some(&overlay)));
}
