/**
 * @file palikey.h
 * @brief C API for PaliKey cross-platform Pāḷi input method engine.
 *
 * Clean-room implementation per DESIGN.md §3 and §5.
 * Compatible with C99, C11, C++11 and higher.
 */

#ifndef PALIKEY_H
#define PALIKEY_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Transliteration scheme IDs.
 */
typedef enum {
    PALIKEY_SCHEME_VELTHUIS    = 0, /**< Velthuis (aa->ā, .t->ṭ, "n->ṅ, ~n->ñ, .m->ṃ) */
    PALIKEY_SCHEME_PALI_TELEX  = 1, /**< PaliTelex (aa->ā, tx->ṭ, ngw->ṅ, nw/ny->ñ, mx->ṃ) */
    PALIKEY_SCHEME_SIL         = 2  /**< SIL / Keyman (-a->ā, .t->ṭ, >n->ṅ, ~n->ñ, .m->ṃ) */
} PalikeySchemeId;

/**
 * Niggahīta orthographic variant.
 */
typedef enum {
    PALIKEY_NIGGAHITA_DOT_BELOW = 0, /**< ṃ (U+1E43) — PTS / SuttaCentral standard (default) */
    PALIKEY_NIGGAHITA_DOT_ABOVE = 1  /**< ṁ (U+1E41) — VRI tradition */
} PalikeyNiggahitaVariant;

/**
 * Result returned by the engine after processing a keystroke.
 */
typedef struct {
    /** Number of characters to delete via backspace before committing */
    uint8_t backspaces;
    /** UTF-8 string to commit to the target text field (null-terminated) */
    char commit[64];
    /** UTF-8 active composition preview (null-terminated, empty in direct-commit) */
    char preedit[64];
} PalikeyProcessResult;

/**
 * Opaque engine instance handle.
 */
typedef struct PalikeyEngine PalikeyEngine;

/* ========================================================================= */
/* Singleton API (for single-context platforms e.g. Windows WH_KEYBOARD_LL)  */
/* ========================================================================= */

/**
 * Configures the global singleton engine.
 *
 * @param scheme_id Transliteration scheme ID (0=Velthuis, 1=PaliTelex, 2=Sil)
 * @param niggahita Niggahīta variant (0=DotBelow ṃ, 1=DotAbove ṁ)
 * @param extended_set Non-zero to enable extended Sanskrit/IAST characters
 * @return 1 on success, 0 on invalid parameters
 */
int32_t palikey_setup(uint32_t scheme_id, uint32_t niggahita, bool extended_set);

/**
 * Processes a keystroke on the global singleton engine.
 *
 * @param key_char Unicode codepoint of the typed character (ignored if is_backspace is true)
 * @param is_backspace True if Backspace key was pressed
 * @param out_result Pointer to structure populated with the processing result
 * @return 1 on success, 0 on error or null pointer
 */
int32_t palikey_process(uint32_t key_char, bool is_backspace, PalikeyProcessResult *out_result);

/**
 * Resets the singleton engine composition buffer and undo history.
 */
void palikey_reset(void);

/* ========================================================================= */
/* Instance API (for multi-context platforms: macOS IMK, Linux Fcitx5/IBus)  */
/* ========================================================================= */

/**
 * Creates a new independent engine instance.
 *
 * @param scheme_id Transliteration scheme ID (0=Velthuis, 1=PaliTelex, 2=Sil)
 * @param niggahita Niggahīta variant (0=DotBelow ṃ, 1=DotAbove ṁ)
 * @param extended_set Non-zero to enable extended Sanskrit/IAST characters
 * @return Pointer to new PalikeyEngine instance, or NULL on allocation failure
 */
PalikeyEngine *palikey_new(uint32_t scheme_id, uint32_t niggahita, bool extended_set);

/**
 * Destroys an engine instance and frees its allocated resources.
 *
 * @param engine Pointer to engine instance (safe to pass NULL)
 */
void palikey_free(PalikeyEngine *engine);

/**
 * Processes a keystroke on a specific engine instance.
 *
 * @param engine Pointer to engine instance
 * @param key_char Unicode codepoint of typed character
 * @param is_backspace True if Backspace key was pressed
 * @param out_result Pointer to structure populated with the processing result
 * @return 1 on success, 0 on error or null pointer
 */
int32_t palikey_process_instance(
    PalikeyEngine *engine,
    uint32_t key_char,
    bool is_backspace,
    PalikeyProcessResult *out_result
);

/**
 * Changes the active transliteration scheme for an engine instance.
 * If engine is NULL, updates the global singleton engine instead.
 *
 * @param engine Pointer to engine instance, or NULL for singleton
 * @param scheme_id Transliteration scheme ID (0=Velthuis, 1=PaliTelex, 2=Sil)
 * @return 1 on success, 0 on invalid scheme ID
 */
int32_t palikey_set_scheme(PalikeyEngine *engine, uint32_t scheme_id);

/**
 * Resets the composition state of an engine instance.
 *
 * @param engine Pointer to engine instance
 */
void palikey_reset_instance(PalikeyEngine *engine);

/**
 * Returns the version string of the engine library (e.g. "0.1.0").
 */
const char *palikey_version(void);

#ifdef __cplusplus
}
#endif

#endif /* PALIKEY_H */
