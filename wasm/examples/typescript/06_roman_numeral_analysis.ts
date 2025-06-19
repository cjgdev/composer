#!/usr/bin/env ts-node
/**
 * Roman Numeral Analysis
 *
 * This example demonstrates advanced Roman numeral analysis capabilities:
 * - Converting chords to Roman numeral notation
 * - Analyzing chord functions in different keys
 * - Understanding harmonic progressions
 * - Cross-scale analysis and modulation detection
 *
 * Based on the Composer specification: chord-analysis-lookup.spec
 */

import * as composer from "../../composer_wasm.js";

/**
 * Initialize the WASM module
 */
async function initializeWasm(): Promise<void> {
  console.log(`Initializing Composer WASM v${composer.getVersion()}`);
}

/**
 * Demonstrate basic Roman numeral conversion
 */
function demonstrateBasicRomanNumerals(): void {
  console.log("=== Basic Roman Numeral Analysis ===");

  const majorScale = composer.WasmScaleFingerprint.major();
  const minorScale = composer.WasmScaleFingerprint.minor();

  // Test chords in major key
  const majorChords = [
    { chord: new composer.WasmChord(1, 5), name: "C major" },
    { chord: new composer.WasmChord(2, 5), name: "D major" },
    { chord: new composer.WasmChord(3, 5), name: "E major" },
    { chord: new composer.WasmChord(4, 5), name: "F major" },
    { chord: new composer.WasmChord(5, 5), name: "G major" },
    { chord: new composer.WasmChord(6, 5), name: "A major" },
    { chord: new composer.WasmChord(7, 5), name: "B major" },
  ];

  console.log("Major scale chord analysis (C major key):");
  majorChords.forEach(({ chord, name }) => {
    try {
      const roman = composer.getRomanNumeral(chord, majorScale);
      const scaleDegrees = composer.getStableScaleDegrees(chord, majorScale);
      const complexity = composer.getChordComplexity(chord, "major");
      console.log(
        `  ${name.padEnd(10)}: ${roman.padEnd(8)} - degrees: [${scaleDegrees.join(", ")}], complexity: ${complexity}`
      );
    } catch (error) {
      console.log(`  ${name.padEnd(10)}: Error - ${error}`);
    }
    chord.free();
  });

  console.log();
  console.log("Same chords in C minor key:");
  majorChords.forEach(({ chord, name }) => {
    const newChord = new composer.WasmChord(chord.root, chord.chordType);
    try {
      const roman = composer.getRomanNumeral(newChord, minorScale);
      const scaleDegrees = composer.getStableScaleDegrees(newChord, minorScale);
      console.log(
        `  ${name.padEnd(10)}: ${roman.padEnd(8)} - degrees: [${scaleDegrees.join(", ")}]`
      );
    } catch (error) {
      console.log(`  ${name.padEnd(10)}: Error - ${error}`);
    }
    newChord.free();
  });

  console.log();
  majorScale.free();
  minorScale.free();
}

/**
 * Demonstrate seventh chord Roman numeral analysis
 */
function demonstrateSeventhChordAnalysis(): void {
  console.log("=== Seventh Chord Roman Numeral Analysis ===");

  const majorScale = composer.WasmScaleFingerprint.major();

  // Diatonic seventh chords in major key
  const seventhChords = [
    { root: 1, type: 7, name: "Cmaj7" },
    { root: 2, type: 7, name: "Dm7" },
    { root: 3, type: 7, name: "Em7" },
    { root: 4, type: 7, name: "Fmaj7" },
    { root: 5, type: 7, name: "G7" },
    { root: 6, type: 7, name: "Am7" },
    { root: 7, type: 7, name: "Bm7b5" },
  ];

  console.log("Diatonic seventh chords in C major:");
  seventhChords.forEach(({ root, type, name }) => {
    try {
      const chord = new composer.WasmChord(root, type);
      const roman = composer.getRomanNumeral(chord, majorScale);
      const scaleDegrees = composer.getStableScaleDegrees(chord, majorScale);
      const complexity = composer.getChordComplexity(chord, "major");

      console.log(
        `  ${name.padEnd(8)}: ${roman.padEnd(10)} - degrees: [${scaleDegrees.join(", ")}], complexity: ${complexity}`
      );
      chord.free();
    } catch (error) {
      console.log(`  ${name.padEnd(8)}: Error - ${error}`);
    }
  });

  console.log();
  majorScale.free();
}

/**
 * Demonstrate common progression analysis
 */
function demonstrateProgressionAnalysis(): void {
  console.log("=== Common Progression Analysis ===");

  const majorScale = composer.WasmScaleFingerprint.major();

  const progressions = [
    {
      name: "I-V-vi-IV (Pop)",
      chords: [
        { root: 1, type: 5 }, // I
        { root: 5, type: 5 }, // V
        { root: 6, type: 5 }, // vi
        { root: 4, type: 5 }, // IV
      ],
    },
    {
      name: "ii-V-I (Jazz)",
      chords: [
        { root: 2, type: 7 }, // ii7
        { root: 5, type: 7 }, // V7
        { root: 1, type: 5 }, // I
      ],
    },
    {
      name: "I-vi-ii-V (Circle)",
      chords: [
        { root: 1, type: 5 }, // I
        { root: 6, type: 5 }, // vi
        { root: 2, type: 7 }, // ii7
        { root: 5, type: 7 }, // V7
      ],
    },
    {
      name: "vi-IV-I-V (Ballad)",
      chords: [
        { root: 6, type: 5 }, // vi
        { root: 4, type: 5 }, // IV
        { root: 1, type: 5 }, // I
        { root: 5, type: 5 }, // V
      ],
    },
  ];

  progressions.forEach(({ name, chords }) => {
    console.log(`${name}:`);

    const romanSequence: string[] = [];
    let totalComplexity = 0;

    chords.forEach(({ root, type }, i) => {
      try {
        const chord = new composer.WasmChord(root, type);
        const roman = composer.getRomanNumeral(chord, majorScale);
        const scaleDegrees = composer.getStableScaleDegrees(chord, majorScale);
        const complexity = composer.getChordComplexity(chord, "major");

        romanSequence.push(roman);
        totalComplexity += complexity;

        console.log(
          `  ${i + 1}. ${chord.toString().padEnd(6)} -> ${roman.padEnd(8)} (degrees: [${scaleDegrees.join(", ")}])`
        );
        chord.free();
      } catch (error) {
        console.log(`  ${i + 1}. Error: ${error}`);
      }
    });

    console.log(`  Roman sequence: ${romanSequence.join(" - ")}`);
    console.log(`  Average complexity: ${(totalComplexity / chords.length).toFixed(2)}`);
    console.log();
  });

  majorScale.free();
}

/**
 * Demonstrate inversions and Roman numeral notation
 */
function demonstrateInversionsAnalysis(): void {
  console.log("=== Inversions and Roman Numeral Analysis ===");

  const majorScale = composer.WasmScaleFingerprint.major();

  // Test chord inversions
  const baseChord = new composer.WasmChord(1, 5); // C major

  console.log("C major chord in different inversions:");

  for (let inversion = 0; inversion <= 2; inversion++) {
    try {
      const invertedChord = baseChord.withInversion(inversion);
      const roman = composer.getRomanNumeral(invertedChord, majorScale);
      const scaleDegrees = composer.getStableScaleDegrees(invertedChord, majorScale);

      const inversionName =
        inversion === 0 ? "root position" : inversion === 1 ? "1st inversion" : "2nd inversion";

      console.log(
        `  ${inversionName.padEnd(15)}: ${invertedChord.toString().padEnd(8)} -> ${roman.padEnd(8)} (degrees: [${scaleDegrees.join(", ")}])`
      );
      invertedChord.free();
    } catch (error) {
      console.log(`  Inversion ${inversion}: Error - ${error}`);
    }
  }

  console.log();

  // Test seventh chord inversions
  const seventhChord = new composer.WasmChord(5, 7); // G7

  console.log("G7 chord in different inversions:");

  for (let inversion = 0; inversion <= 3; inversion++) {
    try {
      const invertedChord = seventhChord.withInversion(inversion);
      const roman = composer.getRomanNumeral(invertedChord, majorScale);
      const scaleDegrees = composer.getStableScaleDegrees(invertedChord, majorScale);

      const inversionName =
        inversion === 0
          ? "root position"
          : inversion === 1
            ? "1st inversion"
            : inversion === 2
              ? "2nd inversion"
              : "3rd inversion";

      console.log(
        `  ${inversionName.padEnd(15)}: ${invertedChord.toString().padEnd(8)} -> ${roman.padEnd(8)} (degrees: [${scaleDegrees.join(", ")}])`
      );
      invertedChord.free();
    } catch (error) {
      console.log(`  Inversion ${inversion}: Error - ${error}`);
    }
  }

  console.log();
  baseChord.free();
  seventhChord.free();
  majorScale.free();
}

/**
 * Demonstrate modal Roman numeral analysis
 */
function demonstrateModalAnalysis(): void {
  console.log("=== Modal Roman Numeral Analysis ===");

  // Create different modal scales
  const modes = [
    {
      name: "Dorian",
      scale: composer.WasmScaleFingerprint.fromArray(
        new Uint8Array([1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0])
      ),
    },
    {
      name: "Mixolydian",
      scale: composer.WasmScaleFingerprint.fromArray(
        new Uint8Array([1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0])
      ),
    },
    {
      name: "Major",
      scale: composer.WasmScaleFingerprint.major(),
    },
    {
      name: "Natural Minor",
      scale: composer.WasmScaleFingerprint.minor(),
    },
  ];

  // Test the same chord progression in different modes
  const testChords = [
    { root: 1, type: 5, name: "I/i" },
    { root: 4, type: 5, name: "IV/iv" },
    { root: 5, type: 5, name: "V/v" },
    { root: 1, type: 5, name: "I/i" },
  ];

  modes.forEach(({ name, scale }) => {
    console.log(`${name} mode analysis:`);

    testChords.forEach(({ root, type, name: chordName }) => {
      try {
        const chord = new composer.WasmChord(root, type);
        const roman = composer.getRomanNumeral(chord, scale);
        const scaleDegrees = composer.getStableScaleDegrees(chord, scale);

        console.log(
          `  ${chordName.padEnd(6)}: ${chord.toString().padEnd(6)} -> ${roman.padEnd(8)} (degrees: [${scaleDegrees.join(", ")}])`
        );
        chord.free();
      } catch (error) {
        console.log(`  ${chordName.padEnd(6)}: Error - ${error}`);
      }
    });

    console.log();
    scale.free();
  });
}

/**
 * Demonstrate harmonic function analysis
 */
function demonstrateHarmonicFunctionAnalysis(): void {
  console.log("=== Harmonic Function Analysis ===");

  const majorScale = composer.WasmScaleFingerprint.major();

  // Categorize chords by harmonic function
  const harmonicFunctions = [
    {
      function: "Tonic",
      chords: [
        { root: 1, type: 5, name: "I" },
        { root: 6, type: 5, name: "vi" },
        { root: 3, type: 5, name: "iii" },
      ],
    },
    {
      function: "Predominant",
      chords: [
        { root: 4, type: 5, name: "IV" },
        { root: 2, type: 7, name: "ii7" },
        { root: 6, type: 5, name: "vi" },
      ],
    },
    {
      function: "Dominant",
      chords: [
        { root: 5, type: 5, name: "V" },
        { root: 5, type: 7, name: "V7" },
        { root: 7, type: 7, name: "vii°7" },
      ],
    },
  ];

  harmonicFunctions.forEach(({ function: funcName, chords }) => {
    console.log(`${funcName} function chords:`);

    chords.forEach(({ root, type, name }) => {
      try {
        const chord = new composer.WasmChord(root, type);
        const roman = composer.getRomanNumeral(chord, majorScale);
        const scaleDegrees = composer.getStableScaleDegrees(chord, majorScale);
        const complexity = composer.getChordComplexity(chord, "major");

        console.log(
          `  ${name.padEnd(8)}: ${chord.toString().padEnd(6)} -> ${roman.padEnd(8)} (complexity: ${complexity}, degrees: [${scaleDegrees.join(", ")}])`
        );
        chord.free();
      } catch (error) {
        console.log(`  ${name.padEnd(8)}: Error - ${error}`);
      }
    });

    console.log();
  });

  majorScale.free();
}

/**
 * Demonstrate chord alteration analysis
 */
function demonstrateAlterationAnalysis(): void {
  console.log("=== Chord Alteration Analysis ===");

  const majorScale = composer.WasmScaleFingerprint.major();

  // Test different alterations
  const baseChords = [
    { root: 1, type: 5, name: "C major" },
    { root: 5, type: 7, name: "G7" },
  ];

  const alterations = ["#5", "b5", "#9", "b9", "#11", "b13"];

  baseChords.forEach(({ root, type, name }) => {
    console.log(`${name} with alterations:`);

    const baseChord = new composer.WasmChord(root, type);
    const baseRoman = composer.getRomanNumeral(baseChord, majorScale);
    console.log(`  Original: ${baseChord.toString().padEnd(10)} -> ${baseRoman}`);

    alterations.forEach((alteration) => {
      try {
        const alteredChord = baseChord.withAlteration(alteration);
        const roman = composer.getRomanNumeral(alteredChord, majorScale);
        const scaleDegrees = composer.getStableScaleDegrees(alteredChord, majorScale);

        console.log(
          `  ${alteration.padEnd(4)}: ${alteredChord.toString().padEnd(10)} -> ${roman.padEnd(8)} (degrees: [${scaleDegrees.join(", ")}])`
        );
        alteredChord.free();
      } catch (error) {
        console.log(`  ${alteration.padEnd(4)}: Error - ${error}`);
      }
    });

    console.log();
    baseChord.free();
  });

  majorScale.free();
}

/**
 * Run all Roman numeral analysis demonstrations
 */
async function main(): Promise<void> {
  console.log("Composer Library - Roman Numeral Analysis Examples (TypeScript)");
  console.log("=".repeat(75));
  console.log();

  try {
    await initializeWasm();

    demonstrateBasicRomanNumerals();
    demonstrateSeventhChordAnalysis();
    demonstrateProgressionAnalysis();
    demonstrateInversionsAnalysis();
    demonstrateModalAnalysis();
    demonstrateHarmonicFunctionAnalysis();
    demonstrateAlterationAnalysis();

    console.log("All Roman numeral analysis examples completed successfully!");
    console.log();
    console.log("🎼 Key insights from Roman numeral analysis:");
    console.log("   - Roman numerals reveal harmonic function and scale context");
    console.log("   - Same chords can have different functions in different keys");
    console.log("   - Inversions and alterations affect Roman numeral notation");
    console.log("   - Modal analysis shows how context changes harmonic meaning");
    console.log("   - Complexity scores help assess harmonic sophistication");
  } catch (error) {
    console.error("Error running examples:", error);
  }
}

// Export for module usage
export {
  demonstrateBasicRomanNumerals,
  demonstrateSeventhChordAnalysis,
  demonstrateProgressionAnalysis,
  demonstrateInversionsAnalysis,
  demonstrateModalAnalysis,
  demonstrateHarmonicFunctionAnalysis,
  demonstrateAlterationAnalysis,
};

// Run if executed directly
if (require.main === module) {
  main().catch(console.error);
}
