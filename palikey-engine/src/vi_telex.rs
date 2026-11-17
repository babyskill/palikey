//! Vietnamese Telex input method composer.
//!
//! Clean-room implementation per BRIEF_P5_VIETELEX.md.
//! Zero third-party dependencies.
//!
//! Maintains raw keystrokes per syllable and re-derives the syllable on every key.

use crate::engine::ProcessResult;

/// Tones in Vietnamese orthography.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tone {
    #[default]
    None,
    Sac,   // 's' = acute (´)
    Huyen, // 'f' = grave (`)
    Hoi,   // 'r' = hook above (ˀ)
    Nga,   // 'x' = tilde (~)
    Nang,  // 'j' = dot below (.)
}

impl Tone {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            's' => Some(Self::Sac),
            'f' => Some(Self::Huyen),
            'r' => Some(Self::Hoi),
            'x' => Some(Self::Nga),
            'j' => Some(Self::Nang),
            _ => None,
        }
    }
}

/// Applies a tone mark to a lowercase vowel character.
pub fn apply_tone(vowel: char, tone: Tone) -> char {
    match (vowel, tone) {
        ('a', Tone::None) => 'a',
        ('a', Tone::Sac) => 'á',
        ('a', Tone::Huyen) => 'à',
        ('a', Tone::Hoi) => 'ả',
        ('a', Tone::Nga) => 'ã',
        ('a', Tone::Nang) => 'ạ',

        ('ă', Tone::None) => 'ă',
        ('ă', Tone::Sac) => 'ắ',
        ('ă', Tone::Huyen) => 'ằ',
        ('ă', Tone::Hoi) => 'ẳ',
        ('ă', Tone::Nga) => 'ẵ',
        ('ă', Tone::Nang) => 'ặ',

        ('â', Tone::None) => 'â',
        ('â', Tone::Sac) => 'ấ',
        ('â', Tone::Huyen) => 'ầ',
        ('â', Tone::Hoi) => 'ẩ',
        ('â', Tone::Nga) => 'ẫ',
        ('â', Tone::Nang) => 'ậ',

        ('e', Tone::None) => 'e',
        ('e', Tone::Sac) => 'é',
        ('e', Tone::Huyen) => 'è',
        ('e', Tone::Hoi) => 'ẻ',
        ('e', Tone::Nga) => 'ẽ',
        ('e', Tone::Nang) => 'ẹ',

        ('ê', Tone::None) => 'ê',
        ('ê', Tone::Sac) => 'ế',
        ('ê', Tone::Huyen) => 'ề',
        ('ê', Tone::Hoi) => 'ể',
        ('ê', Tone::Nga) => 'ễ',
        ('ê', Tone::Nang) => 'ệ',

        ('i', Tone::None) => 'i',
        ('i', Tone::Sac) => 'í',
        ('i', Tone::Huyen) => 'ì',
        ('i', Tone::Hoi) => 'ỉ',
        ('i', Tone::Nga) => 'ĩ',
        ('i', Tone::Nang) => 'ị',

        ('o', Tone::None) => 'o',
        ('o', Tone::Sac) => 'ó',
        ('o', Tone::Huyen) => 'ò',
        ('o', Tone::Hoi) => 'ỏ',
        ('o', Tone::Nga) => 'õ',
        ('o', Tone::Nang) => 'ọ',

        ('ô', Tone::None) => 'ô',
        ('ô', Tone::Sac) => 'ố',
        ('ô', Tone::Huyen) => 'ồ',
        ('ô', Tone::Hoi) => 'ổ',
        ('ô', Tone::Nga) => 'ỗ',
        ('ô', Tone::Nang) => 'ộ',

        ('ơ', Tone::None) => 'ơ',
        ('ơ', Tone::Sac) => 'ớ',
        ('ơ', Tone::Huyen) => 'ờ',
        ('ơ', Tone::Hoi) => 'ở',
        ('ơ', Tone::Nga) => 'ỡ',
        ('ơ', Tone::Nang) => 'ợ',

        ('u', Tone::None) => 'u',
        ('u', Tone::Sac) => 'ú',
        ('u', Tone::Huyen) => 'ù',
        ('u', Tone::Hoi) => 'ủ',
        ('u', Tone::Nga) => 'ũ',
        ('u', Tone::Nang) => 'ụ',

        ('ư', Tone::None) => 'ư',
        ('ư', Tone::Sac) => 'ứ',
        ('ư', Tone::Huyen) => 'ừ',
        ('ư', Tone::Hoi) => 'ử',
        ('ư', Tone::Nga) => 'ữ',
        ('ư', Tone::Nang) => 'ự',

        ('y', Tone::None) => 'y',
        ('y', Tone::Sac) => 'ý',
        ('y', Tone::Huyen) => 'ỳ',
        ('y', Tone::Hoi) => 'ỷ',
        ('y', Tone::Nga) => 'ỹ',
        ('y', Tone::Nang) => 'ỵ',

        _ => vowel,
    }
}

/// Valid single and multi-letter onsets in Vietnamese orthography.
pub const VALID_ONSETS: &[&str] = &[
    "", "b", "c", "ch", "d", "đ", "g", "gh", "gi", "h", "k", "kh",
    "l", "m", "n", "ng", "ngh", "nh", "p", "ph", "qu", "r", "s",
    "t", "th", "tr", "v", "x",
];

/// Valid codas in Vietnamese orthography.
pub const VALID_CODAS: &[&str] = &[
    "", "c", "ch", "m", "n", "ng", "nh", "p", "t",
];

/// Checks if an ASCII character is a basic vowel.
pub fn is_basic_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}

/// Checks if a character is any Vietnamese vowel (including modified vowels).
pub fn is_any_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'ă' | 'â' | 'e' | 'ê' | 'i' | 'o' | 'ô' | 'ơ' | 'u' | 'ư' | 'y'
    )
}

/// Represents the parsed components of a Vietnamese syllable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedSyllable {
    pub onset: String,
    pub vowels: Vec<char>,
    pub coda: String,
    pub tone: Tone,
    pub literal_tail: String,
    pub is_valid: bool,
}

impl ParsedSyllable {
    /// Formats the syllable into its composed UTF-8 string with tone applied.
    pub fn compose(&self) -> String {
        let mut vowels = self.vowels.clone();

        if self.tone != Tone::None && !vowels.is_empty() {
            let tone_idx = determine_tone_position(&vowels, !self.coda.is_empty());
            if tone_idx < vowels.len() {
                vowels[tone_idx] = apply_tone(vowels[tone_idx], self.tone);
            }
        }

        let mut out = String::new();
        out.push_str(&self.onset);
        for v in vowels {
            out.push(v);
        }
        out.push_str(&self.coda);
        out.push_str(&self.literal_tail);
        out
    }
}

/// Modern Vietnamese tone position rules (§A3).
pub fn determine_tone_position(vowels: &[char], has_coda: bool) -> usize {
    match vowels.len() {
        1 => 0,
        2 => {
            if has_coda {
                // Rule 2: 2 vowels WITH coda -> 2nd vowel (hoàn, toán, tiếng, muốn, đường)
                1
            } else {
                // Rule 3: 2 vowels WITHOUT coda
                let pair: String = vowels.iter().collect();
                match pair.as_str() {
                    // Modern pairs: oa, oe, uy, uê -> 2nd vowel (hoá, khoẻ, thuý, huế)
                    "oa" | "oe" | "uy" | "uê" => 1,
                    // Pairs ia, ua, ưa, ai, ao, au, ay, âu, ây, eo, êu, iu, oi, ui, ươ -> 1st vowel
                    "ia" | "ua" | "ưa" | "ai" | "ao" | "au" | "ay" | "âu" | "ây" | "eo"
                    | "êu" | "iu" | "oi" | "ui" | "ươ" => 0,
                    // Default without coda -> 1st vowel
                    _ => 0,
                }
            }
        }
        3 => {
            // Rule 4: 3 vowels -> middle vowel (kiều, ngoài, tuổi, rượu, khuỷu)
            1
        }
        _ => 0,
    }
}

/// Derives a Vietnamese syllable from a slice of raw keystrokes.
/// Returns None if the keystroke buffer cannot form a valid syllable (impossible onset).
pub fn derive_syllable(raw: &[char]) -> Option<String> {
    if raw.is_empty() {
        return Some(String::new());
    }

    let is_all_upper = raw.iter().all(|c| !c.is_alphabetic() || c.is_uppercase());
    let is_first_upper = raw.first().map_or(false, |c| c.is_uppercase());

    let lower: Vec<char> = raw.iter().map(|c| c.to_ascii_lowercase()).collect();

    let parsed = parse_raw_keys(&lower)?;
    let mut composed = parsed.compose();

    if is_all_upper {
        composed = composed.to_uppercase();
    } else if is_first_upper {
        let mut chars = composed.chars();
        composed = match chars.next() {
            None => String::new(),
            Some(first) => {
                let upper_first = if first == 'đ' {
                    'Đ'
                } else {
                    first.to_uppercase().next().unwrap_or(first)
                };
                let mut s = String::new();
                s.push(upper_first);
                s.extend(chars);
                s
            }
        };
    }

    Some(composed)
}

/// Parses lowercase raw keys into components.
pub fn parse_raw_keys(lower: &[char]) -> Option<ParsedSyllable> {
    if lower.is_empty() {
        return Some(ParsedSyllable {
            onset: String::new(),
            vowels: Vec::new(),
            coda: String::new(),
            tone: Tone::None,
            literal_tail: String::new(),
            is_valid: true,
        });
    }

    // 1. Detect Onset
    let mut onset = String::new();
    let mut idx = 0;

    if lower[0] == 'd' {
        let mut d_count = 0;
        while idx < lower.len() && lower[idx] == 'd' {
            d_count += 1;
            idx += 1;
        }
        if d_count == 1 {
            onset = "d".to_string();
        } else if d_count % 2 == 0 {
            onset = "đ".to_string();
        } else {
            // ddd -> dd undo
            onset = "dd".to_string();
        }
    } else if lower[0] == 'w' {
        // 'w' at start produces 'ư'
        // But count consecutive 'w's: ww -> w (undo)
        let mut w_count = 0;
        while idx < lower.len() && lower[idx] == 'w' {
            w_count += 1;
            idx += 1;
        }
        if w_count % 2 == 1 {
            onset = String::new();
            // We will inject 'ư' into vowels below
        } else {
            // ww -> w
            onset = "w".to_string();
        }
    } else if is_basic_vowel(lower[0]) {
        onset = String::new();
    } else {
        // Match standard onset table
        if lower.len() >= 3 && &lower[0..3] == ['n', 'g', 'h'] {
            onset = "ngh".to_string();
            idx = 3;
        } else if lower.len() >= 2 {
            let pair = [lower[0], lower[1]];
            match pair {
                ['c', 'h'] => { onset = "ch".to_string(); idx = 2; }
                ['g', 'h'] => { onset = "gh".to_string(); idx = 2; }
                ['k', 'h'] => { onset = "kh".to_string(); idx = 2; }
                ['n', 'g'] => { onset = "ng".to_string(); idx = 2; }
                ['n', 'h'] => { onset = "nh".to_string(); idx = 2; }
                ['p', 'h'] => { onset = "ph".to_string(); idx = 2; }
                ['q', 'u'] => { onset = "qu".to_string(); idx = 2; }
                ['t', 'h'] => { onset = "th".to_string(); idx = 2; }
                ['t', 'r'] => { onset = "tr".to_string(); idx = 2; }
                ['g', 'i'] => {
                    // "gi is an onset unit when followed by another vowel (già = gi + a)
                    // but in gì the i is the nucleus"
                    if lower.len() > 2 && (is_basic_vowel(lower[2]) || lower[2] == 'w') {
                        onset = "gi".to_string();
                        idx = 2;
                    } else {
                        onset = "g".to_string();
                        idx = 1;
                    }
                }
                _ => {
                    if matches!(lower[0], 'b' | 'c' | 'g' | 'h' | 'k' | 'l' | 'm' | 'n' | 'p' | 'r' | 's' | 't' | 'v' | 'x') {
                        onset.push(lower[0]);
                        idx = 1;
                    } else {
                        return None; // Impossible onset
                    }
                }
            }
        } else if matches!(lower[0], 'b' | 'c' | 'g' | 'h' | 'k' | 'l' | 'm' | 'n' | 'p' | 'r' | 's' | 't' | 'v' | 'x') {
            onset.push(lower[0]);
            idx = 1;
        } else {
            return None; // Impossible onset (e.g. z, j, f)
        }
    }

    // 2. Process Nucleus, Coda, Modifiers, and Tones
    let mut vowels = Vec::new();
    let mut coda = String::new();
    let mut tone = Tone::None;
    let mut last_tone_char: Option<char> = None;
    let mut literal_tail = String::new();

    // If started with 'w' at syllable start (odd count)
    if lower[0] == 'w' && onset.is_empty() {
        vowels.push('ư');
    }

    while idx < lower.len() {
        let c = lower[idx];
        idx += 1;

        // Check for tone keys: s, f, r, x, j, z
        // Tone keys only apply once vowels have begun (or if we already have vowels)
        if !vowels.is_empty() && matches!(c, 's' | 'f' | 'r' | 'x' | 'j' | 'z') {
            if c == 'z' {
                if tone != Tone::None {
                    tone = Tone::None;
                    last_tone_char = None;
                } else {
                    literal_tail.push('z');
                }
            } else if last_tone_char == Some(c) {
                // Pressing the same tone key again removes the tone and emits literally
                tone = Tone::None;
                last_tone_char = None;
                literal_tail.push(c);
            } else {
                tone = Tone::from_char(c).unwrap_or(Tone::None);
                last_tone_char = Some(c);
            }
            continue;
        }

        // Vowel modifier 'w'
        if c == 'w' {
            if vowels.is_empty() {
                vowels.push('ư');
            } else if coda.is_empty() {
                let last = *vowels.last().unwrap();
                match last {
                    'a' => {
                        *vowels.last_mut().unwrap() = 'ă';
                    }
                    'ă' => {
                        // Undo: aww -> aw
                        *vowels.last_mut().unwrap() = 'a';
                        literal_tail.push('w');
                    }
                    'o' => {
                        if vowels.len() >= 2 && vowels[vowels.len() - 2] == 'u' {
                            // u + o + w -> ươ
                            let len = vowels.len();
                            vowels[len - 2] = 'ư';
                            vowels[len - 1] = 'ơ';
                        } else if vowels.len() >= 2 && vowels[vowels.len() - 2] == 'ư' {
                            // ư + o + w -> ươ
                            let len = vowels.len();
                            vowels[len - 1] = 'ơ';
                        } else {
                            *vowels.last_mut().unwrap() = 'ơ';
                        }
                    }
                    'ơ' => {
                        if vowels.len() >= 2 && vowels[vowels.len() - 2] == 'ư' {
                            // Undo: uoww -> uow
                            let len = vowels.len();
                            vowels[len - 2] = 'u';
                            vowels[len - 1] = 'o';
                            literal_tail.push('w');
                        } else {
                            *vowels.last_mut().unwrap() = 'o';
                            literal_tail.push('w');
                        }
                    }
                    'u' => {
                        *vowels.last_mut().unwrap() = 'ư';
                    }
                    'ư' => {
                        // Undo: uww -> uw
                        *vowels.last_mut().unwrap() = 'u';
                        literal_tail.push('w');
                    }
                    _ => {
                        vowels.push('ư');
                    }
                }
            } else {
                // Coda non-empty: e.g. duongw -> dương
                if vowels == ['u', 'o'] {
                    vowels = vec!['ư', 'ơ'];
                } else {
                    literal_tail.push('w');
                }
            }
            continue;
        }

        // Circumflex doubling: aa->â, ee->ê, oo->ô
        if coda.is_empty() {
            if c == 'a' && vowels.last() == Some(&'a') {
                *vowels.last_mut().unwrap() = 'â';
                continue;
            }
            if c == 'a' && vowels.last() == Some(&'â') {
                // Undo: aaa -> aa
                *vowels.last_mut().unwrap() = 'a';
                literal_tail.push('a');
                continue;
            }
            if c == 'e' && vowels.last() == Some(&'e') {
                *vowels.last_mut().unwrap() = 'ê';
                continue;
            }
            if c == 'e' && vowels.last() == Some(&'ê') {
                // Undo: eee -> ee
                *vowels.last_mut().unwrap() = 'e';
                literal_tail.push('e');
                continue;
            }
            if c == 'o' && vowels.last() == Some(&'o') {
                *vowels.last_mut().unwrap() = 'ô';
                continue;
            }
            if c == 'o' && vowels.last() == Some(&'ô') {
                // Undo: ooo -> oo
                *vowels.last_mut().unwrap() = 'o';
                literal_tail.push('o');
                continue;
            }
        }

        // Plain vowel addition
        if is_basic_vowel(c) {
            if coda.is_empty() {
                if vowels.len() < 3 {
                    vowels.push(c);
                } else {
                    return None; // Exceeded max 3 vowels
                }
            } else {
                return None; // Vowel after coda in same syllable is invalid
            }
            continue;
        }

        // Consonant: must be coda (since onset is finished and vowels are present)
        if vowels.is_empty() {
            // Consonant following onset before any vowel -> impossible onset cluster (e.g. "st")
            return None;
        }

        if coda.is_empty() {
            if matches!(c, 'c' | 'm' | 'n' | 'p' | 't') {
                coda.push(c);
            } else {
                literal_tail.push(c);
            }
        } else if coda == "c" && c == 'h' {
            coda.push('h');
        } else if coda == "n" && (c == 'g' || c == 'h') {
            coda.push(c);
        } else {
            literal_tail.push(c);
        }
    }

    let is_valid = !vowels.is_empty()
        && literal_tail.is_empty()
        && VALID_ONSETS.contains(&onset.as_str())
        && VALID_CODAS.contains(&coda.as_str());

    Some(ParsedSyllable {
        onset,
        vowels,
        coda,
        tone,
        literal_tail,
        is_valid,
    })
}

/// The stateful Vietnamese Telex composer.
#[derive(Debug, Clone, Default)]
pub struct VietnameseTelexComposer {
    raw_buffer: Vec<char>,
    composed_buffer: String,
    bypassed: bool,
}

impl VietnameseTelexComposer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn buffer(&self) -> &str {
        &self.composed_buffer
    }

    pub fn raw_buffer(&self) -> &[char] {
        &self.raw_buffer
    }

    /// Processes a printable character.
    pub fn process_char(&mut self, c: char) -> ProcessResult {
        // 1. Digits flush buffer per §A4
        if c.is_ascii_digit() {
            return self.flush_and_emit(c);
        }

        // 2. Word boundary punctuation and whitespace flush buffer
        if is_boundary_key(c) {
            return self.flush_and_emit(c);
        }

        // 3. Letters
        if self.bypassed {
            self.raw_buffer.push(c);
            self.composed_buffer.push(c);
            return ProcessResult::direct(c.to_string());
        }

        self.raw_buffer.push(c);
        match derive_syllable(&self.raw_buffer) {
            Some(new_composed) => {
                let diff = compute_diff(&self.composed_buffer, &new_composed);
                self.composed_buffer = new_composed;
                diff
            }
            None => {
                // Syllable cannot be formed (impossible onset or cluster). Stop composing!
                self.bypassed = true;
                let raw_str: String = self.raw_buffer.iter().collect();
                let diff = compute_diff(&self.composed_buffer, &raw_str);
                self.composed_buffer = raw_str;
                diff
            }
        }
    }

    /// Processes Backspace key.
    pub fn process_backspace(&mut self) -> ProcessResult {
        if self.raw_buffer.is_empty() {
            return ProcessResult::erase(1);
        }

        self.raw_buffer.pop();

        if self.bypassed {
            if self.raw_buffer.is_empty() {
                self.bypassed = false;
                let len = self.composed_buffer.chars().count() as u8;
                self.composed_buffer.clear();
                return ProcessResult::erase(len.max(1));
            } else {
                let mut new_str = self.composed_buffer.clone();
                new_str.pop();
                let diff = compute_diff(&self.composed_buffer, &new_str);
                self.composed_buffer = new_str;
                return diff;
            }
        }

        if self.raw_buffer.is_empty() {
            let len = self.composed_buffer.chars().count() as u8;
            self.composed_buffer.clear();
            ProcessResult::erase(len)
        } else {
            let new_composed = derive_syllable(&self.raw_buffer)
                .unwrap_or_else(|| self.raw_buffer.iter().collect());
            let diff = compute_diff(&self.composed_buffer, &new_composed);
            self.composed_buffer = new_composed;
            diff
        }
    }

    /// Resets the composer state.
    pub fn reset(&mut self) -> ProcessResult {
        self.raw_buffer.clear();
        self.composed_buffer.clear();
        self.bypassed = false;
        ProcessResult::empty()
    }

    /// Flushes the current syllable on word boundaries and emits the boundary character.
    fn flush_and_emit(&mut self, boundary: char) -> ProcessResult {
        if self.raw_buffer.is_empty() {
            self.bypassed = false;
            return ProcessResult::direct(boundary.to_string());
        }

        let lower: Vec<char> = self.raw_buffer.iter().map(|c| c.to_ascii_lowercase()).collect();
        let is_valid = if self.bypassed {
            false
        } else {
            parse_raw_keys(&lower).map_or(false, |p| p.is_valid)
        };

        let result = if is_valid {
            // Structurally valid: composed form is already on screen; just emit boundary!
            ProcessResult::direct(boundary.to_string())
        } else {
            // NOT structurally valid: restore raw keystrokes (English escape) + boundary!
            let mut raw_str: String = self.raw_buffer.iter().collect();
            raw_str.push(boundary);
            compute_diff(&self.composed_buffer, &raw_str)
        };

        self.raw_buffer.clear();
        self.composed_buffer.clear();
        self.bypassed = false;
        result
    }
}

/// Computes the minimal backspace and commit difference between previous and new text.
pub fn compute_diff(prev: &str, next: &str) -> ProcessResult {
    let prev_chars: Vec<char> = prev.chars().collect();
    let next_chars: Vec<char> = next.chars().collect();

    let common_prefix = prev_chars
        .iter()
        .zip(next_chars.iter())
        .take_while(|(a, b)| a == b)
        .count();

    let backspaces = (prev_chars.len() - common_prefix) as u8;
    let commit: String = next_chars[common_prefix..].iter().collect();

    ProcessResult::replace(backspaces, commit)
}

/// Checks if a character is a word boundary in Vietnamese text.
pub fn is_boundary_key(c: char) -> bool {
    c.is_whitespace() || c.is_ascii_punctuation() || (!c.is_alphabetic() && !c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn type_str(composer: &mut VietnameseTelexComposer, input: &str) -> String {
        let mut display = String::new();
        for c in input.chars() {
            let res = composer.process_char(c);
            for _ in 0..res.backspaces {
                display.pop();
            }
            display.push_str(&res.commit);
        }
        display
    }

    #[test]
    fn test_basic_vowels_and_modifiers() {
        let mut comp = VietnameseTelexComposer::new();
        assert_eq!(type_str(&mut comp, "aa"), "â");
        comp.reset();
        assert_eq!(type_str(&mut comp, "ee"), "ê");
        comp.reset();
        assert_eq!(type_str(&mut comp, "oo"), "ô");
        comp.reset();
        assert_eq!(type_str(&mut comp, "aw"), "ă");
        comp.reset();
        assert_eq!(type_str(&mut comp, "ow"), "ơ");
        comp.reset();
        assert_eq!(type_str(&mut comp, "uw"), "ư");
        comp.reset();
        assert_eq!(type_str(&mut comp, "dd"), "đ");
        comp.reset();
        assert_eq!(type_str(&mut comp, "w"), "ư");
    }

    #[test]
    fn test_modifier_undo() {
        let mut comp = VietnameseTelexComposer::new();
        assert_eq!(type_str(&mut comp, "aaa"), "aa");
        comp.reset();
        assert_eq!(type_str(&mut comp, "eee"), "ee");
        comp.reset();
        assert_eq!(type_str(&mut comp, "ooo"), "oo");
        comp.reset();
        assert_eq!(type_str(&mut comp, "aww"), "aw");
        comp.reset();
        assert_eq!(type_str(&mut comp, "oww"), "ow");
        comp.reset();
        assert_eq!(type_str(&mut comp, "uww"), "uw");
        comp.reset();
        assert_eq!(type_str(&mut comp, "ddd"), "dd");
    }

    #[test]
    fn test_tones_and_repositioning() {
        let mut comp = VietnameseTelexComposer::new();
        assert_eq!(type_str(&mut comp, "toans"), "toán");
        comp.reset();
        assert_eq!(type_str(&mut comp, "hosa"), "hoá");
        comp.reset();
        assert_eq!(type_str(&mut comp, "hoasz"), "hoa");
        comp.reset();
        assert_eq!(type_str(&mut comp, "ass"), "as");
    }

    #[test]
    fn test_brief_words() {
        let mut comp = VietnameseTelexComposer::new();
        assert_eq!(type_str(&mut comp, "vieetj"), "việt");
        comp.reset();
        assert_eq!(type_str(&mut comp, "tieengs"), "tiếng");
        comp.reset();
        assert_eq!(type_str(&mut comp, "dduowngf"), "đường");
        comp.reset();
        assert_eq!(type_str(&mut comp, "chaof"), "chào");
    }
}
