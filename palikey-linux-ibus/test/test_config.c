#include <stdio.h>
#include <assert.h>
#include <string.h>
#include "palikey.h"
#include "palikey_config.h"
#include "palikey_easyunicode.h"

int main(void) {
    printf("[TEST] Testing PaliKey Linux shared config and EasyUnicode...\n");

    // 1. Test palikey_version
    const char *ver = palikey_version();
    assert(ver != NULL);
    printf("  Engine version: %s\n", ver);

    // 2. Test default config
    PalikeyConfig cfg;
    palikey_config_init_default(&cfg);
    assert(cfg.scheme == PALIKEY_SCHEME_VELTHUIS);
    assert(cfg.niggahita == PALIKEY_NIGGAHITA_DOT_BELOW);
    assert(cfg.extended_set == false);
    assert(cfg.escape_enabled == true);
    assert(cfg.escape_char == '\\');
    assert(cfg.undo_by_backspace == true);
    assert(cfg.repeat_key_escapes == true);
    printf("  Default config check: PASS\n");

    // 3. Test parsing shared/config.json
    bool loaded = palikey_config_load_file("shared/config.json", &cfg);
    assert(loaded == true);
    assert(cfg.scheme == PALIKEY_SCHEME_VELTHUIS);
    assert(cfg.niggahita == PALIKEY_NIGGAHITA_DOT_BELOW);
    assert(cfg.extended_set == false);
    assert(cfg.escape_char == '\\');
    printf("  shared/config.json parse: PASS\n");

    // 4. Test parsing custom json string
    const char *custom_json = "{\n"
        "  \"scheme\": \"palitelex\",\n"
        "  \"niggahita\": \"dot_above\",\n"
        "  \"extended_set\": true,\n"
        "  \"escape\": {\n"
        "    \"enabled\": false,\n"
        "    \"escape_char\": \"/\",\n"
        "    \"undo_by_backspace\": false,\n"
        "    \"repeat_key_escapes\": false\n"
        "  }\n"
        "}";
    loaded = palikey_config_parse(custom_json, &cfg);
    assert(loaded == true);
    assert(cfg.scheme == PALIKEY_SCHEME_PALI_TELEX);
    assert(cfg.niggahita == PALIKEY_NIGGAHITA_DOT_ABOVE);
    assert(cfg.extended_set == true);
    assert(cfg.escape_enabled == false);
    assert(cfg.escape_char == '/');
    assert(cfg.undo_by_backspace == false);
    assert(cfg.repeat_key_escapes == false);
    printf("  custom JSON parse: PASS\n");

    // 5. Test EasyUnicode mappings
    const char *m1 = palikey_easyunicode_map('a', false, 0);
    assert(m1 != NULL && strcmp(m1, "ā") == 0);
    const char *m2 = palikey_easyunicode_map('a', true, 0);
    assert(m2 != NULL && strcmp(m2, "Ā") == 0);
    const char *m3 = palikey_easyunicode_map('m', false, 0);
    assert(m3 != NULL && strcmp(m3, "ṃ") == 0);
    const char *m4 = palikey_easyunicode_map('m', false, 1);
    assert(m4 != NULL && strcmp(m4, "ṁ") == 0);
    const char *m5 = palikey_easyunicode_map('z', false, 0);
    assert(m5 == NULL);
    printf("  EasyUnicode mappings: PASS\n");

    // 6. Test real engine instance FFI with parsed config
    PalikeyEngine *engine = palikey_new(cfg.scheme, cfg.niggahita, cfg.extended_set);
    assert(engine != NULL);

    // In PaliTelex: 'a' then 'a' gives 'ā'
    PalikeyProcessResult res;
    int ok = palikey_process_instance(engine, 'a', false, &res);
    assert(ok == 1);
    assert(res.backspaces == 0);
    assert(strcmp(res.commit, "a") == 0);

    ok = palikey_process_instance(engine, 'a', false, &res);
    assert(ok == 1);
    assert(res.backspaces == 1);
    assert(strcmp(res.commit, "ā") == 0);

    // Switch scheme to Velthuis (0)
    ok = palikey_set_scheme(engine, PALIKEY_SCHEME_VELTHUIS);
    assert(ok == 1);
    palikey_reset_instance(engine);

    // In Velthuis: '.' then 't' gives 'ṭ'
    ok = palikey_process_instance(engine, '.', false, &res);
    assert(ok == 1);
    assert(res.backspaces == 0);
    assert(strcmp(res.commit, ".") == 0);

    ok = palikey_process_instance(engine, 't', false, &res);
    assert(ok == 1);
    assert(res.backspaces == 1);
    assert(strcmp(res.commit, "ṭ") == 0);

    // Backspace undo: deletes 'ṭ', restores '.t'
    ok = palikey_process_instance(engine, 0, true, &res);
    assert(ok == 1);
    assert(res.backspaces == 1);
    assert(strcmp(res.commit, ".t") == 0);

    palikey_free(engine);
    printf("  Engine instance FFI: PASS\n");

    printf("[SUCCESS] All shared Linux shell tests passed!\n");
    return 0;
}
