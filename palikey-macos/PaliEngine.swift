//
//  PaliEngine.swift
//  PaliKey
//
//  Clean-room Swift wrapper around the Rust palikey-engine C FFI.
//  Compatible with both PaliKeyInputController and the headless test harness.
//

import Foundation

/// Swift wrapper for the PaliKey C engine instance.
public final class PaliEngine {
    private var handle: OpaquePointer?

    /// Active transliteration scheme ID (0: Velthuis, 1: PaliTelex, 2: SIL)
    public var scheme: UInt32 {
        didSet {
            if let handle = handle {
                _ = palikey_set_scheme(handle, scheme)
            }
        }
    }

    /// Niggahīta orthographic variant (0: DotBelow ṃ, 1: DotAbove ṁ)
    public var niggahita: UInt32 {
        didSet {
            recreateEngine()
        }
    }

    /// Extended Sanskrit / IAST character support
    public var extendedSet: Bool {
        didSet {
            recreateEngine()
        }
    }

    /// Result returned after processing a keystroke or backspace
    public struct ProcessOutput: Equatable {
        public let backspaces: UInt8
        public let commit: String
        public let preedit: String

        public init(backspaces: UInt8, commit: String, preedit: String = "") {
            self.backspaces = backspaces
            self.commit = commit
            self.preedit = preedit
        }
    }

    /// Initializes a new independent engine instance.
    public init(scheme: UInt32 = 0, niggahita: UInt32 = 0, extendedSet: Bool = false) {
        self.scheme = scheme
        self.niggahita = niggahita
        self.extendedSet = extendedSet
        self.handle = palikey_new(scheme, niggahita, extendedSet)
    }

    deinit {
        if let handle = handle {
            palikey_free(handle)
            self.handle = nil
        }
    }

    private func recreateEngine() {
        if let oldHandle = handle {
            palikey_free(oldHandle)
        }
        self.handle = palikey_new(scheme, niggahita, extendedSet)
    }

    /// Processes a single typed character.
    public func process(char: Character) -> ProcessOutput {
        guard let handle = handle, let scalar = char.unicodeScalars.first else {
            return ProcessOutput(backspaces: 0, commit: String(char))
        }

        var cResult = PalikeyProcessResult()
        let ok = palikey_process_instance(handle, scalar.value, false, &cResult)
        if ok == 1 {
            return convertResult(cResult)
        }
        return ProcessOutput(backspaces: 0, commit: String(char))
    }

    /// Processes a Backspace key event.
    public func processBackspace() -> ProcessOutput {
        guard let handle = handle else {
            return ProcessOutput(backspaces: 1, commit: "")
        }

        var cResult = PalikeyProcessResult()
        let ok = palikey_process_instance(handle, 0, true, &cResult)
        if ok == 1 {
            return convertResult(cResult)
        }
        return ProcessOutput(backspaces: 1, commit: "")
    }

    /// Resets the engine composition state and undo history.
    public func reset() {
        if let handle = handle {
            palikey_reset_instance(handle)
        }
    }

    /// Returns the engine library version string.
    public static var version: String {
        guard let ptr = palikey_version() else { return "unknown" }
        return String(cString: ptr)
    }

    /// Convenience helper for testing and batch conversion:
    /// Simulates typing a string and applying each ProcessResult to produce output.
    public func typeText(_ text: String) -> String {
        var output = ""
        for ch in text {
            let res = process(char: ch)
            apply(res, to: &output)
        }
        return output
    }

    /// Simulates pressing backspace on an accumulator string.
    public func typeBackspace(to output: inout String) -> ProcessOutput {
        let res = processBackspace()
        apply(res, to: &output)
        return res
    }

    private func apply(_ res: ProcessOutput, to output: inout String) {
        for _ in 0..<res.backspaces {
            if !output.isEmpty {
                output.removeLast()
            }
        }
        output.append(res.commit)
    }

    private func convertResult(_ cResult: PalikeyProcessResult) -> ProcessOutput {
        var mutableResult = cResult
        let commitStr = withUnsafeBytes(of: &mutableResult.commit) { raw -> String in
            guard let base = raw.baseAddress?.assumingMemoryBound(to: CChar.self) else { return "" }
            return String(cString: base)
        }
        let preeditStr = withUnsafeBytes(of: &mutableResult.preedit) { raw -> String in
            guard let base = raw.baseAddress?.assumingMemoryBound(to: CChar.self) else { return "" }
            return String(cString: base)
        }
        return ProcessOutput(backspaces: cResult.backspaces, commit: commitStr, preedit: preeditStr)
    }
}
