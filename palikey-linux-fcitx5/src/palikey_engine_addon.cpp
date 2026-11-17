/**
 * @file palikey_engine_addon.cpp
 * @brief Implementation of Fcitx5 addon lifecycle and factory for PaliKey.
 */

#include "palikey_engine_addon.h"

namespace fcitx {

PalikeyEngineAddon::PalikeyEngineAddon(Instance *instance)
    : m_instance(instance),
      m_factory([this](InputContext &ic) { return new PalikeyState(this, &ic); }) {
    instance->inputContextManager().registerProperty("palikeyState", &m_factory);
}

PalikeyEngineAddon::~PalikeyEngineAddon() = default;

void PalikeyEngineAddon::keyEvent(const InputMethodEntry &entry, KeyEvent &keyEvent) {
    FCITX_UNUSED(entry);
    auto *state = keyEvent.inputContext()->propertyFor(&m_factory);
    if (state) {
        state->keyEvent(keyEvent);
    }
}

void PalikeyEngineAddon::reset(const InputMethodEntry &entry, InputContextEvent &event) {
    FCITX_UNUSED(entry);
    auto *state = event.inputContext()->propertyFor(&m_factory);
    if (state) {
        state->reset();
    }
}

void PalikeyEngineAddon::activate(const InputMethodEntry &entry, InputContextEvent &event) {
    FCITX_UNUSED(entry);
    auto *state = event.inputContext()->propertyFor(&m_factory);
    if (state) {
        state->activate();
    }
}

void PalikeyEngineAddon::deactivate(const InputMethodEntry &entry, InputContextEvent &event) {
    FCITX_UNUSED(entry);
    auto *state = event.inputContext()->propertyFor(&m_factory);
    if (state) {
        state->deactivate();
    }
}

FCITX_ADDON_FACTORY(PalikeyEngineFactory);

} // namespace fcitx
