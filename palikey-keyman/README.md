# PaliKey — Keyman Keyboard Package

Cross-platform Pāḷi input method keyboard for Keyman, supporting both **PaliTelex** and **Velthuis** transliteration conventions.

## Overview

- **Source:** `palikey.kmn`
- **Project file:** `palikey.kpj`
- **Code generator:** `generate_kmn.py`
- **License:** MIT License

The keyboard allows users to type Romanized Pāḷi diacritics using either:
1. **PaliTelex** (Vietnamese Telex muscle memory): `aa`→ā, `ii`→ī, `uu`→ū, `tx`→ṭ, `dx`→ḍ, `nx`→ṇ, `mx`→ṃ, `lx`→ḷ, `ngw`→ṅ, `nw`→ñ.
2. **Velthuis** (academic standard): `aa`→ā, `ii`→ī, `uu`→ū, `.t`→ṭ, `.d`→ḍ, `.n`→ṇ, `.m`→ṃ, `.l`→ḷ, `"n`→ṅ, `~n`→ñ.

Both conventions are unified in `palikey.kmn` with zero conflict because PaliTelex relies on suffix triggers while Velthuis relies on punctuation prefixes.

## Source Generation (Zero-drift Guarantee)

`palikey.kmn` is automatically generated from `shared/schemes.json` (which is in turn exported directly from `palikey-engine/src/schemes.rs`).

To regenerate `palikey.kmn`:
```bash
python3 generate_kmn.py
```
This ensures that Keyman stores and rules can never drift out of sync with the core Rust engine.

## Building with kmcomp

Keyman keyboards are compiled using `kmcomp` (the Keyman command-line compiler bundled with Keyman Developer):

```bash
# Compile keyboard source to target binaries (.kmx for desktop, .js for web/mobile):
kmcomp palikey.kmn

# Build the complete distributable package (.kmp):
# Note: Requires a .kps package source and Keyman Developer.
# .kmp package builds are produced on environments where Keyman Developer is installed
# (Windows, or via Linux/Docker/Wine).
kmcomp palikey.kps -s
```

### Build Environments Note
Keyman Developer is natively supported on Windows, with command-line compilation (`kmcomp`) available on Linux via Wine or official Docker images (`keyman/developer`). The `.kmn` source and `.kpj` project committed here are fully standard and ready for compilation in CI/CD or packaging environments.
