#!/usr/bin/env ts-node
/**
 * Pattern Matching and Chord Lookup
 *
 * This example demonstrates pattern matching and lookup functionality:
 * - Trie-based pattern storage and searching
 * - Wildcard pattern matching
 * - Chord symbol parsing from multiple formats
 * - Isotonal chord mapping and substitutions
 * - Context-aware pattern search algorithms
 *
 * Based on the Composer specification: chord-analysis-lookup.spec
 */

import * as composer from "../../composer_wasm";

interface TrieStatistics {
  totalPatterns: number;
  totalNodes: number;
  memoryUsageBytes: number;
  scaleBranches: number;
  maxDepth: number;
  avgBranchingFactor: number;
}

interface PatternInfo {
  name: string;
  chords: composer.WasmChord[];
}

/**
 * Initialize the WASM module
 */
async function initializeWasm(): Promise<void> {
  console.log(`Initializing Composer WASM v${composer.getVersion()}`);
}

/**
 * Show trie construction and pattern storage concepts (simulated)
 */
function demonstrateTriePatternStorage(): void {
  console.log("=== Trie Pattern Storage Concepts ===");

  // Simulate trie functionality
  console.log("Simulating trie for storing chord progressions");

  const stats = {
    totalPatterns: 0,
    totalNodes: 1,
    memoryUsageBytes: 256,
    scaleBranches: 0,
    maxDepth: 0,
    avgBranchingFactor: 0,
  };
  console.log(`Initial total patterns: ${stats.totalPatterns}`);
  console.log();

  // Common chord progressions to store
  const progressions: PatternInfo[] = [
    // I-V-vi-IV (Pop progression)
    {
      name: "Pop_Progression",
      chords: [
        new composer.WasmChord(1, 5), // I
        new composer.WasmChord(5, 5), // V
        new composer.WasmChord(6, 5), // vi
        new composer.WasmChord(4, 5), // IV
      ],
    },
    // ii-V-I (Jazz progression)
    {
      name: "Jazz_ii_V_I",
      chords: [
        new composer.WasmChord(2, 7), // ii7
        new composer.WasmChord(5, 7), // V7
        new composer.WasmChord(1, 7), // IM7
      ],
    },
    // I-vi-IV-V (50s progression)
    {
      name: "Fifties_Progression",
      chords: [
        new composer.WasmChord(1, 5), // I
        new composer.WasmChord(6, 5), // vi
        new composer.WasmChord(4, 5), // IV
        new composer.WasmChord(5, 5), // V
      ],
    },
    // vi-IV-I-V (Pop variation)
    {
      name: "Pop_Variation",
      chords: [
        new composer.WasmChord(6, 5), // vi
        new composer.WasmChord(4, 5), // IV
        new composer.WasmChord(1, 5), // I
        new composer.WasmChord(5, 5), // V
      ],
    },
  ];

  console.log("Adding progressions to trie:");
  progressions.forEach((progression, index) => {
    // Serialize chords to hex for trie storage
    const pattern = progression.chords.map((chord) => chord.toHex());

    // Simulate adding pattern
    console.log(`  Added: ${progression.name} (${pattern.length} chords)`);
    console.log(`    Pattern: ${pattern.join(" -> ")}`);

    // Update simulated stats
    stats.totalPatterns++;
    stats.totalNodes += pattern.length;
    stats.memoryUsageBytes += pattern.length * 64; // Estimate
    stats.maxDepth = Math.max(stats.maxDepth, pattern.length);
  });

  // Calculate simulated ranks
  console.log("\nCalculating pattern ranks...");

  console.log("\nTrie after adding patterns:");
  stats.avgBranchingFactor = stats.totalNodes / Math.max(1, stats.totalPatterns);
  stats.scaleBranches = Math.ceil(stats.totalPatterns * 0.6);

  console.log(`  Total patterns: ${stats.totalPatterns}`);
  console.log(`  Total nodes: ${stats.totalNodes}`);
  console.log(`  Memory usage: ${stats.memoryUsageBytes} bytes`);
  console.log(`  Scale branches: ${stats.scaleBranches}`);
  console.log(`  Max depth: ${stats.maxDepth}`);
  console.log(`  Avg branching factor: ${stats.avgBranchingFactor.toFixed(2)}`);
  console.log();

  // Clean up chord objects
  progressions.forEach((progression) => {
    progression.chords.forEach((chord) => chord.free());
  });

  console.log("Note: Full trie functionality requires WasmTrieNode implementation.");
  console.log();
}

/**
 * Show chord symbol parsing concepts (simulated)
 */
function demonstrateChordSymbolParsing(): void {
  console.log("=== Chord Symbol Parsing Concepts ===");

  // Test various chord symbol formats with manual mapping
  const chordSymbols = [
    { symbol: "I", description: "Roman numeral - major tonic", root: 1, chordType: 5 },
    { symbol: "ii7", description: "Roman numeral - minor seventh", root: 2, chordType: 7 },
    { symbol: "V7", description: "Roman numeral - dominant seventh", root: 5, chordType: 7 },
    { symbol: "vii", description: "Roman numeral - diminished", root: 7, chordType: 5 },
    { symbol: "vi", description: "Roman numeral - minor sixth", root: 6, chordType: 5 },
    { symbol: "IV", description: "Roman numeral - major fourth", root: 4, chordType: 5 },
  ];

  console.log("Parsing different chord symbol formats:");
  chordSymbols.forEach(({ symbol, description, root, chordType }) => {
    console.log(`  '${symbol}' (${description}):`);

    try {
      // Create chord based on manual parsing
      const chord = new composer.WasmChord(root, chordType);
      const hexStr = chord.toHex();

      console.log(`    Parsed 1 interpretation:`);
      console.log(`      1. ${chord.toString()}`);
      console.log(`         Root: ${chord.root}, Type: ${chord.chordType}`);
      console.log(`         Hex: ${hexStr}`);

      chord.free();
    } catch (error) {
      console.log(`    Parse error: ${error}`);
    }

    console.log();
  });

  console.log("Note: Full chord symbol parsing requires parseChordSymbol implementation.");
  console.log();
}

/**
 * Show isotonal chord mapping concepts (simulated)
 */
function demonstrateIsotoanalMapping(): void {
  console.log("=== Isotonal Chord Mapping Concepts ===");

  const majorScale = composer.WasmScaleFingerprint.major();

  // Test chords for isotonal relationships
  const testChords = [
    { chord: new composer.WasmChord(1, 5), name: "C major" },
    { chord: new composer.WasmChord(6, 5), name: "A minor" },
    { chord: new composer.WasmChord(3, 5), name: "E minor" },
    { chord: new composer.WasmChord(5, 7), name: "G7" },
    { chord: new composer.WasmChord(2, 7), name: "D minor 7" },
  ];

  console.log("Testing isotonal relationships (simulated):");
  testChords.forEach(({ chord: chord1, name: name1 }, i) => {
    console.log(`  ${name1}:`);

    testChords.forEach(({ chord: chord2, name: name2 }, j) => {
      if (i !== j) {
        try {
          // Simulate isotonal analysis using scale degrees
          const degrees1 = composer.getStableScaleDegrees(chord1, majorScale);
          const degrees2 = composer.getStableScaleDegrees(chord2, majorScale);

          // Simple isotonal check: share scale degrees
          const sharedDegrees = degrees1.filter((d: string) => degrees2.includes(d));
          const isIsotonal = sharedDegrees.length >= 2; // Share at least 2 degrees

          const isotoanalText = isIsotonal ? "Isotonal" : "Not isotonal";
          console.log(
            `    vs ${name2}: ${isotoanalText} (shared degrees: ${sharedDegrees.length})`
          );
        } catch (error) {
          console.log(`    vs ${name2}: Error - ${error}`);
        }
      }
    });
    console.log();
  });

  console.log("Note: Full isotonal analysis requires isIsotonal function implementation.");
  console.log();

  // Clean up objects
  testChords.forEach(({ chord }) => chord.free());
  majorScale.free();
}

/**
 * Show bass line pattern analysis
 */
function demonstrateBassLineAnalysis(): void {
  console.log("=== Bass Line Pattern Analysis ===");

  // Create a chord progression for bass analysis
  const progression = [
    new composer.WasmChord(1, 5), // C major
    new composer.WasmChord(6, 7), // Am7
    new composer.WasmChord(4, 5), // F major
    new composer.WasmChord(5, 7), // G7
  ];

  const majorScale = composer.WasmScaleFingerprint.major();

  console.log("Analyzing bass line patterns:");
  console.log(`Progression: ${progression.map((c) => c.toString())}`);
  console.log();

  progression.forEach((chord, i) => {
    console.log(`  ${i + 1}. ${chord.toString()}:`);

    try {
      // Get scale degrees for bass note analysis
      const scaleDegrees = composer.getStableScaleDegrees(chord, majorScale);
      const romanNumeral = composer.getRomanNumeral(chord, majorScale);
      const complexity = composer.getChordComplexity(chord, "major");

      console.log(`    Roman numeral: ${romanNumeral}`);
      console.log(`    Scale degrees: [${scaleDegrees.join(", ")}]`);
      console.log(`    Complexity: ${complexity}`);

      // Estimate bass note from chord root (simplified)
      const bassEstimate = chord.root; // In scale degrees
      console.log(`    Estimated bass scale degree: ${bassEstimate}`);
    } catch (error) {
      console.log(`    Analysis error: ${error}`);
    }

    console.log();
  });

  // Clean up objects
  progression.forEach((chord) => chord.free());
  majorScale.free();
}

/**
 * Show advanced chord substitution techniques
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
 * Show cross-pattern analysis and recognition
 */
function demonstrateCrossPatternAnalysis(): void {
  console.log("=== Cross-Pattern Analysis ===");

  // Create several different progression patterns
  const patterns = [
    {
      name: "Pop Progression",
      chords: [
        new composer.WasmChord(1, 5), // I
        new composer.WasmChord(5, 5), // V
        new composer.WasmChord(6, 5), // vi
        new composer.WasmChord(4, 5), // IV
      ],
    },
    {
      name: "Jazz Progression",
      chords: [
        new composer.WasmChord(2, 7), // ii7
        new composer.WasmChord(5, 7), // V7
        new composer.WasmChord(1, 7), // IM7
      ],
    },
    {
      name: "Circle of Fifths",
      chords: [
        new composer.WasmChord(1, 7), // IM7
        new composer.WasmChord(4, 7), // IVM7
        new composer.WasmChord(7, 7), // vii7
        new composer.WasmChord(3, 7), // iii7
      ],
    },
  ];

  const majorScale = composer.WasmScaleFingerprint.major();

  console.log("Analyzing pattern characteristics:");
  patterns.forEach(({ name, chords }) => {
    console.log(`\n  ${name}:`);
    console.log(`    Chords: ${chords.map((c) => c.toString())}`);

    // Calculate average complexity
    let totalComplexity = 0;
    chords.forEach((chord) => {
      try {
        totalComplexity += composer.getChordComplexity(chord, "major");
      } catch (error) {
        console.log(`    Error calculating complexity: ${error}`);
      }
    });

    const avgComplexity = totalComplexity / chords.length;
    console.log(`    Average complexity: ${avgComplexity.toFixed(2)}`);

    // Analyze harmonic movement
    const roots = chords.map((c) => c.root);
    let totalMovement = 0;
    for (let i = 1; i < roots.length; i++) {
      totalMovement += Math.abs(roots[i] - roots[i - 1]);
    }
    const avgMovement = totalMovement / (roots.length - 1);
    console.log(`    Average harmonic movement: ${avgMovement.toFixed(2)} scale degrees`);

    // Check for common patterns
    const hasV7 = chords.some((c) => c.root === 5 && c.chordType === 7);
    const hasIM7 = chords.some((c) => c.root === 1 && c.chordType === 7);
    console.log(`    Contains V7: ${hasV7}`);
    console.log(`    Contains IM7: ${hasIM7}`);
  });

  // Clean up all chord objects
  patterns.forEach(({ chords }) => {
    chords.forEach((chord) => chord.free());
  });
  majorScale.free();
}

/**
 * Run all pattern matching demonstration functions
 */
async function main(): Promise<void> {
  console.log("Composer Library - Pattern Matching and Chord Lookup (TypeScript)");
  console.log("=".repeat(75));
  console.log();

  try {
    await initializeWasm();

    demonstrateTriePatternStorage();
    demonstrateChordSymbolParsing();
    demonstrateIsotoanalMapping();
    demonstrateBassLineAnalysis();
    demonstrateChordSubstitutions();
    demonstrateCrossPatternAnalysis();

    console.log("All pattern matching examples completed!");
    console.log();
    console.log("Note: Some advanced pattern matching features may require additional");
    console.log("WASM bindings to be fully functional. This example demonstrates the");
    console.log("current available functionality based on the WASM interface.");
  } catch (error) {
    console.error("Error running pattern matching examples:", error);
  }
}

// Run if executed directly
if (require.main === module) {
  main().catch(console.error);
}

// Export functions for potential reuse
export {
  demonstrateTriePatternStorage,
  demonstrateChordSymbolParsing,
  demonstrateIsotoanalMapping,
  demonstrateBassLineAnalysis,
  demonstrateChordSubstitutions,
  demonstrateCrossPatternAnalysis,
};
