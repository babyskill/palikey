#include <iostream>
#include <cassert>
#include <cstring>
#include "palikey.h"
#include "palikey_config.h"
#include "palikey_easyunicode.h"

int main() {
    std::cout << "[TEST] Testing PaliKey Fcitx5 shared config and EasyUnicode (C++17)...\n";

    // 1. Test palikey_version
    const char *ver = palikey_version();
    assert(ver != nullptr);
    std::cout << "  Engine version: " << ver << "\n";

    // 2. Test default config
    PalikeyConfig cfg;
    palikey_config_init_default(&cfg);
    assert(cfg.scheme == PALIKEY_SCHEME_VELTHUIS);
    assert(cfg.niggahita == PALIKEY_NIGGAHITA_DOT_BELOW);
    assert(cfg.extended_set == false);
    std::cout << "  Default config check: PASS\n";

    // 3. Test parsing shared/config.json
    bool loaded = palikey_config_load_file("shared/config.json", &cfg);
    assert(loaded == true);
    assert(cfg.scheme == PALIKEY_SCHEME_VELTHUIS);
    std::cout << "  shared/config.json parse: PASS\n";

    // 4. Test EasyUnicode
    const char *m = palikey_easyunicode_map('a', false, 0);
    assert(m != nullptr && std::strcmp(m, "ā") == 0);
    std::cout << "  EasyUnicode: PASS\n";

    // 5. Test engine instance FFI
    PalikeyEngine *engine = palikey_new(cfg.scheme, cfg.niggahita, cfg.extended_set);
    assert(engine != nullptr);

    PalikeyProcessResult res;
    int ok = palikey_process_instance(engine, 'a', false, &res);
    assert(ok == 1);
    assert(res.backspaces == 0);
    assert(std::strcmp(res.commit, "a") == 0);

    palikey_free(engine);
    std::cout << "  Engine instance FFI: PASS\n";

    std::cout << "[SUCCESS] Fcitx5 C++17 shared config test passed!\n";
    return 0;
}
