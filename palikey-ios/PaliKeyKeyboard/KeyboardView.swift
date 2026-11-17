//
//  KeyboardView.swift
//  PaliKeyKeyboard
//
//  Programmatic, high-performance UIKit keyboard view for iOS.
//  Zero storyboard/XIB overhead, low memory footprint, adaptive styling.
//

import UIKit

protocol KeyboardViewDelegate: AnyObject {
    func keyboardView(_ view: KeyboardView, didTapCharacter char: Character)
    func keyboardView(_ view: KeyboardView, didTapPaliCharacter char: Character)
    func keyboardViewDidTapBackspace(_ view: KeyboardView)
    func keyboardViewDidTapSpace(_ view: KeyboardView)
    func keyboardViewDidTapReturn(_ view: KeyboardView)
    func keyboardViewDidCycleMode(_ view: KeyboardView)
    func keyboardViewDidRequestGlobe(_ view: KeyboardView, sender: UIButton, event: UIEvent)
}

final class KeyboardView: UIView, UIInputViewAudioFeedback {

    weak var delegate: KeyboardViewDelegate?
    private let preferences: PaliPreferences

    // Keyboard state
    private var isShifted: Bool = false
    private var isCapsLock: Bool = false
    private var isSymbolsMode: Bool = false
    private var lastShiftTapTime: TimeInterval = 0

    // Backspace repeat timer
    private var backspaceTimer: Timer?

    // Subviews
    private let mainStackView = UIStackView()
    private var modeButton: UIButton?
    private var shiftButton: UIButton?
    private var globeButton: UIButton?

    // UI Configuration
    var enableInputClicksWhenVisible: Bool { true }
    private let needsGlobeKey: Bool

    // Theme Colors
    private var keyBackgroundColor: UIColor {
        traitCollection.userInterfaceStyle == .dark ? UIColor(white: 0.28, alpha: 1.0) : .white
    }
    private var specialKeyBackgroundColor: UIColor {
        traitCollection.userInterfaceStyle == .dark ? UIColor(white: 0.18, alpha: 1.0) : UIColor(white: 0.76, alpha: 1.0)
    }
    private var paliKeyBackgroundColor: UIColor {
        traitCollection.userInterfaceStyle == .dark ? UIColor(red: 0.15, green: 0.25, blue: 0.40, alpha: 1.0) : UIColor(red: 0.85, green: 0.90, blue: 0.98, alpha: 1.0)
    }
    private var keyTextColor: UIColor {
        traitCollection.userInterfaceStyle == .dark ? .white : .black
    }
    private var keyShadowColor: UIColor {
        UIColor(white: 0.0, alpha: 0.3)
    }

    init(preferences: PaliPreferences, needsGlobeKey: Bool) {
        self.preferences = preferences
        self.needsGlobeKey = needsGlobeKey
        super.init(frame: .zero)
        setupView()
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    private func setupView() {
        backgroundColor = .clear

        mainStackView.axis = .vertical
        mainStackView.distribution = .fillEqually
        mainStackView.spacing = 6
        mainStackView.translatesAutoresizingMaskIntoConstraints = false

        addSubview(mainStackView)
        NSLayoutConstraint.activate([
            mainStackView.topAnchor.constraint(equalTo: topAnchor, constant: 6),
            mainStackView.bottomAnchor.constraint(equalTo: bottomAnchor, constant: -6),
            mainStackView.leadingAnchor.constraint(equalTo: leadingAnchor, constant: 4),
            mainStackView.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -4)
        ])

        rebuildKeyboard()
    }

    override func traitCollectionDidChange(_ previousTraitCollection: UITraitCollection?) {
        super.traitCollectionDidChange(previousTraitCollection)
        if traitCollection.hasDifferentColorAppearance(comparedTo: previousTraitCollection) {
            rebuildKeyboard()
        }
    }

    func reloadPreferences() {
        rebuildKeyboard()
    }

    private func rebuildKeyboard() {
        mainStackView.arrangedSubviews.forEach {
            mainStackView.removeArrangedSubview($0)
            $0.removeFromSuperview()
        }

        if isSymbolsMode {
            buildSymbolsKeyboard()
        } else {
            buildAlphabetKeyboard()
        }
    }

    /* ========================================================================= */
    /* Alphabet & Pāḷi Keyboard Layout                                          */
    /* ========================================================================= */

    private func buildAlphabetKeyboard() {
        let mode = preferences.keyboardMode

        // 1. Dedicated Pāḷi Row (visible in PALI_ROW mode or TRANSDUCER mode)
        if mode == .paliRow || mode == .transducer {
            let paliRow = createPaliRow()
            mainStackView.addArrangedSubview(paliRow)
        }

        // 2. QWERTY Row 1: q w e r t y u i o p
        let row1Keys = ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"]
        let row1 = createKeyRow(keys: row1Keys)
        mainStackView.addArrangedSubview(row1)

        // 3. QWERTY Row 2: a s d f g h j k l (with horizontal padding)
        let row2Keys = ["a", "s", "d", "f", "g", "h", "j", "k", "l"]
        let row2 = createKeyRow(keys: row2Keys, sideMarginMultiplier: 0.5)
        mainStackView.addArrangedSubview(row2)

        // 4. QWERTY Row 3: [Shift] z x c v b n m [Backspace]
        let row3Keys = ["z", "x", "c", "v", "b", "n", "m"]
        let row3 = createShiftLettersBackspaceRow(letters: row3Keys)
        mainStackView.addArrangedSubview(row3)

        // 5. Bottom Control Row: [123] [Globe] [Mode] [Space] [Return]
        let bottomRow = createBottomRow()
        mainStackView.addArrangedSubview(bottomRow)
    }

    private func createPaliRow() -> UIStackView {
        let stack = UIStackView()
        stack.axis = .horizontal
        stack.distribution = .fillEqually
        stack.spacing = 4

        let nig = (isShifted || isCapsLock) ? preferences.capitalNiggahitaCharacter : preferences.niggahitaCharacter
        let baseChars: [Character] = ["ā", "ī", "ū", "ṅ", "ñ", "ṭ", "ḍ", "ṇ", "ḷ"]

        var allChars = baseChars.map { (isShifted || isCapsLock) ? Character($0.uppercased()) : $0 }
        allChars.append(nig)

        for ch in allChars {
            let button = createStandardKey(title: String(ch), bgColor: paliKeyBackgroundColor)
            button.addTarget(self, action: #selector(paliKeyTapped(_:)), for: .touchUpInside)
            stack.addArrangedSubview(button)
        }

        return stack
    }

    private func createKeyRow(keys: [String], sideMarginMultiplier: CGFloat = 0.0) -> UIStackView {
        let stack = UIStackView()
        stack.axis = .horizontal
        stack.distribution = .fillEqually
        stack.spacing = 5

        if sideMarginMultiplier > 0 {
            let leftSpacer = UIView()
            leftSpacer.translatesAutoresizingMaskIntoConstraints = false
            stack.addArrangedSubview(leftSpacer)
        }

        for key in keys {
            let title = (isShifted || isCapsLock) ? key.uppercased() : key.lowercased()
            let button = createStandardKey(title: title)
            button.addTarget(self, action: #selector(alphaKeyTapped(_:)), for: .touchUpInside)
            stack.addArrangedSubview(button)
        }

        if sideMarginMultiplier > 0 {
            let rightSpacer = UIView()
            rightSpacer.translatesAutoresizingMaskIntoConstraints = false
            stack.addArrangedSubview(rightSpacer)
        }

        return stack
    }

    private func createShiftLettersBackspaceRow(letters: [String]) -> UIStackView {
        let stack = UIStackView()
        stack.axis = .horizontal
        stack.distribution = .fillProportionally
        stack.spacing = 5

        // Shift Key
        let shiftBtn = createSpecialKey(title: isCapsLock ? "⇪" : (isShifted ? "⬆" : "⇧"))
        shiftBtn.widthAnchor.constraint(equalToConstant: 44).isActive = true
        shiftBtn.addTarget(self, action: #selector(shiftKeyTapped), for: .touchUpInside)
        self.shiftButton = shiftBtn
        stack.addArrangedSubview(shiftBtn)

        // Letters Stack
        let lettersStack = UIStackView()
        lettersStack.axis = .horizontal
        lettersStack.distribution = .fillEqually
        lettersStack.spacing = 5
        for key in letters {
            let title = (isShifted || isCapsLock) ? key.uppercased() : key.lowercased()
            let button = createStandardKey(title: title)
            button.addTarget(self, action: #selector(alphaKeyTapped(_:)), for: .touchUpInside)
            lettersStack.addArrangedSubview(button)
        }
        stack.addArrangedSubview(lettersStack)

        // Backspace Key
        let backspaceBtn = createSpecialKey(title: "⌫")
        backspaceBtn.widthAnchor.constraint(equalToConstant: 44).isActive = true
        let longPress = UILongPressGestureRecognizer(target: self, action: #selector(backspaceLongPressed(_:)))
        longPress.minimumPressDuration = 0.35
        backspaceBtn.addGestureRecognizer(longPress)
        backspaceBtn.addTarget(self, action: #selector(backspaceTapped), for: .touchUpInside)
        stack.addArrangedSubview(backspaceBtn)

        return stack
    }

    /* ========================================================================= */
    /* Symbols Layout (?123)                                                    */
    /* ========================================================================= */

    private func buildSymbolsKeyboard() {
        // Row 1: 1 2 3 4 5 6 7 8 9 0
        let numbers = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"]
        let row1 = createKeyRow(keys: numbers)
        mainStackView.addArrangedSubview(row1)

        // Row 2: - / : ; ( ) $ & @ "
        let row2Keys = ["-", "/", ":", ";", "(", ")", "$", "&", "@", "\""]
        let row2 = createKeyRow(keys: row2Keys)
        mainStackView.addArrangedSubview(row2)

        // Row 3: . , ? ! ' [Backspace]
        let row3 = UIStackView()
        row3.axis = .horizontal
        row3.distribution = .fillProportionally
        row3.spacing = 5

        let symLettersStack = UIStackView()
        symLettersStack.axis = .horizontal
        symLettersStack.distribution = .fillEqually
        symLettersStack.spacing = 5
        let row3Keys = [".", ",", "?", "!", "'", "%", "*"]
        for key in row3Keys {
            let button = createStandardKey(title: key)
            button.addTarget(self, action: #selector(alphaKeyTapped(_:)), for: .touchUpInside)
            symLettersStack.addArrangedSubview(button)
        }
        row3.addArrangedSubview(symLettersStack)

        let backspaceBtn = createSpecialKey(title: "⌫")
        backspaceBtn.widthAnchor.constraint(equalToConstant: 50).isActive = true
        backspaceBtn.addTarget(self, action: #selector(backspaceTapped), for: .touchUpInside)
        row3.addArrangedSubview(backspaceBtn)

        mainStackView.addArrangedSubview(row3)

        // Row 4: Bottom row with ABC switch
        let bottomRow = createBottomRow()
        mainStackView.addArrangedSubview(bottomRow)
    }

    /* ========================================================================= */
    /* Bottom Controls Row                                                       */
    /* ========================================================================= */

    private func createBottomRow() -> UIStackView {
        let stack = UIStackView()
        stack.axis = .horizontal
        stack.distribution = .fillProportionally
        stack.spacing = 5

        // Toggle Symbols / ABC
        let symLabel = isSymbolsMode ? "ABC" : "123"
        let symBtn = createSpecialKey(title: symLabel)
        symBtn.widthAnchor.constraint(equalToConstant: 44).isActive = true
        symBtn.addTarget(self, action: #selector(toggleSymbolsMode), for: .touchUpInside)
        stack.addArrangedSubview(symBtn)

        // Globe Key (for input mode switching)
        if needsGlobeKey {
            let globeBtn = createSpecialKey(title: "🌐")
            globeBtn.widthAnchor.constraint(equalToConstant: 40).isActive = true
            globeBtn.addTarget(self, action: #selector(globeKeyTapped(_:forEvent:)), for: .allTouchEvents)
            self.globeButton = globeBtn
            stack.addArrangedSubview(globeBtn)
        }

        // Mode cycling button: Transducer → Pāḷi Row → Off
        let mode = preferences.keyboardMode
        let modeTitle: String
        switch mode {
        case .transducer: modeTitle = "⚡ \(preferences.activeScheme.displayName)"
        case .paliRow: modeTitle = "🔤 Pāḷi"
        case .off: modeTitle = "⌨️ Off"
        }
        let modeBtn = createSpecialKey(title: modeTitle)
        modeBtn.widthAnchor.constraint(equalToConstant: 80).isActive = true
        modeBtn.addTarget(self, action: #selector(cycleModeTapped), for: .touchUpInside)
        self.modeButton = modeBtn
        stack.addArrangedSubview(modeBtn)

        // Spacebar
        let spaceTitle: String
        switch mode {
        case .transducer: spaceTitle = "PaliKey (\(preferences.activeScheme.displayName))"
        case .paliRow: spaceTitle = "Space"
        case .off: spaceTitle = "English"
        }
        let spaceBtn = createStandardKey(title: spaceTitle)
        spaceBtn.addTarget(self, action: #selector(spaceTapped), for: .touchUpInside)
        stack.addArrangedSubview(spaceBtn)

        // Return Key
        let returnBtn = createSpecialKey(title: "return")
        returnBtn.widthAnchor.constraint(equalToConstant: 68).isActive = true
        returnBtn.addTarget(self, action: #selector(returnTapped), for: .touchUpInside)
        stack.addArrangedSubview(returnBtn)

        return stack
    }

    /* ========================================================================= */
    /* Key Factory Helpers                                                       */
    /* ========================================================================= */

    private func createStandardKey(title: String, bgColor: UIColor? = nil) -> UIButton {
        let button = UIButton(type: .custom)
        button.setTitle(title, for: .normal)
        button.setTitleColor(keyTextColor, for: .normal)
        button.titleLabel?.font = UIFont.systemFont(ofSize: 18, weight: .regular)
        button.backgroundColor = bgColor ?? keyBackgroundColor
        button.layer.cornerRadius = 5.0
        button.layer.shadowColor = keyShadowColor.cgColor
        button.layer.shadowOffset = CGSize(width: 0, height: 1.0)
        button.layer.shadowOpacity = 0.3
        button.layer.shadowRadius = 0.5
        button.translatesAutoresizingMaskIntoConstraints = false
        button.heightAnchor.constraint(greaterThanOrEqualToConstant: 42).isActive = true
        return button
    }

    private func createSpecialKey(title: String) -> UIButton {
        let button = UIButton(type: .custom)
        button.setTitle(title, for: .normal)
        button.setTitleColor(keyTextColor, for: .normal)
        button.titleLabel?.font = UIFont.systemFont(ofSize: 14, weight: .medium)
        button.backgroundColor = specialKeyBackgroundColor
        button.layer.cornerRadius = 5.0
        button.layer.shadowColor = keyShadowColor.cgColor
        button.layer.shadowOffset = CGSize(width: 0, height: 1.0)
        button.layer.shadowOpacity = 0.3
        button.layer.shadowRadius = 0.5
        button.translatesAutoresizingMaskIntoConstraints = false
        button.heightAnchor.constraint(greaterThanOrEqualToConstant: 42).isActive = true
        return button
    }

    /* ========================================================================= */
    /* User Action Handlers                                                      */
    /* ========================================================================= */

    @objc private func alphaKeyTapped(_ sender: UIButton) {
        guard let title = sender.title(for: .normal), let ch = title.first else { return }
        playKeyFeedback()
        delegate?.keyboardView(self, didTapCharacter: ch)

        if isShifted && !isCapsLock {
            isShifted = false
            rebuildKeyboard()
        }
    }

    @objc private func paliKeyTapped(_ sender: UIButton) {
        guard let title = sender.title(for: .normal), let ch = title.first else { return }
        playKeyFeedback()
        delegate?.keyboardView(self, didTapPaliCharacter: ch)

        if isShifted && !isCapsLock {
            isShifted = false
            rebuildKeyboard()
        }
    }

    @objc private func backspaceTapped() {
        playKeyFeedback()
        delegate?.keyboardViewDidTapBackspace(self)
    }

    @objc private func backspaceLongPressed(_ gesture: UILongPressGestureRecognizer) {
        switch gesture.state {
        case .began:
            playKeyFeedback()
            delegate?.keyboardViewDidTapBackspace(self)
            backspaceTimer = Timer.scheduledTimer(withTimeInterval: 0.1, repeats: true) { [weak self] _ in
                guard let self = self else { return }
                self.playKeyFeedback()
                self.delegate?.keyboardViewDidTapBackspace(self)
            }
        case .ended, .cancelled, .failed:
            backspaceTimer?.invalidate()
            backspaceTimer = nil
        default:
            break
        }
    }

    @objc private func spaceTapped() {
        playKeyFeedback()
        delegate?.keyboardViewDidTapSpace(self)
    }

    @objc private func returnTapped() {
        playKeyFeedback()
        delegate?.keyboardViewDidTapReturn(self)
    }

    @objc private func shiftKeyTapped() {
        playKeyFeedback()
        let now = Date().timeIntervalSince1970
        if now - lastShiftTapTime < 0.35 {
            // Double tap -> Toggle Caps Lock
            isCapsLock = !isCapsLock
            isShifted = isCapsLock
        } else {
            // Single tap -> Toggle Shift
            if isCapsLock {
                isCapsLock = false
                isShifted = false
            } else {
                isShifted = !isShifted
            }
        }
        lastShiftTapTime = now
        rebuildKeyboard()
    }

    @objc private func toggleSymbolsMode() {
        playKeyFeedback()
        isSymbolsMode = !isSymbolsMode
        rebuildKeyboard()
    }

    @objc private func cycleModeTapped() {
        playKeyFeedback()
        delegate?.keyboardViewDidCycleMode(self)
        rebuildKeyboard()
    }

    @objc private func globeKeyTapped(_ sender: UIButton, forEvent event: UIEvent) {
        playKeyFeedback()
        delegate?.keyboardViewDidRequestGlobe(self, sender: sender, event: event)
    }

    private func playKeyFeedback() {
        UIDevice.current.playInputClick()
    }
}
