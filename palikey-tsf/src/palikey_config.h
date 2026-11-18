#pragma once

#ifndef PALIKEY_TSF_CONFIG_H
#define PALIKEY_TSF_CONFIG_H

#include <windows.h>
#include <stdint.h>
#include <stdbool.h>
#include "palikey.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Runtime configuration state for PaliKey TSF.
 * Conforms to shared/config.schema.json.
 */
typedef struct {
    uint32_t scheme;            /**< PalikeySchemeId (0=Velthuis, 1=PaliTelex, 2=SIL, 3=VietnameseTelex) */
    uint32_t niggahita;         /**< PalikeyNiggahitaVariant (0=DotBelow ṃ, 1=DotAbove ṁ) */
    bool extended_set;          /**< Enable Sanskrit/IAST characters */
    bool escape_enabled;        /**< Escape prefix mechanism enabled */
    char escape_char;           /**< Default '\\' */
    bool undo_by_backspace;     /**< Immediate backspace restores sequence */
    bool repeat_key_escapes;    /**< Repeat trigger key escapes */
} PalikeyConfig;

/**
 * Initializes a PalikeyConfig structure with defaults:
 * - scheme: Velthuis (0)
 * - niggahita: DotBelow (0)
 * - extended_set: false
 * - escape_enabled: true
 * - escape_char: '\\'
 * - undo_by_backspace: true
 * - repeat_key_escapes: true
 */
void palikey_config_init_default(PalikeyConfig *config);

/**
 * Resolves the canonical path to %APPDATA%\PaliKey\config.json.
 *
 * @param out_path Wide-character buffer destination.
 * @param max_len Maximum WCHAR count available in destination buffer.
 * @return true if successfully resolved, false on error.
 */
bool palikey_config_get_path(WCHAR *out_path, size_t max_len);

/**
 * Loads configuration from %APPDATA%\PaliKey\config.json.
 * If the file does not exist or has parse errors, applies default configuration.
 *
 * @param config Destination structure to populate.
 * @return true if loaded from file, false if defaults were applied.
 */
bool palikey_config_load(PalikeyConfig *config);

/**
 * Parses configuration from an in-memory JSON string.
 *
 * @param json_str Null-terminated UTF-8 JSON buffer.
 * @param config Destination structure to populate.
 * @return true on successful parse, false on malformed JSON.
 */
bool palikey_config_parse(const char *json_str, PalikeyConfig *config);

#ifdef __cplusplus
}
#endif

#endif // PALIKEY_TSF_CONFIG_H
