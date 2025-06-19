#!/usr/bin/env ts-node
/**
 * Complete Composition Workflow
 *
 * This example demonstrates a complete end-to-end composition workflow:
 * - Creating chord progressions
 * - Analyzing harmonic relationships
 * - Applying music theory rules
 * - Serializing for storage/transmission
 * - Building complete musical phrases
 *
 * Based on the Composer specification: complete music theory workflow
 */

import * as composer from "../../composer_wasm.js";

// Types for better TypeScript experience
interface ChordProgression {
  name: string;
  chords: { chord: composer.WasmChord; roman: string; scaleDegrees: string[] }[];
  key: string;
  scale: composer.WasmScaleFingerprint;
}

interface AnalysisResult {
  totalComplexity: number;
  averageComplexity: number;
  harmonicFunction: string;
  serializedData: string[];
}

/**
 * Initialize the WASM module
 */
async function initializeWasm(): Promise<void> {
  console.log(`Initializing Composer WASM v${composer.getVersion()}`);
}

/**
 * Create a complete chord progression with analysis
 */
function createChordProgression(
  name: string,
  chordSpecs: { root: number; type: number }[],
  key: string,
  scale: composer.WasmScaleFingerprint
): ChordProgression {
  const chords = chordSpecs.map(({ root, type }) => {
    const chord = new composer.WasmChord(root, type);
    const roman = composer.getRomanNumeral(chord, scale);
    const scaleDegrees = composer.getStableScaleDegrees(chord, scale);

    return { chord, roman, scaleDegrees };
  });

  return { name, chords, key, scale };
}

/**
 * Analyze a chord progression
 */
function analyzeProgression(progression: ChordProgression): AnalysisResult {
  const complexities = progression.chords.map(({ chord }) =>
    composer.getChordComplexity(chord, "major")
  );

  const totalComplexity = complexities.reduce((sum, c) => sum + c, 0);
  const averageComplexity = totalComplexity / complexities.length;

  const serializedData = progression.chords.map(({ chord }) => chord.toHex());

  // Simple harmonic function analysis based on Roman numerals
  const romans = progression.chords.map(({ roman }) => roman);
  let harmonicFunction = "Unknown";

  if (romans.includes("I") && romans.includes("V")) {
    if (romans.includes("vi") && romans.includes("IV")) {
      harmonicFunction = "Pop Progression (I-V-vi-IV family)";
    } else if (romans.includes("ii7")) {
      harmonicFunction = "Jazz Progression (ii-V-I family)";
    } else {
      harmonicFunction = "Classical Progression";
    }
  } else if (romans.every((r) => ["i", "iv", "v", "VI", "VII"].includes(r))) {
    harmonicFunction = "Minor Key Progression";
  }

  return {
    totalComplexity,
    averageComplexity,
    harmonicFunction,
    serializedData,
  };
}

/**
 * Clean up memory for a chord progression
 */
function cleanupProgression(progression: ChordProgression): void {
  progression.chords.forEach(({ chord }) => chord.free());
  progression.scale.free();
}

/**
 * Demonstrate a complete pop song structure
 */
function demonstratePopSongWorkflow(): void {
  console.log("=== Complete Pop Song Workflow ===");

  const majorScale = composer.WasmScaleFingerprint.major();

  // Define sections of a pop song
  const sections = [
    {
      name: "Verse",
      chordSpecs: [
        { root: 6, type: 5 }, // vi (Am)
        { root: 4, type: 5 }, // IV (F)
        { root: 1, type: 5 }, // I (C)
        { root: 5, type: 5 }, // V (G)
      ],
    },
    {
      name: "Chorus",
      chordSpecs: [
        { root: 1, type: 5 }, // I (C)
        { root: 5, type: 5 }, // V (G)
        { root: 6, type: 5 }, // vi (Am)
        { root: 4, type: 5 }, // IV (F)
      ],
    },
    {
      name: "Bridge",
      chordSpecs: [
        { root: 4, type: 5 }, // IV (F)
        { root: 5, type: 5 }, // V (G)
        { root: 3, type: 5 }, // iii (Em)
        { root: 6, type: 5 }, // vi (Am)
      ],
    },
  ];

  console.log("Analyzing pop song structure in C major:");
  console.log();

  sections.forEach((section) => {
    console.log(`${section.name.toUpperCase()}:`);

    const progression = createChordProgression(
      section.name,
      section.chordSpecs,
      "C major",
      majorScale
    );

    // Display chord progression
    progression.chords.forEach(({ chord, roman, scaleDegrees }, i) => {
      console.log(
        `  ${i + 1}. ${chord.toString().padEnd(6)} (${roman.padEnd(4)}) - degrees: [${scaleDegrees.join(", ")}]`
      );
    });

    const analysis = analyzeProgression(progression);
    console.log(`  Analysis: ${analysis.harmonicFunction}`);
    console.log(`  Complexity: ${analysis.averageComplexity.toFixed(2)} average`);
    console.log(`  Serialized: [${analysis.serializedData.join(", ")}]`);
    console.log();

    cleanupProgression(progression);
  });
}

/**
 * Demonstrate jazz progression workflow
 */
function demonstrateJazzWorkflow(): void {
  console.log("=== Jazz Progression Workflow ===");

  const majorScale = composer.WasmScaleFingerprint.major();

  // Common jazz progressions
  const jazzProgressions = [
    {
      name: "ii-V-I in C",
      chordSpecs: [
        { root: 2, type: 7 }, // Dm7
        { root: 5, type: 7 }, // G7
        { root: 1, type: 5 }, // Cmaj
      ],
    },
    {
      name: "Circle of Fifths",
      chordSpecs: [
        { root: 1, type: 7 }, // C7
        { root: 6, type: 7 }, // F7
        { root: 11, type: 7 }, // Bb7
        { root: 4, type: 7 }, // Eb7
      ],
    },
    {
      name: "Rhythm Changes (A section)",
      chordSpecs: [
        { root: 1, type: 5 }, // C
        { root: 6, type: 5 }, // Am
        { root: 2, type: 7 }, // Dm7
        { root: 5, type: 7 }, // G7
      ],
    },
  ];

  console.log("Analyzing jazz progressions:");
  console.log();

  jazzProgressions.forEach((progressionSpec) => {
    console.log(`${progressionSpec.name.toUpperCase()}:`);

    const progression = createChordProgression(
      progressionSpec.name,
      progressionSpec.chordSpecs,
      "C major",
      majorScale
    );

    // Show detailed harmonic analysis
    progression.chords.forEach(({ chord, roman, scaleDegrees }, i) => {
      const complexity = composer.getChordComplexity(chord, "major");
      const chordType = chord.isSeventh() ? "7th" : chord.isTriad() ? "triad" : "extended";
      console.log(
        `  ${i + 1}. ${chord.toString().padEnd(8)} (${roman.padEnd(6)}) - ${chordType}, complexity: ${complexity}, degrees: [${scaleDegrees.join(", ")}]`
      );
    });

    const analysis = analyzeProgression(progression);
    console.log(`  Function: ${analysis.harmonicFunction}`);
    console.log(`  Total complexity: ${analysis.totalComplexity.toFixed(2)}`);
    console.log(`  Data: ${analysis.serializedData.join("-")}`);
    console.log();

    cleanupProgression(progression);
  });
}

/**
 * Demonstrate modal composition workflow
 */
function demonstrateModalWorkflow(): void {
  console.log("=== Modal Composition Workflow ===");

  // Create modal scales
  const dorianScale = composer.WasmScaleFingerprint.fromArray(
    new Uint8Array([1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0]) // Dorian mode
  );

  const mixolydianScale = composer.WasmScaleFingerprint.fromArray(
    new Uint8Array([1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0]) // Mixolydian mode
  );

  const modalProgressions = [
    {
      name: "Dorian Progression",
      scale: dorianScale,
      chordSpecs: [
        { root: 1, type: 5 }, // i (Dm in D Dorian)
        { root: 4, type: 5 }, // IV (G)
        { root: 7, type: 5 }, // bVII (C)
        { root: 1, type: 5 }, // i (Dm)
      ],
    },
    {
      name: "Mixolydian Progression",
      scale: mixolydianScale,
      chordSpecs: [
        { root: 1, type: 5 }, // I (G in G Mixolydian)
        { root: 7, type: 5 }, // bVII (F)
        { root: 4, type: 5 }, // IV (C)
        { root: 1, type: 5 }, // I (G)
      ],
    },
  ];

  console.log("Analyzing modal progressions:");
  console.log();

  modalProgressions.forEach((progressionSpec) => {
    console.log(`${progressionSpec.name.toUpperCase()}:`);
    console.log(`  Scale: ${progressionSpec.scale.toString()}`);

    const progression = createChordProgression(
      progressionSpec.name,
      progressionSpec.chordSpecs,
      "Modal",
      progressionSpec.scale
    );

    progression.chords.forEach(({ chord, roman, scaleDegrees }, i) => {
      const complexity = composer.getChordComplexity(chord, "major");
      console.log(
        `  ${i + 1}. ${chord.toString().padEnd(6)} (${roman.padEnd(6)}) - complexity: ${complexity}, degrees: [${scaleDegrees.join(", ")}]`
      );
    });

    const analysis = analyzeProgression(progression);
    console.log(
      `  Character: Modal harmony, avg complexity: ${analysis.averageComplexity.toFixed(2)}`
    );
    console.log(`  Encoded: ${analysis.serializedData.join(" ")}`);
    console.log();

    cleanupProgression(progression);
  });
}

/**
 * Demonstrate chord substitution workflow
 */
function demonstrateSubstitutionWorkflow(): void {
  console.log("=== Chord Substitution Workflow ===");

  const majorScale = composer.WasmScaleFingerprint.major();

  // Original progression
  const originalSpecs = [
    { root: 1, type: 5 }, // I
    { root: 6, type: 5 }, // vi
    { root: 4, type: 5 }, // IV
    { root: 5, type: 5 }, // V
  ];

  // Substitution variants
  const substitutions = [
    {
      name: "Original",
      specs: originalSpecs,
    },
    {
      name: "With 7ths",
      specs: [
        { root: 1, type: 7 }, // Imaj7
        { root: 6, type: 7 }, // vi7
        { root: 4, type: 7 }, // IVmaj7
        { root: 5, type: 7 }, // V7
      ],
    },
    {
      name: "Jazz Substitutions",
      specs: [
        { root: 1, type: 7 }, // Imaj7
        { root: 3, type: 7 }, // iii7 (substitute for vi)
        { root: 2, type: 7 }, // ii7 (substitute for IV)
        { root: 5, type: 7 }, // V7
      ],
    },
  ];

  console.log("Chord substitution analysis:");
  console.log();

  substitutions.forEach((substitution) => {
    console.log(`${substitution.name.toUpperCase()}:`);

    const progression = createChordProgression(
      substitution.name,
      substitution.specs,
      "C major",
      majorScale
    );

    const chordStrings = progression.chords
      .map(({ chord, roman }, i) => `${chord.toString()}(${roman})`)
      .join(" - ");

    const analysis = analyzeProgression(progression);

    console.log(`  Progression: ${chordStrings}`);
    console.log(`  Complexity: ${analysis.averageComplexity.toFixed(2)} average`);
    console.log(`  Function: ${analysis.harmonicFunction}`);
    console.log(`  Binary data: ${analysis.serializedData.join(" ")}`);
    console.log();

    cleanupProgression(progression);
  });
}

/**
 * Demonstrate complete song analysis and storage
 */
function demonstrateCompleteAnalysisWorkflow(): void {
  console.log("=== Complete Song Analysis & Storage ===");

  const majorScale = composer.WasmScaleFingerprint.major();

  // Simulate a complete song structure
  const songStructure = [
    {
      section: "Intro",
      chords: [
        { root: 1, type: 5 },
        { root: 5, type: 5 },
      ],
    },
    {
      section: "Verse 1",
      chords: [
        { root: 6, type: 5 },
        { root: 4, type: 5 },
        { root: 1, type: 5 },
        { root: 5, type: 5 },
      ],
    },
    {
      section: "Chorus 1",
      chords: [
        { root: 1, type: 5 },
        { root: 5, type: 5 },
        { root: 6, type: 5 },
        { root: 4, type: 5 },
      ],
    },
    {
      section: "Verse 2",
      chords: [
        { root: 6, type: 5 },
        { root: 4, type: 5 },
        { root: 1, type: 5 },
        { root: 5, type: 5 },
      ],
    },
    {
      section: "Chorus 2",
      chords: [
        { root: 1, type: 5 },
        { root: 5, type: 5 },
        { root: 6, type: 5 },
        { root: 4, type: 5 },
      ],
    },
    {
      section: "Bridge",
      chords: [
        { root: 4, type: 5 },
        { root: 5, type: 5 },
        { root: 3, type: 5 },
        { root: 6, type: 5 },
      ],
    },
    {
      section: "Final Chorus",
      chords: [
        { root: 1, type: 5 },
        { root: 5, type: 5 },
        { root: 6, type: 5 },
        { root: 4, type: 5 },
      ],
    },
    {
      section: "Outro",
      chords: [
        { root: 4, type: 5 },
        { root: 1, type: 5 },
      ],
    },
  ];

  console.log("Complete song analysis:");
  console.log();

  let totalChords = 0;
  let totalComplexity = 0;
  const allSerializedData: string[] = [];
  const sectionAnalyses: { section: string; complexity: number; data: string }[] = [];

  songStructure.forEach(({ section, chords }) => {
    const progression = createChordProgression(section, chords, "C major", majorScale);

    const analysis = analyzeProgression(progression);
    totalChords += chords.length;
    totalComplexity += analysis.totalComplexity;
    allSerializedData.push(...analysis.serializedData);

    const romanNumerals = progression.chords.map(({ roman }) => roman).join("-");
    const sectionData = analysis.serializedData.join("");

    console.log(`${section.toUpperCase()}:`);
    console.log(`  Chords: ${romanNumerals}`);
    console.log(`  Complexity: ${analysis.averageComplexity.toFixed(2)}`);
    console.log(`  Data: ${sectionData}`);

    sectionAnalyses.push({
      section,
      complexity: analysis.averageComplexity,
      data: sectionData,
    });

    cleanupProgression(progression);
  });

  // Summary analysis
  const averageSongComplexity = totalComplexity / totalChords;
  const totalDataSize = allSerializedData.join("").length;

  console.log();
  console.log("SONG SUMMARY:");
  console.log(`  Total sections: ${songStructure.length}`);
  console.log(`  Total chords: ${totalChords}`);
  console.log(`  Average complexity: ${averageSongComplexity.toFixed(2)}`);
  console.log(`  Data size: ${totalDataSize} characters`);
  console.log(`  Compression ratio: ${((totalChords * 8) / totalDataSize).toFixed(2)}:1`); // Assuming 8 chars average per chord name

  // Most/least complex sections
  const sortedByComplexity = sectionAnalyses.sort((a, b) => b.complexity - a.complexity);
  console.log(
    `  Most complex section: ${sortedByComplexity[0].section} (${sortedByComplexity[0].complexity.toFixed(2)})`
  );
  console.log(
    `  Simplest section: ${sortedByComplexity[sortedByComplexity.length - 1].section} (${sortedByComplexity[sortedByComplexity.length - 1].complexity.toFixed(2)})`
  );

  console.log();
}

/**
 * Run all workflow demonstration functions
 */
async function main(): Promise<void> {
  console.log("Composer Library - Complete Workflow Examples (TypeScript)");
  console.log("=".repeat(75));
  console.log();

  try {
    await initializeWasm();

    demonstratePopSongWorkflow();
    demonstrateJazzWorkflow();
    demonstrateModalWorkflow();
    demonstrateSubstitutionWorkflow();
    demonstrateCompleteAnalysisWorkflow();

    console.log("All workflow examples completed successfully!");
    console.log();
    console.log("🎵 This demonstrates the complete power of the Composer library:");
    console.log("   - Harmonic analysis and Roman numeral notation");
    console.log("   - Complexity assessment for difficulty grading");
    console.log("   - Efficient binary serialization for ML/storage");
    console.log("   - Cross-platform compatibility via WebAssembly");
    console.log("   - Type-safe TypeScript integration");
  } catch (error) {
    console.error("Error running examples:", error);
  }
}

// Export for module usage
export {
  createChordProgression,
  analyzeProgression,
  demonstratePopSongWorkflow,
  demonstrateJazzWorkflow,
  demonstrateModalWorkflow,
  demonstrateSubstitutionWorkflow,
  demonstrateCompleteAnalysisWorkflow,
};

// Run if executed directly
if (require.main === module) {
  main().catch(console.error);
}
