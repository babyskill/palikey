/**
 * PaliKey WebAssembly JS Glue
 *
 * Clean-room implementation per DESIGN.md §3 & §5.
 * Compatible with modern browsers and Node.js.
 */

(function (root, factory) {
  if (typeof define === 'function' && define.amd) {
    define([], factory);
  } else if (typeof module === 'object' && module.exports) {
    module.exports = factory();
  } else {
    root.PaliKey = factory();
  }
}(typeof self !== 'undefined' ? self : this, function () {
  'use strict';

  let wasmModule = null;
  let wasmInstance = null;

  const SCHEME_MAP = {
    velthuis: 0,
    palitelex: 1,
    telex: 1,
    sil: 2,
    keyman: 2,
  };

  const NIGGAHITA_MAP = {
    dot_below: 0,
    below: 0,
    dot_above: 1,
    above: 1,
  };

  function parseScheme(scheme) {
    if (typeof scheme === 'number') return scheme;
    const lower = String(scheme).toLowerCase();
    return SCHEME_MAP[lower] !== undefined ? SCHEME_MAP[lower] : 0;
  }

  function parseNiggahita(nig) {
    if (typeof nig === 'number') return nig;
    const lower = String(nig).toLowerCase();
    return NIGGAHITA_MAP[lower] !== undefined ? NIGGAHITA_MAP[lower] : 0;
  }

  /**
   * Loads and instantiates the WebAssembly module.
   * Accepts an ArrayBuffer, Uint8Array, file path (Node.js), Response, or URL string.
   */
  async function loadWasm(source) {
    if (wasmInstance) return wasmInstance;

    let bytes = null;

    if (source instanceof ArrayBuffer || source instanceof Uint8Array) {
      bytes = source;
    } else if (typeof source === 'string') {
      // Check if running in Node.js
      if (typeof process !== 'undefined' && process.versions && process.versions.node) {
        try {
          const fs = await import('fs');
          bytes = fs.readFileSync(source);
        } catch {
          // Fall back to fetch if available
          if (typeof fetch !== 'undefined') {
            const resp = await fetch(source);
            bytes = await resp.arrayBuffer();
          }
        }
      } else if (typeof fetch !== 'undefined') {
        const resp = await fetch(source);
        bytes = await resp.arrayBuffer();
      }
    } else if (typeof Response !== 'undefined' && source instanceof Response) {
      bytes = await source.arrayBuffer();
    }

    if (!bytes && typeof fetch !== 'undefined') {
      // Default browser fetch relative to this script or current page
      try {
        const resp = await fetch('palikey.wasm');
        bytes = await resp.arrayBuffer();
      } catch {
        // Ignored
      }
    }

    if (!bytes) {
      throw new Error('Unable to load WebAssembly binary. Please provide path, ArrayBuffer, or Response.');
    }

    const { instance, module } = await WebAssembly.instantiate(bytes, {});
    wasmModule = module;
    wasmInstance = instance;
    return instance;
  }

  class PalikeyEngine {
    constructor(wasmInst, options = {}) {
      this.wasm = wasmInst || wasmInstance;
      if (!this.wasm) {
        throw new Error('WASM instance not initialized. Call loadWasm() first.');
      }

      const scheme = parseScheme(options.scheme ?? 0);
      const niggahita = parseNiggahita(options.niggahita ?? 0);
      const extended = options.extended ? 1 : 0;

      this.ptr = this.wasm.exports.palikey_wasm_new(scheme, niggahita, extended);
      this.decoder = new TextDecoder('utf-8');
    }

    setScheme(scheme) {
      if (!this.ptr) return;
      this.wasm.exports.palikey_wasm_set_scheme(this.ptr, parseScheme(scheme));
    }

    setNiggahita(variant) {
      if (!this.ptr) return;
      this.wasm.exports.palikey_wasm_set_niggahita(this.ptr, parseNiggahita(variant));
    }

    setExtended(enabled) {
      if (!this.ptr) return;
      this.wasm.exports.palikey_wasm_set_extended(this.ptr, enabled ? 1 : 0);
    }

    reset() {
      if (!this.ptr) return;
      this.wasm.exports.palikey_wasm_reset(this.ptr);
    }

    processChar(char) {
      if (!this.ptr) throw new Error('Engine disposed');
      const code = char.codePointAt(0);
      this.wasm.exports.palikey_wasm_process_char(this.ptr, code);

      const bs = this.wasm.exports.palikey_wasm_get_backspaces(this.ptr);
      const cPtr = this.wasm.exports.palikey_wasm_get_commit_ptr(this.ptr);
      const cLen = this.wasm.exports.palikey_wasm_get_commit_len(this.ptr);
      const commit = cLen > 0
        ? this.decoder.decode(new Uint8Array(this.wasm.exports.memory.buffer, cPtr, cLen))
        : '';

      const pPtr = this.wasm.exports.palikey_wasm_get_preedit_ptr(this.ptr);
      const pLen = this.wasm.exports.palikey_wasm_get_preedit_len(this.ptr);
      const preedit = pLen > 0
        ? this.decoder.decode(new Uint8Array(this.wasm.exports.memory.buffer, pPtr, pLen))
        : '';

      return { backspaces: bs, commit, preedit };
    }

    backspace() {
      if (!this.ptr) throw new Error('Engine disposed');
      this.wasm.exports.palikey_wasm_backspace(this.ptr);

      const bs = this.wasm.exports.palikey_wasm_get_backspaces(this.ptr);
      const cPtr = this.wasm.exports.palikey_wasm_get_commit_ptr(this.ptr);
      const cLen = this.wasm.exports.palikey_wasm_get_commit_len(this.ptr);
      const commit = cLen > 0
        ? this.decoder.decode(new Uint8Array(this.wasm.exports.memory.buffer, cPtr, cLen))
        : '';

      return { backspaces: bs, commit, preedit: '' };
    }

    simulateTyping(inputStr) {
      const output = [];
      for (const char of inputStr) {
        const res = this.processChar(char);
        for (let i = 0; i < res.backspaces; i++) {
          output.pop();
        }
        if (res.commit) {
          for (const ch of res.commit) {
            output.push(ch);
          }
        }
      }
      return output.join('');
    }

    free() {
      if (this.ptr) {
        this.wasm.exports.palikey_wasm_free(this.ptr);
        this.ptr = 0;
      }
    }

    /**
     * Attaches live IME transliteration to an HTML textarea or input.
     */
    attach(element) {
      if (!element) return () => {};

      const onKeyDown = (e) => {
        // Reset engine on navigation / arrow keys / escape
        if (['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Home', 'End', 'Escape'].includes(e.key)) {
          this.reset();
          return;
        }

        // Handle Backspace undo
        if (e.key === 'Backspace' && !e.ctrlKey && !e.metaKey && !e.altKey) {
          const start = element.selectionStart;
          const end = element.selectionEnd;
          if (start === end && start > 0) {
            const res = this.backspace();
            if (res.commit.length > 0) {
              // Composition undo!
              e.preventDefault();
              const val = element.value;
              const eraseCount = res.backspaces;
              const before = val.slice(0, start - eraseCount);
              const after = val.slice(end);
              element.value = before + res.commit + after;
              const newPos = before.length + res.commit.length;
              element.setSelectionRange(newPos, newPos);
              element.dispatchEvent(new Event('input', { bubbles: true }));
            }
          } else {
            this.reset();
          }
          return;
        }

        // Handle normal character input
        if (e.key.length === 1 && !e.ctrlKey && !e.metaKey && !e.altKey) {
          e.preventDefault();
          const start = element.selectionStart;
          const end = element.selectionEnd;
          const val = element.value;

          // If there is an active selection range, delete it first
          let basePos = start;
          let before = val.slice(0, start);
          let after = val.slice(end);

          const res = this.processChar(e.key);

          if (res.backspaces > 0) {
            const deleteCount = Math.min(res.backspaces, before.length);
            before = before.slice(0, before.length - deleteCount);
          }

          element.value = before + res.commit + after;
          const newPos = before.length + res.commit.length;
          element.setSelectionRange(newPos, newPos);
          element.dispatchEvent(new Event('input', { bubbles: true }));
        }
      };

      const onClickOrFocus = () => {
        this.reset();
      };

      element.addEventListener('keydown', onKeyDown);
      element.addEventListener('click', onClickOrFocus);
      element.addEventListener('focus', onClickOrFocus);

      return () => {
        element.removeEventListener('keydown', onKeyDown);
        element.removeEventListener('click', onClickOrFocus);
        element.removeEventListener('focus', onClickOrFocus);
      };
    }
  }

  return {
    loadWasm,
    PalikeyEngine,
    SCHEME_MAP,
    NIGGAHITA_MAP,
  };
}));
