/**
 * @file palikey_config.h
 * @brief Shared configuration loader for PaliKey Linux shells.
 *
 * Conforms to shared/config.schema.json and loads settings from
 * ~/.config/palikey/config.json (or $XDG_CONFIG_HOME/palikey/config.json).
 */

#ifndef PALIKEY_CONFIG_H
#define PALIKEY_CONFIG_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include "palikey.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Runtime configuration state for PaliKey shells.
 */
typedef struct {
    uint32_t scheme;            /**< PalikeySchemeId (0=Velthuis, 1=PaliTelex, 2=SIL) */
    uint32_t niggahita;         /**< PalikeyNiggahitaVariant (0=DotBelow ṃ, 1=DotAbove ṁ) */
    bool extended_set;          /**< Enable Sanskrit/IAST characters */
    bool escape_enabled;        /**< Escape prefix mechanism enabled */
    char escape_char;           /**< Default '\\' */
    bool undo_by_backspace;     /**< Immediate backspace restores sequence */
    bool repeat_key_escapes;    /**< Repeat trigger key escapes */
} PalikeyConfig;

/**
 * Initializes a PalikeyConfig structure with defaults per shared/config.schema.json:
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
 * Resolves the canonical path to ~/.config/palikey/config.json.
 * Respects $XDG_CONFIG_HOME if defined, falling back to $HOME/.config/palikey/config.json.
 *
 * @param out_path Destination buffer.
 * @param max_len Maximum bytes available in destination buffer.
 * @return true if successfully resolved, false on truncation or missing environment.
 */
bool palikey_config_get_path(char *out_path, size_t max_len);

/**
 * Loads configuration from ~/.config/palikey/config.json.
 * If the file does not exist or has parse errors, applies default configuration.
 *
 * @param config Destination structure to populate.
 * @return true if loaded from file, false if defaults were applied.
 */
bool palikey_config_load(PalikeyConfig *config);

/**
 * Loads and parses configuration from a specific file path.
 *
 * @param file_path File system path to config.json.
 * @param config Destination structure to populate.
 * @return true if successfully loaded and parsed, false otherwise.
 */
bool palikey_config_load_file(const char *file_path, PalikeyConfig *config);

/**
 * Parses configuration from an in-memory JSON string.
 *
 * @param json_str Null-terminated JSON buffer.
 * @param config Destination structure to populate.
 * @return true on successful parse, false on malformed JSON.
 */
bool palikey_config_parse(const char *json_str, PalikeyConfig *config);

#ifdef __cplusplus
}
#endif

#endif /* PALIKEY_CONFIG_H */
