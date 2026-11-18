# PaliKey — Cross-platform Pāḷi Input Method: Design

Status: DESIGN v1 — 2026-11-17, by Muse for Kien. Awaiting Kien's decisions in §9.
Codename: `palikey` (repo name). Product name is an open decision (§9.2) — an iOS app
named "Pali Key" by another author already exists, so the codename must not become
the store name blindly.

## 1. Problem: fragmentation (verified 2026-11-17)

There is no single Pāḷi IME. Each platform has a different tool, a different typing
scheme, and a different abandonment risk:

| Platform | Current option | Scheme | Problem |
|---|---|---|---|
| Windows | Pali Keyboard (tipitaka.org, MSKLC) | AltGr + letter | Built with Microsoft Keyboard Layout Creator, a tool Microsoft has retired; conflicts with ~8 Word shortcuts; install folder must never move |
| Windows/macOS/Linux/mobile | Keyman + SIL Indic Roman Transliteration 1.4.2 (MIT) | `-a`→ā, `.t`→ṭ, `>n`→ṅ, `~n`→ñ | Maintained and truly cross-platform, but Keyman's site/install flow is too complex for beginners (Kien's finding, verified: even Keyman's own App Store link was dead) |
| macOS | EasyUnicode (.keylayout, Prof. Unebe) | Option + letter | Manual copy into `~/Library/Keyboard Layouts` + logout; most beginners fail the install |
| macOS (built-in) | ABC Extended | Option dead-keys | Zero install, but undiscoverable — nobody finds it without a guide |
| Android | "Pali Keyboard" (Tummosoft) | on-screen extra row | **Delisted from Google Play** (verified 404 in 3 checks); only sideload APKs remain — cannot recommend to beginners |
| Android | Gboard long-press | long-press | Works, but slow and undiscoverable per-character |
| iOS | Pali Key (Duc Manh Khong, 2026-07) | extra row | Good design, free, no Full Access; but single-author, brand-new, no reviews — continuity risk |
| Chrome | Pāli Easy Keyboard extension | — | Beta since 2021 (v1.0.4), does not work in Facebook / Google Docs |
| Linux | scattered XKB hacks (tuvok/pali etc.) | RAlt + letter | DIY config files, no package, no maintainer |

Root causes: (1) no shared engine — every tool re-implements the mapping alone;
(2) no shared typing scheme — AltGr, Option, Velthuis, SIL and long-press are all
different, so a user re-learns on every device; (3) no single download hub with
direct links; (4) single-author abandonment.

## 2. What vnkey (marixdev/vnkey) teaches us

Researched 2026-11-17: github.com/marixdev/vnkey — Vietnamese IME, 103★, GPL-3.0,
created 2026-03, 4 releases. Its architecture is the right template:

```
vnkey-engine/   Rust core + C FFI (staticlib)   <- one engine, all platforms
vnkey-windows/  Rust, WH_KEYBOARD_LL hook + SendInput, Direct2D config UI
vnkey-macos/    Obj-C, InputMethodKit (IMK)
vnkey-fcitx5/   C++ addon
vnkey-ibus/     C engine
```

Key patterns worth copying (as architecture, not code — see §3):

1. **Core-and-shell.** Engine is a pure state machine: key in → `ProcessResult
   { backspaces, commit_string }` out. Shells only do OS integration + preedit.
2. **Dual FFI.** Singleton API for the Windows hook (one context) and instance API
   (`VnKeyEngine*` per input context) for IMK/Fcitx5/IBus. Thread-safe per instance.
3. **One `config.json`** shared verbatim across platforms.
4. **Data-driven linguistic rules** (`vnlexi.rs`, charset tables) separated from
   the state machine — schemes are data, not code branches.
5. **Golden tests in the engine crate** (`cargo test`), runnable without any OS.

What vnkey does **not** have, and PaliKey needs: mobile shells (Android/iOS),
a web (WASM) shell, and any notion of multiple input schemes converging.

### License consequence (important)

vnkey is **GPL-3.0-or-later**. Forking or copying its code forces PaliKey to be
GPL-3.0 too, including the mobile apps — which also creates App Store friction
(GPL apps have a long history of store disputes). Recommendation: **clean-room
re-implementation inspired by its architecture** (architecture is not copyrightable;
no vnkey code is copied), PaliKey engine under **MIT**. This also keeps a future
integration into Cùng Tu / PaliDict apps legally trivial. See §9.1.

## 3. PaliKey architecture

```
palikey-engine/        Rust — scheme transducer state machine + C FFI (staticlib/cdylib)
  src/engine.rs        state machine (process_key / backspace / reset), vnkey-style ProcessResult
  src/schemes.rs       scheme tables as data (Velthuis, PaliTelex, SIL) — see §4
  src/chars.rs         Pāḷi character set, ṃ/ṁ variant, case mapping
  src/ffi.rs           singleton + instance C API: palikey_new/free/process/reset/set_scheme
  tests/golden.rs      golden corpus (§7) — every scheme must converge to same output
palikey-windows/       Phase 1: Rust hook app (vnkey pattern). Phase 4: TSF (see §8 note)
palikey-macos/         Swift/Obj-C IMK input method (vnkey-macos pattern, Swift rewrite)
palikey-linux-fcitx5/  C++ addon
palikey-linux-ibus/    C engine
palikey-android/       Kotlin InputMethodService + engine via JNI (Rust .so)
palikey-ios/           Swift keyboard extension + engine via XCFramework; NO Full Access
palikey-web/           WASM build of the engine for the giacngo.org/tools typing page
palikey-keyman/        Keyman keyboard package (.kmp) generated from the same scheme
                       tables — instant distribution on all Keyman platforms in Phase 1
                       while native shells are built; also the fallback forever
shared/config.json     one config schema everywhere: scheme, niggahita variant,
                       extended Sanskrit set on/off, escape behaviour
```

Design rules:

- **Engine knows nothing about any OS.** Input = key event (char or special key),
  output = `{ backspaces: u8, commit: String, preedit: String }`.
- **Schemes are data.** A scheme is a table of (sequence → char) plus rules
  (case handling, word-boundary reset, undo). Adding a scheme = adding a table +
  golden tests, zero engine changes.
- **Forgiving engine.** Within a scheme family, alias sequences are accepted
  (e.g. Velthuis `.t` and SIL `.t` coincide; `~n` and `~na` variants). The point
  of PaliKey is that habits learned from older tools keep working.
- **Undo by backspace.** Backspace immediately after a composed character reverts
  it to its raw sequence (vnkey behaviour), so a wrong conversion is one key away.
- **English escape.** Digits, and word boundaries flush the buffer; a sequence
  that would convert inside an obvious English word can be escaped with the
  scheme's escape key (Velthuis tradition: repeat/quote). Spellcheck-style
  auto-restore (vnkey's "restore ASCII if invalid") is NOT applicable to Pāḷi —
  Pāḷi has no closed syllable grammar like Vietnamese — so PaliKey uses explicit
  escape + backspace-undo instead. This is a deliberate divergence from vnkey.
- **Offline only.** No network, no analytics, no Full Access on iOS. Ever.

## 4. Character set and input schemes

Core Pāḷi set (15): ā ī ū ṅ ñ ṭ ḍ ṇ ḷ ṃ — plus capitals Ā Ī Ū Ṅ Ñ Ṭ Ḍ Ṇ Ḷ Ṃ.
Niggahīta setting: `ṃ` (U+1E43, default; PTS / SuttaCentral practice) or
`ṁ` (U+1E41; VRI tradition) — one config flag, engine-wide.
Extended set (opt-in, for Sanskrit/IAST users): ṛ ṝ ḹ ḥ ś ṣ ē ō.

Three schemes ship in v1, all converging to identical output:

### 4.1 Velthuis (scholar standard — for users coming from academia)

`aa`→ā `ii`→ī `uu`→ū `.t`→ṭ `.d`→ḍ `.n`→ṇ `.m`→ṃ/ṁ `"n`→ṅ `~n`→ñ `.l`→ḷ
Capitals: `AA`→Ā, `.T`→Ṭ, `"N`→Ṅ …

### 4.2 PaliTelex (new default for beginners — modelled on Vietnamese Telex habits)

Doubling for length, single mnemonic suffix for dots, mirroring how Vietnamese
users already type Telex (`aa`→â in Telex, here `aa`→ā):

`aa`→ā `ii`→ī `uu`→ū `ee`→ē* `oo`→ō* · `tx`→ṭ `dx`→ḍ `nx`→ṇ `mx`→ṃ `lx`→ḷ
`ngw`→ṅ … — **draft, needs Kien's decision (§9.3).** The `x` suffix is borrowed
from Telex's dấu sắc muscle memory (x = "extra mark"). Alternative candidate:
trailing `r` (`tr`→ṭ). This is the single most consequential UX choice in the
project; the engine makes it data, so it can change without code churn.
(* ē/ō only matter with the extended set on; in pure Pāḷi they pass through.)

### 4.3 SIL / Keyman-compatible (migration path from the current giacngo.org guide)

`-a`→ā `-i`→ī `-u`→ū `.t`→ṭ `.d`→ḍ `.n`→ṇ `.m`→ṃ `>n`→ṅ `~n`→ñ `.l`→ḷ

Shell-level extra (not engine): AltGr/Option dead-key layout (EasyUnicode mapping)
is provided on desktop shells for users who prefer modifier typing; it bypasses
the transducer and inserts directly, exactly like EasyUnicode did.

## 5. Platform shells — specifics

- **Windows.** Phase 1 = vnkey pattern (low-level hook + SendInput, portable .exe,
  no install) — fast to ship, known limits (some apps with anti-hook, UWP quirks).
  v1.0 target = proper TSF text service; the engine/FFI does not change.
  Signing via SignPath free OSS signing (vnkey uses it) or existing cert.
- **macOS.** IMK bundle in `~/Library/Input Methods/`, universal binary,
  notarized with the Developer ID already used for PaliDict DMG (cert A9K2C955CY).
  Installer = drag-to-folder .dmg with an alias — fixing EasyUnicode's manual
  Library-copy failure mode is a stated goal.
- **Linux.** Fcitx5 (C++) and IBus (C) addons, .deb/.rpm via CPack like vnkey.
- **Android.** Kotlin `InputMethodService`, engine as Rust `.so` via JNI
  (instance FFI). Two layouts: QWERTY + transducer (type `sammādiṭṭhi` via any
  scheme) and a dedicated Pāḷi row layout (Pali Key pattern). Play Store under
  the same account strategy as PaliDict. No `INTERNET` permission.
- **iOS.** Keyboard extension, engine as XCFramework. Extra-row layout +
  long-press. No Full Access → no network, no data collection (matches Pali Key's
  privacy posture, which users trust).
- **Web.** `wasm-bindgen` build; drop-in replacement for the current Toolster
  typing box on giacngo.org/tools, so the web tool and the installed IMEs are
  literally the same engine and scheme.
- **Keyman package.** Generated `.kmp` from the scheme tables (PaliTelex +
  Velthuis) submitted to the Keyman keyboard repo (MIT there) — gives a
  maintained presence on every platform from Phase 1, and a permanent fallback.

## 6. Distribution — fixing the "can't find / can't install" failure

- One hub: **giacngo.org/tools** (already live) becomes the download page with
  **direct file links only** (the rule Kien set 2026-11-17: no link to a vendor
  homepage; every button downloads or opens the exact store listing).
- GitHub Releases as the canonical file source; hub links point at release assets.
- Same icon, same name, same scheme cheat-sheet PDF/PNG on every platform.
- Versioning: engine version is the product version everywhere (like vnkey).

## 7. Testing

Golden corpus (engine tests, all schemes must produce byte-identical output):
`sammādiṭṭhi`, `paṭiccasamuppāda`, `nibbāna`, `saṅgha`, `dhammaṃ saraṇaṃ
gacchāmi`, `buddhaṃ`, `tiṭṭhati`, `ñāṇa`, `kaṇha`, `veḷuriya`, plus a full
Dhammapada verse (Dhp 1: `manopubbaṅgamā dhammā…`) typed in each scheme,
backspace-undo cases, capital cases, ṃ/ṁ flag cases, and English non-conversion
cases (`aaron`, `door` must stay English when escaped / at word level per rules).
Per-shell: only smoke tests (install, switch, type the corpus in Notes/Notepad).

## 8. Roadmap

- **Phase 0 (iMac, code-only):** monorepo scaffold, Rust engine with the three
  scheme tables, FFI (singleton + instance), golden tests green. No shells.
- **Phase 1:** macOS IMK shell + Windows hook shell + Keyman package + web WASM.
  Hub page links. → First public release v0.1.
- **Phase 2:** Android + iOS shells. → v0.5.
- **Phase 3:** Linux Fcitx5/IBus. → v0.8.
- **Phase 4:** Windows TSF, polish, v1.0.

Machine assignment (per Kien's 2026-11-17 rule): all code phases on **iMac**
(code-only node, via git + Tailscale); store submission, device testing and
release signing stay on MacBook / Mac mini when the time comes.

## 9. Decisions for Kien

DECIDED 2026-11-17 11:25 (Kien, via options): (1) clean-room MIT — confirmed;
(3) PaliTelex suffix `x` — confirmed; (2) product name — decide later.

DECIDED 2026-11-17 (Owner, Phase 5): Vietnamese Telex added as scheme 4
(engine id 3), clean-room from orthography spec, Android-first. Enables
bilingual Vietnamese and Pāḷi input in a single keyboard without third-party
IME dependencies.

DECIDED 2026-11-18 (Owner, Phase 6): Three-mode cycle Vietnamese → English → Pāḷi
(and back to Vietnamese) on the phone keyboard in fixed sequence with immediate
persistence. Vietnamese uses engine scheme 3 (vietelex); English operates in plain
passthrough (composition OFF); Pāḷi uses the persisted Pāḷi variant (0/1/2) with
the top diacritic row visible. Pāḷi variant selection stays a dedicated choice in
Settings, decoupling mode cycling from variant selection.

1. **License/approach:** clean-room MIT (recommended, §2) vs fork vnkey under
   GPL-3.0. → DECIDED: clean-room MIT.
2. **Product name:** codename `palikey` for the repo. Store-facing name cannot
   be "Pali Key" (taken on iOS). Candidates: `PaliType`, `SuttaKey`,
   `PāḷiBoard`. → DEFERRED by Kien: keep codename, decide name at Phase 1.
3. **PaliTelex suffix:** `x` (`tx`→ṭ) vs `r` (`tr`→ṭ) vs dot-suffix (`t.`→ṭ,
   closest to Velthuis reversed). → DECIDED: `x`.
4. **GitHub home:** new public repo `babyskill/palikey` (like PaliDict's public
   MIT repo). Default if unanswered: local repo on iMac only, push later.
   (Still open; default applies.)

## 10. Non-goals

No predictive text / dictionary in v1 (a later phase can reuse the DPD data from
PaliDict). No cloud sync of settings (config.json travels with the user). No
non-Roman scripts (Devanagari etc. are Keyman's strength; PaliKey is Roman Pāḷi).
