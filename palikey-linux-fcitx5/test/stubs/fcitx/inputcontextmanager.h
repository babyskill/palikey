#pragma once

namespace fcitx {

class InputContextManager {
public:
    template<typename T>
    void registerProperty(const char *, T *) {}
};

} // namespace fcitx
