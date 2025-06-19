#!/usr/bin/env ts-node
/**
 * Scale Fingerprints and Analysis
 *
 * This example demonstrates scale operations in the Composer library:
 * - Creating different scale types (major, minor, harmonic minor, custom)
 * - Analyzing scale properties
 * - Working with chord-scale relationships
 * - Understanding stable scale degrees
 *
 * Based on the Composer specification: chord-theory-core.spec
 */

import * as composer from "../../composer_wasm.js";

/**
 * Initialize the WASM module
 */
async function initializeWasm(): Promise<void> {
  console.log(`Initializing Composer WASM v${composer.getVersion()}`);
}

/**
 * Demonstrate basic scale creation and properties
 */
function demonstrateBasicScales(): void {
  console.log("=== Basic Scale Creation ===");

  // Create standard scales
  const majorScale = composer.WasmScaleFingerprint.major();
  const minorScale = composer.WasmScaleFingerprint.minor();
  const harmonicMinor = composer.WasmScaleFingerprint.harmonicMinor();

  console.log(`Major scale: ${majorScale.toString()}`);
  console.log(`  Note count: ${majorScale.noteCount()}`);
  console.log(`  Is diatonic: ${majorScale.isDiatonic()}`);
  console.log();

  console.log(`Natural minor scale: ${minorScale.toString()}`);
  console.log(`  Note count: ${minorScale.noteCount()}`);
  console.log(`  Is diatonic: ${minorScale.isDiatonic()}`);
  console.log();

  console.log(`Harmonic minor scale: ${harmonicMinor.toString()}`);
  console.log(`  Note count: ${harmonicMinor.noteCount()}`);
  console.log(`  Is diatonic: ${harmonicMinor.isDiatonic()}`);
  console.log();

  // Clean up memory
  majorScale.free();
  minorScale.free();
  harmonicMinor.free();
}

/**
 * Demonstrate custom scale creation from arrays
 */
function demonstrateCustomScales(): void {
  console.log("=== Custom Scale Creation ===");

  // Create custom scales using semitone patterns
  const scalePatterns: { name: string; pattern: number[] }[] = [
    {
      name: "Whole tone scale",
      pattern: [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0], // All whole steps
    },
    {
      name: "Pentatonic major",
      pattern: [1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0], // C-D-E-G-A
    },
    {
      name: "Blues scale",
      pattern: [1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1], // C-Eb-F-Gb-G-Bb
    },
    {
      name: "Dorian mode",
      pattern: [1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0], // Natural minor with raised 6th
    },
  ];

  scalePatterns.forEach(({ name, pattern }) => {
    try {
      const scale = composer.WasmScaleFingerprint.fromArray(new Uint8Array(pattern));
      console.log(`${name}: ${scale.toString()}`);
      console.log(`  Note count: ${scale.noteCount()}`);
      console.log(`  Is diatonic: ${scale.isDiatonic()}`);
      console.log();
      scale.free();
    } catch (error) {
      console.log(`Error creating ${name}: ${error}`);
    }
  });
}

/**
 * Analyze chord-scale relationships
 */
function demonstrateChordScaleRelationships(): void {
  console.log("=== Chord-Scale Relationships ===");

  const majorScale = composer.WasmScaleFingerprint.major();
  const minorScale = composer.WasmScaleFingerprint.minor();

  // Test various chords against different scales
  const chords = [
    { chord: new composer.WasmChord(1, 5), name: "C major" },
    { chord: new composer.WasmChord(2, 7), name: "D minor 7" },
    { chord: new composer.WasmChord(5, 7), name: "G7" },
    { chord: new composer.WasmChord(6, 5), name: "A minor" },
    { chord: new composer.WasmChord(4, 5), name: "F major" },
  ];

  console.log("Chord analysis in C major scale:");
  chords.forEach(({ chord, name }) => {
    try {
      const scaleDegrees = composer.getStableScaleDegrees(chord, majorScale);
      const romanNumeral = composer.getRomanNumeral(chord, majorScale);
      console.log(`  ${name}: degrees [${scaleDegrees.join(", ")}], Roman: ${romanNumeral}`);
    } catch (error) {
      console.log(`  ${name}: Error - ${error}`);
    }
  });

  console.log();
  console.log("Same chords in C minor scale:");
  chords.forEach(({ chord, name }) => {
    try {
      const scaleDegrees = composer.getStableScaleDegrees(chord, minorScale);
      const romanNumeral = composer.getRomanNumeral(chord, minorScale);
      console.log(`  ${name}: degrees [${scaleDegrees.join(", ")}], Roman: ${romanNumeral}`);
    } catch (error) {
      console.log(`  ${name}: Error - ${error}`);
    }
  });

  console.log();

  // Clean up memory
  chords.forEach(({ chord }) => chord.free());
  majorScale.free();
  minorScale.free();
}

/**
 * Demonstrate scale degree analysis for common progressions
 */
function demonstrateProgressionAnalysis(): void {
  console.log("=== Progression Analysis ===");

  const majorScale = composer.WasmScaleFingerprint.major();

  // Common progressions
  const progressions = [
    {
      name: "I-V-vi-IV (Pop progression)",
      chords: [
        new composer.WasmChord(1, 5), // I
        new composer.WasmChord(5, 5), // V
        new composer.WasmChord(6, 5), // vi
        new composer.WasmChord(4, 5), // IV
      ],
    },
    {
      name: "ii-V-I (Jazz progression)",
      chords: [
        new composer.WasmChord(2, 7), // ii7
        new composer.WasmChord(5, 7), // V7
        new composer.WasmChord(1, 5), // I
      ],
    },
    {
      name: "vi-IV-I-V (Ballad progression)",
      chords: [
        new composer.WasmChord(6, 5), // vi
        new composer.WasmChord(4, 5), // IV
        new composer.WasmChord(1, 5), // I
        new composer.WasmChord(5, 5), // V
      ],
    },
  ];

  progressions.forEach(({ name, chords }) => {
    console.log(`${name}:`);
    chords.forEach((chord, i) => {
      try {
        const scaleDegrees = composer.getStableScaleDegrees(chord, majorScale);
        const romanNumeral = composer.getRomanNumeral(chord, majorScale);
        const complexity = composer.getChordComplexity(chord, "major");
        console.log(
          `  ${i + 1}. ${chord.toString()} (${romanNumeral}) - degrees: [${scaleDegrees.join(", ")}], complexity: ${complexity}`
        );
      } catch (error) {
        console.log(`  ${i + 1}. ${chord.toString()} - Error: ${error}`);
      }
    });
    console.log();

    // Clean up progression chords
    chords.forEach((chord) => chord.free());
  });

  majorScale.free();
}

/**
 * Demonstrate modal analysis
 */
function demonstrateModalAnalysis(): void {
  console.log("=== Modal Analysis ===");

  // Create modes of C major using different starting points
  const modes = [
    { name: "Ionian (Major)", pattern: [1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1] },
    { name: "Dorian", pattern: [1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0] },
    { name: "Phrygian", pattern: [1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0] },
    { name: "Lydian", pattern: [1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1] },
    { name: "Mixolydian", pattern: [1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0] },
    { name: "Aeolian (Natural Minor)", pattern: [1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0] },
    { name: "Locrian", pattern: [1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0] },
  ];

  console.log("Church modes analysis:");
  modes.forEach(({ name, pattern }) => {
    try {
      const scale = composer.WasmScaleFingerprint.fromArray(new Uint8Array(pattern));
      console.log(`${name}: ${scale.toString()}`);
      console.log(`  Note count: ${scale.noteCount()}, Diatonic: ${scale.isDiatonic()}`);

      // Test a few chords in each mode
      const testChord = new composer.WasmChord(1, 5); // Tonic chord
      try {
        const scaleDegrees = composer.getStableScaleDegrees(testChord, scale);
        console.log(`  Tonic chord degrees: [${scaleDegrees.join(", ")}]`);
      } catch (error) {
        console.log(`  Tonic chord analysis failed: ${error}`);
      }
      testChord.free();

      console.log();
      scale.free();
    } catch (error) {
      console.log(`Error creating ${name}: ${error}`);
    }
  });
}

/**
 * Demonstrate scale compatibility testing
 */
function demonstrateScaleCompatibility(): void {
  console.log("=== Scale Compatibility Testing ===");

  const scales = [
    { name: "Major", scale: composer.WasmScaleFingerprint.major() },
    { name: "Natural Minor", scale: composer.WasmScaleFingerprint.minor() },
    { name: "Harmonic Minor", scale: composer.WasmScaleFingerprint.harmonicMinor() },
  ];

  // Test chord that might work in multiple scales
  const testChords = [
    new composer.WasmChord(1, 5), // C major
    new composer.WasmChord(3, 5), // E major/minor
    new composer.WasmChord(5, 7), // G7
    new composer.WasmChord(7, 7), // B7
  ];

  testChords.forEach((chord) => {
    console.log(`Testing chord: ${chord.toString()}`);
    scales.forEach(({ name, scale }) => {
      try {
        const scaleDegrees = composer.getStableScaleDegrees(chord, scale);
        const romanNumeral = composer.getRomanNumeral(chord, scale);
        console.log(`  In ${name}: degrees [${scaleDegrees.join(", ")}], Roman: ${romanNumeral}`);
      } catch (error) {
        console.log(`  In ${name}: Not compatible - ${error}`);
      }
    });
    console.log();
    chord.free();
  });

  // Clean up scales
  scales.forEach(({ scale }) => scale.free());
}

/**
 * Run all scale demonstration functions
 */
async function main(): Promise<void> {
  console.log("Composer Library - Scale Fingerprints Examples (TypeScript)");
  console.log("=".repeat(70));
  console.log();

  try {
    await initializeWasm();

    demonstrateBasicScales();
    demonstrateCustomScales();
    demonstrateChordScaleRelationships();
    demonstrateProgressionAnalysis();
    demonstrateModalAnalysis();
    demonstrateScaleCompatibility();

    console.log("All scale examples completed successfully!");
  } catch (error) {
    console.error("Error running examples:", error);
  }
}

// Export for module usage
export {
  demonstrateBasicScales,
  demonstrateCustomScales,
  demonstrateChordScaleRelationships,
  demonstrateProgressionAnalysis,
  demonstrateModalAnalysis,
  demonstrateScaleCompatibility,
};

// Run if executed directly
if (require.main === module) {
  main().catch(console.error);
}
