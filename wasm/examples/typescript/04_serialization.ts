#!/usr/bin/env ts-node
/**
 * Data Processing and ML Serialization
 *
 * This example demonstrates serialization capabilities in the Composer library:
 * - Binary chord serialization and deserialization
 * - Hex string encoding/decoding
 * - Batch processing for ML applications
 * - Data compression and efficiency analysis
 * - Performance testing for large datasets
 *
 * Based on the Composer specification: data-processing-serialization.spec
 */

import * as composer from "../../composer_wasm.js";

/**
 * Initialize the WASM module
 */
async function initializeWasm(): Promise<void> {
  console.log(`Initializing Composer WASM v${composer.getVersion()}`);
}

/**
 * Demonstrate basic chord serialization and deserialization
 */
function demonstrateBasicSerialization(): void {
  console.log("=== Basic Chord Serialization ===");

  // Create various chord types for testing
  const testChords = [
    { chord: new composer.WasmChord(1, 5), name: "C major" },
    { chord: new composer.WasmChord(5, 7), name: "G7" },
    { chord: new composer.WasmChord(6, 5), name: "A minor" },
    { chord: new composer.WasmChord(3, 7), name: "E7" },
    { chord: composer.WasmChord.rest(), name: "Rest" },
    { chord: composer.WasmChord.triad(8), name: "Ab triad" },
    { chord: composer.WasmChord.seventh(11), name: "B seventh" },
  ];

  console.log("Serialization test results:");
  testChords.forEach(({ chord, name }) => {
    const hexString = chord.toHex();
    const deserialized = composer.WasmChord.fromHex(hexString);

    const original = chord.toString();
    const restored = deserialized.toString();
    const success = original === restored;

    console.log(
      `  ${name.padEnd(12)}: ${original.padEnd(8)} -> ${hexString} -> ${restored.padEnd(8)} ${success ? "✅" : "❌"}`
    );

    deserialized.free();
  });

  console.log();

  // Clean up test chords
  testChords.forEach(({ chord }) => chord.free());
}

/**
 * Demonstrate hex encoding efficiency and analysis
 */
function demonstrateHexEncodingAnalysis(): void {
  console.log("=== Hex Encoding Analysis ===");

  // Create a variety of chords to analyze encoding patterns
  const chordData: { root: number; type: number }[] = [];

  // Generate systematic chord combinations
  for (let root = 0; root < 12; root++) {
    for (const type of [5, 7, 9, 11, 13]) {
      chordData.push({ root, type });
    }
  }

  const hexStrings: string[] = [];
  const encodingStats = new Map<string, number>();

  console.log("Analyzing hex encoding patterns...");

  chordData.forEach(({ root, type }) => {
    try {
      const chord = new composer.WasmChord(root, type);
      const hex = chord.toHex();
      hexStrings.push(hex);

      // Count hex character frequency
      for (const char of hex) {
        encodingStats.set(char, (encodingStats.get(char) || 0) + 1);
      }

      chord.free();
    } catch (error) {
      // Skip invalid chord combinations
    }
  });

  console.log(`Total valid chords analyzed: ${hexStrings.length}`);
  console.log(
    `Average hex length: ${(hexStrings.reduce((sum, hex) => sum + hex.length, 0) / hexStrings.length).toFixed(2)} characters`
  );
  console.log(`Unique hex strings: ${new Set(hexStrings).size}`);

  // Show character frequency distribution
  console.log("\nHex character frequency:");
  const sortedStats = Array.from(encodingStats.entries()).sort((a, b) => b[1] - a[1]);
  sortedStats.forEach(([char, count]) => {
    const percentage = ((count / hexStrings.join("").length) * 100).toFixed(1);
    console.log(`  '${char}': ${count.toString().padStart(4)} times (${percentage}%)`);
  });

  console.log();
}

/**
 * Demonstrate batch processing for ML applications
 */
function demonstrateBatchProcessing(): void {
  console.log("=== Batch Processing for ML ===");

  // Simulate a chord progression for ML training data
  const progressions = [
    // I-V-vi-IV progression in different keys
    [
      { root: 0, type: 5 },
      { root: 7, type: 5 },
      { root: 9, type: 5 },
      { root: 5, type: 5 },
    ], // C major key
    [
      { root: 2, type: 5 },
      { root: 9, type: 5 },
      { root: 11, type: 5 },
      { root: 7, type: 5 },
    ], // D major key
    [
      { root: 4, type: 5 },
      { root: 11, type: 5 },
      { root: 1, type: 5 },
      { root: 9, type: 5 },
    ], // E major key

    // ii-V-I jazz progressions
    [
      { root: 2, type: 7 },
      { root: 7, type: 7 },
      { root: 0, type: 5 },
    ], // Dm7-G7-C
    [
      { root: 6, type: 7 },
      { root: 11, type: 7 },
      { root: 4, type: 5 },
    ], // F#m7-B7-E
    [
      { root: 10, type: 7 },
      { root: 3, type: 7 },
      { root: 8, type: 5 },
    ], // Bbm7-Eb7-Ab
  ];

  console.log("Processing chord progressions for ML training:");

  const batchData: string[] = [];
  let totalProcessingTime = 0;

  progressions.forEach((progression, progIndex) => {
    const start = performance.now();

    const progressionHex: string[] = [];
    progression.forEach(({ root, type }) => {
      const chord = new composer.WasmChord(root, type);
      const hex = chord.toHex();
      progressionHex.push(hex);
      batchData.push(hex);
      chord.free();
    });

    const end = performance.now();
    const processingTime = end - start;
    totalProcessingTime += processingTime;

    console.log(
      `  Progression ${progIndex + 1}: [${progressionHex.join(", ")}] (${processingTime.toFixed(3)}ms)`
    );
  });

  console.log(`\nBatch processing summary:`);
  console.log(`  Total chords processed: ${batchData.length}`);
  console.log(`  Total processing time: ${totalProcessingTime.toFixed(3)}ms`);
  console.log(`  Average time per chord: ${(totalProcessingTime / batchData.length).toFixed(3)}ms`);
  console.log(
    `  Processing rate: ${Math.round(batchData.length / (totalProcessingTime / 1000))} chords/second`
  );
  console.log();
}

/**
 * Demonstrate roundtrip accuracy testing
 */
function demonstrateRoundtripTesting(): void {
  console.log("=== Roundtrip Accuracy Testing ===");

  let totalTests = 0;
  let successfulTests = 0;
  const errors: string[] = [];

  // Test all valid combinations
  console.log("Running comprehensive roundtrip tests...");

  for (let root = 0; root < 12; root++) {
    for (const type of [5, 7, 9, 11, 13]) {
      try {
        const original = new composer.WasmChord(root, type);

        // Test basic serialization
        const hex = original.toHex();
        const deserialized = composer.WasmChord.fromHex(hex);

        totalTests++;

        if (original.toString() === deserialized.toString()) {
          successfulTests++;
        } else {
          errors.push(
            `Root ${root}, Type ${type}: ${original.toString()} != ${deserialized.toString()}`
          );
        }

        original.free();
        deserialized.free();

        // Test with alterations
        if (type === 5 || type === 7) {
          // Only test alterations on simpler chords
          for (const alteration of ["#5", "b5", "#9", "b9"]) {
            try {
              const altered = original.withAlteration(alteration);
              const alteredHex = altered.toHex();
              const alteredDeserialized = composer.WasmChord.fromHex(alteredHex);

              totalTests++;

              if (altered.toString() === alteredDeserialized.toString()) {
                successfulTests++;
              } else {
                errors.push(
                  `Altered chord (${alteration}): ${altered.toString()} != ${alteredDeserialized.toString()}`
                );
              }

              altered.free();
              alteredDeserialized.free();
            } catch (error) {
              // Skip invalid alterations
            }
          }
        }
      } catch (error) {
        // Skip invalid chord combinations
      }
    }
  }

  // Test special chords
  const restChord = composer.WasmChord.rest();
  const restHex = restChord.toHex();
  const restDeserialized = composer.WasmChord.fromHex(restHex);

  totalTests++;
  if (restChord.toString() === restDeserialized.toString()) {
    successfulTests++;
  } else {
    errors.push(`Rest chord: ${restChord.toString()} != ${restDeserialized.toString()}`);
  }

  restChord.free();
  restDeserialized.free();

  // Report results
  const successRate = (successfulTests / totalTests) * 100;
  console.log(`\nRoundtrip test results:`);
  console.log(`  Total tests: ${totalTests}`);
  console.log(`  Successful: ${successfulTests}`);
  console.log(`  Failed: ${totalTests - successfulTests}`);
  console.log(`  Success rate: ${successRate.toFixed(2)}%`);

  if (errors.length > 0) {
    console.log(`\nFirst few errors:`);
    errors.slice(0, 5).forEach((error) => console.log(`  ${error}`));
    if (errors.length > 5) {
      console.log(`  ... and ${errors.length - 5} more`);
    }
  }

  console.log();
}

/**
 * Demonstrate performance benchmarking
 */
function demonstratePerformanceBenchmarking(): void {
  console.log("=== Performance Benchmarking ===");

  const iterations = 1000;
  const testChords: { root: number; type: number }[] = [];

  // Prepare test data
  for (let i = 0; i < iterations; i++) {
    testChords.push({
      root: Math.floor(Math.random() * 12),
      type: [5, 7, 9, 11, 13][Math.floor(Math.random() * 5)],
    });
  }

  console.log(`Running performance tests with ${iterations} iterations...`);

  // Benchmark serialization
  const serializationStart = performance.now();
  const hexStrings: string[] = [];

  testChords.forEach(({ root, type }) => {
    try {
      const chord = new composer.WasmChord(root, type);
      const hex = chord.toHex();
      hexStrings.push(hex);
      chord.free();
    } catch (error) {
      // Skip invalid combinations
    }
  });

  const serializationEnd = performance.now();
  const serializationTime = serializationEnd - serializationStart;

  // Benchmark deserialization
  const deserializationStart = performance.now();
  let deserializedCount = 0;

  hexStrings.forEach((hex) => {
    try {
      const chord = composer.WasmChord.fromHex(hex);
      chord.free();
      deserializedCount++;
    } catch (error) {
      // Skip invalid hex strings
    }
  });

  const deserializationEnd = performance.now();
  const deserializationTime = deserializationEnd - deserializationStart;

  // Report results
  console.log(`\nPerformance results:`);
  console.log(`  Serialization:`);
  console.log(`    Total time: ${serializationTime.toFixed(3)}ms`);
  console.log(`    Average per chord: ${(serializationTime / hexStrings.length).toFixed(3)}ms`);
  console.log(
    `    Rate: ${Math.round(hexStrings.length / (serializationTime / 1000))} chords/second`
  );
  console.log(`  Deserialization:`);
  console.log(`    Total time: ${deserializationTime.toFixed(3)}ms`);
  console.log(`    Average per chord: ${(deserializationTime / deserializedCount).toFixed(3)}ms`);
  console.log(
    `    Rate: ${Math.round(deserializedCount / (deserializationTime / 1000))} chords/second`
  );
  console.log(
    `  Combined throughput: ${Math.round(hexStrings.length / ((serializationTime + deserializationTime) / 1000))} chords/second`
  );

  console.log();
}

/**
 * Demonstrate data compression analysis
 */
function demonstrateCompressionAnalysis(): void {
  console.log("=== Data Compression Analysis ===");

  // Create a large dataset of chords
  const chordTexts: string[] = [];
  const hexStrings: string[] = [];

  for (let root = 0; root < 12; root++) {
    for (const type of [5, 7, 9, 11, 13]) {
      try {
        const chord = new composer.WasmChord(root, type);
        chordTexts.push(chord.toString());
        hexStrings.push(chord.toHex());
        chord.free();

        // Add variations with inversions
        for (let inversion = 1; inversion <= 3; inversion++) {
          try {
            const invertedChord = chord.withInversion(inversion);
            chordTexts.push(invertedChord.toString());
            hexStrings.push(invertedChord.toHex());
            invertedChord.free();
          } catch (error) {
            // Skip invalid inversions
          }
        }
      } catch (error) {
        // Skip invalid chord combinations
      }
    }
  }

  // Calculate compression statistics
  const originalTextSize = chordTexts.join("").length;
  const hexDataSize = hexStrings.join("").length;
  const compressionRatio = originalTextSize / hexDataSize;

  console.log(`Compression analysis results:`);
  console.log(`  Total chords analyzed: ${chordTexts.length}`);
  console.log(`  Original text size: ${originalTextSize} characters`);
  console.log(`  Hex encoded size: ${hexDataSize} characters`);
  console.log(`  Compression ratio: ${compressionRatio.toFixed(2)}:1`);
  console.log(`  Space savings: ${((1 - hexDataSize / originalTextSize) * 100).toFixed(1)}%`);
  console.log(
    `  Average chord text length: ${(originalTextSize / chordTexts.length).toFixed(2)} chars`
  );
  console.log(`  Average hex length: ${(hexDataSize / hexStrings.length).toFixed(2)} chars`);

  console.log();
}

/**
 * Run all serialization demonstration functions
 */
async function main(): Promise<void> {
  console.log("Composer Library - Serialization Examples (TypeScript)");
  console.log("=".repeat(70));
  console.log();

  try {
    await initializeWasm();

    demonstrateBasicSerialization();
    demonstrateHexEncodingAnalysis();
    demonstrateBatchProcessing();
    demonstrateRoundtripTesting();
    demonstratePerformanceBenchmarking();
    demonstrateCompressionAnalysis();

    console.log("All serialization examples completed successfully!");
  } catch (error) {
    console.error("Error running examples:", error);
  }
}

// Export for module usage
export {
  demonstrateBasicSerialization,
  demonstrateHexEncodingAnalysis,
  demonstrateBatchProcessing,
  demonstrateRoundtripTesting,
  demonstratePerformanceBenchmarking,
  demonstrateCompressionAnalysis,
};

// Run if executed directly
if (require.main === module) {
  main().catch(console.error);
}
