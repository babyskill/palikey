//! State machine transducer engine for Pāḷi input methods.
//!
//! Clean-room implementation per DESIGN.md §3.
//!
//! Input: KeyAction (Char / Backspace / Reset)
//! Output: ProcessResult { backspaces: u8, commit: String, preedit: String }

use crate::chars::*;
use crate::schemes::*;
use crate::vi_telex::VietnameseTelexComposer;

/// The result returned by the engine after processing an input action.
///
/// Shells use this to mutate the target text field or preedit buffer:
/// - `backspaces`: number of characters to delete before inserting `commit`.
/// - `commit`: string to commit to the target input field.
/// - `preedit`: active composition preview (empty if direct-commit mode).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct ProcessResult {
    /// Number of characters to erase via backspace before committing.
    pub backspaces: u8,
    /// String to commit to the document.
    pub commit: String,
    /// In-progress composition buffer (for preedit-capable shells).
    pub preedit: String,
}

impl ProcessResult {
    /// Constructs an empty result (no backspaces, nothing to commit).
    pub fn empty() -> Self {
        Self::default()
    }

    /// Constructs a direct commit result with no backspaces.
    pub fn direct(commit: impl Into<String>) -> Self {
        Self {
            backspaces: 0,
            commit: commit.into(),
            preedit: String::new(),
        }
    }

    /// Constructs a replacement result (erase `backspaces` chars, then commit).
    pub fn replace(backspaces: u8, commit: impl Into<String>) -> Self {
        Self {
            backspaces,
            commit: commit.into(),
            preedit: String::new(),
        }
    }

    /// Constructs a backspace-only result (erase `count` chars, commit nothing).
    pub fn erase(count: u8) -> Self {
        Self {
            backspaces: count,
            commit: String::new(),
            preedit: String::new(),
        }
    }
}

/// Keystroke actions accepted by the engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    /// A printable Unicode character was typed.
    Char(char),
    /// The Backspace key was pressed.
    Backspace,
    /// Reset the composition state (e.g. focus lost or cursor moved).
    Reset,
}

/// Configuration settings for the PaliKey engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineConfig {
    /// The active transliteration scheme.
    pub scheme: SchemeId,
    /// Niggahīta variant: ṃ (U+1E43, default) or ṁ (U+1E41).
    pub niggahita: NiggahitaVariant,
    /// Enable extended Sanskrit/IAST characters (ṛ ṝ ḹ ḥ ś ṣ ē ō).
    pub extended_set: bool,
    /// Escape prefix character (default: Some('\\')).
    pub escape_char: Option<char>,
    /// Allow backspace immediately after composition to revert to raw keystrokes.
    pub undo_by_backspace: bool,
    /// Allow repeating the trigger key to revert composition and emit raw characters.
    pub repeat_key_escapes: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            scheme: SchemeId::Velthuis,
            niggahita: NiggahitaVariant::DotBelow,
            extended_set: false,
            escape_char: Some('\\'),
            undo_by_backspace: true,
            repeat_key_escapes: true,
        }
    }
}

/// Record of the most recent composition event, allowing single-level undo.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CompositionRecord {
    /// The raw sequence that triggered the composition (e.g. ".t" or "aa" or "ngw").
    raw_sequence: String,
    /// The composed character produced (e.g. 'ṭ', 'ā', 'ṅ').
    composed_char: char,
    /// The specific key that completed the sequence (e.g. 't' in ".t", 'a' in "aa").
    trigger_char: char,
}

/// The core state machine transducer.
#[derive(Debug, Clone)]
pub struct Engine {
    config: EngineConfig,
    /// Current word buffer of raw keystrokes and composed characters.
    buffer: String,
    /// Last composition event for single-level backspace-undo and repeat-key escape.
    last_composition: Option<CompositionRecord>,
    /// Whether the next character is escaped by the escape prefix.
    escape_active: bool,
    /// Dedicated Vietnamese Telex composer for SchemeId::VietnameseTelex.
    vi_composer: VietnameseTelexComposer,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(EngineConfig::default())
    }
}

impl Engine {
    /// Creates a new engine with the specified configuration.
    pub fn new(config: EngineConfig) -> Self {
        Self {
            config,
            buffer: String::with_capacity(64),
            last_composition: None,
            escape_active: false,
            vi_composer: VietnameseTelexComposer::new(),
        }
    }

    /// Creates a new engine with the default configuration using a specific scheme.
    pub fn with_scheme(scheme: SchemeId) -> Self {
        Self::new(EngineConfig {
            scheme,
            ..EngineConfig::default()
        })
    }

    /// Returns a reference to the active configuration.
    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    /// Updates the configuration.
    pub fn set_config(&mut self, config: EngineConfig) {
        self.config = config;
    }

    /// Changes the active scheme.
    pub fn set_scheme(&mut self, scheme: SchemeId) {
        self.config.scheme = scheme;
        self.reset();
    }

    /// Changes the niggahīta variant.
    pub fn set_niggahita(&mut self, variant: NiggahitaVariant) {
        self.config.niggahita = variant;
    }

    /// Sets whether the extended Sanskrit/IAST set is active.
    pub fn set_extended_set(&mut self, enabled: bool) {
        self.config.extended_set = enabled;
    }

    /// Returns the contents of the current word buffer.
    pub fn buffer(&self) -> &str {
        if self.config.scheme == SchemeId::VietnameseTelex {
            self.vi_composer.buffer()
        } else {
            &self.buffer
        }
    }

    /// Main entry point: process a KeyAction through the state machine.
    pub fn process(&mut self, action: KeyAction) -> ProcessResult {
        if self.config.scheme == SchemeId::VietnameseTelex {
            return match action {
                KeyAction::Char(c) => self.vi_composer.process_char(c),
                KeyAction::Backspace => self.vi_composer.process_backspace(),
                KeyAction::Reset => self.vi_composer.reset(),
            };
        }

        match action {
            KeyAction::Char(c) => self.process_char(c),
            KeyAction::Backspace => self.process_backspace(),
            KeyAction::Reset => self.reset(),
        }
    }

    /// Processes a single typed character.
    pub fn process_char(&mut self, c: char) -> ProcessResult {
        // Rule: Escape prefix handling (e.g. '\')
        if let Some(esc) = self.config.escape_char {
            if c == esc {
                if self.escape_active {
                    // Double escape (\\) produces literal escape char
                    self.escape_active = false;
                    self.buffer.clear();
                    self.last_composition = None;
                    return ProcessResult::direct(c.to_string());
                } else {
                    // Activate escape: swallow the escape char and escape following word
                    self.escape_active = true;
                    self.buffer.clear();
                    self.last_composition = None;
                    return ProcessResult::empty();
                }
            }
        }

        // Rule: Digits flush the buffer per DESIGN.md §3
        if c.is_ascii_digit() {
            self.buffer.clear();
            self.last_composition = None;
            self.escape_active = false;
            return ProcessResult::direct(c.to_string());
        }

        // Rule: Word boundaries flush the buffer
        // Spaces, standard punctuation (comma, semicolon, colon, brackets, slash) flush the buffer.
        // We preserve scheme prefix punctuation ('.', '"', '~', '-', '>') within the word.
        if self.is_word_boundary(c) {
            self.buffer.clear();
            self.last_composition = None;
            self.escape_active = false;
            return ProcessResult::direct(c.to_string());
        }

        // If escape is currently active, bypass composition for characters in this word
        if self.escape_active {
            self.buffer.clear();
            self.last_composition = None;
            return ProcessResult::direct(c.to_string());
        }

        // Sequence matching
        self.buffer.push(c);

        let scheme = get_scheme(self.config.scheme);

        // Find the longest matching sequence that matches the buffer's suffix
        let mut best_match: Option<(usize, char, &'static str)> = None;

        for rule in scheme.mappings {
            if rule.is_extended && !self.config.extended_set {
                continue;
            }

            if self.buffer.ends_with(rule.sequence) {
                if let Some(target) = rule.target.resolve(self.config.niggahita, self.config.extended_set) {
                    let seq_len = rule.sequence.chars().count();
                    match &best_match {
                        Some((best_len, _, _)) if *best_len >= seq_len => {}
                        _ => {
                            best_match = Some((seq_len, target, rule.sequence));
                        }
                    }
                }
            }
        }

        if let Some((seq_len, target_char, raw_seq)) = best_match {
            // The last char of raw_seq was just typed (c).
            // The previous (seq_len - 1) chars are already on screen.
            // We need to erase (seq_len - 1) characters and commit the target character.
            let backspaces = (seq_len - 1) as u8;

            // Replace the matched sequence at the tail of the buffer with the composed character
            let truncate_to = self.buffer.chars().count() - seq_len;
            let mut new_buf: String = self.buffer.chars().take(truncate_to).collect();
            new_buf.push(target_char);
            self.buffer = new_buf;

            // Record this composition for undo-by-backspace and repeat-key escape
            self.last_composition = Some(CompositionRecord {
                raw_sequence: raw_seq.to_string(),
                composed_char: target_char,
                trigger_char: c,
            });

            ProcessResult::replace(backspaces, target_char.to_string())
        } else if self.config.repeat_key_escapes {
            // Rule: Repeat-key escape (Velthuis / Telex tradition)
            // If the key does not form a longer transliteration sequence but repeats the
            // trigger key of the previous composition (e.g. 'aa' -> 'ā', now types 'a' again),
            // revert to the raw sequence + key (e.g. "aaa").
            if let Some(comp) = self.last_composition.take() {
                if c == comp.trigger_char {
                    let mut raw_expanded = comp.raw_sequence.clone();
                    raw_expanded.push(c);

                    // Backspace the composed char (1 backspace) and commit the expanded raw string
                    let res = ProcessResult::replace(1, raw_expanded.clone());

                    // In buffer, we already pushed c. Pop c and pop the composed char, then push raw_expanded
                    self.buffer.pop(); // pop c
                    if !self.buffer.is_empty() {
                        self.buffer.pop(); // pop composed char
                    }
                    self.buffer.push_str(&raw_expanded);
                    return res;
                }
            }
            // No composition and no repeat escape: pass character through directly
            self.last_composition = None;
            ProcessResult::direct(c.to_string())
        } else {
            // No composition match: pass character through directly
            self.last_composition = None;
            ProcessResult::direct(c.to_string())
        }
    }

    /// Processes a Backspace key event.
    ///
    /// Implements: "backspace-after-compose reverts to the raw sequence (one level)".
    pub fn process_backspace(&mut self) -> ProcessResult {
        // If backspace-after-compose is enabled and we just composed a character:
        if self.config.undo_by_backspace {
            if let Some(comp) = self.last_composition.take() {
                // Erase the 1 composed character on screen and commit the original raw sequence
                let res = ProcessResult::replace(1, comp.raw_sequence.clone());

                // In internal buffer: pop the composed character and restore the raw sequence
                if !self.buffer.is_empty() {
                    self.buffer.pop();
                }
                self.buffer.push_str(&comp.raw_sequence);

                // Clear composition record so subsequent backspace behaves normally (one level only)
                return res;
            }
        }

        // Standard backspace: pop one character from the buffer and emit 1 backspace
        self.last_composition = None;
        if !self.buffer.is_empty() {
            self.buffer.pop();
        }
        ProcessResult::erase(1)
    }

    /// Resets the engine state (clears buffer and undo history).
    pub fn reset(&mut self) -> ProcessResult {
        self.buffer.clear();
        self.last_composition = None;
        self.escape_active = false;
        self.vi_composer.reset();
        ProcessResult::empty()
    }

    /// Convenience helper for tests and verification:
    /// Simulates typing a string and applying each ProcessResult to produce the final text.
    pub fn type_text(&mut self, text: &str) -> String {
        let mut output = String::new();
        for c in text.chars() {
            let res = self.process_char(c);
            self.apply_result(&mut output, res);
        }
        output
    }

    /// Simulates pressing backspace on an output accumulator.
    pub fn type_backspace(&mut self, output: &mut String) {
        let res = self.process_backspace();
        self.apply_result(output, res);
    }

    /// Applies a ProcessResult to a mutable accumulator string (as an OS text field would).
    pub fn apply_result(&self, output: &mut String, res: ProcessResult) {
        for _ in 0..res.backspaces {
            output.pop();
        }
        output.push_str(&res.commit);
    }

    /// Checks if a character is a word boundary that resets composition.
    fn is_word_boundary(&self, c: char) -> bool {
        if c.is_whitespace() {
            return true;
        }

        // Punctuation that delimits words (excluding scheme prefixes: . " ~ - >)
        matches!(
            c,
            ',' | ';'
                | ':'
                | '!'
                | '?'
                | '('
                | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | '/'
                | '|'
                | '_'
                | '+'
                | '='
                | '*'
                | '&'
                | '%'
                | '$'
                | '#'
                | '@'
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velthuis_basic() {
        let mut engine = Engine::with_scheme(SchemeId::Velthuis);
        assert_eq!(engine.type_text("aa"), "ā");
        assert_eq!(engine.type_text(" .t"), " ṭ");
        assert_eq!(engine.type_text(" \"n"), " ṅ");
        assert_eq!(engine.type_text(" ~n"), " ñ");
        assert_eq!(engine.type_text(" .m"), " ṃ");
    }

    #[test]
    fn test_palitelex_basic() {
        let mut engine = Engine::with_scheme(SchemeId::PaliTelex);
        assert_eq!(engine.type_text("aa"), "ā");
        assert_eq!(engine.type_text(" tx"), " ṭ");
        assert_eq!(engine.type_text(" ngw"), " ṅ");
        assert_eq!(engine.type_text(" nw"), " ñ");
        assert_eq!(engine.type_text(" mx"), " ṃ");
    }

    #[test]
    fn test_sil_basic() {
        let mut engine = Engine::with_scheme(SchemeId::Sil);
        assert_eq!(engine.type_text("-a"), "ā");
        assert_eq!(engine.type_text(" .t"), " ṭ");
        assert_eq!(engine.type_text(" >n"), " ṅ");
        assert_eq!(engine.type_text(" ~n"), " ñ");
        assert_eq!(engine.type_text(" .m"), " ṃ");
    }

    #[test]
    fn test_backspace_undo_one_level() {
        let mut engine = Engine::with_scheme(SchemeId::Velthuis);
        let mut text = String::new();

        // Type .t -> ṭ
        for c in ".t".chars() {
            let res = engine.process_char(c);
            engine.apply_result(&mut text, res);
        }
        assert_eq!(text, "ṭ");

        // Backspace immediately after compose reverts to raw ".t"
        engine.type_backspace(&mut text);
        assert_eq!(text, ".t");

        // Backspace again is a normal backspace, deleting 't'
        engine.type_backspace(&mut text);
        assert_eq!(text, ".");

        // Backspace again deletes '.'
        engine.type_backspace(&mut text);
        assert_eq!(text, "");
    }

    #[test]
    fn test_digits_flush_buffer() {
        let mut engine = Engine::with_scheme(SchemeId::Velthuis);
        let mut text = String::new();

        // Type '.' then '1' then 't'
        // '1' flushes the buffer, so 't' does not combine with '.'
        for c in ".1t".chars() {
            let res = engine.process_char(c);
            engine.apply_result(&mut text, res);
        }
        assert_eq!(text, ".1t");
    }

    #[test]
    fn test_repeat_key_escape() {
        let mut engine = Engine::with_scheme(SchemeId::Velthuis);
        // "aa" -> "ā", third 'a' expands to "aaa"
        assert_eq!(engine.type_text("aaa"), "aaa");
    }
}
