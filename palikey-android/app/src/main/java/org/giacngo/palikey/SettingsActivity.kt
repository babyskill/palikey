package org.giacngo.palikey

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.graphics.Color
import android.graphics.Typeface
import android.graphics.drawable.GradientDrawable
import android.graphics.drawable.StateListDrawable
import android.os.Bundle
import android.provider.Settings
import android.util.TypedValue
import android.view.Gravity
import android.view.View
import android.view.ViewGroup
import android.view.inputmethod.InputMethodManager
import android.widget.*

/**
 * Settings Activity for PaliKey.
 *
 * Provides:
 * 1. Pāḷi transliteration variant picker (Velthuis, PaliTelex, SIL)
 * 2. Niggahīta orthography toggle (ṃ Dot Below vs ṁ Dot Above)
 * 3. Extended Sanskrit/IAST character set toggle
 * 4. "Enable PaliKey" button (Settings.ACTION_INPUT_METHOD_SETTINGS)
 * 5. "Select PaliKey" button (InputMethodManager.showInputMethodPicker)
 * 6. Live test typing input field in active input mode
 * 7. Engine version and status indicator
 */
class SettingsActivity : Activity() {

    private lateinit var preferences: AppPreferences

    // Dark theme palette
    private val colorBg = Color.parseColor("#0D1117")
    private val colorCardBg = Color.parseColor("#161B22")
    private val colorBorder = Color.parseColor("#30363D")
    private val colorTextPrimary = Color.parseColor("#F0F6FC")
    private val colorTextSecondary = Color.parseColor("#8B949E")
    private val colorAccent = Color.parseColor("#1F6FEB")
    private val colorAccentHover = Color.parseColor("#388BFD")

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        preferences = AppPreferences(this)

        val rootScrollView = ScrollView(this).apply {
            setBackgroundColor(colorBg)
            layoutParams = ViewGroup.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.MATCH_PARENT
            )
        }

        val container = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            val pad = dp(20)
            setPadding(pad, pad, pad, pad * 2)
            layoutParams = ViewGroup.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
            )
        }

        rootScrollView.addView(container)
        setContentView(rootScrollView)

        buildUI(container)
    }

    private fun buildUI(container: LinearLayout) {
        // Header Title
        val titleText = TextView(this).apply {
            text = "PaliKey"
            textSize = 28f
            setTextColor(colorTextPrimary)
            typeface = Typeface.DEFAULT_BOLD
        }
        container.addView(titleText)

        val subtitleText = TextView(this).apply {
            text = "Clean-room cross-platform Pāḷi input method"
            textSize = 14f
            setTextColor(colorTextSecondary)
            setPadding(0, dp(4), 0, dp(16))
        }
        container.addView(subtitleText)

        // Activation Card (Enable & Select buttons)
        container.addView(createSectionHeader("KEYBOARD SETUP"))
        val activationCard = createCardLayout()

        val enableButton = createActionButton("1. Enable PaliKey in Settings") {
            startActivity(Intent(Settings.ACTION_INPUT_METHOD_SETTINGS))
        }
        activationCard.addView(enableButton)

        val selectButton = createActionButton("2. Select PaliKey Keyboard") {
            val imm = getSystemService(Context.INPUT_METHOD_SERVICE) as? InputMethodManager
            imm?.showInputMethodPicker()
        }
        activationCard.addView(selectButton)

        container.addView(activationCard)

        // Pāḷi Transliteration Variant Card (§Part B.1 & B.2)
        container.addView(createSectionHeader("PĀḶI TRANSLITERATION VARIANT"))
        val variantCard = createCardLayout()

        val variantGroup = RadioGroup(this).apply {
            orientation = RadioGroup.VERTICAL
        }

        val variants = listOf(
            Triple(NativeEngine.SCHEME_VELTHUIS, "Velthuis (Academic Standard)", "aa→ā, .t→ṭ, \"n→ṅ, ~n→ñ, .m→ṃ"),
            Triple(NativeEngine.SCHEME_PALI_TELEX, "PaliTelex (Vietnamese-friendly)", "aa→ā, tx→ṭ, ngw→ṅ, nw/ny→ñ, mx→ṃ"),
            Triple(NativeEngine.SCHEME_SIL, "SIL / Keyman Compatible", "-a→ā, .t→ṭ, >n→ṅ, ~n→ñ, .m→ṃ")
        )

        val currentVariant = preferences.paliVariant
        for ((id, title, example) in variants) {
            val rb = RadioButton(this).apply {
                this.id = View.generateViewId()
                text = "$title\n$example"
                setTextColor(colorTextPrimary)
                textSize = 14f
                setLineSpacing(dp(2).toFloat(), 1.1f)
                setPadding(dp(8), dp(8), dp(8), dp(8))
                isChecked = (id == currentVariant)
                setOnCheckedChangeListener { _, isChecked ->
                    if (isChecked) {
                        preferences.paliVariant = id
                    }
                }
            }
            variantGroup.addView(rb)
        }
        variantCard.addView(variantGroup)
        container.addView(variantCard)

        // Hint line under the card (§Part B.3)
        val modeHintText = TextView(this).apply {
            text = "Switch mode (Tiếng Việt / English / Pāḷi) with the mode key on the keyboard."
            textSize = 12f
            setTextColor(colorTextSecondary)
            setPadding(dp(4), 0, dp(4), dp(8))
        }
        container.addView(modeHintText)

        // Orthography Card (Niggahīta & Extended Set)
        container.addView(createSectionHeader("ORTHOGRAPHY & VARIANTS"))
        val orthographyCard = createCardLayout()

        val niggahitaTitle = TextView(this).apply {
            text = "Niggahīta Character Variant:"
            setTextColor(colorTextPrimary)
            textSize = 14f
            typeface = Typeface.DEFAULT_BOLD
            setPadding(0, 0, 0, dp(6))
        }
        orthographyCard.addView(niggahitaTitle)

        val niggahitaGroup = RadioGroup(this).apply {
            orientation = RadioGroup.VERTICAL
        }

        val niggahitaVariants = listOf(
            Pair(NativeEngine.NIGGAHITA_DOT_BELOW, "ṃ (U+1E43) — Dot Below (PTS / SuttaCentral)"),
            Pair(NativeEngine.NIGGAHITA_DOT_ABOVE, "ṁ (U+1E41) — Dot Above (VRI Tradition)")
        )

        val currentNiggahita = preferences.niggahitaVariant
        for ((id, desc) in niggahitaVariants) {
            val rb = RadioButton(this).apply {
                this.id = View.generateViewId()
                text = desc
                setTextColor(colorTextPrimary)
                textSize = 13f
                isChecked = (id == currentNiggahita)
                setOnCheckedChangeListener { _, isChecked ->
                    if (isChecked) {
                        preferences.niggahitaVariant = id
                    }
                }
            }
            niggahitaGroup.addView(rb)
        }
        orthographyCard.addView(niggahitaGroup)

        // Extended Set Switch
        val extLayout = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
            gravity = Gravity.CENTER_VERTICAL
            setPadding(0, dp(12), 0, 0)
        }
        val extText = TextView(this).apply {
            text = "Extended Sanskrit / IAST set\n(Adds ṛ ṝ ḹ ḥ ś ṣ ē ō)"
            setTextColor(colorTextPrimary)
            textSize = 13f
            layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1.0f)
        }
        val extSwitch = Switch(this).apply {
            isChecked = preferences.isExtendedSetEnabled
            setOnCheckedChangeListener { _, isChecked ->
                preferences.isExtendedSetEnabled = isChecked
            }
        }
        extLayout.addView(extText)
        extLayout.addView(extSwitch)
        orthographyCard.addView(extLayout)

        container.addView(orthographyCard)

        // Live Test Field Card (§Part B.4)
        container.addView(createSectionHeader("TEST TYPING AREA"))
        val testCard = createCardLayout()

        val testInstruction = TextView(this).apply {
            text = "Tap below to test typing with PaliKey (e.g. 'sammaadit.thi'):"
            setTextColor(colorTextSecondary)
            textSize = 13f
            setPadding(0, 0, 0, dp(8))
        }
        testCard.addView(testInstruction)

        val testEditText = EditText(this).apply {
            hint = "Type Pāḷi text here..."
            setHintTextColor(colorTextSecondary)
            setTextColor(colorTextPrimary)
            textSize = 16f
            setBackground(createCardDrawable(Color.parseColor("#090D13"), colorBorder))
            val pad = dp(12)
            setPadding(pad, pad, pad, pad)
            layoutParams = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.WRAP_CONTENT
            )
        }
        testCard.addView(testEditText)
        container.addView(testCard)

        // Footer / Version Info
        val versionStr = try {
            NativeEngine.nativeVersion()
        } catch (e: Throwable) {
            "0.1.0"
        }

        val footerText = TextView(this).apply {
            text = "PaliKey Engine v$versionStr • Offline • Zero telemetry\nOrganized by Giac Ngo • Open-source MIT"
            textSize = 12f
            setTextColor(colorTextSecondary)
            gravity = Gravity.CENTER
            setPadding(0, dp(24), 0, dp(8))
        }
        container.addView(footerText)
    }

    private fun createSectionHeader(title: String): TextView {
        return TextView(this).apply {
            text = title
            textSize = 12f
            typeface = Typeface.DEFAULT_BOLD
            setTextColor(colorAccent)
            setPadding(dp(4), dp(16), 0, dp(6))
        }
    }

    private fun createCardLayout(): LinearLayout {
        return LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            background = createCardDrawable(colorCardBg, colorBorder)
            val pad = dp(14)
            setPadding(pad, pad, pad, pad)
            layoutParams = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.WRAP_CONTENT
            ).apply {
                bottomMargin = dp(8)
            }
        }
    }

    private fun createActionButton(label: String, onClick: () -> Unit): Button {
        return Button(this).apply {
            text = label
            setTextColor(Color.WHITE)
            textSize = 14f
            typeface = Typeface.DEFAULT_BOLD
            background = createButtonDrawable(colorAccent, colorAccentHover)
            layoutParams = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.WRAP_CONTENT
            ).apply {
                topMargin = dp(4)
                bottomMargin = dp(4)
            }
            setOnClickListener { onClick() }
        }
    }

    private fun createCardDrawable(bgColor: Int, borderColor: Int): GradientDrawable {
        return GradientDrawable().apply {
            setColor(bgColor)
            cornerRadius = dp(8).toFloat()
            setStroke(dp(1), borderColor)
        }
    }

    private fun createButtonDrawable(normalColor: Int, pressedColor: Int): StateListDrawable {
        val normal = GradientDrawable().apply {
            setColor(normalColor)
            cornerRadius = dp(6).toFloat()
        }
        val pressed = GradientDrawable().apply {
            setColor(pressedColor)
            cornerRadius = dp(6).toFloat()
        }
        return StateListDrawable().apply {
            addState(intArrayOf(android.R.attr.state_pressed), pressed)
            addState(intArrayOf(), normal)
        }
    }

    private fun dp(value: Int): Int {
        return TypedValue.applyDimension(
            TypedValue.COMPLEX_UNIT_DIP,
            value.toFloat(),
            resources.displayMetrics
        ).toInt()
    }
}
