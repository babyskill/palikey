#include "palikey_config.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <shlobj.h>

void palikey_config_init_default(PalikeyConfig *config) {
    if (!config) return;
    config->scheme = (uint32_t)PALIKEY_SCHEME_VELTHUIS;
    config->niggahita = (uint32_t)PALIKEY_NIGGAHITA_DOT_BELOW;
    config->extended_set = false;
    config->escape_enabled = true;
    config->escape_char = '\\';
    config->undo_by_backspace = true;
    config->repeat_key_escapes = true;
}

bool palikey_config_get_path(WCHAR *out_path, size_t max_len) {
    if (!out_path || max_len == 0) return false;

    // Check %APPDATA%
    WCHAR appData[MAX_PATH] = {0};
    DWORD ret = GetEnvironmentVariableW(L"APPDATA", appData, MAX_PATH);
    if (ret > 0 && ret < MAX_PATH) {
        int n = _snwprintf_s(out_path, max_len, _TRUNCATE, L"%s\\PaliKey\\config.json", appData);
        return (n > 0);
    }

    // Fallback: SHGetFolderPathW CSIDL_APPDATA
    if (SUCCEEDED(SHGetFolderPathW(NULL, CSIDL_APPDATA, NULL, 0, appData))) {
        int n = _snwprintf_s(out_path, max_len, _TRUNCATE, L"%s\\PaliKey\\config.json", appData);
        return (n > 0);
    }

    return false;
}

static const char *skip_ws(const char *p) {
    while (*p && (*p == ' ' || *p == '\t' || *p == '\r' || *p == '\n')) {
        p++;
    }
    return p;
}

static bool parse_string(const char **p, char *out_buf, size_t max_len) {
    const char *s = *p;
    if (*s != '"') return false;
    s++; // Skip opening quote

    size_t idx = 0;
    while (*s && *s != '"') {
        if (*s == '\\') {
            s++;
            if (!*s) return false;
            char c = *s;
            if (c == 'n') c = '\n';
            else if (c == 'r') c = '\r';
            else if (c == 't') c = '\t';
            else if (c == '\\') c = '\\';
            else if (c == '"') c = '"';
            if (idx + 1 < max_len) {
                out_buf[idx++] = c;
            }
        } else {
            if (idx + 1 < max_len) {
                out_buf[idx++] = *s;
            }
        }
        s++;
    }

    if (*s != '"') return false;
    s++; // Skip closing quote
    out_buf[idx] = '\0';
    *p = s;
    return true;
}

static bool parse_bool(const char **p, bool *out_val) {
    const char *s = *p;
    if (strncmp(s, "true", 4) == 0) {
        *out_val = true;
        *p = s + 4;
        return true;
    }
    if (strncmp(s, "false", 5) == 0) {
        *out_val = false;
        *p = s + 5;
        return true;
    }
    return false;
}

static void skip_value(const char **p) {
    const char *s = skip_ws(*p);
    if (*s == '"') {
        s++;
        while (*s && *s != '"') {
            if (*s == '\\' && *(s + 1)) s++;
            s++;
        }
        if (*s == '"') s++;
    } else if (*s == '{') {
        int depth = 1;
        s++;
        while (*s && depth > 0) {
            if (*s == '{') depth++;
            else if (*s == '}') depth--;
            s++;
        }
    } else if (*s == '[') {
        int depth = 1;
        s++;
        while (*s && depth > 0) {
            if (*s == '[') depth++;
            else if (*s == ']') depth--;
            s++;
        }
    } else {
        while (*s && *s != ',' && *s != '}' && *s != ']' && *s != ' ' && *s != '\n' && *s != '\r' && *s != '\t') {
            s++;
        }
    }
    *p = s;
}

bool palikey_config_parse(const char *json_str, PalikeyConfig *config) {
    if (!json_str || !config) return false;
    palikey_config_init_default(config);

    const char *p = skip_ws(json_str);
    if (*p != '{') return false;
    p++;

    while (*p) {
        p = skip_ws(p);
        if (*p == '}') {
            p++;
            break;
        }
        if (*p == ',') {
            p++;
            continue;
        }

        char key[64] = {0};
        if (!parse_string(&p, key, sizeof(key))) {
            return false;
        }

        p = skip_ws(p);
        if (*p != ':') return false;
        p++;
        p = skip_ws(p);

        if (strcmp(key, "scheme") == 0) {
            char val[32] = {0};
            if (parse_string(&p, val, sizeof(val))) {
                if (strcmp(val, "velthuis") == 0) config->scheme = (uint32_t)PALIKEY_SCHEME_VELTHUIS;
                else if (strcmp(val, "palitelex") == 0) config->scheme = (uint32_t)PALIKEY_SCHEME_PALI_TELEX;
                else if (strcmp(val, "sil") == 0) config->scheme = (uint32_t)PALIKEY_SCHEME_SIL;
            } else {
                skip_value(&p);
            }
        } else if (strcmp(key, "niggahita") == 0) {
            char val[32] = {0};
            if (parse_string(&p, val, sizeof(val))) {
                if (strcmp(val, "dot_below") == 0) config->niggahita = (uint32_t)PALIKEY_NIGGAHITA_DOT_BELOW;
                else if (strcmp(val, "dot_above") == 0) config->niggahita = (uint32_t)PALIKEY_NIGGAHITA_DOT_ABOVE;
            } else {
                skip_value(&p);
            }
        } else if (strcmp(key, "extended_set") == 0) {
            bool val = false;
            if (parse_bool(&p, &val)) {
                config->extended_set = val;
            } else {
                skip_value(&p);
            }
        } else if (strcmp(key, "escape") == 0) {
            if (*p == '{') {
                p++;
                while (*p) {
                    p = skip_ws(p);
                    if (*p == '}') { p++; break; }
                    if (*p == ',') { p++; continue; }

                    char subkey[64] = {0};
                    if (!parse_string(&p, subkey, sizeof(subkey))) break;

                    p = skip_ws(p);
                    if (*p != ':') break;
                    p++;
                    p = skip_ws(p);

                    if (strcmp(subkey, "enabled") == 0) {
                        bool val = true;
                        if (parse_bool(&p, &val)) config->escape_enabled = val;
                        else skip_value(&p);
                    } else if (strcmp(subkey, "escape_char") == 0) {
                        char escStr[8] = {0};
                        if (parse_string(&p, escStr, sizeof(escStr)) && escStr[0]) {
                            config->escape_char = escStr[0];
                        } else skip_value(&p);
                    } else if (strcmp(subkey, "undo_by_backspace") == 0) {
                        bool val = true;
                        if (parse_bool(&p, &val)) config->undo_by_backspace = val;
                        else skip_value(&p);
                    } else if (strcmp(subkey, "repeat_key_escapes") == 0) {
                        bool val = true;
                        if (parse_bool(&p, &val)) config->repeat_key_escapes = val;
                        else skip_value(&p);
                    } else {
                        skip_value(&p);
                    }
                }
            } else {
                skip_value(&p);
            }
        } else {
            skip_value(&p);
        }
    }

    return true;
}

bool palikey_config_load(PalikeyConfig *config) {
    if (!config) return false;
    palikey_config_init_default(config);

    WCHAR path[MAX_PATH] = {0};
    if (!palikey_config_get_path(path, MAX_PATH)) {
        return false;
    }

    FILE *fp = nullptr;
    if (_wfopen_s(&fp, path, L"rb") != 0 || !fp) {
        return false;
    }

    fseek(fp, 0, SEEK_END);
    long size = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    if (size <= 0 || size > 65536) {
        fclose(fp);
        return false;
    }

    char *buf = (char *)malloc(size + 1);
    if (!buf) {
        fclose(fp);
        return false;
    }

    size_t read_bytes = fread(buf, 1, size, fp);
    fclose(fp);
    buf[read_bytes] = '\0';

    bool success = palikey_config_parse(buf, config);
    free(buf);
    return success;
}
