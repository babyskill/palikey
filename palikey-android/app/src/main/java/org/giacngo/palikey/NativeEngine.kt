package org.giacngo.palikey

/**
 * Result returned by the PaliKey engine after processing a keystroke.
 *
 * @property backspaces Number of characters to delete via backspace before committing
 * @property commit UTF-8 string to commit to target input connection
 * @property preedit UTF-8 composition preedit string (empty in direct commit)
 */
data class ProcessResult(
    val backspaces: Int,
    val commit: String,
    val preedit: String
)

/**
 * High-level Kotlin wrapper around the native PaliKey Rust engine via JNI.
 *
 * Manages an independent PalikeyEngine pointer instance per input session,
 * allowing thread-safe, isolated transliteration states.
 */
class NativeEngine(
    schemeId: Int = SCHEME_VELTHUIS,
    niggahita: Int = NIGGAHITA_DOT_BELOW,
    extendedSet: Boolean = false
) : AutoCloseable {

    companion object {
        const val SCHEME_VELTHUIS = 0
        const val SCHEME_PALI_TELEX = 1
        const val SCHEME_SIL = 2
        const val SCHEME_VIETNAMESE_TELEX = 3

        const val NIGGAHITA_DOT_BELOW = 0 // ṃ (U+1E43) — PTS / SuttaCentral
        const val NIGGAHITA_DOT_ABOVE = 1 // ṁ (U+1E41) — VRI tradition

        init {
            System.loadLibrary("palikey_jni")
        }

        @JvmStatic
        external fun nativeVersion(): String
    }

    private var nativeHandle: Long = nativeCreate(schemeId, niggahita, extendedSet)

    val isValid: Boolean
        get() = nativeHandle != 0L

    /**
     * Processes a single character input.
     *
     * @param keyChar Unicode character typed
     * @return Result containing characters to delete and text to commit
     */
    fun process(keyChar: Char): ProcessResult {
        if (nativeHandle == 0L) {
            return ProcessResult(0, keyChar.toString(), "")
        }
        return nativeProcess(nativeHandle, keyChar.code, isBackspace = false)
    }

    /**
     * Processes a Backspace key press, enabling single-step undo of transliterations.
     *
     * @return Result specifying characters to erase/revert, if any
     */
    fun processBackspace(): ProcessResult {
        if (nativeHandle == 0L) {
            return ProcessResult(0, "", "")
        }
        return nativeProcess(nativeHandle, 0, isBackspace = true)
    }

    /**
     * Updates the transliteration scheme for this engine instance.
     *
     * @param schemeId Transliteration scheme ID (0=Velthuis, 1=PaliTelex, 2=SIL)
     */
    fun setScheme(schemeId: Int): Boolean {
        if (nativeHandle == 0L) return false
        return nativeSetScheme(nativeHandle, schemeId)
    }

    /**
     * Resets the composition state and history buffer.
     */
    fun reset() {
        if (nativeHandle != 0L) {
            nativeReset(nativeHandle)
        }
    }

    /**
     * Releases the native C/Rust engine instance.
     */
    override fun close() {
        if (nativeHandle != 0L) {
            nativeDestroy(nativeHandle)
            nativeHandle = 0L
        }
    }

    protected fun finalize() {
        close()
    }

    // Native JNI methods
    private external fun nativeCreate(schemeId: Int, niggahita: Int, extendedSet: Boolean): Long
    private external fun nativeDestroy(handle: Long)
    private external fun nativeProcess(handle: Long, keyChar: Int, isBackspace: Boolean): ProcessResult
    private external fun nativeSetScheme(handle: Long, schemeId: Int): Boolean
    private external fun nativeReset(handle: Long)
}
