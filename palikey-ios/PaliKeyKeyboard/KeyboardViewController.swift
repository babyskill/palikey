//
//  KeyboardViewController.swift
//  PaliKeyKeyboard
//
//  UIInputViewController implementation for PaliKey iOS keyboard extension.
//  Hosts an independent PaliEngine instance per typing session and translates
//  keystrokes via the engine's state machine.
//

import UIKit

final class KeyboardViewController: UIInputViewController, KeyboardViewDelegate {

    private var engine: PaliEngine?
    private var preferences = PaliPreferences.shared
    private var keyboardView: KeyboardView?

    override func viewDidLoad() {
        super.viewDidLoad()
        setupEngine()
        setupKeyboardView()
    }

    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        preferences.reload()
        syncEngineWithPreferences()
        keyboardView?.reloadPreferences()
    }

    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        engine?.reset()
    }

    deinit {
        engine = nil
    }

    private func setupEngine() {
        preferences.reload()
        engine = PaliEngine(
            scheme: preferences.schemeId,
            niggahita: preferences.niggahitaVariant,
            extendedSet: preferences.extendedSet
        )
    }

    private func syncEngineWithPreferences() {
        guard let engine = engine else {
            setupEngine()
            return
        }
        engine.scheme = preferences.schemeId
        engine.niggahita = preferences.niggahitaVariant
        engine.extendedSet = preferences.extendedSet
        engine.reset()
    }

    private func setupKeyboardView() {
        let view = KeyboardView(preferences: preferences, needsGlobeKey: self.needsInputModeSwitchKey)
        view.delegate = self
        view.translatesAutoresizingMaskIntoConstraints = false

        self.view.addSubview(view)
        NSLayoutConstraint.activate([
            view.topAnchor.constraint(equalTo: self.view.topAnchor),
            view.bottomAnchor.constraint(equalTo: self.view.bottomAnchor),
            view.leadingAnchor.constraint(equalTo: self.view.leadingAnchor),
            view.trailingAnchor.constraint(equalTo: self.view.trailingAnchor)
        ])

        self.keyboardView = view
    }

    override func textDidChange(_ textInput: UITextInput?) {
        super.textDidChange(textInput)
        // Whenever the cursor or surrounding text changes externally, flush composition buffer
        engine?.reset()
    }

    /* ========================================================================= */
    /* KeyboardViewDelegate Implementation                                       */
    /* ========================================================================= */

    func keyboardView(_ view: KeyboardView, didTapCharacter char: Character) {
        let proxy = textDocumentProxy

        switch preferences.keyboardMode {
        case .transducer:
            guard let engine = engine else {
                proxy.insertText(String(char))
                return
            }
            let result = engine.process(char: char)
            applyProcessResult(result)

        case .paliRow, .off:
            proxy.insertText(String(char))
        }
    }

    func keyboardView(_ view: KeyboardView, didTapPaliCharacter char: Character) {
        let proxy = textDocumentProxy
        // Tapping a dedicated Pāḷi key immediately commits the character
        // and flushes active composition
        engine?.reset()
        proxy.insertText(String(char))
    }

    func keyboardViewDidTapBackspace(_ view: KeyboardView) {
        let proxy = textDocumentProxy

        if preferences.keyboardMode == .transducer, let engine = engine {
            let result = engine.processBackspace()
            if result.backspaces > 0 || !result.commit.isEmpty {
                applyProcessResult(result)
                return
            }
        }

        proxy.deleteBackward()
    }

    func keyboardViewDidTapSpace(_ view: KeyboardView) {
        let proxy = textDocumentProxy
        engine?.reset()
        proxy.insertText(" ")
    }

    func keyboardViewDidTapReturn(_ view: KeyboardView) {
        let proxy = textDocumentProxy
        engine?.reset()
        proxy.insertText("\n")
    }

    func keyboardViewDidCycleMode(_ view: KeyboardView) {
        let nextMode: KeyboardMode
        switch preferences.keyboardMode {
        case .transducer: nextMode = .paliRow
        case .paliRow: nextMode = .off
        case .off: nextMode = .transducer
        }
        preferences.keyboardMode = nextMode
        engine?.reset()
    }

    func keyboardViewDidRequestGlobe(_ view: KeyboardView, sender: UIButton, event: UIEvent) {
        self.handleInputModeList(from: sender, with: event)
    }

    /* ========================================================================= */
    /* Internal Helpers                                                          */
    /* ========================================================================= */

    private func applyProcessResult(_ result: PaliEngine.ProcessOutput) {
        let proxy = textDocumentProxy

        for _ in 0..<result.backspaces {
            proxy.deleteBackward()
        }

        if !result.commit.isEmpty {
            proxy.insertText(result.commit)
        }
    }
}
