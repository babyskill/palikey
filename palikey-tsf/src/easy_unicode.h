#pragma once

#ifndef PALIKEY_TSF_EASY_UNICODE_H
#define PALIKEY_TSF_EASY_UNICODE_H

#include <windows.h>

/**
 * Maps a typed character with AltGr / Option modifier to its Pāḷi Unicode equivalent.
 * Per DESIGN.md §4: shell-level extra bypassing the transducer.
 *
 * @param ch The unadorned key character.
 * @param isShift Whether Shift is active.
 * @param niggahita Niggahīta variant (0: DotBelow ṃ, 1: DotAbove ṁ).
 * @return Wide string literal with the Pāḷi character, or nullptr if no mapping exists.
 */
inline const WCHAR* EasyUnicodeMap(WCHAR ch, bool isShift, uint32_t niggahita = 0) {
    WCHAR lower = (WCHAR)(ULONG_PTR)CharLowerW((LPWSTR)(ULONG_PTR)ch);
    bool upper = isShift || (ch != lower);

    switch (lower) {
    // Long vowels (macrons)
    case L'a':
        return upper ? L"\x0100" : L"\x0101"; // Ā / ā
    case L'i':
        return upper ? L"\x012A" : L"\x012B"; // Ī / ī
    case L'u':
        return upper ? L"\x016A" : L"\x016B"; // Ū / ū
    case L'e':
        return upper ? L"\x0112" : L"\x0113"; // Ē / ē
    case L'o':
        return upper ? L"\x014C" : L"\x014D"; // Ō / ō

    // Retroflex consonants (dots below)
    case L't':
        return upper ? L"\x1E6C" : L"\x1E6D"; // Ṭ / ṭ
    case L'd':
        return upper ? L"\x1E0C" : L"\x1E0D"; // Ḍ / ḍ
    case L'n':
        return upper ? L"\x1E46" : L"\x1E47"; // Ṇ / ṇ
    case L'l':
        return upper ? L"\x1E36" : L"\x1E37"; // Ḷ / ḷ

    // Anusvāra / Niggahīta
    case L'm':
        if (niggahita == 1) {
            return upper ? L"\x1E40" : L"\x1E41"; // Ṁ / ṁ (Dot Above)
        } else {
            return upper ? L"\x1E42" : L"\x1E43"; // Ṃ / ṃ (Dot Below)
        }

    // Velar nasal (ṅ / Ṅ) — 'g' or 'k'
    case L'g':
    case L'k':
        return upper ? L"\x1E44" : L"\x1E45"; // Ṅ / ṅ

    // Palatal nasal (ñ / Ñ) — 'j' or 'c' or '~'
    case L'j':
    case L'c':
    case L'~':
        return upper ? L"\x00D1" : L"\x00F1"; // Ñ / ñ

    // Sanskrit / IAST extensions
    case L'r':
        return upper ? L"\x1E5A" : L"\x1E5B"; // Ṛ / ṛ
    case L's':
        return upper ? L"\x1E62" : L"\x1E63"; // Ṣ / ṣ
    case L'h':
        return upper ? L"\x1E24" : L"\x1E25"; // Ḥ / ḥ

    default:
        return nullptr;
    }
}

#endif // PALIKEY_TSF_EASY_UNICODE_H
