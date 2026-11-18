#!/usr/bin/env python3
"""
generate_demo_html.py — Generate self-contained single-file dist/index.html demo.

Embeds palikey.wasm (base64 fallback) and schemes.json so the demo works
both over HTTP/HTTPS (giacngo.org/tools) and offline via file:// in any browser.
"""

import base64
import json
from pathlib import Path


def main():
    web_dir = Path(__file__).resolve().parent
    workspace = web_dir.parent

    wasm_path = web_dir / "dist" / "palikey.wasm"
    schemes_path = workspace / "shared" / "schemes.json"
    glue_path = web_dir / "dist" / "palikey.js"
    output_html = web_dir / "dist" / "index.html"

    with open(wasm_path, "rb") as f:
        wasm_b64 = base64.b64encode(f.read()).decode("ascii")

    with open(schemes_path, "r", encoding="utf-8") as f:
        schemes_json = f.read()

    with open(glue_path, "r", encoding="utf-8") as f:
        glue_js = f.read()

    html = f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Pāḷi Keyboard — Pāḷi Input Method Demo (Web WASM)</title>
  <style>
    :root {{
      --bg: #0f141c;
      --surface: #18202c;
      --surface-border: #283548;
      --primary: #d97706;
      --primary-hover: #b45309;
      --text: #f1f5f9;
      --text-muted: #94a3b8;
      --accent: #38bdf8;
      --card-bg: #1e293b;
      --font-mono: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
      --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    }}

    * {{
      box-sizing: border-box;
      margin: 0;
      padding: 0;
    }}

    body {{
      background: var(--bg);
      color: var(--text);
      font-family: var(--font-sans);
      line-height: 1.5;
      padding: 24px 16px;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
    }}

    .container {{
      max-width: 900px;
      width: 100%;
      display: flex;
      flex-direction: column;
      gap: 20px;
    }}

    header {{
      text-align: center;
      padding: 12px 0;
    }}

    header h1 {{
      font-size: 2rem;
      font-weight: 700;
      letter-spacing: -0.02em;
      color: #fbbf24;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 10px;
    }}

    header p {{
      color: var(--text-muted);
      margin-top: 6px;
      font-size: 0.95rem;
    }}

    .card {{
      background: var(--surface);
      border: 1px solid var(--surface-border);
      border-radius: 12px;
      padding: 20px;
      box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25);
    }}

    .toolbar {{
      display: flex;
      flex-wrap: wrap;
      align-items: center;
      justify-content: space-between;
      gap: 14px;
      margin-bottom: 16px;
    }}

    .control-group {{
      display: flex;
      align-items: center;
      gap: 8px;
    }}

    label.control-label {{
      font-size: 0.85rem;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      color: var(--text-muted);
    }}

    select, button {{
      background: var(--card-bg);
      color: var(--text);
      border: 1px solid var(--surface-border);
      border-radius: 8px;
      padding: 8px 12px;
      font-size: 0.9rem;
      font-family: inherit;
      cursor: pointer;
      outline: none;
      transition: all 0.15s ease;
    }}

    select:focus, button:focus {{
      border-color: var(--primary);
    }}

    button.btn-primary {{
      background: var(--primary);
      color: #0f141c;
      font-weight: 600;
      border: none;
    }}

    button.btn-primary:hover {{
      background: var(--primary-hover);
    }}

    .segmented-control {{
      display: inline-flex;
      background: var(--card-bg);
      border: 1px solid var(--surface-border);
      border-radius: 8px;
      padding: 2px;
      gap: 2px;
    }}

    .segmented-control button {{
      border: none;
      background: transparent;
      padding: 6px 12px;
      font-size: 0.85rem;
      border-radius: 6px;
    }}

    .segmented-control button.active {{
      background: var(--primary);
      color: #0f141c;
      font-weight: 600;
    }}

    .checkbox-label {{
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 0.85rem;
      color: var(--text-muted);
      cursor: pointer;
      user-select: none;
    }}

    .checkbox-label input {{
      cursor: pointer;
      accent-color: var(--primary);
    }}

    textarea#editor {{
      width: 100%;
      height: 180px;
      background: var(--card-bg);
      border: 1px solid var(--surface-border);
      border-radius: 8px;
      color: var(--text);
      font-size: 1.15rem;
      line-height: 1.6;
      padding: 14px;
      font-family: var(--font-sans);
      resize: vertical;
      outline: none;
      transition: border-color 0.2s;
    }}

    textarea#editor:focus {{
      border-color: var(--primary);
      box-shadow: 0 0 0 2px rgba(217, 119, 6, 0.2);
    }}

    .editor-footer {{
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-top: 10px;
      font-size: 0.85rem;
      color: var(--text-muted);
    }}

    .stats {{
      display: flex;
      gap: 16px;
    }}

    .cheat-sheet-section {{
      margin-top: 10px;
    }}

    .cheat-sheet-header {{
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 12px;
    }}

    .cheat-sheet-header h3 {{
      font-size: 1rem;
      font-weight: 600;
      color: #fbbf24;
    }}

    .scheme-desc {{
      font-size: 0.85rem;
      color: var(--text-muted);
      font-style: italic;
    }}

    .grid {{
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
      gap: 8px;
    }}

    .key-badge {{
      background: var(--card-bg);
      border: 1px solid var(--surface-border);
      border-radius: 6px;
      padding: 6px 10px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      font-size: 0.85rem;
      cursor: pointer;
      transition: background 0.1s, border-color 0.1s;
    }}

    .key-badge:hover {{
      background: rgba(217, 119, 6, 0.15);
      border-color: var(--primary);
    }}

    .key-badge kbd {{
      background: rgba(255, 255, 255, 0.08);
      border-radius: 4px;
      padding: 2px 6px;
      font-family: var(--font-mono);
      font-size: 0.8rem;
      color: #38bdf8;
    }}

    .key-badge span.char {{
      font-size: 1rem;
      font-weight: 600;
      color: #fbbf24;
    }}

    .status-badge {{
      display: inline-flex;
      align-items: center;
      gap: 6px;
      font-size: 0.8rem;
      padding: 4px 8px;
      border-radius: 9999px;
      background: rgba(16, 185, 129, 0.15);
      color: #34d399;
      border: 1px solid rgba(16, 185, 129, 0.3);
    }}

    .status-badge.loading {{
      background: rgba(217, 119, 6, 0.15);
      color: #fbbf24;
      border-color: rgba(217, 119, 6, 0.3);
    }}

    footer {{
      text-align: center;
      font-size: 0.85rem;
      color: var(--text-muted);
      margin-top: 24px;
    }}

    footer a {{
      color: var(--accent);
      text-decoration: none;
    }}

    footer a:hover {{
      text-decoration: underline;
    }}
  </style>
</head>
<body>
  <div class="container">
    <header>
      <h1>☸ Pāḷi Keyboard — Web WASM Shell</h1>
      <p>Clean-room cross-platform Pāḷi input method engine running in WebAssembly.</p>
    </header>

    <div class="card">
      <div class="toolbar">
        <div class="control-group">
          <label class="control-label" for="scheme-select">Scheme:</label>
          <select id="scheme-select">
            <option value="velthuis">Velthuis (aa, .t, "n, ~n, .m)</option>
            <option value="palitelex" selected>PaliTelex (aa, tx, ngw, nw, mx)</option>
            <option value="sil">SIL / Keyman (-a, .t, >n, ~n, .m)</option>
          </select>
        </div>

        <div class="control-group">
          <label class="control-label">Niggahīta:</label>
          <div class="segmented-control" id="niggahita-control">
            <button type="button" class="active" data-val="dot_below" title="ṃ (U+1E43) - PTS & SuttaCentral">ṃ (below)</button>
            <button type="button" data-val="dot_above" title="ṁ (U+1E41) - VRI tradition">ṁ (above)</button>
          </div>
        </div>

        <div class="control-group">
          <label class="checkbox-label" title="Enable Sanskrit/IAST characters (ṛ, ṝ, ḹ, ḥ, ś, ṣ, ē, ō)">
            <input type="checkbox" id="extended-toggle">
            Extended Sanskrit
          </label>
        </div>

        <div class="control-group" style="margin-left: auto;">
          <span class="status-badge loading" id="engine-status">Loading WASM...</span>
        </div>
      </div>

      <textarea id="editor" placeholder="Start typing Pāḷi here... Try typing: sammaaditxthi (PaliTelex) or sammaadi.t.thi (Velthuis)"></textarea>

      <div class="editor-footer">
        <div class="stats">
          <span id="char-count">0 characters</span>
          <span id="word-count">0 words</span>
        </div>
        <div style="display: flex; gap: 8px;">
          <button type="button" id="btn-copy">📋 Copy</button>
          <button type="button" id="btn-clear">🧹 Clear</button>
        </div>
      </div>
    </div>

    <!-- Cheat Sheet Table -->
    <div class="card cheat-sheet-section">
      <div class="cheat-sheet-header">
        <h3>Transliteration Cheat Sheet</h3>
        <span class="scheme-desc" id="scheme-desc">PaliTelex scheme</span>
      </div>
      <div class="grid" id="cheat-sheet-grid">
        <!-- Rendered dynamically from shared/schemes.json -->
      </div>
    </div>

    <footer>
      <p>Pāḷi Keyboard is free and open-source under MIT License · Engine compiled to WebAssembly (41 KB)</p>
      <p>Zero network requests · Fully offline capable</p>
    </footer>
  </div>

  <!-- Embedded Schemes JSON -->
  <script type="application/json" id="schemes-json-data">
{schemes_json}
  </script>

  <!-- Embedded WASM Fallback (Base64) -->
  <script>
    window.__PALIKEY_WASM_B64 = "{wasm_b64}";
  </script>

  <!-- JS Glue -->
  <script>
{glue_js}
  </script>

  <!-- Application Logic -->
  <script>
    (async function() {{
      const statusBadge = document.getElementById('engine-status');
      const editor = document.getElementById('editor');
      const schemeSelect = document.getElementById('scheme-select');
      const niggahitaBtns = document.querySelectorAll('#niggahita-control button');
      const extendedToggle = document.getElementById('extended-toggle');
      const charCount = document.getElementById('char-count');
      const wordCount = document.getElementById('word-count');
      const btnCopy = document.getElementById('btn-copy');
      const btnClear = document.getElementById('btn-clear');
      const cheatSheetGrid = document.getElementById('cheat-sheet-grid');
      const schemeDesc = document.getElementById('scheme-desc');

      // 1. Load schemes data
      let schemesData = null;
      try {{
        const rawJson = document.getElementById('schemes-json-data').textContent;
        schemesData = JSON.parse(rawJson);
      }} catch (e) {{
        console.error('Failed to parse embedded schemes JSON:', e);
      }}

      // 2. Load WASM binary (try fetch, fallback to base64)
      let wasmInstance = null;
      try {{
        try {{
          wasmInstance = await PaliKey.loadWasm('palikey.wasm');
        }} catch (fetchErr) {{
          console.warn('Fetch palikey.wasm failed, using base64 fallback:', fetchErr);
          const binaryStr = atob(window.__PALIKEY_WASM_B64);
          const bytes = new Uint8Array(binaryStr.length);
          for (let i = 0; i < binaryStr.length; i++) {{
            bytes[i] = binaryStr.charCodeAt(i);
          }}
          wasmInstance = await PaliKey.loadWasm(bytes.buffer);
        }}

        statusBadge.textContent = 'WASM Ready';
        statusBadge.classList.remove('loading');
      }} catch (err) {{
        console.error('Fatal: Failed to initialize WebAssembly:', err);
        statusBadge.textContent = 'WASM Failed';
        statusBadge.style.color = '#ef4444';
        return;
      }}

      // 3. Initialize Engine
      let currentNiggahita = 'dot_below';
      const engine = new PaliKey.PalikeyEngine(wasmInstance, {{
        scheme: schemeSelect.value,
        niggahita: currentNiggahita,
        extended: extendedToggle.checked,
      }});

      // 4. Attach Live IME to Textarea
      engine.attach(editor);

      // 5. Update Cheat Sheet
      function renderCheatSheet() {{
        if (!schemesData || !schemesData.schemes) return;
        const schemeKey = schemeSelect.value;
        const scheme = schemesData.schemes[schemeKey];
        if (!scheme) return;

        schemeDesc.textContent = scheme.description;
        cheatSheetGrid.innerHTML = '';

        const isExtended = extendedToggle.checked;
        const mappings = scheme.mappings.filter(m => isExtended || !m.is_extended);

        // Deduplicate lowercase mappings for cheat-sheet view
        const displayed = [];
        const seen = new Set();
        for (const m of mappings) {{
          // Filter out upper-case variants from quick-table for clarity
          if (m.sequence === m.sequence.toUpperCase() && m.sequence !== m.sequence.toLowerCase()) {{
            continue;
          }}
          const key = m.sequence + '->' + (m.char);
          if (!seen.has(key)) {{
            seen.add(key);
            displayed.push(m);
          }}
        }}

        for (const rule of displayed) {{
          const item = document.createElement('div');
          item.className = 'key-badge';
          const displayChar = (rule.target_type === 'niggahita' && currentNiggahita === 'dot_above')
            ? (rule.char_dot_above || rule.char)
            : rule.char;

          item.innerHTML = `<kbd>${{rule.sequence}}</kbd><span class="char">${{displayChar}}</span>`;
          item.title = `Click to type ${{displayChar}} (${{rule.sequence}})`;

          item.addEventListener('click', () => {{
            editor.focus();
            const start = editor.selectionStart;
            const end = editor.selectionEnd;
            const val = editor.value;
            editor.value = val.slice(0, start) + displayChar + val.slice(end);
            const pos = start + displayChar.length;
            editor.setSelectionRange(pos, pos);
            engine.reset();
            updateStats();
          }});

          cheatSheetGrid.appendChild(item);
        }}
      }}

      renderCheatSheet();

      // 6. Event listeners
      schemeSelect.addEventListener('change', () => {{
        engine.setScheme(schemeSelect.value);
        engine.reset();
        renderCheatSheet();
      }});

      niggahitaBtns.forEach(btn => {{
        btn.addEventListener('click', () => {{
          niggahitaBtns.forEach(b => b.classList.remove('active'));
          btn.classList.add('active');
          currentNiggahita = btn.dataset.val;
          engine.setNiggahita(currentNiggahita);
          engine.reset();
          renderCheatSheet();
        }});
      }});

      extendedToggle.addEventListener('change', () => {{
        engine.setExtended(extendedToggle.checked);
        engine.reset();
        renderCheatSheet();
      }});

      function updateStats() {{
        const text = editor.value;
        charCount.textContent = `${{text.length}} characters`;
        const words = text.trim() ? text.trim().split(/\\s+/).length : 0;
        wordCount.textContent = `${{words}} words`;
      }}

      editor.addEventListener('input', updateStats);

      btnClear.addEventListener('click', () => {{
        editor.value = '';
        editor.focus();
        engine.reset();
        updateStats();
      }});

      btnCopy.addEventListener('click', async () => {{
        if (!editor.value) return;
        try {{
          await navigator.clipboard.writeText(editor.value);
          const oldText = btnCopy.textContent;
          btnCopy.textContent = '✓ Copied!';
          setTimeout(() => {{ btnCopy.textContent = oldText; }}, 1500);
        }} catch (e) {{
          editor.select();
          document.execCommand('copy');
        }}
      }});
    }})();
  </script>
</body>
</html>
"""

    with open(output_html, "w", encoding="utf-8") as f:
        f.write(html)

    print(f"Generated {output_html} ({len(html)} bytes)")


if __name__ == "__main__":
    main()
