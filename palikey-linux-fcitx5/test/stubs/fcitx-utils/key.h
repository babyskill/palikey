#pragma once
#include <cstdint>
#include "keysym.h"

namespace fcitx {

enum class KeyState {
    Alt = 1,
    Ctrl = 2,
    Super = 4,
    Mod5 = 8,
    Shift = 16
};

class KeyStates {
public:
    KeyStates(uint32_t m = 0) : m_mask(m) {}
    bool test(KeyState s) const { return (m_mask & static_cast<uint32_t>(s)) != 0; }
private:
    uint32_t m_mask;
};

class Key {
public:
    Key(uint32_t sym = 0, KeyStates s = {}) : m_sym(sym), m_states(s) {}
    uint32_t sym() const { return m_sym; }
    KeyStates states() const { return m_states; }
    bool check(uint32_t s) const { return m_sym == s; }
    static uint32_t keySymToUnicode(uint32_t sym) {
        if (sym >= 0x20 && sym <= 0x7e) return sym;
        return 0;
    }
private:
    uint32_t m_sym;
    KeyStates m_states;
};

} // namespace fcitx
