#pragma once

namespace fcitx {

class InputContextProperty {
public:
    virtual ~InputContextProperty() = default;
};

template<typename IC, typename T>
class FactoryFor {
public:
    using PropertyType = T;
    template<typename F>
    FactoryFor(F &&) {}
};

} // namespace fcitx
