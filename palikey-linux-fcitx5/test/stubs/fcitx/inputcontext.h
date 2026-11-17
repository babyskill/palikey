#pragma once

#include <string>
#include "inputcontextproperty.h"
#include "inputpanel.h"
#include <fcitx-utils/key.h>

namespace fcitx {

enum class CapabilityFlag {
    SurroundingText = 1
};

class CapabilityFlags {
public:
    bool test(CapabilityFlag) const { return true; }
};

enum class UserInterfaceComponent {
    InputPanel = 1
};

class InputContext {
public:
    CapabilityFlags capabilityFlags() const { return {}; }
    void deleteSurroundingText(int, int) {}
    void forwardKey(const Key &) {}
    void commitString(const std::string &) {}
    InputPanel &inputPanel() { return m_panel; }
    void updatePreedit() {}
    void updateUserInterface(UserInterfaceComponent) {}

    template<typename F>
    typename F::PropertyType *propertyFor(F *) {
        return nullptr;
    }

private:
    InputPanel m_panel;
};

} // namespace fcitx
