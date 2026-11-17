package org.giacngo.palikey

import android.content.Intent
import android.inputmethodservice.InputMethodService
import android.view.KeyEvent
import android.view.View
import android.view.inputmethod.EditorInfo

/**
 * Android InputMethodService implementation for PaliKey.
 *
 * Hosts a NativeEngine instance per session, translates keystrokes via
 * the Rust engine's state machine, and applies ProcessResults through InputConnection.
 */
class PaliKeyService : InputMethodService(), KeyboardActionListener {

    private lateinit var preferences: AppPreferences
    private var engine: NativeEngine? = null
    private var keyboardView: KeyboardView? = null

    override fun onCreate() {
        super.onCreate()
        preferences = AppPreferences(this)
        initializeEngine()
    }

    private fun initializeEngine() {
        engine?.close()
        engine = NativeEngine(
            schemeId = preferences.schemeId,
            niggahita = preferences.niggahitaVariant,
            extendedSet = preferences.isExtendedSetEnabled
        )
    }

    override fun onCreateInputView(): View {
        keyboardView = KeyboardView(this, preferences, this)
        return keyboardView!!
    }

    override fun onStartInputView(info: EditorInfo?, restarting: Boolean) {
        super.onStartInputView(info, restarting)

        // Ensure engine is alive and sync latest scheme
        if (engine == null || !engine!!.isValid) {
            initializeEngine()
        } else {
            engine?.setScheme(preferences.schemeId)
            engine?.reset()
        }

        keyboardView?.refresh()
    }

    override fun onFinishInputView(finishingInput: Boolean) {
        super.onFinishInputView(finishingInput)
        engine?.reset()
    }

    override fun onDestroy() {
        super.onDestroy()
        engine?.close()
        engine = null
    }

    /* ========================================================================= */
    /* KeyboardActionListener Implementation                                     */
    /* ========================================================================= */

    override fun onChar(c: Char) {
        val ic = currentInputConnection ?: return

        when (preferences.keyboardMode) {
            KeyboardMode.TRANSDUCER -> {
                val safeEngine = engine ?: return ic.commitText(c.toString(), 1).let { }
                val result = safeEngine.process(c)
                applyProcessResult(result)
            }
            KeyboardMode.PALI_ROW, KeyboardMode.OFF -> {
                ic.commitText(c.toString(), 1)
            }
        }
    }

    override fun onPaliChar(c: Char) {
        val ic = currentInputConnection ?: return
        // Tapping a dedicated Pāḷi row key immediately commits the character
        // and flushes/resets composition state
        engine?.reset()
        ic.commitText(c.toString(), 1)
    }

    override fun onBackspace() {
        val ic = currentInputConnection ?: return

        if (preferences.keyboardMode == KeyboardMode.TRANSDUCER) {
            val safeEngine = engine
            if (safeEngine != null) {
                val result = safeEngine.processBackspace()
                // If the engine handled undo (e.g. reverted composed character)
                if (result.backspaces > 0 || result.commit.isNotEmpty()) {
                    applyProcessResult(result)
                    return
                }
            }
        }

        // Standard backspace fallback: delete 1 character before cursor
        ic.deleteSurroundingText(1, 0)
    }

    override fun onSpace() {
        val ic = currentInputConnection ?: return

        if (preferences.keyboardMode == KeyboardMode.TRANSDUCER) {
            val safeEngine = engine
            if (safeEngine != null) {
                val result = safeEngine.process(' ')
                applyProcessResult(result)
                return
            }
        }

        ic.commitText(" ", 1)
    }

    override fun onEnter() {
        val ic = currentInputConnection ?: return
        val editorInfo = currentInputEditorInfo
        val imeAction = editorInfo?.imeOptions?.and(EditorInfo.IME_MASK_ACTION)
            ?: EditorInfo.IME_ACTION_NONE

        if (imeAction != EditorInfo.IME_ACTION_NONE && imeAction != EditorInfo.IME_ACTION_UNSPECIFIED) {
            ic.performEditorAction(imeAction)
        } else {
            sendDownUpKeyEvents(KeyEvent.KEYCODE_ENTER)
        }
    }

    override fun onModeChanged(newMode: KeyboardMode) {
        engine?.reset()
    }

    override fun onOpenSettings() {
        val intent = Intent(this, SettingsActivity::class.java).apply {
            addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        }
        startActivity(intent)
    }

    private fun applyProcessResult(result: ProcessResult) {
        val ic = currentInputConnection ?: return

        if (result.backspaces > 0) {
            ic.deleteSurroundingText(result.backspaces, 0)
        }
        if (result.commit.isNotEmpty()) {
            ic.commitText(result.commit, 1)
        }
    }
}
