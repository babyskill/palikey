#pragma once

#include "inputcontextmanager.h"

namespace fcitx {

class Instance {
public:
    InputContextManager &inputContextManager() { return m_mgr; }
private:
    InputContextManager m_mgr;
};

} // namespace fcitx
