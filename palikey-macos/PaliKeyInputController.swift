//
//  PaliKeyInputController.swift
//  PaliKey
//
//  IMKInputController subclass managing per-client input sessions.
//  Coordinates keystroke interception, engine transduction, EasyUnicode direct-insert,
//  and client text buffer updates.
//

import Cocoa
import InputMethodKit

@objc(PaliKeyInputController)
public class PaliKeyInputController: IMKInputController {
    /// Dedicated engine instance for this client connection
    private let engine: PaliEngine

    public override init!(server: IMKServer!, delegate: Any!, client: Any!) {
        let config = PaliConfig.shared
        self.engine = PaliEngine(
            scheme: config.scheme,
            niggahita: config.niggahita,
            extendedSet: config.extendedSet
        )
        super.init(server: server, delegate: delegate, client: client)
    }

    // MARK: - Event Handling

    public override func handle(_ event: NSEvent!, client sender: Any!) -> Bool {
        guard let event = event, event.type == .keyDown else {
            return false
        }

        // Sync configuration before processing
        syncConfig()

        let flags = event.modifierFlags.intersection(.deviceIndependentFlagsMask)

        // 1. Command / Control shortcuts bypass the IME and reset composition
        if flags.contains(.command) || flags.contains(.control) {
            engine.reset()
            return false
        }

        // 2. AltGr / Option direct-insert layer (EasyUnicode mapping per DESIGN §4)
        if flags.contains(.option) {
            if let chars = event.charactersIgnoringModifiers, let firstChar = chars.first {
                let isShift = flags.contains(.shift)
                if let paliChar = EasyUnicode.map(char: firstChar, isShift: isShift, niggahita: engine.niggahita) {
                    engine.reset()
                    insertDirect(paliChar, client: sender)
                    return true
                }
            }
            // Pass through unmapped Option combinations (e.g. Option+Tab, Option+Esc)
            return false
        }

        // 3. Navigation and action keys flush composition and pass through
        let keyCode = event.keyCode
        if keyCode == 36 || keyCode == 76 || keyCode == 48 || keyCode == 53 || (keyCode >= 123 && keyCode <= 126) {
            engine.reset()
            return false
        }

        // 4. Backspace key (macOS virtual key 51)
        if keyCode == 51 {
            let res = engine.processBackspace()
            if !res.commit.isEmpty {
                // Undo composition: replace the composed char with raw sequence
                applyResult(res, client: sender)
                return true
            } else {
                // Standard deletion: pass through to client text view
                return false
            }
        }

        // 5. Printable characters
        guard let chars = event.characters, !chars.isEmpty else {
            return false
        }

        // Check if characters are printable (skip function keys / control characters)
        for scalar in chars.unicodeScalars {
            if scalar.value < 0x20 && scalar.value != 0x09 && scalar.value != 0x0A && scalar.value != 0x0D {
                engine.reset()
                return false
            }
        }

        for ch in chars {
            let res = engine.process(char: ch)
            applyResult(res, client: sender)
        }
        return true
    }

    // MARK: - Text Modification via IMKTextInput

    private func applyResult(_ result: PaliEngine.ProcessOutput, client sender: Any?) {
        let clientObj: (any IMKTextInput)? = (sender as? IMKTextInput) ?? self.client()
        guard let client = clientObj else { return }

        if result.backspaces > 0 {
            let selRange = client.selectedRange()
            let count = Int(result.backspaces)

            if selRange.location != NSNotFound && selRange.location >= count {
                let replaceRange = NSRange(location: selRange.location - count, length: count)
                client.insertText(result.commit, replacementRange: replaceRange)
            } else {
                // Fallback for clients without selectedRange support
                for _ in 0..<count {
                    if let inputClient = client as? NSTextInputClient {
                        inputClient.doCommand(by: #selector(NSResponder.deleteBackward(_:)))
                    } else if let nsClient = client as? NSObjectProtocol, nsClient.responds(to: #selector(NSResponder.deleteBackward(_:))) {
                        _ = (nsClient as AnyObject).perform(#selector(NSResponder.deleteBackward(_:)))
                    }
                }
                if !result.commit.isEmpty {
                    client.insertText(result.commit, replacementRange: NSRange(location: NSNotFound, length: 0))
                }
            }
        } else {
            if !result.commit.isEmpty {
                client.insertText(result.commit, replacementRange: NSRange(location: NSNotFound, length: 0))
            }
        }
    }

    private func insertDirect(_ text: String, client sender: Any?) {
        let clientObj: (any IMKTextInput)? = (sender as? IMKTextInput) ?? self.client()
        guard let client = clientObj else { return }
        client.insertText(text, replacementRange: NSRange(location: NSNotFound, length: 0))
    }

    private func syncConfig() {
        let config = PaliConfig.shared
        if engine.scheme != config.scheme {
            engine.scheme = config.scheme
        }
        if engine.niggahita != config.niggahita {
            engine.niggahita = config.niggahita
        }
        if engine.extendedSet != config.extendedSet {
            engine.extendedSet = config.extendedSet
        }
    }

    // MARK: - Lifecycle Notifications

    public override func activateServer(_ sender: Any!) {
        super.activateServer(sender)
        engine.reset()
    }

    public override func deactivateServer(_ sender: Any!) {
        super.deactivateServer(sender)
        engine.reset()
    }

    public override func commitComposition(_ sender: Any!) {
        super.commitComposition(sender)
        engine.reset()
    }

    public override func cancelComposition() {
        super.cancelComposition()
        engine.reset()
    }

    // MARK: - Menu Support

    public override func menu() -> NSMenu! {
        return AppDelegate.buildMenu()
    }

    @objc public func selectScheme(_ sender: NSMenuItem) {
        (NSApp.delegate as? AppDelegate)?.selectScheme(sender)
    }

    @objc public func toggleNiggahita(_ sender: NSMenuItem) {
        (NSApp.delegate as? AppDelegate)?.toggleNiggahita(sender)
    }

    @objc public func toggleExtendedSet(_ sender: NSMenuItem) {
        (NSApp.delegate as? AppDelegate)?.toggleExtendedSet(sender)
    }

    @objc public func showAbout(_ sender: NSMenuItem) {
        (NSApp.delegate as? AppDelegate)?.showAbout(sender)
    }
}
