//! Pāḷi and extended Sanskrit/IAST character set definitions and case mapping.
//!
//! Clean-room implementation per DESIGN.md §4.

/// Niggahīta orthographic variant setting.
///
/// In Romanized Pāḷi, the nasal nasalizer (niggahīta) is represented either
/// with a dot below (ṃ, U+1E43) or with a dot above (ṁ, U+1E41).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub enum NiggahitaVariant {
    /// Dot below: ṃ (U+1E43), Ṃ (U+1E42). Standard in Pali Text Society (PTS)
    /// and SuttaCentral publications. Default.
    #[default]
    DotBelow = 0,
    /// Dot above: ṁ (U+1E41), Ṁ (U+1E40). Standard in Vipassana Research
    /// Institute (VRI) publications and Burmese Romanizations.
    DotAbove = 1,
}

impl NiggahitaVariant {
    /// Returns the lowercase niggahīta character for this variant.
    pub const fn lowercase(self) -> char {
        match self {
            Self::DotBelow => CHAR_M_DOT_BELOW_LOWER,
            Self::DotAbove => CHAR_M_DOT_ABOVE_LOWER,
        }
    }

    /// Returns the uppercase niggahīta character for this variant.
    pub const fn uppercase(self) -> char {
        match self {
            Self::DotBelow => CHAR_M_DOT_BELOW_UPPER,
            Self::DotAbove => CHAR_M_DOT_ABOVE_UPPER,
        }
    }

    /// Returns the character for this variant with the specified capitalization.
    pub const fn char_for(self, uppercase: bool) -> char {
        if uppercase {
            self.uppercase()
        } else {
            self.lowercase()
        }
    }
}

// =============================================================================
// Core Pāḷi Character Set (10 diacritic letters + capitals)
// =============================================================================

// Long vowels
pub const CHAR_A_MACRON_LOWER: char = '\u{0101}'; // ā
pub const CHAR_A_MACRON_UPPER: char = '\u{0100}'; // Ā
pub const CHAR_I_MACRON_LOWER: char = '\u{012B}'; // ī
pub const CHAR_I_MACRON_UPPER: char = '\u{012A}'; // Ī
pub const CHAR_U_MACRON_LOWER: char = '\u{016B}'; // ū
pub const CHAR_U_MACRON_UPPER: char = '\u{016A}'; // Ū

// Consonants with diacritics
pub const CHAR_N_DOT_ABOVE_LOWER: char = '\u{1E45}'; // ṅ (velar nasal)
pub const CHAR_N_DOT_ABOVE_UPPER: char = '\u{1E44}'; // Ṅ
pub const CHAR_N_TILDE_LOWER: char = '\u{00F1}';     // ñ (palatal nasal)
pub const CHAR_N_TILDE_UPPER: char = '\u{00D1}';     // Ñ
pub const CHAR_T_DOT_BELOW_LOWER: char = '\u{1E6D}'; // ṭ (retroflex voiceless plosive)
pub const CHAR_T_DOT_BELOW_UPPER: char = '\u{1E6C}'; // Ṭ
pub const CHAR_D_DOT_BELOW_LOWER: char = '\u{1E0D}'; // ḍ (retroflex voiced plosive)
pub const CHAR_D_DOT_BELOW_UPPER: char = '\u{1E0C}'; // Ḍ
pub const CHAR_N_DOT_BELOW_LOWER: char = '\u{1E47}'; // ṇ (retroflex nasal)
pub const CHAR_N_DOT_BELOW_UPPER: char = '\u{1E46}'; // Ṇ
pub const CHAR_L_DOT_BELOW_LOWER: char = '\u{1E37}'; // ḷ (retroflex lateral)
pub const CHAR_L_DOT_BELOW_UPPER: char = '\u{1E36}'; // Ḷ

// Niggahīta variants
pub const CHAR_M_DOT_BELOW_LOWER: char = '\u{1E43}'; // ṃ
pub const CHAR_M_DOT_BELOW_UPPER: char = '\u{1E42}'; // Ṃ
pub const CHAR_M_DOT_ABOVE_LOWER: char = '\u{1E41}'; // ṁ
pub const CHAR_M_DOT_ABOVE_UPPER: char = '\u{1E40}'; // Ṁ

// =============================================================================
// Extended Sanskrit / IAST Character Set (ṛ ṝ ḹ ḥ ś ṣ ē ō)
// =============================================================================

pub const CHAR_R_RING_BELOW_LOWER: char = '\u{1E5B}';  // ṛ
pub const CHAR_R_RING_BELOW_UPPER: char = '\u{1E5A}';  // Ṛ
pub const CHAR_R_RING_MACRON_LOWER: char = '\u{1E5D}'; // ṝ
pub const CHAR_R_RING_MACRON_UPPER: char = '\u{1E5C}'; // Ṝ
pub const CHAR_L_RING_MACRON_LOWER: char = '\u{1E39}'; // ḹ
pub const CHAR_L_RING_MACRON_UPPER: char = '\u{1E38}'; // Ḹ
pub const CHAR_H_DOT_BELOW_LOWER: char = '\u{1E25}';   // ḥ (visarga)
pub const CHAR_H_DOT_BELOW_UPPER: char = '\u{1E24}';   // Ḥ
pub const CHAR_S_ACUTE_LOWER: char = '\u{015B}';       // ś (palatal sibilant)
pub const CHAR_S_ACUTE_UPPER: char = '\u{015A}';       // Ś
pub const CHAR_S_DOT_BELOW_LOWER: char = '\u{1E63}';   // ṣ (retroflex sibilant)
pub const CHAR_S_DOT_BELOW_UPPER: char = '\u{1E62}';   // Ṣ
pub const CHAR_E_MACRON_LOWER: char = '\u{0113}';      // ē (extended long e)
pub const CHAR_E_MACRON_UPPER: char = '\u{0112}';      // Ē
pub const CHAR_O_MACRON_LOWER: char = '\u{014D}';      // ō (extended long o)
pub const CHAR_O_MACRON_UPPER: char = '\u{014C}';      // Ō

/// Returns true if the character is in the core Pāḷi diacritic set.
pub fn is_core_pali_diacritic(c: char) -> bool {
    matches!(
        c,
        CHAR_A_MACRON_LOWER
            | CHAR_A_MACRON_UPPER
            | CHAR_I_MACRON_LOWER
            | CHAR_I_MACRON_UPPER
            | CHAR_U_MACRON_LOWER
            | CHAR_U_MACRON_UPPER
            | CHAR_N_DOT_ABOVE_LOWER
            | CHAR_N_DOT_ABOVE_UPPER
            | CHAR_N_TILDE_LOWER
            | CHAR_N_TILDE_UPPER
            | CHAR_T_DOT_BELOW_LOWER
            | CHAR_T_DOT_BELOW_UPPER
            | CHAR_D_DOT_BELOW_LOWER
            | CHAR_D_DOT_BELOW_UPPER
            | CHAR_N_DOT_BELOW_LOWER
            | CHAR_N_DOT_BELOW_UPPER
            | CHAR_L_DOT_BELOW_LOWER
            | CHAR_L_DOT_BELOW_UPPER
            | CHAR_M_DOT_BELOW_LOWER
            | CHAR_M_DOT_BELOW_UPPER
            | CHAR_M_DOT_ABOVE_LOWER
            | CHAR_M_DOT_ABOVE_UPPER
    )
}

/// Returns true if the character is in the extended Sanskrit/IAST diacritic set.
pub fn is_extended_diacritic(c: char) -> bool {
    matches!(
        c,
        CHAR_R_RING_BELOW_LOWER
            | CHAR_R_RING_BELOW_UPPER
            | CHAR_R_RING_MACRON_LOWER
            | CHAR_R_RING_MACRON_UPPER
            | CHAR_L_RING_MACRON_LOWER
            | CHAR_L_RING_MACRON_UPPER
            | CHAR_H_DOT_BELOW_LOWER
            | CHAR_H_DOT_BELOW_UPPER
            | CHAR_S_ACUTE_LOWER
            | CHAR_S_ACUTE_UPPER
            | CHAR_S_DOT_BELOW_LOWER
            | CHAR_S_DOT_BELOW_UPPER
            | CHAR_E_MACRON_LOWER
            | CHAR_E_MACRON_UPPER
            | CHAR_O_MACRON_LOWER
            | CHAR_O_MACRON_UPPER
    )
}

/// Returns the uppercase equivalent of a Pāḷi / extended character if applicable.
pub fn to_pali_uppercase(c: char) -> char {
    match c {
        CHAR_A_MACRON_LOWER => CHAR_A_MACRON_UPPER,
        CHAR_I_MACRON_LOWER => CHAR_I_MACRON_UPPER,
        CHAR_U_MACRON_LOWER => CHAR_U_MACRON_UPPER,
        CHAR_N_DOT_ABOVE_LOWER => CHAR_N_DOT_ABOVE_UPPER,
        CHAR_N_TILDE_LOWER => CHAR_N_TILDE_UPPER,
        CHAR_T_DOT_BELOW_LOWER => CHAR_T_DOT_BELOW_UPPER,
        CHAR_D_DOT_BELOW_LOWER => CHAR_D_DOT_BELOW_UPPER,
        CHAR_N_DOT_BELOW_LOWER => CHAR_N_DOT_BELOW_UPPER,
        CHAR_L_DOT_BELOW_LOWER => CHAR_L_DOT_BELOW_UPPER,
        CHAR_M_DOT_BELOW_LOWER => CHAR_M_DOT_BELOW_UPPER,
        CHAR_M_DOT_ABOVE_LOWER => CHAR_M_DOT_ABOVE_UPPER,
        CHAR_R_RING_BELOW_LOWER => CHAR_R_RING_BELOW_UPPER,
        CHAR_R_RING_MACRON_LOWER => CHAR_R_RING_MACRON_UPPER,
        CHAR_L_RING_MACRON_LOWER => CHAR_L_RING_MACRON_UPPER,
        CHAR_H_DOT_BELOW_LOWER => CHAR_H_DOT_BELOW_UPPER,
        CHAR_S_ACUTE_LOWER => CHAR_S_ACUTE_UPPER,
        CHAR_S_DOT_BELOW_LOWER => CHAR_S_DOT_BELOW_UPPER,
        CHAR_E_MACRON_LOWER => CHAR_E_MACRON_UPPER,
        CHAR_O_MACRON_LOWER => CHAR_O_MACRON_UPPER,
        other => other.to_ascii_uppercase(),
    }
}

/// Returns the lowercase equivalent of a Pāḷi / extended character if applicable.
pub fn to_pali_lowercase(c: char) -> char {
    match c {
        CHAR_A_MACRON_UPPER => CHAR_A_MACRON_LOWER,
        CHAR_I_MACRON_UPPER => CHAR_I_MACRON_LOWER,
        CHAR_U_MACRON_UPPER => CHAR_U_MACRON_LOWER,
        CHAR_N_DOT_ABOVE_UPPER => CHAR_N_DOT_ABOVE_LOWER,
        CHAR_N_TILDE_UPPER => CHAR_N_TILDE_LOWER,
        CHAR_T_DOT_BELOW_UPPER => CHAR_T_DOT_BELOW_LOWER,
        CHAR_D_DOT_BELOW_UPPER => CHAR_D_DOT_BELOW_LOWER,
        CHAR_N_DOT_BELOW_UPPER => CHAR_N_DOT_BELOW_LOWER,
        CHAR_L_DOT_BELOW_UPPER => CHAR_L_DOT_BELOW_LOWER,
        CHAR_M_DOT_BELOW_UPPER => CHAR_M_DOT_BELOW_LOWER,
        CHAR_M_DOT_ABOVE_UPPER => CHAR_M_DOT_ABOVE_LOWER,
        CHAR_R_RING_BELOW_UPPER => CHAR_R_RING_BELOW_LOWER,
        CHAR_R_RING_MACRON_UPPER => CHAR_R_RING_MACRON_LOWER,
        CHAR_L_RING_MACRON_UPPER => CHAR_L_RING_MACRON_LOWER,
        CHAR_H_DOT_BELOW_UPPER => CHAR_H_DOT_BELOW_LOWER,
        CHAR_S_ACUTE_UPPER => CHAR_S_ACUTE_LOWER,
        CHAR_S_DOT_BELOW_UPPER => CHAR_S_DOT_BELOW_LOWER,
        CHAR_E_MACRON_UPPER => CHAR_E_MACRON_LOWER,
        CHAR_O_MACRON_UPPER => CHAR_O_MACRON_LOWER,
        other => other.to_ascii_lowercase(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_codepoints() {
        assert_eq!(CHAR_A_MACRON_LOWER, '\u{0101}');
        assert_eq!(CHAR_A_MACRON_UPPER, '\u{0100}');
        assert_eq!(CHAR_I_MACRON_LOWER, '\u{012B}');
        assert_eq!(CHAR_I_MACRON_UPPER, '\u{012A}');
        assert_eq!(CHAR_U_MACRON_LOWER, '\u{016B}');
        assert_eq!(CHAR_U_MACRON_UPPER, '\u{016A}');
        assert_eq!(CHAR_N_DOT_ABOVE_LOWER, '\u{1E45}');
        assert_eq!(CHAR_N_DOT_ABOVE_UPPER, '\u{1E44}');
        assert_eq!(CHAR_N_TILDE_LOWER, '\u{00F1}');
        assert_eq!(CHAR_N_TILDE_UPPER, '\u{00D1}');
        assert_eq!(CHAR_T_DOT_BELOW_LOWER, '\u{1E6D}');
        assert_eq!(CHAR_T_DOT_BELOW_UPPER, '\u{1E6C}');
        assert_eq!(CHAR_D_DOT_BELOW_LOWER, '\u{1E0D}');
        assert_eq!(CHAR_D_DOT_BELOW_UPPER, '\u{1E0C}');
        assert_eq!(CHAR_N_DOT_BELOW_LOWER, '\u{1E47}');
        assert_eq!(CHAR_N_DOT_BELOW_UPPER, '\u{1E46}');
        assert_eq!(CHAR_L_DOT_BELOW_LOWER, '\u{1E37}');
        assert_eq!(CHAR_L_DOT_BELOW_UPPER, '\u{1E36}');
    }

    #[test]
    fn test_niggahita_variants() {
        assert_eq!(NiggahitaVariant::DotBelow.lowercase(), '\u{1E43}');
        assert_eq!(NiggahitaVariant::DotBelow.uppercase(), '\u{1E42}');
        assert_eq!(NiggahitaVariant::DotAbove.lowercase(), '\u{1E41}');
        assert_eq!(NiggahitaVariant::DotAbove.uppercase(), '\u{1E40}');
    }

    #[test]
    fn test_case_mapping() {
        assert_eq!(to_pali_uppercase(CHAR_A_MACRON_LOWER), CHAR_A_MACRON_UPPER);
        assert_eq!(to_pali_lowercase(CHAR_A_MACRON_UPPER), CHAR_A_MACRON_LOWER);
        assert_eq!(to_pali_uppercase(CHAR_T_DOT_BELOW_LOWER), CHAR_T_DOT_BELOW_UPPER);
        assert_eq!(to_pali_lowercase(CHAR_T_DOT_BELOW_UPPER), CHAR_T_DOT_BELOW_LOWER);
    }
}
