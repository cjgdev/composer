#!/usr/bin/env ts-node
/**
 * Advanced Music Theory Analysis
 *
 * This example demonstrates advanced music theory functionality:
 * - Scale degree analysis in various contexts
 * - Voice leading analysis and validation
 * - Harmonic function classification
 * - Tritone substitution validation
 * - Advanced chord substitutions and transformations
 * - Cross-scale modulation analysis
 *
 * Based on the Composer specification: chord-theory-core.spec
 */

import * as composer from "../../composer_wasm";

interface ProgressionInfo {
  name: string;
  chords: composer.WasmChord[];
}

interface HarmonicFunction {
  name: string;
  scaleDegrees: number[];
}

/**
 * Initialize the WASM module
 */
async function initializeWasm(): Promise<void> {
  console.log(`Initializing Composer WASM v${composer.getVersion()}`);
}

/**
 * Show comprehensive scale degree analysis
 */
function demonstrateScaleDegreeAnalysis(): void {
  console.log("=== Scale Degree Analysis ===");

  const majorScale = composer.WasmScaleFingerprint.major();
  const minorScale = composer.WasmScaleFingerprint.minor();

  // Test chords for scale degree analysis
  const testChords = [
    { chord: new composer.WasmChord(1, 5), name: "I major" },
    { chord: new composer.WasmChord(2, 7), name: "ii7" },
    { chord: new composer.WasmChord(5, 7), name: "V7" },
    { chord: new composer.WasmChord(6, 5), name: "vi" },
    { chord: new composer.WasmChord(4, 9), name: "IV9" },
  ];

  console.log("Scale degree analysis in major scale:");
  testChords.forEach(({ chord, name }) => {
    console.log(`  ${name}:`);

    try {
      // Get stable scale degrees (absolute)
      const scaleDegrees = composer.getStableScaleDegrees(chord, majorScale);
      console.log(`    Stable scale degrees: [${scaleDegrees.join(", ")}]`);

      // Show chord properties
      console.log(`    Root: ${chord.root}, Type: ${chord.chordType}`);
      console.log(`    Inversion: ${chord.inversion}`);

      // Calculate complexity
      const complexity = composer.getChordComplexity(chord, "major");
      console.log(`    Complexity: ${complexity.toFixed(2)}`);
    } catch (error) {
      console.log(`    Analysis error: ${error}`);
    }

    console.log();
  });

  console.log("Comparing scale degree analysis across different scales:");
  const testChord = new composer.WasmChord(5, 7); // V7 chord
  const scales = [
    { name: "Major", scale: majorScale },
    { name: "Natural Minor", scale: minorScale },
  ];

  scales.forEach(({ name, scale }) => {
    console.log(`  ${testChord.toString()} in ${name}:`);

    try {
      const scaleDegrees = composer.getStableScaleDegrees(testChord, scale);
      console.log(`    Scale degrees: [${scaleDegrees.join(", ")}]`);
    } catch (error) {
      console.log(`    Analysis error: ${error}`);
    }

    console.log();
  });

  // Clean up objects
  testChords.forEach(({ chord }) => chord.free());
  testChord.free();
  majorScale.free();
  minorScale.free();
}

/**
 * Show harmonic function classification
 */
function demonstrateHarmonicFunctions(): void {
  console.log("=== Harmonic Function Analysis ===");

  // Define harmonic functions
  const harmonicFunctions: HarmonicFunction[] = [
    { name: "Tonic", scaleDegrees: [1, 6, 3] }, // I, vi, iii
    { name: "Predominant", scaleDegrees: [4, 2] }, // IV, ii
    { name: "Dominant", scaleDegrees: [5, 7] }, // V, vii°
  ];

  console.log("Harmonic function classification:");
  harmonicFunctions.forEach(({ name, scaleDegrees }) => {
    console.log(`  ${name} function:`);

    scaleDegrees.forEach((degree) => {
      // Create different chord types on each degree
      const chordTypes = [5, 7]; // Triad and seventh

      chordTypes.forEach((chordType) => {
        try {
          const chord = new composer.WasmChord(degree, chordType);
          console.log(`    ${chord.toString()} (scale degree ${degree})`);

          // Manual function classification
          let functionName: string;
          if (harmonicFunctions[0].scaleDegrees.includes(degree)) {
            functionName = "Tonic";
          } else if (harmonicFunctions[1].scaleDegrees.includes(degree)) {
            functionName = "Predominant";
          } else if (harmonicFunctions[2].scaleDegrees.includes(degree)) {
            functionName = "Dominant";
          } else {
            functionName = "Chromatic";
          }

          console.log(`      Function: ${functionName}`);

          // Show complexity
          const complexity = composer.getChordComplexity(chord, "major");
          console.log(`      Complexity: ${complexity.toFixed(2)}`);

          chord.free();
        } catch (error) {
          console.log(`    Error creating chord ${degree}/${chordType}: ${error}`);
        }
      });
    });

    console.log();
  });
}

/**
 * Show voice leading analysis and validation
 */
function demonstrateVoiceLeading(): void {
  console.log("=== Voice Leading Analysis ===");

  // Common chord progressions with different voice leading qualities
  const progressions: ProgressionInfo[] = [
    {
      name: "Good voice leading (I-vi-IV-V)",
      chords: [
        new composer.WasmChord(1, 5), // I
        new composer.WasmChord(6, 5), // vi
        new composer.WasmChord(4, 5), // IV
        new composer.WasmChord(5, 5), // V
      ],
    },
    {
      name: "Parallel motion (I-II-III)",
      chords: [
        new composer.WasmChord(1, 5), // I
        new composer.WasmChord(2, 5), // II (parallel major chords)
        new composer.WasmChord(3, 5), // III
      ],
    },
    {
      name: "Large leaps (I-♭VI-I)",
      chords: [
        new composer.WasmChord(1, 5), // I
        new composer.WasmChord(6, 5), // ♭VI (simulated)
        new composer.WasmChord(1, 5), // I
      ],
    },
  ];

  console.log("Voice leading analysis:");
  progressions.forEach(({ name, chords }) => {
    console.log(`  ${name}:`);
    console.log(`    Progression: ${chords.map((c) => c.toString())}`);

    // Manual voice leading analysis
    const totalComplexity = chords.reduce((sum, chord) => {
      try {
        return sum + composer.getChordComplexity(chord, "major");
      } catch (error) {
        return sum;
      }
    }, 0);
    const avgComplexity = totalComplexity / chords.length;

    console.log(`    Average complexity: ${avgComplexity.toFixed(2)}`);

    // Check for chromatic movement
    const roots = chords.map((c) => c.root);
    const chromaticSteps = roots.reduce((count, root, i) => {
      if (i > 0) {
        const interval = Math.abs(root - roots[i - 1]);
        return count + (interval === 1 ? 1 : 0);
      }
      return count;
    }, 0);
    console.log(`    Chromatic steps: ${chromaticSteps}`);

    // Estimate voice leading quality
    const qualityEstimate = Math.max(0, 1 - (avgComplexity - 2) / 8);
    console.log(`    Estimated quality: ${qualityEstimate.toFixed(3)}`);

    console.log();
  });

  // Clean up objects
  progressions.forEach(({ chords }) => {
    chords.forEach((chord) => chord.free());
  });
}

/**
 * Show tritone substitution analysis and validation
 */
function demonstrateTritoneSubstitution(): void {
  console.log("=== Tritone Substitution Analysis ===");

  // Test chords for tritone substitution
  const testChords = [
    { chord: new composer.WasmChord(5, 7), name: "V7", shouldBeValid: true },
    { chord: new composer.WasmChord(1, 5), name: "I", shouldBeValid: false },
    { chord: new composer.WasmChord(2, 7), name: "ii7", shouldBeValid: false },
    { chord: new composer.WasmChord(7, 7), name: "vii7", shouldBeValid: false },
  ];

  console.log("Tritone substitution validation:");
  testChords.forEach(({ chord, name, shouldBeValid }) => {
    console.log(`  ${name} (${chord.toString()}):`);

    // Manual tritone sub validation
    const isDominantSeventh = chord.chordType === 7 && chord.root === 5;

    console.log(`    Is dominant seventh: ${isDominantSeventh}`);
    console.log(`    Expected validity: ${shouldBeValid}`);
    console.log(`    Actual validity: ${isDominantSeventh}`);

    if (isDominantSeventh) {
      // Calculate manual substitute (tritone away)
      const subRoot = ((chord.root + 3) % 7) + 1; // Tritone = 3 scale degrees (simplified)
      console.log(`    Tritone substitute root: ${subRoot}`);

      try {
        const substitute = new composer.WasmChord(subRoot, 7);
        console.log(`    Tritone substitute: ${substitute.toString()}`);
        substitute.free();
      } catch (error) {
        console.log(`    Error creating substitute: ${error}`);
      }
    }

    console.log();
  });

  // Clean up objects
  testChords.forEach(({ chord }) => chord.free());
}

/**
 * Show various chord substitution techniques
 */
function demonstrateChordSubstitutions(): void {
  console.log("=== Chord Substitution Techniques ===");

  const originalProgression = [
    new composer.WasmChord(1, 5), // I
    new composer.WasmChord(6, 5), // vi
    new composer.WasmChord(4, 5), // IV
    new composer.WasmChord(5, 5), // V
  ];

  console.log(`Original progression: ${originalProgression.map((c) => c.toString())}`);
  console.log();

  // Different substitution techniques
  const substitutionTechniques = [
    {
      name: "Relative minor/major substitution",
      substitutions: [
        { position: 0, newChord: new composer.WasmChord(6, 5) }, // I -> vi
        { position: 1, newChord: new composer.WasmChord(1, 5) }, // vi -> I
      ],
    },
    {
      name: "Extended harmonies",
      substitutions: [
        { position: 0, newChord: new composer.WasmChord(1, 7) }, // I -> IM7
        { position: 3, newChord: new composer.WasmChord(5, 9) }, // V -> V9 (if supported)
      ],
    },
  ];

  substitutionTechniques.forEach((technique) => {
    console.log(`  ${technique.name}:`);

    // Apply substitutions
    const substituted = [...originalProgression];
    technique.substitutions.forEach(({ position, newChord }) => {
      const originalChord = substituted[position];
      console.log(
        `    Position ${position + 1}: ${originalChord.toString()} -> ${newChord.toString()}`
      );

      // Analyze the substitution
      try {
        const origComplexity = composer.getChordComplexity(originalChord, "major");
        const subComplexity = composer.getChordComplexity(newChord, "major");

        const complexityChange = `${origComplexity.toFixed(2)} -> ${subComplexity.toFixed(2)}`;
        console.log(`      Complexity change: ${complexityChange}`);

        // Simulate isotonal check using scale degrees
        const majorScale = composer.WasmScaleFingerprint.major();
        try {
          const origDegrees = composer.getStableScaleDegrees(originalChord, majorScale);
          const subDegrees = composer.getStableScaleDegrees(newChord, majorScale);
          const sharedDegrees = origDegrees.filter((d: string) => subDegrees.includes(d));
          const areIsotonal = sharedDegrees.length >= 2;
          console.log(`      Isotonal (simulated): ${areIsotonal}`);
        } catch (error) {
          console.log(`      Isotonal analysis failed: ${error}`);
        }
        majorScale.free();
      } catch (error) {
        console.log(`      Analysis error: ${error}`);
      }

      substituted[position] = newChord;
    });

    console.log(`    Result: ${substituted.map((c) => c.toString())}`);
    console.log();

    // Clean up substitution chords
    technique.substitutions.forEach(({ newChord }) => newChord.free());
  });

  // Clean up original progression
  originalProgression.forEach((chord) => chord.free());
}

/**
 * Show analysis of modulation between keys/scales
 */
function demonstrateModulationAnalysis(): void {
  console.log("=== Modulation Analysis ===");

  // Common modulation progression: C major to G major
  const modulationProgression = [
    new composer.WasmChord(1, 5), // I in C (C major)
    new composer.WasmChord(6, 5), // vi in C (A minor)
    new composer.WasmChord(2, 7), // ii7 in C (D minor 7) - becomes vi7 in G
    new composer.WasmChord(5, 7), // V7 in C (G7) - becomes I7 in G
    new composer.WasmChord(1, 5), // I in G (G major) - new key
  ];

  const majorScale = composer.WasmScaleFingerprint.major();

  console.log("Analyzing modulation from C major to G major:");
  console.log(`Progression: ${modulationProgression.map((c) => c.toString())}`);
  console.log();

  // Analyze each chord in both key contexts
  modulationProgression.forEach((chord, i) => {
    console.log(`  Chord ${i + 1}: ${chord.toString()}`);

    // Analyze in C major context
    try {
      const degrees = composer.getStableScaleDegrees(chord, majorScale);
      const roman = composer.getRomanNumeral(chord, majorScale);
      console.log(`    In C major: ${roman} (degrees: [${degrees.join(", ")}])`);
    } catch (error) {
      console.log(`    In C major: Analysis error - ${error}`);
    }

    // Manual analysis for G major context (simplified)
    // In a real implementation, this would require transposition
    const transposedRoot = ((chord.root + 4) % 7) + 1; // G is 5th degree of C
    console.log(`    In G major: estimated root degree ${transposedRoot}`);

    // Determine pivot chord quality
    if (i === 2 || i === 3) {
      // These are pivot chords
      console.log("    *** Pivot chord function ***");
    }

    console.log();
  });

  // Analyze modulation smoothness
  console.log("Modulation analysis:");
  const totalComplexity = modulationProgression.reduce((sum, chord) => {
    try {
      return sum + composer.getChordComplexity(chord, "major");
    } catch (error) {
      return sum;
    }
  }, 0);
  const avgComplexity = totalComplexity / modulationProgression.length;

  console.log(`  Average complexity: ${avgComplexity.toFixed(2)}`);
  console.log(`  Modulation smoothness: ${avgComplexity < 3 ? "Smooth" : "Complex"}`);
  console.log();

  // Clean up objects
  modulationProgression.forEach((chord) => chord.free());
  majorScale.free();
}

/**
 * Show analysis of chords across different scale types
 */
function demonstrateCrossScaleAnalysis(): void {
  console.log("=== Cross-Scale Analysis ===");

  const testChord = new composer.WasmChord(5, 7); // V7 chord

  // Analyze the same chord in different scale contexts
  const scaleContexts = [
    { name: "Major", scale: composer.WasmScaleFingerprint.major() },
    { name: "Natural Minor", scale: composer.WasmScaleFingerprint.minor() },
    { name: "Harmonic Minor", scale: composer.WasmScaleFingerprint.harmonicMinor() },
  ];

  console.log(`Analyzing ${testChord.toString()} across different scales:`);
  console.log();

  scaleContexts.forEach(({ name, scale }) => {
    console.log(`  In ${name} scale:`);
    console.log(`    Scale note count: ${scale.noteCount()}`);
    console.log(`    Is diatonic: ${scale.isDiatonic()}`);

    try {
      const scaleDegrees = composer.getStableScaleDegrees(testChord, scale);
      console.log(`    Scale degrees: [${scaleDegrees.join(", ")}]`);

      // Check if chord fits naturally in scale (simplified analysis)
      const chordInScale = scaleDegrees.length > 0;
      console.log(`    Fits in scale: ${chordInScale}`);
    } catch (error) {
      console.log(`    Analysis error: ${error}`);
    }

    // Function in this scale context
    let harmonicFunction: string;
    if (testChord.root === 5) {
      if (name === "Major") {
        harmonicFunction = "Dominant (V7)";
      } else if (name === "Mixolydian") {
        harmonicFunction = "Tonic (I7)";
      } else {
        harmonicFunction = "Fifth degree seventh";
      }
    } else {
      harmonicFunction = `Degree ${testChord.root} seventh`;
    }

    console.log(`    Harmonic function: ${harmonicFunction}`);
    console.log();
  });

  // Clean up objects
  testChord.free();
  scaleContexts.forEach(({ scale }) => scale.free());
}

/**
 * Run all advanced theory demonstration functions
 */
async function main(): Promise<void> {
  console.log("Composer Library - Advanced Music Theory Examples (TypeScript)");
  console.log("=".repeat(70));
  console.log();

  try {
    await initializeWasm();

    demonstrateScaleDegreeAnalysis();
    demonstrateHarmonicFunctions();
    demonstrateVoiceLeading();
    demonstrateTritoneSubstitution();
    demonstrateChordSubstitutions();
    demonstrateModulationAnalysis();
    demonstrateCrossScaleAnalysis();

    console.log("All advanced theory examples completed!");
    console.log();
    console.log("Note: Some advanced theory features may require additional WASM");
    console.log("bindings to be fully functional. This example demonstrates the");
    console.log("intended usage patterns based on current available functionality.");
  } catch (error) {
    console.error("Error running advanced theory examples:", error);
  }
}

// Run if executed directly
if (require.main === module) {
  main().catch(console.error);
}

// Export functions for potential reuse
export {
  demonstrateScaleDegreeAnalysis,
  demonstrateHarmonicFunctions,
  demonstrateVoiceLeading,
  demonstrateTritoneSubstitution,
  demonstrateChordSubstitutions,
  demonstrateModulationAnalysis,
  demonstrateCrossScaleAnalysis,
};
