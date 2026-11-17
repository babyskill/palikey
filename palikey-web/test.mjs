/**
 * test.mjs — Integration test suite for PaliKey WebAssembly shell.
 *
 * Runs on Node.js.
 * Loads palikey.wasm via palikey.js glue, types the golden corpus across
 * all three schemes (Velthuis, PaliTelex, SIL), and asserts byte-identical output.
 *
 * Usage:
 *   node palikey-web/test.mjs
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Import CommonJS glue in ESM
import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
const PaliKey = require('./dist/palikey.js');

// Golden corpus matching palikey-engine/tests/golden.rs and DESIGN.md §7
const GOLDEN_CORPUS = [
  {
    expected: 'sammādiṭṭhi',
    velthuis: 'sammaadi.t.thi',
    palitelex: 'sammaaditxtxhi',
    sil: 'samm-adi.t.thi',
  },
  {
    expected: 'paṭiccasamuppāda',
    velthuis: 'pa.ticcasamuppaada',
    palitelex: 'patxiccasamuppaada',
    sil: 'pa.ticcasamupp-ada',
  },
  {
    expected: 'nibbāna',
    velthuis: 'nibbaana',
    palitelex: 'nibbaana',
    sil: 'nibb-ana',
  },
  {
    expected: 'saṅgha',
    velthuis: 'sa"ngha',
    palitelex: 'sangwgha',
    sil: 'sa>ngha',
  },
  {
    expected: 'dhammaṃ saraṇaṃ gacchāmi',
    velthuis: 'dhamma.m sara.na.m gacchaami',
    palitelex: 'dhammamx saranxamx gacchaami',
    sil: 'dhamma.m sara.na.m gacch-ami',
  },
  {
    expected: 'buddhaṃ',
    velthuis: 'buddha.m',
    palitelex: 'buddhamx',
    sil: 'buddha.m',
  },
  {
    expected: 'tiṭṭhati',
    velthuis: 'ti.t.thati',
    palitelex: 'titxtxhati',
    sil: 'ti.t.thati',
  },
  {
    expected: 'ñāṇa',
    velthuis: '~naa.na',
    palitelex: 'nwaanxa',
    sil: '~n-a.na',
  },
  {
    expected: 'kaṇha',
    velthuis: 'ka.nha',
    palitelex: 'kanxha',
    sil: 'ka.nha',
  },
  {
    expected: 'veḷuriya',
    velthuis: 've.luriya',
    palitelex: 'velxuriya',
    sil: 've.luriya',
  },
];

async function runTests() {
  console.log('=== PaliKey Web WASM Golden Corpus Test Suite ===\n');

  const wasmPath = path.join(__dirname, 'dist', 'palikey.wasm');
  if (!fs.existsSync(wasmPath)) {
    console.error(`WASM binary not found at ${wasmPath}. Run ./build.sh first.`);
    process.exit(1);
  }

  const wasmBytes = fs.readFileSync(wasmPath);
  const wasmInstance = await PaliKey.loadWasm(wasmBytes);
  console.log(`Loaded WebAssembly module (${wasmBytes.length} bytes)\n`);

  let passed = 0;
  let failed = 0;

  function assertEqual(actual, expected, message) {
    if (actual === expected) {
      passed++;
      console.log(`  ✓ ${message}`);
    } else {
      failed++;
      console.error(`  ✗ ${message}\n    Expected: ${JSON.stringify(expected)}\n    Actual:   ${JSON.stringify(actual)}`);
    }
  }

  // 1. Test Golden Corpus words across all 3 schemes
  console.log('Test 1: Golden Corpus Convergence (All 3 schemes)');
  const schemes = [
    { name: 'velthuis', key: 'velthuis' },
    { name: 'palitelex', key: 'palitelex' },
    { name: 'sil', key: 'sil' },
  ];

  for (const item of GOLDEN_CORPUS) {
    for (const s of schemes) {
      const engine = new PaliKey.PalikeyEngine(wasmInstance, { scheme: s.name });
      const input = item[s.key];
      const actual = engine.simulateTyping(input);
      assertEqual(
        actual,
        item.expected,
        `[${s.name.padEnd(9)}] "${input}" -> "${item.expected}"`
      );
      engine.free();
    }
  }

  // 2. Test Dhammapada verse 1 opening
  console.log('\nTest 2: Dhammapada Verse 1 Opening');
  {
    const expected = 'manopubbaṅgamā dhammā';
    const inputs = {
      velthuis: 'manopubba"ngamaa dhammaa',
      palitelex: 'manopubbangwgamaa dhammaa',
      sil: 'manopubba>ngam-a dhamm-a',
    };

    for (const [scheme, input] of Object.entries(inputs)) {
      const engine = new PaliKey.PalikeyEngine(wasmInstance, { scheme });
      const actual = engine.simulateTyping(input);
      assertEqual(actual, expected, `Dhp 1 in ${scheme}: "${input}" -> "${expected}"`);
      engine.free();
    }
  }

  // 3. Test Capital cases
  console.log('\nTest 3: Capital Cases');
  {
    // Velthuis
    const engV = new PaliKey.PalikeyEngine(wasmInstance, { scheme: 'velthuis' });
    assertEqual(engV.simulateTyping('AA'), 'Ā', 'Velthuis AA -> Ā');
    engV.reset();
    assertEqual(engV.simulateTyping('Aa'), 'Ā', 'Velthuis Aa -> Ā');
    engV.reset();
    assertEqual(engV.simulateTyping('.T'), 'Ṭ', 'Velthuis .T -> Ṭ');
    engV.reset();
    assertEqual(engV.simulateTyping('"N'), 'Ṅ', 'Velthuis "N -> Ṅ');
    engV.free();

    // PaliTelex
    const engT = new PaliKey.PalikeyEngine(wasmInstance, { scheme: 'palitelex' });
    assertEqual(engT.simulateTyping('AA'), 'Ā', 'PaliTelex AA -> Ā');
    engT.reset();
    assertEqual(engT.simulateTyping('TX'), 'Ṭ', 'PaliTelex TX -> Ṭ');
    engT.reset();
    assertEqual(engT.simulateTyping('Tx'), 'Ṭ', 'PaliTelex Tx -> Ṭ');
    engT.reset();
    assertEqual(engT.simulateTyping('NGW'), 'Ṅ', 'PaliTelex NGW -> Ṅ');
    engT.free();
  }

  // 4. Test Niggahīta variant toggle (ṃ vs ṁ)
  console.log('\nTest 4: Niggahīta Variant Toggle (ṃ vs ṁ)');
  {
    const engBelow = new PaliKey.PalikeyEngine(wasmInstance, { scheme: 'velthuis', niggahita: 'dot_below' });
    assertEqual(engBelow.simulateTyping('.m'), 'ṃ', 'Velthuis .m with dot_below -> ṃ (U+1E43)');
    engBelow.free();

    const engAbove = new PaliKey.PalikeyEngine(wasmInstance, { scheme: 'velthuis', niggahita: 'dot_above' });
    assertEqual(engAbove.simulateTyping('.m'), 'ṁ', 'Velthuis .m with dot_above -> ṁ (U+1E41)');
    engAbove.free();

    const engTelexAbove = new PaliKey.PalikeyEngine(wasmInstance, { scheme: 'palitelex', niggahita: 'dot_above' });
    assertEqual(engTelexAbove.simulateTyping('mx'), 'ṁ', 'PaliTelex mx with dot_above -> ṁ (U+1E41)');
    engTelexAbove.free();
  }

  // 5. Test Backspace Undo
  console.log('\nTest 5: Backspace Undo (Revert composed char)');
  {
    const eng = new PaliKey.PalikeyEngine(wasmInstance, { scheme: 'velthuis' });
    // Type 'a', then 'a' -> 'ā'
    eng.processChar('a');
    const r2 = eng.processChar('a');
    assertEqual(r2.backspaces, 1, 'Type "aa": backspaces = 1');
    assertEqual(r2.commit, 'ā', 'Type "aa": commit = "ā"');

    // Press backspace immediately -> reverts to "aa"
    const undo = eng.backspace();
    assertEqual(undo.backspaces, 1, 'Backspace after "ā": backspaces = 1');
    assertEqual(undo.commit, 'aa', 'Backspace after "ā": reverts to raw "aa"');

    // Press backspace again -> standard deletion
    const erase = eng.backspace();
    assertEqual(erase.backspaces, 1, 'Subsequent backspace: backspaces = 1');
    assertEqual(erase.commit, '', 'Subsequent backspace: commit = ""');
    eng.free();
  }

  // 6. Test Repeat-key escape
  console.log('\nTest 6: Repeat-key Escape (e.g. aaa -> aaa)');
  {
    const eng = new PaliKey.PalikeyEngine(wasmInstance, { scheme: 'velthuis' });
    const res = eng.simulateTyping('aaa');
    assertEqual(res, 'aaa', 'Velthuis "aaa" reverts to "aaa"');
    eng.free();

    const engT = new PaliKey.PalikeyEngine(wasmInstance, { scheme: 'palitelex' });
    const resT = engT.simulateTyping('txx');
    assertEqual(resT, 'txx', 'PaliTelex "txx" reverts to "txx"');
    engT.free();
  }

  console.log(`\n=== Results: ${passed} passed, ${failed} failed ===`);
  if (failed > 0) {
    process.exit(1);
  }
}

runTests().catch((err) => {
  console.error('Test execution failed:', err);
  process.exit(1);
});
