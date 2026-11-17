//! Input scheme definitions as pure data tables and rules.
//!
//! Clean-room implementation per DESIGN.md §4.
//! Adding a new scheme requires only adding a static data definition here.
//! The state machine engine requires zero changes.

use crate::chars::*;

/// Identifier for supported input transliteration schemes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub enum SchemeId {
    /// Velthuis scheme (scholar standard: aa->ā, .t->ṭ, "n->ṅ, ~n->ñ, .m->ṃ)
    #[default]
    Velthuis = 0,
    /// PaliTelex scheme (Vietnamese Telex muscle memory: aa->ā, tx->ṭ, ngw->ṅ, nw/ny->ñ, mx->ṃ)
    PaliTelex = 1,
    /// SIL / Keyman scheme (giacngo.org guide: -a->ā, .t->ṭ, >n->ṅ, ~n->ñ, .m->ṃ)
    Sil = 2,
}

impl SchemeId {
    /// Returns the scheme identifier from an integer code (0=Velthuis, 1=PaliTelex, 2=Sil).
    pub const fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Self::Velthuis),
            1 => Some(Self::PaliTelex),
            2 => Some(Self::Sil),
            _ => None,
        }
    }

    /// Returns the canonical name string of the scheme.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Velthuis => "velthuis",
            Self::PaliTelex => "palitelex",
            Self::Sil => "sil",
        }
    }
}

/// Target character produced by a sequence mapping rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetChar {
    /// Fixed character (e.g. 'ā', 'ṭ', 'ṅ').
    Fixed(char),
    /// Resolved dynamically from the engine's `NiggahitaVariant` configuration.
    /// The boolean flag indicates whether uppercase is requested.
    Niggahita(bool),
    /// Character in the extended Sanskrit/IAST set (e.g. 'ṛ', 'ē', 'ḥ').
    Extended(char),
}

impl TargetChar {
    /// Resolves this target into a concrete char based on settings.
    pub fn resolve(self, variant: NiggahitaVariant, extended_enabled: bool) -> Option<char> {
        match self {
            Self::Fixed(c) => Some(c),
            Self::Niggahita(is_upper) => Some(variant.char_for(is_upper)),
            Self::Extended(c) => {
                if extended_enabled {
                    Some(c)
                } else {
                    None
                }
            }
        }
    }
}

/// A single mapping rule: sequence -> target character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MappingRule {
    /// The keystroke sequence triggering this mapping (e.g. "aa", ".t", "ngw").
    pub sequence: &'static str,
    /// The character target produced by this sequence.
    pub target: TargetChar,
    /// Whether this mapping requires the extended Sanskrit/IAST set to be enabled.
    pub is_extended: bool,
}

/// Complete definition of an input transliteration scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemeDefinition {
    /// Scheme enum ID.
    pub id: SchemeId,
    /// Canonical short name (e.g. "velthuis").
    pub name: &'static str,
    /// Human-readable display label.
    pub display_name: &'static str,
    /// Brief description of the scheme's lineage and convention.
    pub description: &'static str,
    /// Ordered table of mapping rules.
    pub mappings: &'static [MappingRule],
}

// =============================================================================
// Scheme 1: Velthuis (DESIGN.md §4.1)
// =============================================================================

static VELTHUIS_MAPPINGS: &[MappingRule] = &[
    // Long vowels
    MappingRule { sequence: "aa", target: TargetChar::Fixed(CHAR_A_MACRON_LOWER), is_extended: false },
    MappingRule { sequence: "Aa", target: TargetChar::Fixed(CHAR_A_MACRON_UPPER), is_extended: false },
    MappingRule { sequence: "AA", target: TargetChar::Fixed(CHAR_A_MACRON_UPPER), is_extended: false },
    MappingRule { sequence: "ii", target: TargetChar::Fixed(CHAR_I_MACRON_LOWER), is_extended: false },
    MappingRule { sequence: "Ii", target: TargetChar::Fixed(CHAR_I_MACRON_UPPER), is_extended: false },
    MappingRule { sequence: "II", target: TargetChar::Fixed(CHAR_I_MACRON_UPPER), is_extended: false },
    MappingRule { sequence: "uu", target: TargetChar::Fixed(CHAR_U_MACRON_LOWER), is_extended: false },
    MappingRule { sequence: "Uu", target: TargetChar::Fixed(CHAR_U_MACRON_UPPER), is_extended: false },
    MappingRule { sequence: "UU", target: TargetChar::Fixed(CHAR_U_MACRON_UPPER), is_extended: false },

    // Retroflex and diacritic consonants
    MappingRule { sequence: ".t", target: TargetChar::Fixed(CHAR_T_DOT_BELOW_LOWER), is_extended: false },
    MappingRule { sequence: ".T", target: TargetChar::Fixed(CHAR_T_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: ".d", target: TargetChar::Fixed(CHAR_D_DOT_BELOW_LOWER), is_extended: false },
    MappingRule { sequence: ".D", target: TargetChar::Fixed(CHAR_D_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: ".n", target: TargetChar::Fixed(CHAR_N_DOT_BELOW_LOWER), is_extended: false },
    MappingRule { sequence: ".N", target: TargetChar::Fixed(CHAR_N_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: ".m", target: TargetChar::Niggahita(false), is_extended: false },
    MappingRule { sequence: ".M", target: TargetChar::Niggahita(true), is_extended: false },
    MappingRule { sequence: ".l", target: TargetChar::Fixed(CHAR_L_DOT_BELOW_LOWER), is_extended: false },
    MappingRule { sequence: ".L", target: TargetChar::Fixed(CHAR_L_DOT_BELOW_UPPER), is_extended: false },

    // Velar nasal and palatal nasal
    MappingRule { sequence: "\"n", target: TargetChar::Fixed(CHAR_N_DOT_ABOVE_LOWER), is_extended: false },
    MappingRule { sequence: "\"N", target: TargetChar::Fixed(CHAR_N_DOT_ABOVE_UPPER), is_extended: false },
    MappingRule { sequence: "~n", target: TargetChar::Fixed(CHAR_N_TILDE_LOWER), is_extended: false },
    MappingRule { sequence: "~N", target: TargetChar::Fixed(CHAR_N_TILDE_UPPER), is_extended: false },

    // Extended Sanskrit / IAST set
    MappingRule { sequence: "ee", target: TargetChar::Extended(CHAR_E_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: "Ee", target: TargetChar::Extended(CHAR_E_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "EE", target: TargetChar::Extended(CHAR_E_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "oo", target: TargetChar::Extended(CHAR_O_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: "Oo", target: TargetChar::Extended(CHAR_O_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "OO", target: TargetChar::Extended(CHAR_O_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: ".rr", target: TargetChar::Extended(CHAR_R_RING_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: ".Rr", target: TargetChar::Extended(CHAR_R_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: ".RR", target: TargetChar::Extended(CHAR_R_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "ṛr", target: TargetChar::Extended(CHAR_R_RING_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: "Ṛr", target: TargetChar::Extended(CHAR_R_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "ṚR", target: TargetChar::Extended(CHAR_R_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: ".r", target: TargetChar::Extended(CHAR_R_RING_BELOW_LOWER), is_extended: true },
    MappingRule { sequence: ".R", target: TargetChar::Extended(CHAR_R_RING_BELOW_UPPER), is_extended: true },
    MappingRule { sequence: ".ll", target: TargetChar::Extended(CHAR_L_RING_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: ".Ll", target: TargetChar::Extended(CHAR_L_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: ".LL", target: TargetChar::Extended(CHAR_L_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "ḷl", target: TargetChar::Extended(CHAR_L_RING_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: "Ḷl", target: TargetChar::Extended(CHAR_L_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "ḶL", target: TargetChar::Extended(CHAR_L_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: ".h", target: TargetChar::Extended(CHAR_H_DOT_BELOW_LOWER), is_extended: true },
    MappingRule { sequence: ".H", target: TargetChar::Extended(CHAR_H_DOT_BELOW_UPPER), is_extended: true },
    MappingRule { sequence: "\"s", target: TargetChar::Extended(CHAR_S_ACUTE_LOWER), is_extended: true },
    MappingRule { sequence: "\"S", target: TargetChar::Extended(CHAR_S_ACUTE_UPPER), is_extended: true },
    MappingRule { sequence: ".s", target: TargetChar::Extended(CHAR_S_DOT_BELOW_LOWER), is_extended: true },
    MappingRule { sequence: ".S", target: TargetChar::Extended(CHAR_S_DOT_BELOW_UPPER), is_extended: true },
];

pub static SCHEME_VELTHUIS: SchemeDefinition = SchemeDefinition {
    id: SchemeId::Velthuis,
    name: "velthuis",
    display_name: "Velthuis",
    description: "Academic standard (aa->ā, .t->ṭ, \"n->ṅ, ~n->ñ, .m->ṃ)",
    mappings: VELTHUIS_MAPPINGS,
};

// =============================================================================
// Scheme 2: PaliTelex (DESIGN.md §4.2)
// =============================================================================

static PALI_TELEX_MAPPINGS: &[MappingRule] = &[
    // Long vowels (vowel doubling)
    MappingRule { sequence: "aa", target: TargetChar::Fixed(CHAR_A_MACRON_LOWER), is_extended: false },
    MappingRule { sequence: "Aa", target: TargetChar::Fixed(CHAR_A_MACRON_UPPER), is_extended: false },
    MappingRule { sequence: "AA", target: TargetChar::Fixed(CHAR_A_MACRON_UPPER), is_extended: false },
    MappingRule { sequence: "ii", target: TargetChar::Fixed(CHAR_I_MACRON_LOWER), is_extended: false },
    MappingRule { sequence: "Ii", target: TargetChar::Fixed(CHAR_I_MACRON_UPPER), is_extended: false },
    MappingRule { sequence: "II", target: TargetChar::Fixed(CHAR_I_MACRON_UPPER), is_extended: false },
    MappingRule { sequence: "uu", target: TargetChar::Fixed(CHAR_U_MACRON_LOWER), is_extended: false },
    MappingRule { sequence: "Uu", target: TargetChar::Fixed(CHAR_U_MACRON_UPPER), is_extended: false },
    MappingRule { sequence: "UU", target: TargetChar::Fixed(CHAR_U_MACRON_UPPER), is_extended: false },

    // Suffix 'x' for dots below
    MappingRule { sequence: "tx", target: TargetChar::Fixed(CHAR_T_DOT_BELOW_LOWER), is_extended: false },
    MappingRule { sequence: "Tx", target: TargetChar::Fixed(CHAR_T_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: "TX", target: TargetChar::Fixed(CHAR_T_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: "dx", target: TargetChar::Fixed(CHAR_D_DOT_BELOW_LOWER), is_extended: false },
    MappingRule { sequence: "Dx", target: TargetChar::Fixed(CHAR_D_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: "DX", target: TargetChar::Fixed(CHAR_D_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: "nx", target: TargetChar::Fixed(CHAR_N_DOT_BELOW_LOWER), is_extended: false },
    MappingRule { sequence: "Nx", target: TargetChar::Fixed(CHAR_N_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: "NX", target: TargetChar::Fixed(CHAR_N_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: "mx", target: TargetChar::Niggahita(false), is_extended: false },
    MappingRule { sequence: "Mx", target: TargetChar::Niggahita(true), is_extended: false },
    MappingRule { sequence: "MX", target: TargetChar::Niggahita(true), is_extended: false },
    MappingRule { sequence: "lx", target: TargetChar::Fixed(CHAR_L_DOT_BELOW_LOWER), is_extended: false },
    MappingRule { sequence: "Lx", target: TargetChar::Fixed(CHAR_L_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: "LX", target: TargetChar::Fixed(CHAR_L_DOT_BELOW_UPPER), is_extended: false },

    // Velar nasal ṅ: "ngw" (canonical per §4.2) + aliases "ngx"
    MappingRule { sequence: "ngw", target: TargetChar::Fixed(CHAR_N_DOT_ABOVE_LOWER), is_extended: false },
    MappingRule { sequence: "Ngw", target: TargetChar::Fixed(CHAR_N_DOT_ABOVE_UPPER), is_extended: false },
    MappingRule { sequence: "NGW", target: TargetChar::Fixed(CHAR_N_DOT_ABOVE_UPPER), is_extended: false },
    MappingRule { sequence: "ngx", target: TargetChar::Fixed(CHAR_N_DOT_ABOVE_LOWER), is_extended: false },
    MappingRule { sequence: "Ngx", target: TargetChar::Fixed(CHAR_N_DOT_ABOVE_UPPER), is_extended: false },
    MappingRule { sequence: "NGX", target: TargetChar::Fixed(CHAR_N_DOT_ABOVE_UPPER), is_extended: false },

    // Palatal nasal ñ: "nw" (mnemonic companion to ngw) + aliases "ny" and "~n"
    MappingRule { sequence: "nw", target: TargetChar::Fixed(CHAR_N_TILDE_LOWER), is_extended: false },
    MappingRule { sequence: "Nw", target: TargetChar::Fixed(CHAR_N_TILDE_UPPER), is_extended: false },
    MappingRule { sequence: "NW", target: TargetChar::Fixed(CHAR_N_TILDE_UPPER), is_extended: false },
    MappingRule { sequence: "ny", target: TargetChar::Fixed(CHAR_N_TILDE_LOWER), is_extended: false },
    MappingRule { sequence: "Ny", target: TargetChar::Fixed(CHAR_N_TILDE_UPPER), is_extended: false },
    MappingRule { sequence: "NY", target: TargetChar::Fixed(CHAR_N_TILDE_UPPER), is_extended: false },
    MappingRule { sequence: "~n", target: TargetChar::Fixed(CHAR_N_TILDE_LOWER), is_extended: false },
    MappingRule { sequence: "~N", target: TargetChar::Fixed(CHAR_N_TILDE_UPPER), is_extended: false },

    // Extended Sanskrit / IAST set
    MappingRule { sequence: "ee", target: TargetChar::Extended(CHAR_E_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: "Ee", target: TargetChar::Extended(CHAR_E_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "EE", target: TargetChar::Extended(CHAR_E_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "oo", target: TargetChar::Extended(CHAR_O_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: "Oo", target: TargetChar::Extended(CHAR_O_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "OO", target: TargetChar::Extended(CHAR_O_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "rrx", target: TargetChar::Extended(CHAR_R_RING_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: "Rrx", target: TargetChar::Extended(CHAR_R_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "RRX", target: TargetChar::Extended(CHAR_R_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "rx", target: TargetChar::Extended(CHAR_R_RING_BELOW_LOWER), is_extended: true },
    MappingRule { sequence: "Rx", target: TargetChar::Extended(CHAR_R_RING_BELOW_UPPER), is_extended: true },
    MappingRule { sequence: "RX", target: TargetChar::Extended(CHAR_R_RING_BELOW_UPPER), is_extended: true },
    MappingRule { sequence: "llx", target: TargetChar::Extended(CHAR_L_RING_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: "Llx", target: TargetChar::Extended(CHAR_L_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "LLX", target: TargetChar::Extended(CHAR_L_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "hx", target: TargetChar::Extended(CHAR_H_DOT_BELOW_LOWER), is_extended: true },
    MappingRule { sequence: "Hx", target: TargetChar::Extended(CHAR_H_DOT_BELOW_UPPER), is_extended: true },
    MappingRule { sequence: "HX", target: TargetChar::Extended(CHAR_H_DOT_BELOW_UPPER), is_extended: true },
    MappingRule { sequence: "sx", target: TargetChar::Extended(CHAR_S_DOT_BELOW_LOWER), is_extended: true },
    MappingRule { sequence: "Sx", target: TargetChar::Extended(CHAR_S_DOT_BELOW_UPPER), is_extended: true },
    MappingRule { sequence: "SX", target: TargetChar::Extended(CHAR_S_DOT_BELOW_UPPER), is_extended: true },
    MappingRule { sequence: "sw", target: TargetChar::Extended(CHAR_S_ACUTE_LOWER), is_extended: true },
    MappingRule { sequence: "Sw", target: TargetChar::Extended(CHAR_S_ACUTE_UPPER), is_extended: true },
    MappingRule { sequence: "SW", target: TargetChar::Extended(CHAR_S_ACUTE_UPPER), is_extended: true },
];

pub static SCHEME_PALI_TELEX: SchemeDefinition = SchemeDefinition {
    id: SchemeId::PaliTelex,
    name: "palitelex",
    display_name: "PaliTelex",
    description: "Vietnamese Telex muscle memory (aa->ā, tx->ṭ, ngw->ṅ, nw/ny->ñ, mx->ṃ)",
    mappings: PALI_TELEX_MAPPINGS,
};

// =============================================================================
// Scheme 3: SIL / Keyman (DESIGN.md §4.3)
// =============================================================================

static SIL_MAPPINGS: &[MappingRule] = &[
    // Vowels with leading hyphen
    MappingRule { sequence: "-a", target: TargetChar::Fixed(CHAR_A_MACRON_LOWER), is_extended: false },
    MappingRule { sequence: "-A", target: TargetChar::Fixed(CHAR_A_MACRON_UPPER), is_extended: false },
    MappingRule { sequence: "-i", target: TargetChar::Fixed(CHAR_I_MACRON_LOWER), is_extended: false },
    MappingRule { sequence: "-I", target: TargetChar::Fixed(CHAR_I_MACRON_UPPER), is_extended: false },
    MappingRule { sequence: "-u", target: TargetChar::Fixed(CHAR_U_MACRON_LOWER), is_extended: false },
    MappingRule { sequence: "-U", target: TargetChar::Fixed(CHAR_U_MACRON_UPPER), is_extended: false },

    // Retroflex and diacritic consonants with dot
    MappingRule { sequence: ".t", target: TargetChar::Fixed(CHAR_T_DOT_BELOW_LOWER), is_extended: false },
    MappingRule { sequence: ".T", target: TargetChar::Fixed(CHAR_T_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: ".d", target: TargetChar::Fixed(CHAR_D_DOT_BELOW_LOWER), is_extended: false },
    MappingRule { sequence: ".D", target: TargetChar::Fixed(CHAR_D_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: ".n", target: TargetChar::Fixed(CHAR_N_DOT_BELOW_LOWER), is_extended: false },
    MappingRule { sequence: ".N", target: TargetChar::Fixed(CHAR_N_DOT_BELOW_UPPER), is_extended: false },
    MappingRule { sequence: ".m", target: TargetChar::Niggahita(false), is_extended: false },
    MappingRule { sequence: ".M", target: TargetChar::Niggahita(true), is_extended: false },
    MappingRule { sequence: ".l", target: TargetChar::Fixed(CHAR_L_DOT_BELOW_LOWER), is_extended: false },
    MappingRule { sequence: ".L", target: TargetChar::Fixed(CHAR_L_DOT_BELOW_UPPER), is_extended: false },

    // Velar nasal '>n' and palatal nasal '~n'
    MappingRule { sequence: ">n", target: TargetChar::Fixed(CHAR_N_DOT_ABOVE_LOWER), is_extended: false },
    MappingRule { sequence: ">N", target: TargetChar::Fixed(CHAR_N_DOT_ABOVE_UPPER), is_extended: false },
    MappingRule { sequence: "~n", target: TargetChar::Fixed(CHAR_N_TILDE_LOWER), is_extended: false },
    MappingRule { sequence: "~N", target: TargetChar::Fixed(CHAR_N_TILDE_UPPER), is_extended: false },

    // Extended Sanskrit / IAST set
    MappingRule { sequence: "-e", target: TargetChar::Extended(CHAR_E_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: "-E", target: TargetChar::Extended(CHAR_E_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: "-o", target: TargetChar::Extended(CHAR_O_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: "-O", target: TargetChar::Extended(CHAR_O_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: ".r", target: TargetChar::Extended(CHAR_R_RING_BELOW_LOWER), is_extended: true },
    MappingRule { sequence: ".R", target: TargetChar::Extended(CHAR_R_RING_BELOW_UPPER), is_extended: true },
    MappingRule { sequence: "~r", target: TargetChar::Extended(CHAR_R_RING_MACRON_LOWER), is_extended: true },
    MappingRule { sequence: "~R", target: TargetChar::Extended(CHAR_R_RING_MACRON_UPPER), is_extended: true },
    MappingRule { sequence: ".h", target: TargetChar::Extended(CHAR_H_DOT_BELOW_LOWER), is_extended: true },
    MappingRule { sequence: ".H", target: TargetChar::Extended(CHAR_H_DOT_BELOW_UPPER), is_extended: true },
    MappingRule { sequence: ".s", target: TargetChar::Extended(CHAR_S_DOT_BELOW_LOWER), is_extended: true },
    MappingRule { sequence: ".S", target: TargetChar::Extended(CHAR_S_DOT_BELOW_UPPER), is_extended: true },
    MappingRule { sequence: "~s", target: TargetChar::Extended(CHAR_S_ACUTE_LOWER), is_extended: true },
    MappingRule { sequence: "~S", target: TargetChar::Extended(CHAR_S_ACUTE_UPPER), is_extended: true },
];

pub static SCHEME_SIL: SchemeDefinition = SchemeDefinition {
    id: SchemeId::Sil,
    name: "sil",
    display_name: "SIL / Keyman",
    description: "Keyman Indic Roman Transliteration standard (-a->ā, .t->ṭ, >n->ṅ, ~n->ñ)",
    mappings: SIL_MAPPINGS,
};

// =============================================================================
// Scheme Registry & Accessors
// =============================================================================

/// Returns the static scheme definition for a given scheme ID.
pub fn get_scheme(id: SchemeId) -> &'static SchemeDefinition {
    match id {
        SchemeId::Velthuis => &SCHEME_VELTHUIS,
        SchemeId::PaliTelex => &SCHEME_PALI_TELEX,
        SchemeId::Sil => &SCHEME_SIL,
    }
}

/// Finds a scheme definition by its canonical string name.
pub fn find_scheme_by_name(name: &str) -> Option<&'static SchemeDefinition> {
    match name.to_ascii_lowercase().as_str() {
        "velthuis" => Some(&SCHEME_VELTHUIS),
        "palitelex" | "telex" => Some(&SCHEME_PALI_TELEX),
        "sil" | "keyman" => Some(&SCHEME_SIL),
        _ => None,
    }
}

static ALL_SCHEMES: [SchemeDefinition; 3] = [SCHEME_VELTHUIS, SCHEME_PALI_TELEX, SCHEME_SIL];

/// Returns a slice of all available schemes in the engine.
pub fn all_schemes() -> &'static [SchemeDefinition] {
    &ALL_SCHEMES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheme_retrieval() {
        assert_eq!(get_scheme(SchemeId::Velthuis).name, "velthuis");
        assert_eq!(get_scheme(SchemeId::PaliTelex).name, "palitelex");
        assert_eq!(get_scheme(SchemeId::Sil).name, "sil");
    }

    #[test]
    fn test_find_by_name() {
        assert_eq!(find_scheme_by_name("velthuis").unwrap().id, SchemeId::Velthuis);
        assert_eq!(find_scheme_by_name("PaliTelex").unwrap().id, SchemeId::PaliTelex);
        assert_eq!(find_scheme_by_name("sil").unwrap().id, SchemeId::Sil);
        assert!(find_scheme_by_name("nonexistent").is_none());
    }

    #[test]
    fn test_mappings_non_empty() {
        for scheme in all_schemes() {
            assert!(!scheme.mappings.is_empty(), "Scheme {} has no mappings", scheme.name);
        }
    }
}
