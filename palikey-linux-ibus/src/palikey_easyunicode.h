/**
 * @file palikey_easyunicode.h
 * @brief AltGr / Option direct-insert modifier layer for EasyUnicode mapping.
 *
 * Per DESIGN.md §4: shell-level extra bypassing the transducer.
 */

#ifndef PALIKEY_EASYUNICODE_H
#define PALIKEY_EASYUNICODE_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Maps a key character with AltGr / Option modifier to its Pāḷi Unicode equivalent.
 *
 * @param key_char Unicode codepoint of the pressed character (e.g. 'a').
 * @param is_shift True if Shift modifier is active.
 * @param niggahita Niggahīta variant (0: DotBelow ṃ, 1: DotAbove ṁ).
 * @return Null-terminated UTF-8 string with the character, or NULL if no mapping exists.
 */
static inline const char *palikey_easyunicode_map(uint32_t key_char, bool is_shift, uint32_t niggahita) {
    bool upper = is_shift;
    uint32_t lower = key_char;
    if (lower >= 'A' && lower <= 'Z') {
        upper = true;
        lower += ('a' - 'A');
    }

    switch (lower) {
        /* Long vowels (macrons) */
        case 'a': return upper ? "Ā" : "ā";
        case 'i': return upper ? "Ī" : "ī";
        case 'u': return upper ? "Ū" : "ū";
        case 'e': return upper ? "Ē" : "ē";
        case 'o': return upper ? "Ō" : "ō";

        /* Retroflex consonants (dots below) */
        case 't': return upper ? "Ṭ" : "ṭ";
        case 'd': return upper ? "Ḍ" : "ḍ";
        case 'n': return upper ? "Ṇ" : "ṇ";
        case 'l': return upper ? "Ḷ" : "ḷ";

        /* Anusvāra / Niggahīta */
        case 'm':
            if (niggahita == 1) {
                return upper ? "Ṁ" : "ṁ";
            } else {
                return upper ? "Ṃ" : "ṃ";
            }

        /* Velar nasal (ṅ / Ṅ) — 'g' or 'k' */
        case 'g':
        case 'k':
            return upper ? "Ṅ" : "ṅ";

        /* Palatal nasal (ñ / Ñ) — 'j' or 'c' or '~' */
        case 'j':
        case 'c':
        case '~':
            return upper ? "Ñ" : "ñ";

        /* Sanskrit / IAST extensions */
        case 'r': return upper ? "Ṛ" : "ṛ";
        case 's': return upper ? "Ṣ" : "ṣ";
        case 'h': return upper ? "Ḥ" : "ḥ";

        default:
            return (const char *)0;
    }
}

#ifdef __cplusplus
}
#endif

#endif /* PALIKEY_EASYUNICODE_H */
