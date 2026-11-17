//
//  smoke_test.swift
//  PaliKey
//
//  Headless smoke test verifying PaliEngine + C FFI + EasyUnicode.
//

import Foundation

@main
struct SmokeTestRunner {
    static func assertEqual<T: Equatable>(_ actual: T, _ expected: T, _ message: String, file: StaticString = #file, line: UInt = #line) {
        if actual != expected {
            fputs("FAIL [\(file):\(line)]: \(message)\n  Expected: '\(expected)'\n  Actual:   '\(actual)'\n", stderr)
            exit(1)
        }
    }

    static func main() {
        print("==> Starting PaliKey macOS Smoke Tests...")

        // 1. Version check
        let ver = PaliEngine.version
        print("Engine version: \(ver)")
        assertEqual(ver, "0.1.0", "Engine version must match 0.1.0")

        // 2. Primary acceptance criteria: Velthuis "sammaadi.t.thi" -> "sammādiṭṭhi"
        let engine = PaliEngine(scheme: 0, niggahita: 0, extendedSet: false)
        let velthuisResult = engine.typeText("sammaadi.t.thi")
        print("Velthuis test: 'sammaadi.t.thi' -> '\(velthuisResult)'")
        assertEqual(velthuisResult, "sammādiṭṭhi", "Velthuis sammaadi.t.thi must yield sammādiṭṭhi")

        // Additional golden words in Velthuis
        engine.reset()
        assertEqual(engine.typeText("pa.ticcasamuppaada"), "paṭiccasamuppāda", "pa.ticcasamuppaada -> paṭiccasamuppāda")
        engine.reset()
        assertEqual(engine.typeText("sa\"ngha"), "saṅgha", "sa\"ngha -> saṅgha")
        engine.reset()
        assertEqual(engine.typeText("~naa.na"), "ñāṇa", "~naa.na -> ñāṇa")
        engine.reset()
        assertEqual(engine.typeText("ve.luriya"), "veḷuriya", "ve.luriya -> veḷuriya")

        // 3. Backspace undo
        print("Testing backspace undo...")
        engine.reset()
        var buf = ""
        for ch in "aa" {
            let res = engine.process(char: ch)
            for _ in 0..<res.backspaces { buf.removeLast() }
            buf.append(res.commit)
        }
        assertEqual(buf, "ā", "aa should produce ā")

        // Undo backspace: reverts ā to aa
        let undoRes = engine.typeBackspace(to: &buf)
        assertEqual(undoRes.backspaces, 1, "Undo should backspace 1 char")
        assertEqual(undoRes.commit, "aa", "Undo should commit aa")
        assertEqual(buf, "aa", "Buffer should now be aa")

        // Standard backspace 1: deletes second 'a'
        let bs1 = engine.typeBackspace(to: &buf)
        assertEqual(bs1.backspaces, 1, "Standard backspace deletes 1 char")
        assertEqual(bs1.commit, "", "Standard backspace commits empty")
        assertEqual(buf, "a", "Buffer should now be a")

        // Standard backspace 2: deletes first 'a'
        let bs2 = engine.typeBackspace(to: &buf)
        assertEqual(bs2.backspaces, 1, "Standard backspace deletes 1 char")
        assertEqual(buf, "", "Buffer should now be empty")

        // 4. PaliTelex scheme
        print("Testing PaliTelex scheme...")
        engine.scheme = 1
        engine.reset()
        let telexResult = engine.typeText("sammaaditxtxhi")
        print("PaliTelex test: 'sammaaditxtxhi' -> '\(telexResult)'")
        assertEqual(telexResult, "sammādiṭṭhi", "PaliTelex sammaaditxtxhi must yield sammādiṭṭhi")

        engine.reset()
        assertEqual(engine.typeText("sa ngw ha"), "sa ṅ ha", "ngw -> ṅ")
        engine.reset()
        assertEqual(engine.typeText("nw aa nx a"), "ñ ā ṇ a", "nw -> ñ, nx -> ṇ")
        engine.reset()
        assertEqual(engine.typeText("dhamma mx"), "dhamma ṃ", "mx -> ṃ")

        // 5. SIL scheme
        print("Testing SIL scheme...")
        engine.scheme = 2
        engine.reset()
        let silResult = engine.typeText("samm-adi.t.thi")
        print("SIL test: 'samm-adi.t.thi' -> '\(silResult)'")
        assertEqual(silResult, "sammādiṭṭhi", "SIL samm-adi.t.thi must yield sammādiṭṭhi")

        engine.reset()
        assertEqual(engine.typeText(">n"), "ṅ", ">n -> ṅ")
        engine.reset()
        assertEqual(engine.typeText("~n"), "ñ", "~n -> ñ")
        engine.reset()
        assertEqual(engine.typeText(".m"), "ṃ", ".m -> ṃ")

        // 6. Niggahīta variants
        print("Testing Niggahīta variants...")
        engine.scheme = 0
        engine.niggahita = 0
        engine.reset()
        assertEqual(engine.typeText(".m"), "ṃ", "DotBelow niggahita must produce ṃ (U+1E43)")

        engine.niggahita = 1
        engine.reset()
        assertEqual(engine.typeText(".m"), "ṁ", "DotAbove niggahita must produce ṁ (U+1E41)")

        // 7. EasyUnicode direct-insert mapping
        print("Testing EasyUnicode mapping...")
        assertEqual(EasyUnicode.map(char: "a", isShift: false, niggahita: 0), "ā", "EasyUnicode ⌥a -> ā")
        assertEqual(EasyUnicode.map(char: "a", isShift: true, niggahita: 0), "Ā", "EasyUnicode ⌥A -> Ā")
        assertEqual(EasyUnicode.map(char: "i", isShift: false, niggahita: 0), "ī", "EasyUnicode ⌥i -> ī")
        assertEqual(EasyUnicode.map(char: "u", isShift: false, niggahita: 0), "ū", "EasyUnicode ⌥u -> ū")
        assertEqual(EasyUnicode.map(char: "t", isShift: false, niggahita: 0), "ṭ", "EasyUnicode ⌥t -> ṭ")
        assertEqual(EasyUnicode.map(char: "t", isShift: true, niggahita: 0), "Ṭ", "EasyUnicode ⌥T -> Ṭ")
        assertEqual(EasyUnicode.map(char: "d", isShift: false, niggahita: 0), "ḍ", "EasyUnicode ⌥d -> ḍ")
        assertEqual(EasyUnicode.map(char: "n", isShift: false, niggahita: 0), "ṇ", "EasyUnicode ⌥n -> ṇ")
        assertEqual(EasyUnicode.map(char: "l", isShift: false, niggahita: 0), "ḷ", "EasyUnicode ⌥l -> ḷ")
        assertEqual(EasyUnicode.map(char: "g", isShift: false, niggahita: 0), "ṅ", "EasyUnicode ⌥g -> ṅ")
        assertEqual(EasyUnicode.map(char: "k", isShift: false, niggahita: 0), "ṅ", "EasyUnicode ⌥k -> ṅ")
        assertEqual(EasyUnicode.map(char: "j", isShift: false, niggahita: 0), "ñ", "EasyUnicode ⌥j -> ñ")
        assertEqual(EasyUnicode.map(char: "m", isShift: false, niggahita: 0), "ṃ", "EasyUnicode ⌥m (DotBelow) -> ṃ")
        assertEqual(EasyUnicode.map(char: "m", isShift: false, niggahita: 1), "ṁ", "EasyUnicode ⌥m (DotAbove) -> ṁ")
        assertEqual(EasyUnicode.map(char: "m", isShift: true, niggahita: 0), "Ṃ", "EasyUnicode ⌥M -> Ṃ")
        assertEqual(EasyUnicode.map(char: "r", isShift: false, niggahita: 0), "ṛ", "EasyUnicode ⌥r -> ṛ")
        assertEqual(EasyUnicode.map(char: "s", isShift: false, niggahita: 0), "ṣ", "EasyUnicode ⌥s -> ṣ")
        assertEqual(EasyUnicode.map(char: "h", isShift: false, niggahita: 0), "ḥ", "EasyUnicode ⌥h -> ḥ")
        assertEqual(EasyUnicode.map(char: "z", isShift: false, niggahita: 0), nil, "Unmapped char returns nil")

        print("==> ALL SMOKE TESTS PASSED (100% SUCCESS)!")
    }
}
