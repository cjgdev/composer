#!/usr/bin/env ts-node
/**
 * Basic Chord Creation and Manipulation
 *
 * This example demonstrates fundamental chord operations in the Composer library:
 * - Creating chords with different roots, types, and inversions
 * - Examining chord properties
 * - Converting chords to different representations
 * - Working with chord characteristics
 *
 * Based on the Composer specification: chord-theory-core.spec
 */

import * as composer from "../../composer_wasm.js";

/**
 * Initialize the WASM module
 */
async function initializeWasm(): Promise<void> {
  // For TypeScript/Node.js usage, we need to handle WASM initialization
  // In a real Node.js environment, you'd use the nodejs build
  console.log(`Initializing Composer WASM v${composer.getVersion()}`);
}

/**
 * Show basic chord creation with different parameters
 */
function demonstrateChordCreation(): void {
  console.log("=== Basic Chord Creation ===");

  // Create a simple C major chord (root=1, type=5)
  const cMajor = new composer.WasmChord(1, 5); // C major triad
  console.log(`C Major: ${cMajor.toString()}`);
  console.log(`  Root: ${cMajor.root}`);
  console.log(`  Type: ${cMajor.chordType}`);
  console.log(`  Inversion: ${cMajor.inversion}`);
  console.log();

  // Create chord with inversion
  const cMajorFirst = cMajor.withInversion(1);
  console.log(`C Major (1st inversion): ${cMajorFirst.toString()}`);
  console.log(`  Inversion: ${cMajorFirst.inversion}`);
  console.log();

  // Create more complex chords
  const dMinor7 = new composer.WasmChord(2, 7); // D minor 7
  console.log(`D Minor 7: ${dMinor7.toString()}`);
  console.log();

  // Create chord using static methods
  const gTriad = composer.WasmChord.triad(5); // G triad
  console.log(`G Triad: ${gTriad.toString()}`);
  console.log();

  // Clean up memory
  cMajor.free();
  cMajorFirst.free();
  dMinor7.free();
  gTriad.free();
}

/**
 * Explore various chord properties and methods
 */
function demonstrateChordProperties(): void {
  console.log("=== Chord Properties ===");

  const chord = new composer.WasmChord(4, 5); // E major
  console.log(`Chord: ${chord.toString()}`);

  // Check various properties
  console.log(`  Root note: ${chord.root}`);
  console.log(`  Chord type: ${chord.chordType}`);
  console.log(`  Inversion: ${chord.inversion}`);
  console.log(`  Is rest: ${chord.isRest}`);
  console.log(`  Is triad: ${chord.isTriad()}`);
  console.log(`  Is seventh: ${chord.isSeventh()}`);
  console.log(`  Is extended: ${chord.isExtended()}`);
  console.log();

  // Create chord with alterations
  const alteredChord = chord.withAlteration("#5");
  console.log(`E Major with #5: ${alteredChord.toString()}`);
  console.log();

  // Clean up memory
  chord.free();
  alteredChord.free();
}

/**
 * Show chord serialization to binary and hex formats
 */
function demonstrateChordSerialization(): void {
  console.log("=== Chord Serialization ===");

  const chord = new composer.WasmChord(3, 7); // Eb minor 7
  console.log(`Original chord: ${chord.toString()}`);

  // Convert to hex for readable representation
  const hexString = chord.toHex();
  console.log(`Hex representation: ${hexString}`);
  console.log(`Hex length: ${hexString.length} characters`);

  // Deserialize back
  const restoredChord = composer.WasmChord.fromHex(hexString);
  console.log(`Restored chord: ${restoredChord.toString()}`);

  // Verify they match
  const match = chord.toString() === restoredChord.toString();
  console.log(`✓ Serialization roundtrip: ${match ? "SUCCESS" : "FAILED"}`);
  console.log();

  // Clean up memory
  chord.free();
  restoredChord.free();
}

/**
 * Show chord comparison and equality operations
 */
function demonstrateChordComparison(): void {
  console.log("=== Chord Comparison ===");

  const chord1 = new composer.WasmChord(1, 5); // C major
  const chord2 = new composer.WasmChord(1, 5); // C major (identical)
  const chord3 = chord1.withInversion(1); // C major first inversion
  const chord4 = new composer.WasmChord(2, 5); // D major

  console.log(`Chord 1: ${chord1.toString()}`);
  console.log(`Chord 2: ${chord2.toString()}`);
  console.log(`Chord 3: ${chord3.toString()}`);
  console.log(`Chord 4: ${chord4.toString()}`);
  console.log();

  // Note: Direct comparison would need custom implementation
  // since WASM objects don't have built-in equality
  const chord1Hex = chord1.toHex();
  const chord2Hex = chord2.toHex();
  const chord3Hex = chord3.toHex();
  const chord4Hex = chord4.toHex();

  console.log(`Chord 1 == Chord 2: ${chord1Hex === chord2Hex}`);
  console.log(`Chord 1 == Chord 3: ${chord1Hex === chord3Hex}`);
  console.log(`Chord 1 == Chord 4: ${chord1Hex === chord4Hex}`);
  console.log();

  // Clean up memory
  chord1.free();
  chord2.free();
  chord3.free();
  chord4.free();
}

/**
 * Show music theory related chord operations
 */
function demonstrateChordTheory(): void {
  console.log("=== Chord Theory ===");

  // Create a progression
  const progression = [
    new composer.WasmChord(1, 5), // I major (C)
    new composer.WasmChord(4, 5), // IV major (F)
    new composer.WasmChord(5, 5), // V major (G)
    new composer.WasmChord(1, 5), // I major (C)
  ];

  const majorScale = composer.WasmScaleFingerprint.major();

  console.log("Chord progression:");
  progression.forEach((chord, i) => {
    const complexity = composer.getChordComplexity(chord, "major");
    const roman = composer.getRomanNumeral(chord, majorScale);
    console.log(`  ${i + 1}. ${chord.toString()} (${roman}, complexity: ${complexity.toFixed(2)})`);
  });

  console.log();

  // Clean up memory
  progression.forEach((chord) => chord.free());
  majorScale.free();
}

/**
 * Demonstrate different chord creation methods
 */
function demonstrateChordTypes(): void {
  console.log("=== Different Chord Types ===");

  // Create using different static methods
  const restChord = composer.WasmChord.rest();
  const triad = composer.WasmChord.triad(8); // Ab triad
  const seventh = composer.WasmChord.seventh(10); // Bb seventh

  console.log(`Rest chord: ${restChord.toString()} (is rest: ${restChord.isRest})`);
  console.log(`Ab triad: ${triad.toString()} (is triad: ${triad.isTriad()})`);
  console.log(`Bb seventh: ${seventh.toString()} (is seventh: ${seventh.isSeventh()})`);
  console.log();

  // Test various chord types by number
  const chordTypes = [5, 7, 9, 11, 13];
  const root = 6; // F#

  console.log(`Different chord types on F# (root=${root}):`);
  chordTypes.forEach((type) => {
    try {
      const chord = new composer.WasmChord(root, type);
      console.log(`  Type ${type}: ${chord.toString()}`);
      chord.free();
    } catch (error) {
      console.log(`  Type ${type}: Error - ${error}`);
    }
  });

  console.log();

  // Clean up memory
  restChord.free();
  triad.free();
  seventh.free();
}

/**
 * Run all chord demonstration functions
 */
async function main(): Promise<void> {
  console.log("Composer Library - Basic Chord Examples (TypeScript)");
  console.log("=".repeat(60));
  console.log();

  try {
    await initializeWasm();

    demonstrateChordCreation();
    demonstrateChordProperties();
    demonstrateChordSerialization();
    demonstrateChordComparison();
    demonstrateChordTheory();
    demonstrateChordTypes();

    console.log("All examples completed successfully!");
  } catch (error) {
    console.error("Error running examples:", error);
  }
}

// Export for module usage
export {
  demonstrateChordCreation,
  demonstrateChordProperties,
  demonstrateChordSerialization,
  demonstrateChordComparison,
  demonstrateChordTheory,
  demonstrateChordTypes,
};

// Run if executed directly
if (require.main === module) {
  main().catch(console.error);
}
