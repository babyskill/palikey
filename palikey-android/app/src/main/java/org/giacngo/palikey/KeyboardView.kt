package org.giacngo.palikey

import android.annotation.SuppressLint
import android.content.Context
import android.graphics.Color
import android.graphics.Typeface
import android.graphics.drawable.GradientDrawable
import android.graphics.drawable.StateListDrawable
import android.os.Handler
import android.os.Looper
import android.util.TypedValue
import android.view.Gravity
import android.view.MotionEvent
import android.view.View
import android.widget.LinearLayout
import android.widget.TextView

/**
 * Interface through which KeyboardView communicates user actions to PaliKeyService.
 */
interface KeyboardActionListener {
    fun onChar(c: Char)
    fun onPaliChar(c: Char)
    fun onBackspace()
    fun onSpace()
    fun onEnter()
    fun onModeChanged(newMode: InputMode)
    fun onOpenSettings()
}

/**
 * Clean, modern programmatic keyboard view without XML inflation or deprecated APIs.
 *
 * Provides:
 * 1. Standard QWERTY letter rows
 * 2. Dedicated Pāḷi diacritic top row (ā ī ū ṅ ñ ṭ ḍ ṇ ḷ ṃ) shown exclusively in PALI mode
 * 3. Three-mode cycling key: Tiếng Việt → English → Pāḷi → (back to Tiếng Việt)
 * 4. Numeric & Symbol layout (?123 / ABC)
 * 5. Smooth touch & ripple effects with backspace repeat
 */
@SuppressLint("ViewConstructor")
class KeyboardView(
    context: Context,
    private val preferences: AppPreferences,
    private val listener: KeyboardActionListener
) : LinearLayout(context) {

    private var isShifted = false
    private var isSymbolsMode = false

    private val handler = Handler(Looper.getMainLooper())
    private var backspaceRunnable: Runnable? = null

    // Theme color constants (Dark Slate / Material theme)
    private val colorBg = Color.parseColor("#181A1F")
    private val colorKeyNormal = Color.parseColor("#282C34")
    private val colorKeyPressed = Color.parseColor("#3E4451")
    private val colorKeySpecial = Color.parseColor("#21252B")
    private val colorKeyAccent = Color.parseColor("#2D5B9A")
    private val colorKeyPali = Color.parseColor("#1E3A5F")
    private val colorTextNormal = Color.parseColor("#E6EDF3")
    private val colorTextMuted = Color.parseColor("#8B949E")

    private val dp3 = dpToPx(3)
    private val dp4 = dpToPx(4)
    private val dp6 = dpToPx(6)
    private val rowHeight = dpToPx(52)

    init {
        orientation = VERTICAL
        setBackgroundColor(colorBg)
        setPadding(dp4, dp6, dp4, dp6)
        rebuildLayout()
    }

    /**
     * Refreshes the keyboard display (e.g. when preferences change).
     */
    fun refresh() {
        rebuildLayout()
    }

    private fun rebuildLayout() {
        removeAllViews()

        if (isSymbolsMode) {
            buildSymbolsLayout()
        } else {
            buildAlphaLayout()
        }
    }

    /* ========================================================================= */
    /* Alpha (QWERTY + Pāḷi Row) Layout                                         */
    /* ========================================================================= */

    private fun buildAlphaLayout() {
        val mode = preferences.inputMode

        // 1. Pāḷi Row: Shown exclusively in PALI mode (§Part A.4)
        // VIETNAMESE and ENGLISH modes have NO Pāḷi diacritic top row
        if (mode == InputMode.PALI) {
            addView(createPaliRow())
        }

        // 2. QWERTY Row 1: q w e r t y u i o p
        val row1 = listOf("q", "w", "e", "r", "t", "y", "u", "i", "o", "p")
        addView(createKeyRow(row1))

        // 3. QWERTY Row 2: a s d f g h j k l
        val row2 = listOf("a", "s", "d", "f", "g", "h", "j", "k", "l")
        addView(createKeyRow(row2, sideMarginWeight = 0.5f))

        // 4. QWERTY Row 3: [Shift] z x c v b n m [Backspace]
        val row3Layout = LinearLayout(context).apply {
            orientation = HORIZONTAL
            layoutParams = LayoutParams(LayoutParams.MATCH_PARENT, rowHeight).apply {
                topMargin = dp3
                bottomMargin = dp3
            }
        }

        // Shift key
        val shiftLabel = if (isShifted) "⇪" else "⇧"
        val shiftKey = createSpecialKey(shiftLabel, weight = 1.4f, isShiftKey = true) {
            isShifted = !isShifted
            rebuildLayout()
        }
        row3Layout.addView(shiftKey)

        // Letters: z x c v b n m
        val row3Letters = listOf("z", "x", "c", "v", "b", "n", "m")
        for (letter in row3Letters) {
            val display = if (isShifted) letter.uppercase() else letter
            val key = createKey(display, weight = 1.0f) {
                listener.onChar(display[0])
                if (isShifted) {
                    isShifted = false
                    rebuildLayout()
                }
            }
            row3Layout.addView(key)
        }

        // Backspace key with repeat
        val backspaceKey = createBackspaceKey(weight = 1.4f)
        row3Layout.addView(backspaceKey)
        addView(row3Layout)

        // 5. Bottom Row: [?123] [Mode] [Space] [.] [Enter]
        addView(createBottomRow())
    }

    /* ========================================================================= */
    /* Pāḷi Dedicated Row                                                        */
    /* ========================================================================= */

    private fun createPaliRow(): View {
        val row = LinearLayout(context).apply {
            orientation = HORIZONTAL
            layoutParams = LayoutParams(LayoutParams.MATCH_PARENT, rowHeight).apply {
                topMargin = dp3
                bottomMargin = dp3
            }
        }

        val nigChar = if (isShifted) preferences.capitalNiggahitaChar else preferences.niggahitaChar
        val basePaliChars = listOf('ā', 'ī', 'ū', 'ṅ', 'ñ', 'ṭ', 'ḍ', 'ṇ', 'ḷ')

        val chars = basePaliChars.map { if (isShifted) it.uppercaseChar() else it } + nigChar

        for (ch in chars) {
            val key = createKey(ch.toString(), weight = 1.0f, customBgColor = colorKeyPali) {
                listener.onPaliChar(ch)
                if (isShifted) {
                    isShifted = false
                    rebuildLayout()
                }
            }
            row.addView(key)
        }
        return row
    }

    /* ========================================================================= */
    /* Symbols Layout (?123)                                                    */
    /* ========================================================================= */

    private fun buildSymbolsLayout() {
        // Row 1: Numbers 1 2 3 4 5 6 7 8 9 0
        val numbers = listOf("1", "2", "3", "4", "5", "6", "7", "8", "9", "0")
        addView(createKeyRow(numbers))

        // Row 2: @ # $ % & - + ( ) /
        val row2Symbols = listOf("@", "#", "$", "%", "&", "-", "+", "(", ")", "/")
        addView(createKeyRow(row2Symbols))

        // Row 3: * " ' : ; ! ? [Backspace]
        val row3 = LinearLayout(context).apply {
            orientation = HORIZONTAL
            layoutParams = LayoutParams(LayoutParams.MATCH_PARENT, rowHeight).apply {
                topMargin = dp3
                bottomMargin = dp3
            }
        }
        val symbols3 = listOf("*", "\"", "'", ":", ";", "!", "?")
        for (sym in symbols3) {
            val key = createKey(sym, weight = 1.0f) {
                listener.onChar(sym[0])
            }
            row3.addView(key)
        }
        row3.addView(createBackspaceKey(weight = 1.5f))
        addView(row3)

        // Row 4: [ABC] [,] [Space] [.] [Enter]
        addView(createSymbolsBottomRow())
    }

    /* ========================================================================= */
    /* Bottom Rows                                                               */
    /* ========================================================================= */

    private fun createBottomRow(): View {
        val row = LinearLayout(context).apply {
            orientation = HORIZONTAL
            layoutParams = LayoutParams(LayoutParams.MATCH_PARENT, rowHeight).apply {
                topMargin = dp3
                bottomMargin = dp3
            }
        }

        // [?123] key
        val symKey = createSpecialKey("?123", weight = 1.3f) {
            isSymbolsMode = true
            rebuildLayout()
        }
        row.addView(symKey)

        // Mode cycling key: Tiếng Việt → English → Pāḷi → Tiếng Việt
        val mode = preferences.inputMode
        val modeLabel = mode.label
        val modeKey = createSpecialKey(modeLabel, weight = 1.7f) {
            val nextMode = mode.next()
            preferences.inputMode = nextMode
            listener.onModeChanged(nextMode)
            rebuildLayout()
        }
        modeKey.setOnLongClickListener {
            listener.onOpenSettings()
            true
        }
        row.addView(modeKey)

        // Spacebar
        val spaceKey = createKey("Space", weight = 3.6f) {
            listener.onSpace()
        }
        row.addView(spaceKey)

        // Dot key '.' present across all modes (§Part A.4)
        val dotKey = createKey(".", weight = 1.1f) {
            listener.onChar('.')
        }
        row.addView(dotKey)

        // Enter key
        val enterKey = createSpecialKey("↵", weight = 1.4f, customBgColor = colorKeyAccent) {
            listener.onEnter()
        }
        row.addView(enterKey)

        return row
    }

    private fun createSymbolsBottomRow(): View {
        val row = LinearLayout(context).apply {
            orientation = HORIZONTAL
            layoutParams = LayoutParams(LayoutParams.MATCH_PARENT, rowHeight).apply {
                topMargin = dp3
                bottomMargin = dp3
            }
        }

        // [ABC] key
        val abcKey = createSpecialKey("ABC", weight = 1.4f) {
            isSymbolsMode = false
            rebuildLayout()
        }
        row.addView(abcKey)

        // Comma
        val commaKey = createKey(",", weight = 1.0f) {
            listener.onChar(',')
        }
        row.addView(commaKey)

        // Space
        val spaceKey = createKey("Space", weight = 4.2f) {
            listener.onSpace()
        }
        row.addView(spaceKey)

        // Dot
        val dotKey = createKey(".", weight = 1.0f) {
            listener.onChar('.')
        }
        row.addView(dotKey)

        // Enter
        val enterKey = createSpecialKey("↵", weight = 1.4f, customBgColor = colorKeyAccent) {
            listener.onEnter()
        }
        row.addView(enterKey)

        return row
    }

    /* ========================================================================= */
    /* View Helpers & Key Builders                                               */
    /* ========================================================================= */

    private fun createKeyRow(
        keys: List<String>,
        sideMarginWeight: Float = 0f
    ): LinearLayout {
        val row = LinearLayout(context).apply {
            orientation = HORIZONTAL
            layoutParams = LayoutParams(LayoutParams.MATCH_PARENT, rowHeight).apply {
                topMargin = dp3
                bottomMargin = dp3
            }
        }

        if (sideMarginWeight > 0f) {
            row.addView(View(context).apply {
                layoutParams = LayoutParams(0, LayoutParams.MATCH_PARENT, sideMarginWeight)
            })
        }

        for (item in keys) {
            val display = if (isShifted) item.uppercase() else item
            val key = createKey(display, weight = 1.0f) {
                listener.onChar(display[0])
                if (isShifted) {
                    isShifted = false
                    rebuildLayout()
                }
            }
            row.addView(key)
        }

        if (sideMarginWeight > 0f) {
            row.addView(View(context).apply {
                layoutParams = LayoutParams(0, LayoutParams.MATCH_PARENT, sideMarginWeight)
            })
        }

        return row
    }

    private fun createKey(
        text: String,
        weight: Float,
        customBgColor: Int = colorKeyNormal,
        onClick: () -> Unit
    ): TextView {
        return TextView(context).apply {
            this.text = text
            gravity = Gravity.CENTER
            setTextColor(colorTextNormal)
            setTextSize(TypedValue.COMPLEX_UNIT_SP, if (text.length > 2) 13f else 18f)
            typeface = Typeface.DEFAULT_BOLD
            background = createKeyDrawable(customBgColor, colorKeyPressed)
            isClickable = true
            isFocusable = false

            layoutParams = LayoutParams(0, LayoutParams.MATCH_PARENT, weight).apply {
                leftMargin = dp3
                rightMargin = dp3
            }

            setOnClickListener { onClick() }
        }
    }

    private fun createSpecialKey(
        text: String,
        weight: Float,
        isShiftKey: Boolean = false,
        customBgColor: Int = colorKeySpecial,
        onClick: () -> Unit
    ): TextView {
        val bg = if (isShiftKey && isShifted) colorKeyAccent else customBgColor
        return TextView(context).apply {
            this.text = text
            gravity = Gravity.CENTER
            setTextColor(colorTextNormal)
            setTextSize(TypedValue.COMPLEX_UNIT_SP, if (text.length > 2) 13f else 16f)
            typeface = Typeface.DEFAULT_BOLD
            background = createKeyDrawable(bg, colorKeyPressed)
            isClickable = true
            isFocusable = false

            layoutParams = LayoutParams(0, LayoutParams.MATCH_PARENT, weight).apply {
                leftMargin = dp3
                rightMargin = dp3
            }

            setOnClickListener { onClick() }
        }
    }

    @SuppressLint("ClickableViewAccessibility")
    private fun createBackspaceKey(weight: Float): TextView {
        return TextView(context).apply {
            text = "⌫"
            gravity = Gravity.CENTER
            setTextColor(colorTextNormal)
            setTextSize(TypedValue.COMPLEX_UNIT_SP, 17f)
            typeface = Typeface.DEFAULT_BOLD
            background = createKeyDrawable(colorKeySpecial, colorKeyPressed)
            isClickable = true
            isFocusable = false

            layoutParams = LayoutParams(0, LayoutParams.MATCH_PARENT, weight).apply {
                leftMargin = dp3
                rightMargin = dp3
            }

            setOnTouchListener { _, event ->
                when (event.action) {
                    MotionEvent.ACTION_DOWN -> {
                        listener.onBackspace()
                        startBackspaceRepeat()
                    }
                    MotionEvent.ACTION_UP, MotionEvent.ACTION_CANCEL -> {
                        stopBackspaceRepeat()
                    }
                }
                false
            }
        }
    }

    private fun startBackspaceRepeat() {
        stopBackspaceRepeat()
        backspaceRunnable = object : Runnable {
            override fun run() {
                listener.onBackspace()
                handler.postDelayed(this, 60)
            }
        }
        handler.postDelayed(backspaceRunnable!!, 350)
    }

    private fun stopBackspaceRepeat() {
        backspaceRunnable?.let { handler.removeCallbacks(it) }
        backspaceRunnable = null
    }

    private fun createKeyDrawable(normalColor: Int, pressedColor: Int): StateListDrawable {
        val normalState = GradientDrawable().apply {
            shape = GradientDrawable.RECTANGLE
            setColor(normalColor)
            cornerRadius = dpToPx(5).toFloat()
        }
        val pressedState = GradientDrawable().apply {
            shape = GradientDrawable.RECTANGLE
            setColor(pressedColor)
            cornerRadius = dpToPx(5).toFloat()
        }

        return StateListDrawable().apply {
            addState(intArrayOf(android.R.attr.state_pressed), pressedState)
            addState(intArrayOf(), normalState)
        }
    }

    private fun dpToPx(dp: Int): Int {
        return TypedValue.applyDimension(
            TypedValue.COMPLEX_UNIT_DIP,
            dp.toFloat(),
            resources.displayMetrics
        ).toInt()
    }
}
