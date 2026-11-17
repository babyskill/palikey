#pragma once
#include "addoninstance.h"
#include "addonmanager.h"

namespace fcitx {

class AddonFactory {
public:
    virtual ~AddonFactory() = default;
    virtual AddonInstance *create(AddonManager *manager) = 0;
};

#define FCITX_ADDON_FACTORY(FactoryType) \
    extern "C" fcitx::AddonFactory *fcitx_addon_factory() { \
        return new FactoryType(); \
    }

#define FCITX_UNUSED(x) (void)(x)

} // namespace fcitx
