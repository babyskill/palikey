# palikey-engine

Clean-room, cross-platform Pāḷi input method engine written in Rust.

`palikey-engine` is the shared linguistic and transliteration core of the PaliKey project. It compiles to a Rust library (`lib`), C static library (`staticlib`), and C dynamic library (`cdylib`), enabling zero-duplication integration across Windows, macOS, Linux, Android, iOS, and Web (WASM).

## Architecture & How the State Machine Works

The engine is designed as an OS-agnostic, pure state machine inspired by the core-and-shell pattern (DESIGN.md §3).

```
   Key Event (Char / Backspace / Reset)
                   │
                   ▼
       ┌────────────────────────┐
       │   palikey-engine       │
       │   transducer state     │
       └────────────────────────┘
                   │
                   ▼
  ProcessResult { backspaces, commit, preedit }
```

### Input & Output
- **Input:** `KeyAction::Char(char)`, `KeyAction::Backspace`, or `KeyAction::Reset`.
- **Output:** `ProcessResult { backspaces: u8, commit: String, preedit: String }`.
  - `backspaces`: Number of backspace delete operations the host platform should send before committing.
  - `commit`: UTF-8 string to insert into the target application.
  - `preedit`: Active composition preview (for preedit-capable shells like macOS IMK or Linux IBus/Fcitx5).

### Key Behaviors
1. **Sequence Completion:** When the tail of the buffer matches an active transliteration rule (e.g. `aa` → `ā`, `.t` → `ṭ`, `ngw` → `ṅ`), the engine emits the required backspaces to erase previously output raw characters and commits the diacritic character.
2. **Backspace-after-compose Undo (One Level):** Pressing Backspace immediately after a composed character reverts it to the raw typed sequence (e.g. `ṭ` reverts to `.t` in Velthuis or `tx` in PaliTelex). A subsequent Backspace deletes the last character of the raw sequence.
3. **Word-Boundary and Digit Flushes:** Whitespace, boundary punctuation, and digits (`0`–`9`) immediately flush the transducer buffer, preventing cross-word or cross-number sequence composition.
4. **Repeat-Key Escape:** In Velthuis and Telex traditions, typing the trigger key a third time (e.g. `aa` → `ā`, then `a`) reverts and emits the raw expanded sequence `aaa`.
5. **Escape Prefix:** A configurable escape prefix (default `\`) allows typing literal sequences without triggering transliteration (e.g. `\aa` → `aa`).

## Supported Schemes

| Scheme | Identifier | Examples | Description |
|---|---|---|---|
| **Velthuis** | `velthuis` | `aa`→ā, `.t`→ṭ, `"n`→ṅ, `~n`→ñ, `.m`→ṃ | Academic standard (DESIGN.md §4.1) |
| **PaliTelex** | `palitelex` | `aa`→ā, `tx`→ṭ, `ngw`→ṅ, `nw`→ñ, `mx`→ṃ | Vietnamese Telex muscle memory (DESIGN.md §4.2) |
| **SIL** | `sil` | `-a`→ā, `.t`→ṭ, `>n`→ṅ, `~n`→ñ, `.m`→ṃ | Keyman Indic Roman transliteration (DESIGN.md §4.3) |

## How to Add a Scheme

Schemes in `palikey-engine` are **100% data**. Adding a scheme requires **zero engine changes**:

1. Open `src/schemes.rs`.
2. Define a static slice of `MappingRule`:
   ```rust
   static MY_SCHEME_MAPPINGS: &[MappingRule] = &[
       MappingRule { sequence: "aa", target: TargetChar::Fixed(CHAR_A_MACRON_LOWER), is_extended: false },
       MappingRule { sequence: ".t", target: TargetChar::Fixed(CHAR_T_DOT_BELOW_LOWER), is_extended: false },
       // ...
   ];
   ```
3. Instantiate a `SchemeDefinition`:
   ```rust
   pub static SCHEME_MY_SCHEME: SchemeDefinition = SchemeDefinition {
       id: SchemeId::MyScheme,
       name: "myscheme",
       display_name: "My Scheme",
       description: "Custom mapping rules",
       mappings: MY_SCHEME_MAPPINGS,
   };
   ```
4. Register the scheme in `get_scheme()`, `find_scheme_by_name()`, and `all_schemes()`.
5. Add golden tests in `tests/golden.rs`.

## FFI Surface Summary

The C API (`include/palikey.h`) provides both singleton and instance-based workflows:

### Singleton API (Windows low-level hook)
- `int32_t palikey_setup(uint32_t scheme_id, uint32_t niggahita, bool extended_set);`
- `int32_t palikey_process(uint32_t key_char, bool is_backspace, PalikeyProcessResult *out_result);`
- `void palikey_reset(void);`

### Instance API (macOS IMK, Linux Fcitx5/IBus, mobile JNI)
- `PalikeyEngine *palikey_new(uint32_t scheme_id, uint32_t niggahita, bool extended_set);`
- `void palikey_free(PalikeyEngine *engine);`
- `int32_t palikey_process_instance(PalikeyEngine *engine, uint32_t key_char, bool is_backspace, PalikeyProcessResult *out_result);`
- `int32_t palikey_set_scheme(PalikeyEngine *engine, uint32_t scheme_id);`
- `void palikey_reset_instance(PalikeyEngine *engine);`

### Version & Metadata
- `const char *palikey_version(void);`

## Building and Testing

```bash
# Run all unit and golden corpus tests
cargo test

# Build release staticlib and cdylib
cargo build --release
```

Static library output: `target/release/libpalikey_engine.a`
C Header: `include/palikey.h`
