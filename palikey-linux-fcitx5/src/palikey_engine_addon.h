/**
 * @file palikey_engine_addon.h
 * @brief Fcitx5 InputMethodEngineV2 addon subclass for PaliKey.
 */

#pragma once

#include <fcitx/addonfactory.h>
#include <fcitx/addoninstance.h>
#include <fcitx/addonmanager.h>
#include <fcitx/inputcontext.h>
#include <fcitx/inputcontextmanager.h>
#include <fcitx/inputcontextproperty.h>
#include <fcitx/inputmethodengine.h>
#include <fcitx/instance.h>

#include "palikey.h"
#include "palikey_state.h"

namespace fcitx {

class PalikeyEngineAddon : public InputMethodEngineV2 {
public:
    PalikeyEngineAddon(Instance *instance);
    ~PalikeyEngineAddon() override;

    void keyEvent(const InputMethodEntry &entry, KeyEvent &keyEvent) override;
    void reset(const InputMethodEntry &entry, InputContextEvent &event) override;
    void activate(const InputMethodEntry &entry, InputContextEvent &event) override;
    void deactivate(const InputMethodEntry &entry, InputContextEvent &event) override;

    Instance *instance() const { return m_instance; }
    FactoryFor<InputContext, PalikeyState> &factory() { return m_factory; }

private:
    Instance *m_instance;
    FactoryFor<InputContext, PalikeyState> m_factory;
};

class PalikeyEngineFactory : public AddonFactory {
public:
    AddonInstance *create(AddonManager *manager) override {
        return new PalikeyEngineAddon(manager->instance());
    }
};

} // namespace fcitx
