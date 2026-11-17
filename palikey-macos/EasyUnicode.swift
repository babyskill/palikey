//
//  EasyUnicode.swift
//  PaliKey
//
//  Option / AltGr direct-insert modifier layer for EasyUnicode mapping.
//  Per DESIGN.md §4: shell-level extra bypassing the transducer.
//

import Foundation

public enum EasyUnicode {
    /// Maps a typed character with Option modifier to its Pāḷi Unicode equivalent.
    ///
    /// - Parameters:
    ///   - char: The unadorned key character (e.g. from charactersIgnoringModifiers).
    ///   - isShift: Whether Shift is pressed.
    ///   - niggahita: Niggahīta variant (0: DotBelow ṃ, 1: DotAbove ṁ).
    /// - Returns: The composed Pāḷi string, or nil if no mapping exists.
    public static func map(char: Character, isShift: Bool, niggahita: UInt32 = 0) -> String? {
        let lower = Character(char.lowercased())
        let upper = isShift || char.isUppercase

        switch lower {
        // Long vowels (macrons)
        case "a":
            return upper ? "Ā" : "ā"
        case "i":
            return upper ? "Ī" : "ī"
        case "u":
            return upper ? "Ū" : "ū"
        case "e":
            return upper ? "Ē" : "ē"
        case "o":
            return upper ? "Ō" : "ō"

        // Retroflex consonants (dots below)
        case "t":
            return upper ? "Ṭ" : "ṭ"
        case "d":
            return upper ? "Ḍ" : "ḍ"
        case "n":
            return upper ? "Ṇ" : "ṇ"
        case "l":
            return upper ? "Ḷ" : "ḷ"

        // Anusvāra / Niggahīta
        case "m":
            if niggahita == 1 {
                return upper ? "Ṁ" : "ṁ"
            } else {
                return upper ? "Ṃ" : "ṃ"
            }

        // Velar nasal (ṅ / Ṅ) — 'g' or 'k'
        case "g", "k":
            return upper ? "Ṅ" : "ṅ"

        // Palatal nasal (ñ / Ñ) — 'j' or 'c' or '~'
        case "j", "c", "~":
            return upper ? "Ñ" : "ñ"

        // Sanskrit / IAST extensions
        case "r":
            return upper ? "Ṛ" : "ṛ"
        case "s":
            return upper ? "Ṣ" : "ṣ"
        case "h":
            return upper ? "Ḥ" : "ḥ"

        default:
            return nil
        }
    }
}
