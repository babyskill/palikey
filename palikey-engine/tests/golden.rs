//! Golden corpus integration tests for PaliKey engine.
//!
//! Clean-room tests per DESIGN.md §7:
//! 1. Every word in the golden corpus typed in EACH of the three schemes
//!    (Velthuis, PaliTelex, SIL) must produce byte-identical output.
//! 2. Dhammapada verse 1 opening.
//! 3. Backspace-undo single-level reversion.
//! 4. Capital cases (AA->Ā, .T->Ṭ, etc.).
//! 5. Niggahīta variant flag (ṃ U+1E43 vs ṁ U+1E41).
//! 6. Extended Sanskrit/IAST flag.
//! 7. English non-conversion and escape rules.

use palikey_engine::chars::NiggahitaVariant;
use palikey_engine::engine::{Engine, EngineConfig, KeyAction, ProcessResult};
use palikey_engine::schemes::SchemeId;

/// Test fixture representing a golden corpus entry with input strings for each scheme.
struct GoldenCorpusWord {
    expected: &'static str,
    velthuis_input: &'static str,
    palitelex_input: &'static str,
    sil_input: &'static str,
}

// Golden corpus defined in DESIGN.md §7
static GOLDEN_CORPUS: &[GoldenCorpusWord] = &[
    GoldenCorpusWord {
        expected: "sammādiṭṭhi",
        velthuis_input: "sammaadi.t.thi",
        palitelex_input: "sammaaditxtxhi",
        sil_input: "samm-adi.t.thi",
    },
    GoldenCorpusWord {
        expected: "paṭiccasamuppāda",
        velthuis_input: "pa.ticcasamuppaada",
        palitelex_input: "patxiccasamuppaada",
        sil_input: "pa.ticcasamupp-ada",
    },
    GoldenCorpusWord {
        expected: "nibbāna",
        velthuis_input: "nibbaana",
        palitelex_input: "nibbaana",
        sil_input: "nibb-ana",
    },
    GoldenCorpusWord {
        expected: "saṅgha",
        velthuis_input: "sa\"ngha",
        palitelex_input: "sangwgha",
        sil_input: "sa>ngha",
    },
    GoldenCorpusWord {
        expected: "dhammaṃ saraṇaṃ gacchāmi",
        velthuis_input: "dhamma.m sara.na.m gacchaami",
        palitelex_input: "dhammamx saranxamx gacchaami",
        sil_input: "dhamma.m sara.na.m gacch-ami",
    },
    GoldenCorpusWord {
        expected: "buddhaṃ",
        velthuis_input: "buddha.m",
        palitelex_input: "buddhamx",
        sil_input: "buddha.m",
    },
    GoldenCorpusWord {
        expected: "tiṭṭhati",
        velthuis_input: "ti.t.thati",
        palitelex_input: "titxtxhati",
        sil_input: "ti.t.thati",
    },
    GoldenCorpusWord {
        expected: "ñāṇa",
        velthuis_input: "~naa.na",
        palitelex_input: "nwaanxa",
        sil_input: "~n-a.na",
    },
    GoldenCorpusWord {
        expected: "kaṇha",
        velthuis_input: "ka.nha",
        palitelex_input: "kanxha",
        sil_input: "ka.nha",
    },
    GoldenCorpusWord {
        expected: "veḷuriya",
        velthuis_input: "ve.luriya",
        palitelex_input: "velxuriya",
        sil_input: "ve.luriya",
    },
];

/// Helper to simulate human typing into a host application text field.
fn simulate_typing(engine: &mut Engine, input: &str) -> String {
    let mut display = String::new();
    for c in input.chars() {
        let res = if c == '\x08' {
            engine.process(KeyAction::Backspace)
        } else {
            engine.process(KeyAction::Char(c))
        };
        apply_result(&mut display, res);
    }
    display
}

fn apply_result(display: &mut String, res: ProcessResult) {
    for _ in 0..res.backspaces {
        display.pop();
    }
    display.push_str(&res.commit);
}

// =============================================================================
// Test 1: Golden Corpus Convergence
// =============================================================================

#[test]
fn test_golden_corpus_words_identical_across_schemes() {
    for item in GOLDEN_CORPUS {
        let mut eng_v = Engine::with_scheme(SchemeId::Velthuis);
        let mut eng_t = Engine::with_scheme(SchemeId::PaliTelex);
        let mut eng_s = Engine::with_scheme(SchemeId::Sil);

        let out_v = simulate_typing(&mut eng_v, item.velthuis_input);
        let out_t = simulate_typing(&mut eng_t, item.palitelex_input);
        let out_s = simulate_typing(&mut eng_s, item.sil_input);

        assert_eq!(
            out_v, item.expected,
            "Velthuis mismatch for expected '{}': got '{}'",
            item.expected, out_v
        );
        assert_eq!(
            out_t, item.expected,
            "PaliTelex mismatch for expected '{}': got '{}'",
            item.expected, out_t
        );
        assert_eq!(
            out_s, item.expected,
            "SIL mismatch for expected '{}': got '{}'",
            item.expected, out_s
        );

        // Byte-identical check
        assert_eq!(out_v.as_bytes(), item.expected.as_bytes());
        assert_eq!(out_t.as_bytes(), item.expected.as_bytes());
        assert_eq!(out_s.as_bytes(), item.expected.as_bytes());
    }
}

// =============================================================================
// Test 2: Dhammapada Verse 1 Opening
// =============================================================================

#[test]
fn test_dhammapada_verse_1_opening() {
    let expected = "manopubbaṅgamā dhammā";

    let mut eng_v = Engine::with_scheme(SchemeId::Velthuis);
    let mut eng_t = Engine::with_scheme(SchemeId::PaliTelex);
    let mut eng_s = Engine::with_scheme(SchemeId::Sil);

    let out_v = simulate_typing(&mut eng_v, "manopubba\"ngamaa dhammaa");
    let out_t = simulate_typing(&mut eng_t, "manopubbangwgamaa dhammaa");
    let out_s = simulate_typing(&mut eng_s, "manopubba>ngam-a dhamm-a");

    assert_eq!(out_v, expected);
    assert_eq!(out_t, expected);
    assert_eq!(out_s, expected);

    // Verify alias in PaliTelex: 'ngx' also works for ṅ
    let mut eng_t2 = Engine::with_scheme(SchemeId::PaliTelex);
    let out_t2 = simulate_typing(&mut eng_t2, "manopubbangxgamaa dhammaa");
    assert_eq!(out_t2, expected);
}

// =============================================================================
// Test 3: Backspace-Undo Reversion (One Level)
// =============================================================================

#[test]
fn test_backspace_undo_velthuis() {
    let mut eng = Engine::with_scheme(SchemeId::Velthuis);
    let mut display = String::new();

    // Type .t -> becomes ṭ
    for c in ".t".chars() {
        let res = eng.process(KeyAction::Char(c));
        apply_result(&mut display, res);
    }
    assert_eq!(display, "ṭ");

    // Press Backspace immediately: reverts to raw ".t"
    let res = eng.process(KeyAction::Backspace);
    apply_result(&mut display, res);
    assert_eq!(display, ".t");

    // Subsequent Backspace deletes 't'
    let res = eng.process(KeyAction::Backspace);
    apply_result(&mut display, res);
    assert_eq!(display, ".");

    // Subsequent Backspace deletes '.'
    let res = eng.process(KeyAction::Backspace);
    apply_result(&mut display, res);
    assert_eq!(display, "");
}

#[test]
fn test_backspace_undo_palitelex() {
    let mut eng = Engine::with_scheme(SchemeId::PaliTelex);
    let mut display = String::new();

    // Type tx -> becomes ṭ
    for c in "tx".chars() {
        let res = eng.process(KeyAction::Char(c));
        apply_result(&mut display, res);
    }
    assert_eq!(display, "ṭ");

    // Press Backspace immediately: reverts to raw "tx"
    let res = eng.process(KeyAction::Backspace);
    apply_result(&mut display, res);
    assert_eq!(display, "tx");

    // Second Backspace deletes 'x'
    let res = eng.process(KeyAction::Backspace);
    apply_result(&mut display, res);
    assert_eq!(display, "t");
}

#[test]
fn test_backspace_undo_multi_char_sequence() {
    let mut eng = Engine::with_scheme(SchemeId::PaliTelex);
    let mut display = String::new();

    // Type ngw -> becomes ṅ
    for c in "ngw".chars() {
        let res = eng.process(KeyAction::Char(c));
        apply_result(&mut display, res);
    }
    assert_eq!(display, "ṅ");

    // Immediate Backspace reverts to raw sequence "ngw"
    let res = eng.process(KeyAction::Backspace);
    apply_result(&mut display, res);
    assert_eq!(display, "ngw");
}

// =============================================================================
// Test 4: Capital Cases
// =============================================================================

#[test]
fn test_capital_cases_velthuis() {
    let mut eng = Engine::with_scheme(SchemeId::Velthuis);
    assert_eq!(simulate_typing(&mut eng, "AA"), "Ā");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "Aa"), "Ā");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "II"), "Ī");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "UU"), "Ū");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".T"), "Ṭ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".D"), "Ḍ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".N"), "Ṇ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".L"), "Ḷ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "\"N"), "Ṅ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "~N"), "Ñ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".M"), "Ṃ");
}

#[test]
fn test_capital_cases_palitelex() {
    let mut eng = Engine::with_scheme(SchemeId::PaliTelex);
    assert_eq!(simulate_typing(&mut eng, "AA"), "Ā");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "Aa"), "Ā");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "Tx"), "Ṭ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "TX"), "Ṭ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "Dx"), "Ḍ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "Nx"), "Ṇ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "Lx"), "Ḷ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "Ngw"), "Ṅ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "NGW"), "Ṅ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "Nw"), "Ñ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "NW"), "Ñ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "Mx"), "Ṃ");
}

#[test]
fn test_capital_cases_sil() {
    let mut eng = Engine::with_scheme(SchemeId::Sil);
    assert_eq!(simulate_typing(&mut eng, "-A"), "Ā");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "-I"), "Ī");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "-U"), "Ū");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".T"), "Ṭ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".D"), "Ḍ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".N"), "Ṇ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".L"), "Ḷ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ">N"), "Ṅ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "~N"), "Ñ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".M"), "Ṃ");
}

// =============================================================================
// Test 5: Niggahīta Variant Flag (ṃ U+1E43 vs ṁ U+1E41)
// =============================================================================

#[test]
fn test_niggahita_dot_below_default() {
    // Default is DotBelow: ṃ (U+1E43)
    let mut eng_v = Engine::with_scheme(SchemeId::Velthuis);
    let mut eng_t = Engine::with_scheme(SchemeId::PaliTelex);
    let mut eng_s = Engine::with_scheme(SchemeId::Sil);

    assert_eq!(simulate_typing(&mut eng_v, "buddha.m"), "buddhaṃ");
    assert_eq!(simulate_typing(&mut eng_t, "buddhamx"), "buddhaṃ");
    assert_eq!(simulate_typing(&mut eng_s, "buddha.m"), "buddhaṃ");
}

#[test]
fn test_niggahita_dot_above_variant() {
    // Configured for DotAbove: ṁ (U+1E41)
    let config = EngineConfig {
        niggahita: NiggahitaVariant::DotAbove,
        ..EngineConfig::default()
    };

    let mut eng_v = Engine::new(EngineConfig { scheme: SchemeId::Velthuis, ..config.clone() });
    let mut eng_t = Engine::new(EngineConfig { scheme: SchemeId::PaliTelex, ..config.clone() });
    let mut eng_s = Engine::new(EngineConfig { scheme: SchemeId::Sil, ..config });

    assert_eq!(simulate_typing(&mut eng_v, "buddha.m"), "buddhaṁ");
    assert_eq!(simulate_typing(&mut eng_t, "buddhamx"), "buddhaṁ");
    assert_eq!(simulate_typing(&mut eng_s, "buddha.m"), "buddhaṁ");

    // Capital version
    assert_eq!(simulate_typing(&mut eng_v, ".M"), "Ṁ");
    assert_eq!(simulate_typing(&mut eng_t, "Mx"), "Ṁ");
    assert_eq!(simulate_typing(&mut eng_s, ".M"), "Ṁ");
}

// =============================================================================
// Test 6: Extended Sanskrit/IAST Flag
// =============================================================================

#[test]
fn test_extended_set_disabled_by_default() {
    let mut eng = Engine::with_scheme(SchemeId::Velthuis);
    // When extended set is false, 'ee' and 'oo' pass through unmodified
    assert_eq!(simulate_typing(&mut eng, "teesta"), "teesta");
    assert_eq!(simulate_typing(&mut eng, "door"), "door");
    assert_eq!(simulate_typing(&mut eng, ".r"), ".r");
}

#[test]
fn test_extended_set_enabled() {
    let config = EngineConfig {
        scheme: SchemeId::Velthuis,
        extended_set: true,
        ..EngineConfig::default()
    };
    let mut eng = Engine::new(config);

    assert_eq!(simulate_typing(&mut eng, "ee"), "ē");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "oo"), "ō");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".r"), "ṛ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".rr"), "ṝ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".h"), "ḥ");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, "\"s"), "ś");
    eng.reset();
    assert_eq!(simulate_typing(&mut eng, ".s"), "ṣ");
}

// =============================================================================
// Test 7: English Non-conversion and Escape Rules
// =============================================================================

/// DOCUMENTATION OF CHOSEN ENGLISH ESCAPE AND BOUNDARY RULES:
///
/// 1. Pure Pāḷi Pass-Through for Non-Pāḷi Vowels:
///    In core Pāḷi, 'ē' and 'ō' are written as simple Latin 'e' and 'o' (macrons
///    are never used in standard Romanized Pāḷi for e and o since they are
///    always long vowels). Therefore, in the default configuration
///    (extended_set = false), doubling 'oo' or 'ee' does NOT trigger any diacritic.
///    As a direct consequence, English words such as "door", "book", "good",
///    "meet", "speed" remain untouched English text with zero user friction.
///
/// 2. Backspace-Undo Escape for Colliding Sequences (e.g. "aaron"):
///    When a sequence converts (e.g. 'a' + 'a' -> 'ā'), hitting Backspace
///    immediately restores the raw keystrokes ("aa") without deleting them.
///    The user then resumes typing ("ron") to produce "aaron".
///
/// 3. Repeat-Key Escape:
///    Typing the trigger key a third time (e.g. 'a' + 'a' -> 'ā', then 'a' again)
///    reverts 'ā' to the raw expanded string "aaa".
///
/// 4. Prefix Escape Character ('\\'):
///    Typing a backslash '\' acts as an escape prefix: the immediately following
///    character is treated as raw text and is not combined into a transliteration
///    sequence. For example: '\' + 'a' + 'a' -> "aa".
///
/// 5. Word-Boundary and Digits Reset:
///    Encountering any whitespace, boundary punctuation (commas, parentheses,
///    brackets, colons), or ASCII digit flushes the transducer buffer, preventing
///    unintended cross-word or cross-number sequence composition.

#[test]
fn test_english_door_stays_english() {
    // In standard Pāḷi mode, 'door' typed naturally stays 'door' across all schemes
    let mut eng_v = Engine::with_scheme(SchemeId::Velthuis);
    let mut eng_t = Engine::with_scheme(SchemeId::PaliTelex);
    let mut eng_s = Engine::with_scheme(SchemeId::Sil);

    assert_eq!(simulate_typing(&mut eng_v, "door"), "door");
    assert_eq!(simulate_typing(&mut eng_t, "door"), "door");
    assert_eq!(simulate_typing(&mut eng_s, "door"), "door");
}

#[test]
fn test_english_aaron_via_backspace_undo() {
    let mut eng = Engine::with_scheme(SchemeId::Velthuis);
    let mut display = String::new();

    // Type "aa" -> becomes "ā"
    for c in "aa".chars() {
        let res = eng.process(KeyAction::Char(c));
        apply_result(&mut display, res);
    }
    assert_eq!(display, "ā");

    // Hit Backspace: reverts to raw "aa"
    let res = eng.process(KeyAction::Backspace);
    apply_result(&mut display, res);
    assert_eq!(display, "aa");

    // Type "ron"
    for c in "ron".chars() {
        let res = eng.process(KeyAction::Char(c));
        apply_result(&mut display, res);
    }
    assert_eq!(display, "aaron");
}

#[test]
fn test_english_aaron_via_backslash_escape() {
    let mut eng = Engine::with_scheme(SchemeId::Velthuis);
    let mut display = String::new();

    // Typing '\\' escapes the first 'a'
    let res = eng.process(KeyAction::Char('\\'));
    apply_result(&mut display, res);

    for c in "aaron".chars() {
        let res = eng.process(KeyAction::Char(c));
        apply_result(&mut display, res);
    }
    assert_eq!(display, "aaron");
}

#[test]
fn test_english_repeat_key_escape() {
    let mut eng = Engine::with_scheme(SchemeId::Velthuis);
    // "aa" -> "ā", pressing 'a' again expands to "aaa"
    assert_eq!(simulate_typing(&mut eng, "aaa"), "aaa");
}

#[test]
fn test_digits_and_word_boundaries_flush_composition() {
    let mut eng = Engine::with_scheme(SchemeId::Velthuis);

    // Period followed by space, digit, then 't' does NOT form .t
    assert_eq!(simulate_typing(&mut eng, ". 1 t"), ". 1 t");

    // Period followed by comma then 't' does NOT form .t
    assert_eq!(simulate_typing(&mut eng, ".,t"), ".,t");
}

// =============================================================================
// Test 8: Vietnamese Telex Golden Corpus (Phase 5, >= 150 cases)
// =============================================================================

struct VietTelexCase {
    input: &'static str,
    expected: &'static str,
}

static VIETNAMESE_TELEX_CORPUS: &[VietTelexCase] = &[
    // 1. Modifiers (9 cases)
    VietTelexCase { input: "aa", expected: "â" },
    VietTelexCase { input: "ee", expected: "ê" },
    VietTelexCase { input: "oo", expected: "ô" },
    VietTelexCase { input: "aw", expected: "ă" },
    VietTelexCase { input: "ow", expected: "ơ" },
    VietTelexCase { input: "uw", expected: "ư" },
    VietTelexCase { input: "dd", expected: "đ" },
    VietTelexCase { input: "w", expected: "ư" },
    VietTelexCase { input: "uow", expected: "ươ" },

    // 2. Modifier undos (9 cases)
    VietTelexCase { input: "aaa", expected: "aa" },
    VietTelexCase { input: "eee", expected: "ee" },
    VietTelexCase { input: "ooo", expected: "oo" },
    VietTelexCase { input: "aww", expected: "aw" },
    VietTelexCase { input: "oww", expected: "ow" },
    VietTelexCase { input: "uww", expected: "uw" },
    VietTelexCase { input: "ddd", expected: "dd" },
    VietTelexCase { input: "ww", expected: "w" },
    VietTelexCase { input: "uoww", expected: "uow" },

    // 3. All 5 tones on plain vowels (30 cases)
    VietTelexCase { input: "as", expected: "á" },
    VietTelexCase { input: "af", expected: "à" },
    VietTelexCase { input: "ar", expected: "ả" },
    VietTelexCase { input: "ax", expected: "ã" },
    VietTelexCase { input: "aj", expected: "ạ" },
    VietTelexCase { input: "es", expected: "é" },
    VietTelexCase { input: "ef", expected: "è" },
    VietTelexCase { input: "er", expected: "ẻ" },
    VietTelexCase { input: "ex", expected: "ẽ" },
    VietTelexCase { input: "ej", expected: "ẹ" },
    VietTelexCase { input: "is", expected: "í" },
    VietTelexCase { input: "if", expected: "ì" },
    VietTelexCase { input: "ir", expected: "ỉ" },
    VietTelexCase { input: "ix", expected: "ĩ" },
    VietTelexCase { input: "ij", expected: "ị" },
    VietTelexCase { input: "os", expected: "ó" },
    VietTelexCase { input: "of", expected: "ò" },
    VietTelexCase { input: "or", expected: "ỏ" },
    VietTelexCase { input: "ox", expected: "õ" },
    VietTelexCase { input: "oj", expected: "ọ" },
    VietTelexCase { input: "us", expected: "ú" },
    VietTelexCase { input: "uf", expected: "ù" },
    VietTelexCase { input: "ur", expected: "ủ" },
    VietTelexCase { input: "ux", expected: "ũ" },
    VietTelexCase { input: "uj", expected: "ụ" },
    VietTelexCase { input: "ys", expected: "ý" },
    VietTelexCase { input: "yf", expected: "ỳ" },
    VietTelexCase { input: "yr", expected: "ỷ" },
    VietTelexCase { input: "yx", expected: "ỹ" },
    VietTelexCase { input: "yj", expected: "ỵ" },

    // 4. Tone z-removal (7 cases)
    VietTelexCase { input: "asz", expected: "a" },
    VietTelexCase { input: "afz", expected: "a" },
    VietTelexCase { input: "arz", expected: "a" },
    VietTelexCase { input: "axz", expected: "a" },
    VietTelexCase { input: "ajz", expected: "a" },
    VietTelexCase { input: "hoasz", expected: "hoa" },
    VietTelexCase { input: "toansz", expected: "toan" },

    // 5. Same-tone-twice -> literal (7 cases)
    VietTelexCase { input: "ass", expected: "as" },
    VietTelexCase { input: "aff", expected: "af" },
    VietTelexCase { input: "arr", expected: "ar" },
    VietTelexCase { input: "axx", expected: "ax" },
    VietTelexCase { input: "ajj", expected: "aj" },
    VietTelexCase { input: "toanss", expected: "toans" },
    VietTelexCase { input: "hoass", expected: "hoas" },

    // 6. Modifiers + tones combined (40 cases)
    VietTelexCase { input: "aas", expected: "ấ" },
    VietTelexCase { input: "aaf", expected: "ầ" },
    VietTelexCase { input: "aar", expected: "ẩ" },
    VietTelexCase { input: "aax", expected: "ẫ" },
    VietTelexCase { input: "aaj", expected: "ậ" },
    VietTelexCase { input: "aws", expected: "ắ" },
    VietTelexCase { input: "awf", expected: "ằ" },
    VietTelexCase { input: "awr", expected: "ẳ" },
    VietTelexCase { input: "awx", expected: "ẵ" },
    VietTelexCase { input: "awj", expected: "ặ" },
    VietTelexCase { input: "ees", expected: "ế" },
    VietTelexCase { input: "eef", expected: "ề" },
    VietTelexCase { input: "eer", expected: "ể" },
    VietTelexCase { input: "eex", expected: "ễ" },
    VietTelexCase { input: "eej", expected: "ệ" },
    VietTelexCase { input: "oos", expected: "ố" },
    VietTelexCase { input: "oof", expected: "ồ" },
    VietTelexCase { input: "oor", expected: "ổ" },
    VietTelexCase { input: "oox", expected: "ỗ" },
    VietTelexCase { input: "ooj", expected: "ộ" },
    VietTelexCase { input: "ows", expected: "ớ" },
    VietTelexCase { input: "owf", expected: "ờ" },
    VietTelexCase { input: "owr", expected: "ở" },
    VietTelexCase { input: "owx", expected: "ỡ" },
    VietTelexCase { input: "owj", expected: "ợ" },
    VietTelexCase { input: "uws", expected: "ứ" },
    VietTelexCase { input: "uwf", expected: "ừ" },
    VietTelexCase { input: "uwr", expected: "ử" },
    VietTelexCase { input: "uwx", expected: "ữ" },
    VietTelexCase { input: "uwj", expected: "ự" },
    VietTelexCase { input: "uows", expected: "ứơ" },
    VietTelexCase { input: "uowf", expected: "ừơ" },
    VietTelexCase { input: "uowr", expected: "ửơ" },
    VietTelexCase { input: "uowx", expected: "ữơ" },
    VietTelexCase { input: "uowj", expected: "ựơ" },
    VietTelexCase { input: "ddas", expected: "đá" },
    VietTelexCase { input: "ddaf", expected: "đà" },
    VietTelexCase { input: "ddar", expected: "đả" },
    VietTelexCase { input: "ddax", expected: "đã" },
    VietTelexCase { input: "ddaj", expected: "đạ" },

    // 7. Rule 1: Nucleus of 1 vowel (12 cases)
    VietTelexCase { input: "mas", expected: "má" },
    VietTelexCase { input: "maf", expected: "mà" },
    VietTelexCase { input: "mar", expected: "mả" },
    VietTelexCase { input: "max", expected: "mã" },
    VietTelexCase { input: "maj", expected: "mạ" },
    VietTelexCase { input: "mes", expected: "mé" },
    VietTelexCase { input: "mef", expected: "mè" },
    VietTelexCase { input: "mer", expected: "mẻ" },
    VietTelexCase { input: "mex", expected: "mẽ" },
    VietTelexCase { input: "mej", expected: "mẹ" },
    VietTelexCase { input: "gif", expected: "gì" },
    VietTelexCase { input: "gis", expected: "gí" },

    // 8. Rule 2: 2 vowels WITH coda -> 2nd vowel (12 cases)
    VietTelexCase { input: "hoan", expected: "hoan" },
    VietTelexCase { input: "hoanf", expected: "hoàn" },
    VietTelexCase { input: "hoans", expected: "hoán" },
    VietTelexCase { input: "toan", expected: "toan" },
    VietTelexCase { input: "toans", expected: "toán" },
    VietTelexCase { input: "toanf", expected: "toàn" },
    VietTelexCase { input: "tieengs", expected: "tiếng" },
    VietTelexCase { input: "tieengf", expected: "tiềng" },
    VietTelexCase { input: "muoons", expected: "muốn" },
    VietTelexCase { input: "muoonf", expected: "muồn" },
    VietTelexCase { input: "dduowng", expected: "đương" },
    VietTelexCase { input: "dduowngf", expected: "đường" },

    // 9. Rule 3: 2 vowels WITHOUT coda (24 cases)
    VietTelexCase { input: "biaj", expected: "bịa" },
    VietTelexCase { input: "bias", expected: "bía" },
    VietTelexCase { input: "muaf", expected: "mùa" },
    VietTelexCase { input: "muas", expected: "múa" },
    VietTelexCase { input: "cuwar", expected: "cửa" },
    VietTelexCase { input: "cuwas", expected: "cứa" },
    VietTelexCase { input: "cais", expected: "cái" },
    VietTelexCase { input: "caif", expected: "cài" },
    VietTelexCase { input: "caos", expected: "cáo" },
    VietTelexCase { input: "caof", expected: "cào" },
    VietTelexCase { input: "caus", expected: "cáu" },
    VietTelexCase { input: "cays", expected: "cáy" },
    VietTelexCase { input: "caaus", expected: "cấu" },
    VietTelexCase { input: "caauf", expected: "cầu" },
    VietTelexCase { input: "caays", expected: "cấy" },
    VietTelexCase { input: "keos", expected: "kéo" },
    VietTelexCase { input: "keeus", expected: "kếu" },
    VietTelexCase { input: "chiuj", expected: "chịu" },
    VietTelexCase { input: "tuis", expected: "túi" },
    VietTelexCase { input: "nois", expected: "nói" },
    VietTelexCase { input: "hoas", expected: "hoá" },
    VietTelexCase { input: "khoer", expected: "khoẻ" },
    VietTelexCase { input: "thuys", expected: "thuý" },
    VietTelexCase { input: "huees", expected: "huế" },

    // 10. Rule 4: 3 vowels -> middle vowel (10 cases)
    VietTelexCase { input: "kieeuf", expected: "kiều" },
    VietTelexCase { input: "kieeus", expected: "kiếu" },
    VietTelexCase { input: "ngoaif", expected: "ngoài" },
    VietTelexCase { input: "ngoais", expected: "ngoái" },
    VietTelexCase { input: "tuooir", expected: "tuổi" },
    VietTelexCase { input: "tuoois", expected: "tuối" },
    VietTelexCase { input: "ruowuj", expected: "rượu" },
    VietTelexCase { input: "ruowus", expected: "rướu" },
    VietTelexCase { input: "khuyur", expected: "khuỷu" },
    VietTelexCase { input: "khuyus", expected: "khuýu" },

    // 11. Early tone + repositioning (4 cases)
    VietTelexCase { input: "hosa", expected: "hoá" },
    VietTelexCase { input: "toasn", expected: "toán" },
    VietTelexCase { input: "tieefng", expected: "tiềng" },
    VietTelexCase { input: "dduowjng", expected: "đượng" },

    // 12. Post-coda tone (4 cases)
    VietTelexCase { input: "sachs", expected: "sách" },
    VietTelexCase { input: "hocj", expected: "học" },
    VietTelexCase { input: "bieets", expected: "biết" },
    VietTelexCase { input: "cuoocs", expected: "cuốc" },

    // 13. Backspace chains (5 cases)
    VietTelexCase { input: "as\x08", expected: "a" },
    VietTelexCase { input: "aas\x08", expected: "â" },
    VietTelexCase { input: "aa\x08", expected: "a" },
    VietTelexCase { input: "dduowngf\x08", expected: "đương" },
    VietTelexCase { input: "vieetj\x08", expected: "viêt" },

    // 14. Full words and multi-word sentences (9 cases)
    VietTelexCase { input: "vieetj", expected: "việt" },
    VietTelexCase { input: "tieengs", expected: "tiếng" },
    VietTelexCase { input: "dduowngf", expected: "đường" },
    VietTelexCase { input: "chaof", expected: "chào" },
    VietTelexCase { input: "xin chaof", expected: "xin chào" },
    VietTelexCase { input: "tieengs vieetj", expected: "tiếng việt" },
    VietTelexCase { input: "dduowngf veef", expected: "đường về" },
    VietTelexCase { input: "nguowif vieetj", expected: "người việt" },
    VietTelexCase { input: "haoj nam", expected: "hạo nam" },

    // 15. Invalid syllable restore at word boundary (3 cases)
    VietTelexCase { input: "qw ", expected: "qw " },
    VietTelexCase { input: "xyz ", expected: "xyz " },
    VietTelexCase { input: "st ", expected: "st " },

    // 16. Standalone w (8 cases)
    VietTelexCase { input: "w", expected: "ư" },
    VietTelexCase { input: "ws", expected: "ứ" },
    VietTelexCase { input: "wf", expected: "ừ" },
    VietTelexCase { input: "wr", expected: "ử" },
    VietTelexCase { input: "wx", expected: "ữ" },
    VietTelexCase { input: "wj", expected: "ự" },
    VietTelexCase { input: "wa", expected: "ưa" },
    VietTelexCase { input: "was", expected: "ứa" },

    // 17. Capitals (10 cases)
    VietTelexCase { input: "DD", expected: "Đ" },
    VietTelexCase { input: "Dd", expected: "Đ" },
    VietTelexCase { input: "Aa", expected: "Â" },
    VietTelexCase { input: "Aas", expected: "Ấ" },
    VietTelexCase { input: "VIEETJ", expected: "VIỆT" },
    VietTelexCase { input: "Vieetj", expected: "Việt" },
    VietTelexCase { input: "TIEENGS", expected: "TIẾNG" },
    VietTelexCase { input: "Tieengs", expected: "Tiếng" },
    VietTelexCase { input: "DDUOWNGF", expected: "ĐƯỜNG" },
    VietTelexCase { input: "Dduowngf", expected: "Đường" },
];

#[test]
fn test_vietnamese_telex_golden_corpus() {
    println!("Vietnamese Telex corpus size: {}", VIETNAMESE_TELEX_CORPUS.len());
    assert!(
        VIETNAMESE_TELEX_CORPUS.len() >= 150,
        "Corpus must have at least 150 cases, but has {}",
        VIETNAMESE_TELEX_CORPUS.len()
    );

    for (idx, tc) in VIETNAMESE_TELEX_CORPUS.iter().enumerate() {
        let mut eng = Engine::with_scheme(SchemeId::VietnameseTelex);
        let actual = simulate_typing(&mut eng, tc.input);
        assert_eq!(
            actual, tc.expected,
            "Case #{idx} failed for input '{}': expected '{}', got '{}'",
            tc.input, tc.expected, actual
        );
    }
}
