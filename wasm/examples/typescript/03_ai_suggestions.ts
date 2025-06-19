#!/usr/bin/env ts-node
/**
 * AI-Powered Chord Suggestions
 *
 * This example demonstrates the AI engine for chord progression suggestions:
 * - Initializing the AI engine with training data
 * - Context-aware chord suggestions
 * - Magic chord solutions using statistical weighting
 * - Bass harmonization algorithms
 * - Scale degree harmonization
 *
 * Based on the Composer specification: ai-powered-features.spec
 */

import * as composer from "../../composer_wasm";

interface TrainingPattern {
  chords: string[]; // Hex-encoded chords
  name: string;
  key?: string;
}

interface ChordSuggestion {
  chordHex: string;
  confidence: number;
  weightedScore: number;
  reasoning: string;
}

interface DifficultyAssessment {
  overallScore: number;
  skillLevel: string;
  confidence: number;
}

interface EngineMetrics {
  totalRequests: number;
  avgResponseTimeMs: number;
  memoryUsageBytes: number;
  cacheHitRate: number;
  totalPatterns: number;
  uptimeSeconds: number;
}

/**
 * Initialize the WASM module
 */
async function initializeWasm(): Promise<void> {
  console.log(`Initializing Composer WASM v${composer.getVersion()}`);
}

/**
 * Create sample chord progressions for training the AI engine
 */
function createSampleTrainingData(): TrainingPattern[] {
  console.log("=== Creating Training Data ===");

  // Common chord progressions in various keys
  const progressions: TrainingPattern[] = [];

  // I-V-vi-IV progression in C major
  const popProgression = {
    chords: [
      new composer.WasmChord(1, 5).toHex(), // I
      new composer.WasmChord(5, 5).toHex(), // V
      new composer.WasmChord(6, 5).toHex(), // vi
      new composer.WasmChord(4, 5).toHex(), // IV
    ],
    name: "pop_progression_1",
    key: "C",
  };

  // ii-V-I progression in C major
  const jazzProgression = {
    chords: [
      new composer.WasmChord(2, 7).toHex(), // ii7
      new composer.WasmChord(5, 7).toHex(), // V7
      new composer.WasmChord(1, 5).toHex(), // I
    ],
    name: "jazz_ii_v_i",
    key: "C",
  };

  // I-vi-IV-V progression in C major
  const classicProgression = {
    chords: [
      new composer.WasmChord(1, 5).toHex(), // I
      new composer.WasmChord(6, 5).toHex(), // vi
      new composer.WasmChord(4, 5).toHex(), // IV
      new composer.WasmChord(5, 5).toHex(), // V
    ],
    name: "classic_progression",
    key: "C",
  };

  // Minor progression: i-VII-VI-VII in A minor
  const minorProgression = {
    chords: [
      new composer.WasmChord(1, 5).toHex(), // i
      new composer.WasmChord(7, 5).toHex(), // VII
      new composer.WasmChord(6, 5).toHex(), // VI
      new composer.WasmChord(7, 5).toHex(), // VII
    ],
    name: "minor_progression",
    key: "Am",
  };

  // Jazz progression: IM7-VI7-ii7-V7 in C major
  const jazzCircle = {
    chords: [
      new composer.WasmChord(1, 7).toHex(), // IM7
      new composer.WasmChord(6, 7).toHex(), // VI7
      new composer.WasmChord(2, 7).toHex(), // ii7
      new composer.WasmChord(5, 7).toHex(), // V7
    ],
    name: "jazz_circle",
    key: "C",
  };

  progressions.push(
    popProgression,
    jazzProgression,
    classicProgression,
    minorProgression,
    jazzCircle
  );

  console.log(`Created ${progressions.length} training progressions`);
  progressions.forEach((prog, i) => {
    console.log(`  ${i + 1}. ${prog.name} in ${prog.key}: ${prog.chords.length} chords`);
  });

  console.log();
  return progressions;
}

/**
 * Demonstrate AI-powered features (simulated with available functionality)
 */
function demonstrateAiFeatures(): void {
  console.log("=== AI-Powered Features (Simulated) ===");

  // Since the full AI engine isn't available in the current WASM build,
  // we'll demonstrate the concepts using the available chord analysis functions

  console.log("Demonstrating chord analysis and suggestion concepts:");

  // Create sample training data
  const trainingData = createSampleTrainingData();
  console.log(`Created ${trainingData.length} sample progressions for analysis`);

  // Analyze complexity of different chord types for difficulty assessment
  const chordTypes = [
    { chord: new composer.WasmChord(1, 5), name: "I (simple triad)" },
    { chord: new composer.WasmChord(2, 7), name: "ii7 (moderate complexity)" },
    { chord: new composer.WasmChord(5, 9), name: "V9 (high complexity)" },
  ];

  console.log("\nChord complexity analysis (basis for difficulty assessment):");
  chordTypes.forEach(({ chord, name }) => {
    try {
      const complexity = composer.getChordComplexity(chord, "major");
      console.log(`  ${name}: complexity ${complexity.toFixed(2)}`);
      chord.free();
    } catch (error) {
      console.log(`  ${name}: Error - ${error}`);
    }
  });

  console.log("\nNote: Full AI engine with suggestion algorithms requires additional");
  console.log("WASM bindings that would include pattern matching and statistical models.");
  console.log();
}

/**
 * Get and analyze magic chord solutions
 */
function demonstrateMagicChordConcepts(): void {
  console.log("=== Magic Chord Solutions (Concept) ===");

  // Define previous and following chords
  const previousChords = [
    new composer.WasmChord(1, 5), // C major
    new composer.WasmChord(5, 5), // G major
  ];

  const followingChords = [
    new composer.WasmChord(1, 5), // C major (back to tonic)
  ];

  console.log("Finding magic chord between:");
  console.log(`  Previous: ${previousChords.map((c) => c.toString())}`);
  console.log(`  Following: ${followingChords.map((c) => c.toString())}`);
  console.log();

  // Demonstrate chord analysis that would be used in magic chord algorithms
  const candidateChords = [
    new composer.WasmChord(4, 5), // IV
    new composer.WasmChord(2, 7), // ii7
    new composer.WasmChord(6, 5), // vi
  ];

  console.log("Analyzing candidate connecting chords:");
  candidateChords.forEach((chord, i) => {
    try {
      const complexity = composer.getChordComplexity(chord, "major");
      const majorScale = composer.WasmScaleFingerprint.major();
      const roman = composer.getRomanNumeral(chord, majorScale);

      console.log(`  ${i + 1}. ${chord.toString()} (${roman})`);
      console.log(`     Complexity: ${complexity.toFixed(3)}`);
      console.log(`     Harmonic weight: ${(10 - complexity) / 10}`);

      majorScale.free();
    } catch (error) {
      console.log(`  ${i + 1}. Error analyzing chord: ${error}`);
    }
  });

  console.log("\nNote: Full magic chord algorithm would use statistical models");
  console.log("and pattern matching to suggest optimal connecting chords.");
  console.log();

  // Clean up
  [...previousChords, ...followingChords, ...candidateChords].forEach((c) => c.free());
}

/**
 * Show bass harmonization concepts (simulated)
 */
function demonstrateBassHarmonization(): void {
  console.log("=== Bass Harmonization Concepts ===");

  console.log("Demonstrating bass harmonization for C bass note:");

  // Common chords that work well with C bass
  const bassSolutions = [
    { chord: new composer.WasmChord(1, 5), confidence: 0.95, reasoning: "Root position C major" },
    {
      chord: new composer.WasmChord(4, 5),
      confidence: 0.8,
      reasoning: "F major with C bass (first inversion)",
    },
    {
      chord: new composer.WasmChord(6, 7),
      confidence: 0.75,
      reasoning: "A minor 7 with C bass (♭3)",
    },
  ];

  bassSolutions.forEach((solution, i) => {
    try {
      console.log(`  ${i + 1}. ${solution.chord.toString()} for C bass`);
      console.log(`     Confidence: ${solution.confidence.toFixed(3)}`);
      console.log(`     Reasoning: ${solution.reasoning}`);
      console.log();
      solution.chord.free();
    } catch (error) {
      console.log(`  ${i + 1}. Error displaying bass solution: ${error}`);
    }
  });

  console.log("Note: Full bass harmonization requires AI engine with statistical models.");
  console.log();
}

/**
 * Show scale degree harmonization concepts (simulated)
 */
function demonstrateScaleDegreeHarmonization(): void {
  console.log("=== Scale Degree Harmonization Concepts ===");

  // Test different scale degree combinations (as bit patterns)
  const scalePatterns = [
    { bits: 0b000000001001, description: "Root and Fifth", chords: [new composer.WasmChord(1, 5)] },
    {
      bits: 0b000100001001,
      description: "Major Triad",
      chords: [new composer.WasmChord(1, 5), new composer.WasmChord(4, 5)],
    },
    {
      bits: 0b001100001001,
      description: "Seventh Chord",
      chords: [new composer.WasmChord(1, 7), new composer.WasmChord(5, 7)],
    },
    {
      bits: 0b000010001001,
      description: "Sus2",
      chords: [new composer.WasmChord(2, 5), new composer.WasmChord(5, 5)],
    },
    {
      bits: 0b000001001001,
      description: "Sus4",
      chords: [new composer.WasmChord(4, 5), new composer.WasmChord(1, 5)],
    },
  ];

  scalePatterns.forEach(({ bits, description, chords }) => {
    console.log(`\nHarmonizing ${description} (bits: ${bits.toString(2).padStart(12, "0")}):`);

    chords.forEach((chord, i) => {
      try {
        const complexity = composer.getChordComplexity(chord, "major");
        const confidence = Math.max(0.1, 1.0 - complexity / 10);
        const score = confidence * (1.0 + Math.random() * 0.3);

        console.log(`  ${i + 1}. ${chord.toString()}`);
        console.log(`     Confidence: ${confidence.toFixed(3)}`);
        console.log(`     Score: ${score.toFixed(3)}`);
        console.log(`     Reasoning: Harmonizes scale degrees for ${description}`);
      } catch (error) {
        console.log(`  ${i + 1}. Error analyzing chord: ${error}`);
      }
    });

    // Clean up chords
    chords.forEach((chord) => chord.free());
  });

  console.log("\nNote: Full scale degree harmonization requires AI engine with pattern matching.");
  console.log();
}

/**
 * Show difficulty assessment concepts (simulated)
 */
function demonstrateDifficultyAssessment(): void {
  console.log("=== Difficulty Assessment Concepts ===");

  // Test progressions with different complexity levels
  const progressions = [
    // Simple progression
    {
      chords: [
        new composer.WasmChord(1, 5), // I
        new composer.WasmChord(5, 5), // V
        new composer.WasmChord(1, 5), // I
      ],
      name: "Simple I-V-I",
    },
    // Moderate progression
    {
      chords: [
        new composer.WasmChord(1, 5), // I
        new composer.WasmChord(6, 7), // vi7
        new composer.WasmChord(4, 5), // IV
        new composer.WasmChord(5, 5), // V
      ],
      name: "Pop progression",
    },
    // Complex jazz progression
    {
      chords: [
        new composer.WasmChord(1, 7), // IM7
        new composer.WasmChord(6, 7), // VI7
        new composer.WasmChord(2, 7), // ii7
        new composer.WasmChord(5, 7), // V7
      ],
      name: "Jazz progression",
    },
  ];

  progressions.forEach(({ chords, name }) => {
    console.log(`\nAssessing: ${name}`);
    console.log(`Chords: ${chords.map((c) => c.toString())}`);

    try {
      // Calculate overall complexity using available functions
      const complexities = chords.map((chord) => composer.getChordComplexity(chord, "major"));
      const avgComplexity = complexities.reduce((sum, c) => sum + c, 0) / complexities.length;

      // Simulate difficulty assessment
      const overallScore = Math.min(10, avgComplexity * 1.5 + Math.random() * 0.5);
      const skillLevel =
        overallScore < 3 ? "Beginner" : overallScore < 6 ? "Intermediate" : "Advanced";
      const confidence = Math.max(0.7, 1.0 - Math.abs(avgComplexity - 3) / 10);

      console.log(`  Overall score: ${overallScore.toFixed(1)}/10`);
      console.log(`  Skill level: ${skillLevel}`);
      console.log(`  Confidence: ${confidence.toFixed(3)}`);
      console.log(`  Average complexity: ${avgComplexity.toFixed(2)}`);
    } catch (error) {
      console.log(`  Error: ${error}`);
    }

    // Clean up chords
    chords.forEach((chord) => chord.free());
  });

  console.log("\nNote: Full difficulty assessment requires AI engine with statistical models.");
  console.log();
}

/**
 * Show performance metrics concepts (simulated)
 */
function demonstratePerformanceMetrics(): void {
  console.log("=== Performance Metrics Concepts ===");

  // Simulate engine metrics
  const metrics = {
    totalRequests: 1250,
    avgResponseTimeMs: 0.8,
    memoryUsageBytes: 45 * 1024 * 1024, // 45MB
    cacheHitRate: 0.892,
    totalPatterns: 15000,
    uptimeSeconds: 3600,
  };

  console.log("Simulated AI engine metrics:");
  console.log(`  Total requests: ${metrics.totalRequests}`);
  console.log(`  Average response time: ${metrics.avgResponseTimeMs.toFixed(2)} ms`);
  console.log(`  Memory usage: ${metrics.memoryUsageBytes.toLocaleString()} bytes`);
  console.log(`  Cache hit rate: ${(metrics.cacheHitRate * 100).toFixed(1)}%`);
  console.log(`  Total patterns: ${metrics.totalPatterns}`);
  console.log(`  Uptime: ${metrics.uptimeSeconds} seconds`);

  console.log("\nNote: Full performance monitoring requires AI engine implementation.");
  console.log();
}

/**
 * Run all AI suggestion demonstration functions
 */
async function main(): Promise<void> {
  console.log("Composer Library - AI-Powered Chord Suggestions (TypeScript)");
  console.log("=".repeat(70));
  console.log();

  try {
    await initializeWasm();

    // Demonstrate available functionality
    demonstrateAiFeatures();
    demonstrateMagicChordConcepts();
    demonstrateComplexityAnalysis();
    demonstrateHarmonicAnalysis();
    demonstrateBassHarmonization();
    demonstrateScaleDegreeHarmonization();
    demonstrateDifficultyAssessment();
    demonstratePerformanceMetrics();

    console.log("=== Summary ===");
    console.log("Demonstrated core music theory concepts that form the basis for:");
    console.log("- AI-powered chord suggestions");
    console.log("- Difficulty assessment algorithms");
    console.log("- Pattern matching and analysis");
    console.log("- Advanced harmonic analysis");

    console.log("\nAll AI examples completed successfully!");
  } catch (error) {
    console.error("Error running AI examples:", error);
  }
}

// Run if executed directly
if (require.main === module) {
  main().catch(console.error);
}

/**
 * Demonstrate complexity analysis for difficulty assessment
 */
function demonstrateComplexityAnalysis(): void {
  console.log("=== Complexity Analysis (Difficulty Assessment Basis) ===");

  const progressions = [
    {
      name: "Simple I-V-I",
      chords: [
        new composer.WasmChord(1, 5),
        new composer.WasmChord(5, 5),
        new composer.WasmChord(1, 5),
      ],
    },
    {
      name: "Jazz ii-V-I",
      chords: [
        new composer.WasmChord(2, 7),
        new composer.WasmChord(5, 7),
        new composer.WasmChord(1, 7),
      ],
    },
  ];

  progressions.forEach(({ name, chords }) => {
    console.log(`\n${name}:`);
    let totalComplexity = 0;

    chords.forEach((chord, i) => {
      try {
        const complexity = composer.getChordComplexity(chord, "major");
        console.log(`  ${i + 1}. ${chord.toString()}: complexity ${complexity}`);
        totalComplexity += complexity;
      } catch (error) {
        console.log(`  ${i + 1}. Error: ${error}`);
      }
    });

    const avgComplexity = totalComplexity / chords.length;
    const difficultyLevel =
      avgComplexity < 2 ? "Beginner" : avgComplexity < 4 ? "Intermediate" : "Advanced";
    console.log(`  Average complexity: ${avgComplexity.toFixed(2)} (${difficultyLevel})`);

    chords.forEach((c) => c.free());
  });

  console.log();
}

/**
 * Demonstrate harmonic analysis capabilities
 */
function demonstrateHarmonicAnalysis(): void {
  console.log("=== Harmonic Analysis ===");

  const progression = [
    new composer.WasmChord(1, 5), // I
    new composer.WasmChord(6, 5), // vi
    new composer.WasmChord(4, 5), // IV
    new composer.WasmChord(5, 5), // V
  ];

  const majorScale = composer.WasmScaleFingerprint.major();

  console.log("Analyzing I-vi-IV-V progression:");
  progression.forEach((chord, i) => {
    try {
      const roman = composer.getRomanNumeral(chord, majorScale);
      const degrees = composer.getStableScaleDegrees(chord, majorScale);
      const complexity = composer.getChordComplexity(chord, "major");

      console.log(`  ${i + 1}. ${chord.toString()} = ${roman}`);
      console.log(`     Scale degrees: [${degrees.join(", ")}]`);
      console.log(`     Complexity: ${complexity}`);
    } catch (error) {
      console.log(`  ${i + 1}. Analysis error: ${error}`);
    }
  });

  console.log();

  // Clean up
  progression.forEach((c) => c.free());
  majorScale.free();
}

// Export functions for potential reuse
export {
  demonstrateAiFeatures,
  demonstrateMagicChordConcepts,
  demonstrateComplexityAnalysis,
  demonstrateHarmonicAnalysis,
};
