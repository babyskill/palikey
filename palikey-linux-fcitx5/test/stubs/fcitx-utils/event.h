#pragma once
#include "key.h"

namespace fcitx {

class InputContext;

class KeyEvent {
public:
    KeyEvent(InputContext *ic = nullptr) : m_ic(ic) {}
    bool isRelease() const { return false; }
    Key rawKey() const { return Key('a'); }
    InputContext *inputContext() const { return m_ic; }
    void filterAndAccept() {}
private:
    InputContext *m_ic;
};

class InputContextEvent {
public:
    InputContextEvent(InputContext *ic = nullptr) : m_ic(ic) {}
    InputContext *inputContext() const { return m_ic; }
private:
    InputContext *m_ic;
};

} // namespace fcitx
