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
        let res = engine.process(KeyAction::Char(c));
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
