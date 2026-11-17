/**
 * @file palikey_state.h
 * @brief Per-InputContext state for PaliKey Fcitx5 addon.
 */

#pragma once

#include <fcitx/inputcontext.h>
#include <fcitx/inputcontextproperty.h>
#include <fcitx/inputpanel.h>
#include <fcitx/text.h>
#include <fcitx-utils/event.h>
#include <fcitx-utils/key.h>
#include <fcitx-utils/keysym.h>
#include <fcitx-utils/keysymgen.h>

#include "palikey.h"
#include "palikey_config.h"
#include "palikey_easyunicode.h"

namespace fcitx {

class PalikeyEngineAddon;

/**
 * Manages engine lifecycle and composition state for a single input context.
 */
class PalikeyState : public InputContextProperty {
public:
    PalikeyState(PalikeyEngineAddon *addon, InputContext *ic);
    ~PalikeyState() override;

    void keyEvent(KeyEvent &keyEvent);
    void reset();
    void activate();
    void deactivate();
    void reloadConfig();

    PalikeyEngine *engine() const { return m_engine; }
    PalikeyEngineAddon *addon() const { return m_addon; }

private:
    void applyResult(const PalikeyProcessResult &result, KeyEvent &keyEvent);

    PalikeyEngineAddon *m_addon;
    InputContext *m_ic;
    PalikeyEngine *m_engine = nullptr;
    PalikeyConfig m_config;
};

} // namespace fcitx
