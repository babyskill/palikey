package org.giacngo.palikey

import android.content.Context
import android.content.SharedPreferences

enum class KeyboardMode(val id: Int, val label: String) {
    TRANSDUCER(0, "Transducer"),
    PALI_ROW(1, "Pāḷi Row"),
    OFF(2, "Off");

    companion object {
        fun fromId(id: Int): KeyboardMode =
            values().firstOrNull { it.id == id } ?: TRANSDUCER
    }
}

/**
 * Manages PaliKey persistent configuration in SharedPreferences.
 * Synchronized with shared/config.json specifications.
 */
class AppPreferences(context: Context) {

    private val prefs: SharedPreferences =
        context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

    var schemeId: Int
        get() = prefs.getInt(KEY_SCHEME, NativeEngine.SCHEME_VELTHUIS)
        set(value) = prefs.edit().putInt(KEY_SCHEME, value).apply()

    var niggahitaVariant: Int
        get() = prefs.getInt(KEY_NIGGAHITA, NativeEngine.NIGGAHITA_DOT_BELOW)
        set(value) = prefs.edit().putInt(KEY_NIGGAHITA, value).apply()

    var isExtendedSetEnabled: Boolean
        get() = prefs.getBoolean(KEY_EXTENDED_SET, false)
        set(value) = prefs.edit().putBoolean(KEY_EXTENDED_SET, value).apply()

    var keyboardMode: KeyboardMode
        get() = KeyboardMode.fromId(prefs.getInt(KEY_KEYBOARD_MODE, KeyboardMode.TRANSDUCER.id))
        set(value) = prefs.edit().putInt(KEY_KEYBOARD_MODE, value.id).apply()

    val schemeName: String
        get() = when (schemeId) {
            NativeEngine.SCHEME_VELTHUIS -> "Velthuis"
            NativeEngine.SCHEME_PALI_TELEX -> "PaliTelex"
            NativeEngine.SCHEME_SIL -> "SIL"
            else -> "Velthuis"
        }

    val niggahitaChar: Char
        get() = if (niggahitaVariant == NativeEngine.NIGGAHITA_DOT_ABOVE) 'ṁ' else 'ṃ'

    val capitalNiggahitaChar: Char
        get() = if (niggahitaVariant == NativeEngine.NIGGAHITA_DOT_ABOVE) 'Ṁ' else 'Ṃ'

    companion object {
        private const val PREFS_NAME = "palikey_preferences"
        private const val KEY_SCHEME = "scheme_id"
        private const val KEY_NIGGAHITA = "niggahita_variant"
        private const val KEY_EXTENDED_SET = "extended_set"
        private const val KEY_KEYBOARD_MODE = "keyboard_mode"
    }
}
