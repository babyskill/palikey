#pragma once

#include "addoninstance.h"
#include <fcitx-utils/event.h>
#include <string>

namespace fcitx {

class InputMethodEntry {
public:
    const std::string &uniqueName() const { return m_name; }
private:
    std::string m_name;
};

class InputMethodEngineV2 : public AddonInstance {
public:
    virtual void keyEvent(const InputMethodEntry &entry, KeyEvent &keyEvent) = 0;
    virtual void reset(const InputMethodEntry &entry, InputContextEvent &event) = 0;
    virtual void activate(const InputMethodEntry &entry, InputContextEvent &event) = 0;
    virtual void deactivate(const InputMethodEntry &entry, InputContextEvent &event) = 0;
};

} // namespace fcitx
