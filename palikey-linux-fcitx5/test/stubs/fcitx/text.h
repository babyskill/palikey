#pragma once
#include <string>

namespace fcitx {

class Text {
public:
    Text(const std::string &s) : m_s(s) {}
    const std::string &string() const { return m_s; }
private:
    std::string m_s;
};

class InputPanel {
public:
    void setPreedit(const Text &) {}
    void reset() {}
};

} // namespace fcitx
