/**
 * @file palikey_config.c
 * @brief Portable configuration parser for PaliKey Linux shells.
 */

#include "palikey_config.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

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

bool palikey_config_get_path(char *out_path, size_t max_len) {
    if (!out_path || max_len == 0) return false;

    const char *xdg = getenv("XDG_CONFIG_HOME");
    if (xdg && xdg[0] != '\0') {
        int n = snprintf(out_path, max_len, "%s/palikey/config.json", xdg);
        return (n > 0 && (size_t)n < max_len);
    }

    const char *home = getenv("HOME");
    if (home && home[0] != '\0') {
        int n = snprintf(out_path, max_len, "%s/.config/palikey/config.json", home);
        return (n > 0 && (size_t)n < max_len);
    }

    int n = snprintf(out_path, max_len, "./config.json");
    return (n > 0 && (size_t)n < max_len);
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

static bool parse_int(const char **p, int *out_val) {
    char *endptr = NULL;
    long val = strtol(*p, &endptr, 10);
    if (endptr == *p) return false;
    *out_val = (int)val;
    *p = endptr;
    return true;
}

static void skip_value(const char **p) {
    const char *s = skip_ws(*p);
    if (*s == '"') {
        char dummy[128];
        parse_string(&s, dummy, sizeof(dummy));
        *p = s;
        return;
    }
    if (*s == '{') {
        int depth = 1;
        s++;
        while (*s && depth > 0) {
            if (*s == '"') {
                char dummy[128];
                parse_string(&s, dummy, sizeof(dummy));
                continue;
            }
            if (*s == '{') depth++;
            else if (*s == '}') depth--;
            s++;
        }
        *p = s;
        return;
    }
    if (*s == '[') {
        int depth = 1;
        s++;
        while (*s && depth > 0) {
            if (*s == '"') {
                char dummy[128];
                parse_string(&s, dummy, sizeof(dummy));
                continue;
            }
            if (*s == '[') depth++;
            else if (*s == ']') depth--;
            s++;
        }
        *p = s;
        return;
    }
    while (*s && *s != ',' && *s != '}' && *s != ']' && *s != '\r' && *s != '\n') {
        s++;
    }
    *p = s;
}

static void parse_escape_object(const char **p, PalikeyConfig *config) {
    const char *s = skip_ws(*p);
    if (*s != '{') return;
    s++;

    while (*s && *s != '}') {
        s = skip_ws(s);
        if (*s == '}') break;
        if (*s == ',') { s++; continue; }

        char key[64];
        if (!parse_string(&s, key, sizeof(key))) {
            break;
        }

        s = skip_ws(s);
        if (*s != ':') break;
        s++; // skip ':'
        s = skip_ws(s);

        if (strcmp(key, "enabled") == 0) {
            bool val = true;
            if (parse_bool(&s, &val)) {
                config->escape_enabled = val;
            } else {
                skip_value(&s);
            }
        } else if (strcmp(key, "escape_char") == 0) {
            char val_str[16];
            if (parse_string(&s, val_str, sizeof(val_str))) {
                if (val_str[0] != '\0') {
                    config->escape_char = val_str[0];
                }
            } else {
                skip_value(&s);
            }
        } else if (strcmp(key, "undo_by_backspace") == 0) {
            bool val = true;
            if (parse_bool(&s, &val)) {
                config->undo_by_backspace = val;
            } else {
                skip_value(&s);
            }
        } else if (strcmp(key, "repeat_key_escapes") == 0) {
            bool val = true;
            if (parse_bool(&s, &val)) {
                config->repeat_key_escapes = val;
            } else {
                skip_value(&s);
            }
        } else {
            skip_value(&s);
        }
    }

    if (*s == '}') s++;
    *p = s;
}

bool palikey_config_parse(const char *json_str, PalikeyConfig *config) {
    if (!json_str || !config) return false;

    palikey_config_init_default(config);

    const char *p = skip_ws(json_str);
    if (*p != '{') return false;
    p++;

    while (*p && *p != '}') {
        p = skip_ws(p);
        if (*p == '}') break;
        if (*p == ',') { p++; continue; }

        char key[64];
        if (!parse_string(&p, key, sizeof(key))) {
            return false;
        }

        p = skip_ws(p);
        if (*p != ':') return false;
        p++; // skip ':'
        p = skip_ws(p);

        if (strcmp(key, "scheme") == 0) {
            if (*p == '"') {
                char scheme_str[32];
                if (parse_string(&p, scheme_str, sizeof(scheme_str))) {
                    if (strcasecmp(scheme_str, "velthuis") == 0) {
                        config->scheme = (uint32_t)PALIKEY_SCHEME_VELTHUIS;
                    } else if (strcasecmp(scheme_str, "palitelex") == 0 || strcasecmp(scheme_str, "telex") == 0) {
                        config->scheme = (uint32_t)PALIKEY_SCHEME_PALI_TELEX;
                    } else if (strcasecmp(scheme_str, "sil") == 0 || strcasecmp(scheme_str, "keyman") == 0) {
                        config->scheme = (uint32_t)PALIKEY_SCHEME_SIL;
                    }
                }
            } else {
                int id = 0;
                if (parse_int(&p, &id)) {
                    if (id >= 0 && id <= 2) {
                        config->scheme = (uint32_t)id;
                    }
                }
            }
        } else if (strcmp(key, "niggahita") == 0) {
            if (*p == '"') {
                char nig_str[32];
                if (parse_string(&p, nig_str, sizeof(nig_str))) {
                    if (strcasecmp(nig_str, "dot_above") == 0) {
                        config->niggahita = (uint32_t)PALIKEY_NIGGAHITA_DOT_ABOVE;
                    } else {
                        config->niggahita = (uint32_t)PALIKEY_NIGGAHITA_DOT_BELOW;
                    }
                }
            } else {
                int id = 0;
                if (parse_int(&p, &id)) {
                    config->niggahita = (id == 1) ? (uint32_t)PALIKEY_NIGGAHITA_DOT_ABOVE : (uint32_t)PALIKEY_NIGGAHITA_DOT_BELOW;
                }
            }
        } else if (strcmp(key, "extended_set") == 0) {
            bool ext = false;
            if (parse_bool(&p, &ext)) {
                config->extended_set = ext;
            } else {
                skip_value(&p);
            }
        } else if (strcmp(key, "escape") == 0) {
            parse_escape_object(&p, config);
        } else {
            skip_value(&p);
        }
    }

    return true;
}

bool palikey_config_load_file(const char *file_path, PalikeyConfig *config) {
    if (!file_path || !config) return false;

    FILE *f = fopen(file_path, "rb");
    if (!f) return false;

    if (fseek(f, 0, SEEK_END) != 0) {
        fclose(f);
        return false;
    }
    long len = ftell(f);
    if (len < 0 || len > 1024 * 1024) { // Guard max 1MB config
        fclose(f);
        return false;
    }
    if (fseek(f, 0, SEEK_SET) != 0) {
        fclose(f);
        return false;
    }

    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        fclose(f);
        return false;
    }

    size_t read_bytes = fread(buf, 1, (size_t)len, f);
    fclose(f);
    buf[read_bytes] = '\0';

    bool ok = palikey_config_parse(buf, config);
    free(buf);
    return ok;
}

bool palikey_config_load(PalikeyConfig *config) {
    palikey_config_init_default(config);

    char path[1024];
    if (!palikey_config_get_path(path, sizeof(path))) {
        return false;
    }

    return palikey_config_load_file(path, config);
}
