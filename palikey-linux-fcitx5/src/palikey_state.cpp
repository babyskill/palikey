/**
 * @file palikey_state.cpp
 * @brief Implementation of per-context PaliKey engine state for Fcitx5.
 */

#include "palikey_state.h"
#include "palikey_engine_addon.h"

namespace fcitx {

PalikeyState::PalikeyState(PalikeyEngineAddon *addon, InputContext *ic)
    : m_addon(addon), m_ic(ic) {
    palikey_config_load(&m_config);
    m_engine = palikey_new(m_config.scheme, m_config.niggahita, m_config.extended_set);
}

PalikeyState::~PalikeyState() {
    if (m_engine) {
        palikey_free(m_engine);
        m_engine = nullptr;
    }
}

void PalikeyState::reset() {
    if (m_engine) {
        palikey_reset_instance(m_engine);
    }
    m_ic->inputPanel().reset();
    m_ic->updatePreedit();
    m_ic->updateUserInterface(UserInterfaceComponent::InputPanel);
}

void PalikeyState::activate() {
    reloadConfig();
    reset();
}

void PalikeyState::deactivate() {
    reset();
}

void PalikeyState::reloadConfig() {
    PalikeyConfig new_cfg;
    if (palikey_config_load(&new_cfg)) {
        if (new_cfg.niggahita != m_config.niggahita || new_cfg.extended_set != m_config.extended_set) {
            if (m_engine) {
                palikey_free(m_engine);
            }
            m_config = new_cfg;
            m_engine = palikey_new(m_config.scheme, m_config.niggahita, m_config.extended_set);
        } else if (new_cfg.scheme != m_config.scheme) {
            m_config = new_cfg;
            if (m_engine) {
                palikey_set_scheme(m_engine, m_config.scheme);
            }
        } else {
            m_config = new_cfg;
        }
    }
}

void PalikeyState::applyResult(const PalikeyProcessResult &result, KeyEvent &keyEvent) {
    if (result.backspaces > 0) {
        if (m_ic->capabilityFlags().test(CapabilityFlag::SurroundingText)) {
            m_ic->deleteSurroundingText(-static_cast<int>(result.backspaces), static_cast<int>(result.backspaces));
        } else {
            for (uint8_t i = 0; i < result.backspaces; ++i) {
                m_ic->forwardKey(Key(FcitxKey_BackSpace));
            }
        }
    }

    if (result.commit[0] != '\0') {
        m_ic->commitString(result.commit);
    }

    if (result.preedit[0] != '\0') {
        Text preedit(result.preedit);
        m_ic->inputPanel().setPreedit(preedit);
    } else {
        m_ic->inputPanel().reset();
    }
    m_ic->updatePreedit();
    m_ic->updateUserInterface(UserInterfaceComponent::InputPanel);

    keyEvent.filterAndAccept();
}

void PalikeyState::keyEvent(KeyEvent &keyEvent) {
    if (keyEvent.isRelease()) {
        return;
    }

    auto key = keyEvent.rawKey();

    // 1. Check Alt / AltGr modifier for EasyUnicode direct-insert mapping
    if (key.states().test(KeyState::Alt) || key.states().test(KeyState::Mod5)) {
        if (!key.states().test(KeyState::Ctrl)) {
            uint32_t sym = key.sym();
            bool isShift = key.states().test(KeyState::Shift);
            const char *mapped = palikey_easyunicode_map(sym, isShift, m_config.niggahita);
            if (mapped) {
                reset();
                m_ic->commitString(mapped);
                keyEvent.filterAndAccept();
                return;
            }
        }
        // Unmapped modifier combination: reset and pass through
        reset();
        return;
    }

    // 2. Control / Super shortcuts bypass IME and flush composition buffer
    if (key.states().test(KeyState::Ctrl) || key.states().test(KeyState::Super)) {
        reset();
        return;
    }

    // 3. Navigation and action keys flush composition and pass through
    if (key.check(FcitxKey_Return) || key.check(FcitxKey_KP_Enter) ||
        key.check(FcitxKey_Escape) || key.check(FcitxKey_Tab) ||
        key.check(FcitxKey_Left)   || key.check(FcitxKey_Right) ||
        key.check(FcitxKey_Up)     || key.check(FcitxKey_Down) ||
        key.check(FcitxKey_Home)   || key.check(FcitxKey_End) ||
        key.check(FcitxKey_Page_Up)|| key.check(FcitxKey_Page_Down)) {
        reset();
        return;
    }

    // 4. Backspace key handling
    if (key.check(FcitxKey_BackSpace)) {
        if (!m_engine) return;
        PalikeyProcessResult result;
        if (palikey_process_instance(m_engine, 0, true, &result)) {
            if (result.backspaces > 0 || result.commit[0] != '\0') {
                applyResult(result, keyEvent);
                return;
            }
        }
        // No undo sequence available: allow default backspace to pass through
        return;
    }

    // 5. Printable character keys
    uint32_t codepoint = Key::keySymToUnicode(key.sym());
    if (codepoint == 0 && key.sym() >= 0x20 && key.sym() <= 0x7e) {
        codepoint = key.sym();
    }

    if (codepoint >= 0x20) {
        if (!m_engine) return;
        PalikeyProcessResult result;
        if (palikey_process_instance(m_engine, codepoint, false, &result)) {
            applyResult(result, keyEvent);
            return;
        }
    }

    // Any other unhandled key
    reset();
}

} // namespace fcitx
