#pragma once
#include "instance.h"

namespace fcitx {

class AddonManager {
public:
    Instance *instance() { return &m_instance; }
private:
    Instance m_instance;
};

} // namespace fcitx
