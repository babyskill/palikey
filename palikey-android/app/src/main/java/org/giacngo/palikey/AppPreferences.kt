package org.giacngo.palikey

import android.content.Context
import android.content.SharedPreferences

/**
 * Three-state keyboard input mode: Vietnamese → English → Pāḷi → Vietnamese.
 */
enum class InputMode(val id: Int, val label: String) {
    VIETNAMESE(0, "Tiếng Việt"),
    ENGLISH(1, "English"),
    PALI(2, "Pāḷi");

    fun next(): InputMode = when (this) {
        VIETNAMESE -> ENGLISH
        ENGLISH -> PALI
        PALI -> VIETNAMESE
    }

    companion object {
        fun fromId(id: Int): InputMode =
            values().firstOrNull { it.id == id } ?: PALI
    }
}


/**
 * Manages PaliKey persistent configuration in SharedPreferences.
 * Single source of truth for input mode, Pāḷi transliteration variant, and orthography.
 */
class AppPreferences(context: Context) {

    val prefs: SharedPreferences =
        context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

    init {
        migrateIfNeeded()
    }

    /**
     * Migration from Phase 5 preferences:
     * - First run (no persisted mode): default mode = PALI, variant = PaliTelex (1).
     * - Stored scheme is 3 (Vietnamese Telex): mode = VIETNAMESE, Pāḷi variant = PaliTelex (1).
     * - Stored scheme is 0/1/2: mode = PALI, variant = stored scheme.
     */
    fun migrateIfNeeded() {
        if (!prefs.contains(KEY_INPUT_MODE)) {
            val editor = prefs.edit()
            if (prefs.contains(KEY_SCHEME)) {
                val stored = prefs.getInt(KEY_SCHEME, NativeEngine.SCHEME_PALI_TELEX)
                if (stored == NativeEngine.SCHEME_VIETNAMESE_TELEX) {
                    editor.putInt(KEY_INPUT_MODE, InputMode.VIETNAMESE.id)
                    editor.putInt(KEY_PALI_VARIANT, NativeEngine.SCHEME_PALI_TELEX)
                } else if (stored in NativeEngine.SCHEME_VELTHUIS..NativeEngine.SCHEME_SIL) {
                    editor.putInt(KEY_INPUT_MODE, InputMode.PALI.id)
                    editor.putInt(KEY_PALI_VARIANT, stored)
                } else {
                    editor.putInt(KEY_INPUT_MODE, InputMode.PALI.id)
                    editor.putInt(KEY_PALI_VARIANT, NativeEngine.SCHEME_PALI_TELEX)
                }
            } else {
                // First run: default mode = PALI, variant = PaliTelex (1)
                editor.putInt(KEY_INPUT_MODE, InputMode.PALI.id)
                editor.putInt(KEY_PALI_VARIANT, NativeEngine.SCHEME_PALI_TELEX)
            }
            editor.apply()
        }
    }

    var inputMode: InputMode
        get() {
            migrateIfNeeded()
            return InputMode.fromId(prefs.getInt(KEY_INPUT_MODE, InputMode.PALI.id))
        }
        set(value) = prefs.edit().putInt(KEY_INPUT_MODE, value.id).apply()

    var paliVariant: Int
        get() {
            migrateIfNeeded()
            return prefs.getInt(KEY_PALI_VARIANT, NativeEngine.SCHEME_PALI_TELEX)
        }
        set(value) {
            prefs.edit()
                .putInt(KEY_PALI_VARIANT, value)
                .putInt(KEY_SCHEME, value) // keep legacy KEY_SCHEME synchronized
                .apply()
        }

    /**
     * Engine scheme resolution (Part A.2):
     * - VIETNAMESE -> engine scheme 3 (vietelex)
     * - ENGLISH -> null (composition OFF: plain passthrough typing)
     * - PALI -> persisted Pāḷi variant (0 Velthuis, 1 PaliTelex, 2 SIL)
     */
    val activeEngineScheme: Int?
        get() = when (inputMode) {
            InputMode.VIETNAMESE -> NativeEngine.SCHEME_VIETNAMESE_TELEX
            InputMode.ENGLISH -> null
            InputMode.PALI -> paliVariant
        }

    val paliVariantName: String
        get() = when (paliVariant) {
            NativeEngine.SCHEME_VELTHUIS -> "Velthuis"
            NativeEngine.SCHEME_PALI_TELEX -> "PaliTelex"
            NativeEngine.SCHEME_SIL -> "SIL"
            else -> "PaliTelex"
        }

    var schemeId: Int
        get() = activeEngineScheme ?: paliVariant
        set(value) {
            paliVariant = value
        }

    val schemeName: String
        get() = paliVariantName

    var niggahitaVariant: Int
        get() = prefs.getInt(KEY_NIGGAHITA, NativeEngine.NIGGAHITA_DOT_BELOW)
        set(value) = prefs.edit().putInt(KEY_NIGGAHITA, value).apply()

    var isExtendedSetEnabled: Boolean
        get() = prefs.getBoolean(KEY_EXTENDED_SET, false)
        set(value) = prefs.edit().putBoolean(KEY_EXTENDED_SET, value).apply()

    val niggahitaChar: Char
        get() = if (niggahitaVariant == NativeEngine.NIGGAHITA_DOT_ABOVE) 'ṁ' else 'ṃ'

    val capitalNiggahitaChar: Char
        get() = if (niggahitaVariant == NativeEngine.NIGGAHITA_DOT_ABOVE) 'Ṁ' else 'Ṃ'

    fun registerListener(listener: SharedPreferences.OnSharedPreferenceChangeListener) {
        prefs.registerOnSharedPreferenceChangeListener(listener)
    }

    fun unregisterListener(listener: SharedPreferences.OnSharedPreferenceChangeListener) {
        prefs.unregisterOnSharedPreferenceChangeListener(listener)
    }

    companion object {
        const val PREFS_NAME = "palikey_preferences"
        const val KEY_INPUT_MODE = "input_mode"
        const val KEY_PALI_VARIANT = "pali_variant"
        const val KEY_SCHEME = "scheme_id"
        const val KEY_NIGGAHITA = "niggahita_variant"
        const val KEY_EXTENDED_SET = "extended_set"
        const val KEY_KEYBOARD_MODE = "keyboard_mode"
    }
}
