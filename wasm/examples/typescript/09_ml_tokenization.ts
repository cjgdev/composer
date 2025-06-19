#!/usr/bin/env ts-node
/**
 * Advanced ML Tokenization and Optimization
 *
 * This example demonstrates advanced tokenization and ML features:
 * - Timeline reconstruction from tokenized data
 * - MIDI-like token processing and conversion
 * - Cluster-based detokenization algorithms
 * - ML vocabulary optimization techniques
 * - Batch processing for large datasets
 * - Performance optimization for ML pipelines
 *
 * Based on the Composer specification: data-processing-serialization.spec
 */

import * as composer from "../../composer_wasm";

interface TimelineEvent {
  type: "chord" | "note" | "rest";
  beat: number;
  duration: number;
  content?: composer.WasmChord | SimulatedNote;
}

interface SimulatedNote {
  scaleDegree: number;
  octave: number;
  toString(): string;
  free(): void;
}

// Simulate WasmNote functionality
class MockNote implements SimulatedNote {
  scaleDegree: number;
  octave: number;

  constructor(scaleDegree: number, octave: number) {
    this.scaleDegree = scaleDegree;
    this.octave = octave;
  }

  toString(): string {
    const noteNames = ["C", "D", "E", "F", "G", "A", "B"];
    const noteName = noteNames[(this.scaleDegree - 1) % 7];
    return `${noteName}${this.octave}`;
  }

  free(): void {
    // No-op for mock
  }
}

interface MidiLikeToken {
  type: "note-on" | "note-off" | "chord-on" | "chord-off" | "delay";
  data: string;
}

/**
 * Initialize the WASM module
 */
async function initializeWasm(): Promise<void> {
  console.log(`Initializing Composer WASM v${composer.getVersion()}`);
}

/**
 * Show timeline reconstruction from tokenized musical data
 */
function demonstrateTimelineReconstruction(): void {
  console.log("=== Timeline Reconstruction ===");

  // Create a musical timeline with notes and chords
  const timelineEvents: TimelineEvent[] = [
    { type: "chord", beat: 0.0, duration: 2.0, content: new composer.WasmChord(1, 5) }, // C major
    { type: "note", beat: 0.0, duration: 1.0, content: new MockNote(1, 4) }, // C4
    { type: "note", beat: 1.0, duration: 1.0, content: new MockNote(3, 4) }, // E4
    { type: "chord", beat: 2.0, duration: 2.0, content: new composer.WasmChord(5, 5) }, // G major
    { type: "note", beat: 2.0, duration: 0.5, content: new MockNote(5, 4) }, // G4
    { type: "rest", beat: 2.5, duration: 0.5 },
    { type: "note", beat: 3.0, duration: 1.0, content: new MockNote(3, 4) }, // E4
  ];

  console.log("Original timeline:");
  timelineEvents.forEach((event, i) => {
    console.log(`  ${i + 1}. Beat ${event.beat}: ${event.type}`);
    if (event.type === "chord" && event.content) {
      console.log(`     Chord: ${event.content.toString()}`);
    } else if (event.type === "note" && event.content) {
      console.log(`     Note: ${event.content.toString()}`);
    }
    console.log(`     Duration: ${event.duration} beats`);
  });
  console.log();

  // Tokenize the timeline
  console.log("Tokenizing timeline:");
  const tokens: string[] = [];

  timelineEvents.forEach((event) => {
    // Simulate duration token
    const durationToken = `D_${Math.round(event.duration * 24)}`; // 24 ticks per beat
    tokens.push(durationToken);
    console.log(`  Duration: ${durationToken}`);

    if (event.type === "chord" && event.content && "toHex" in event.content) {
      const chordToken = event.content.toHex();
      tokens.push(chordToken);
      console.log(`  Chord: ${chordToken}`);
    } else if (event.type === "note" && event.content && "scaleDegree" in event.content) {
      // Manual note tokenization (simplified)
      const chromatic = (event.content.scaleDegree - 1) % 12;
      const octave = event.content.octave;
      const noteToken = `R_${chromatic.toString(16).toUpperCase()}_O_${octave}`;
      tokens.push(noteToken);
      console.log(`  Note: ${noteToken}`);
    } else if (event.type === "rest") {
      tokens.push("NOTE-REST");
      console.log("  Rest: NOTE-REST");
    }
  });

  console.log(`\nTotal tokens generated: ${tokens.length}`);
  console.log();

  // Demonstrate token parsing
  console.log("Parsing tokens:");
  let currentBeat = 0.0;
  let i = 0;
  while (i < tokens.length) {
    const token = tokens[i];

    // Parse duration tokens
    if (token.startsWith("D_")) {
      try {
        // Simulate duration parsing
        const ticks = parseInt(token.substring(2));
        const duration = ticks / 24.0; // Convert back to beats
        console.log(`  Duration: ${duration} beats`);

        // Get next token for content
        if (i + 1 < tokens.length) {
          const contentToken = tokens[i + 1];
          if (contentToken.startsWith("R_")) {
            console.log(`  Note content: ${contentToken}`);
          } else if (contentToken.includes("NOTE-REST")) {
            console.log(`  Rest content: ${contentToken}`);
          } else {
            console.log(`  Chord content: ${contentToken}`);
          }
          i += 1;
        }

        currentBeat += duration;
      } catch (error) {
        console.log(`  Unrecognized token: ${token} (${error})`);
      }
    }

    i += 1;
  }

  // Clean up objects
  timelineEvents.forEach((event) => {
    if (event.content && event.content.free) {
      event.content.free();
    }
  });
}

/**
 * Show MIDI-like token processing
 */
function demonstrateMidiLikeProcessing(): void {
  console.log("=== MIDI-like Token Processing ===");

  // Create MIDI-like token sequence
  const midiLikeTokens: MidiLikeToken[] = [
    { type: "note-on", data: "60" }, // C4 on
    { type: "delay", data: "24" }, // Quarter note delay (24 ticks)
    { type: "note-off", data: "60" }, // C4 off
    { type: "note-on", data: "64" }, // E4 on
    { type: "delay", data: "24" }, // Quarter note delay
    { type: "note-off", data: "64" }, // E4 off
    { type: "note-on", data: "67" }, // G4 on
    { type: "delay", data: "48" }, // Half note delay (48 ticks)
    { type: "note-off", data: "67" }, // G4 off
    { type: "chord-on", data: "60-64-67" }, // C major chord on
    { type: "delay", data: "96" }, // Whole note delay (96 ticks)
    { type: "chord-off", data: "60-64-67" }, // C major chord off
  ];

  console.log("MIDI-like token sequence:");
  midiLikeTokens.forEach((token, i) => {
    console.log(`  ${i + 1}. ${token.type.toUpperCase()}-${token.data}`);
  });
  console.log();

  // Process MIDI-like tokens
  console.log("Processing MIDI-like tokens:");
  let currentTime = 0;
  const activeNotes: { [midiNum: number]: number } = {};

  midiLikeTokens.forEach((token) => {
    if (token.type === "note-on") {
      const midiNum = parseInt(token.data);
      const noteInfo = midiToNoteName(midiNum);
      console.log(
        `  Time ${currentTime}: Note ON - ${noteInfo.name}${noteInfo.octave} (MIDI ${midiNum})`
      );
      activeNotes[midiNum] = currentTime;
    } else if (token.type === "note-off") {
      const midiNum = parseInt(token.data);
      if (midiNum in activeNotes) {
        const startTime = activeNotes[midiNum];
        const duration = currentTime - startTime;
        const noteInfo = midiToNoteName(midiNum);
        delete activeNotes[midiNum];

        console.log(
          `  Time ${currentTime}: Note OFF - ${noteInfo.name}${noteInfo.octave} (duration: ${duration} ticks)`
        );
      }
    } else if (token.type === "chord-on") {
      const midiNums = token.data.split("-").map((n) => parseInt(n));
      const noteNames = midiNums.map(midiToNoteName).map((n) => `${n.name}${n.octave}`);
      console.log(`  Time ${currentTime}: Chord ON - ${noteNames.join(", ")}`);
    } else if (token.type === "delay") {
      const delayTicks = parseInt(token.data);
      currentTime += delayTicks;
      console.log(`  Advance time by ${delayTicks} ticks to ${currentTime}`);
    }
  });

  console.log();
}

/**
 * Convert MIDI number to note name and octave
 */
function midiToNoteName(midiNum: number): { name: string; octave: number } {
  const noteNames = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
  const name = noteNames[midiNum % 12];
  const octave = Math.floor(midiNum / 12) - 1;
  return { name, octave };
}

/**
 * Show vocabulary optimization for ML applications
 */
function demonstrateVocabularyOptimization(): void {
  console.log("=== ML Vocabulary Optimization ===");

  // Create a large vocabulary of chords
  const originalChords: composer.WasmChord[] = [];
  const chordTypes = [5, 7, 9, 11, 13];
  const roots = [1, 2, 3, 4, 5, 6, 7];

  console.log("Creating large chord vocabulary:");
  for (const root of roots) {
    for (const chordType of chordTypes) {
      for (let inversion = 0; inversion < 3; inversion++) {
        try {
          const chord = new composer.WasmChord(root, chordType);
          if (inversion > 0) {
            const invertedChord = chord.withInversion(inversion);
            originalChords.push(invertedChord);
            chord.free();
          } else {
            originalChords.push(chord);
          }
        } catch (error) {
          // Skip invalid chords
        }
      }
    }
  }

  console.log(`Original vocabulary: ${originalChords.length} chords`);

  // Convert to binary for ML processing
  const chordHexStrings = originalChords.map((chord) => chord.toHex());

  console.log(`Hex representations: ${chordHexStrings.length} entries`);
  const totalSize = chordHexStrings.reduce((sum, hex) => sum + hex.length, 0);
  console.log(`Total size: ${totalSize} characters`);
  console.log();

  // Vocabulary reduction simulation
  console.log("Vocabulary reduction:");
  const targetSizes = [50, 25, 10];

  targetSizes.forEach((targetSize) => {
    // Simple frequency-based reduction (simplified)
    const complexityScores: Array<{ chord: composer.WasmChord; complexity: number; hex: string }> =
      [];

    originalChords.forEach((chord, i) => {
      try {
        const complexity = composer.getChordComplexity(chord, "major");
        complexityScores.push({ chord, complexity, hex: chordHexStrings[i] });
      } catch (error) {
        // Skip chords that can't be analyzed
      }
    });

    // Sort by complexity (keep simpler chords)
    complexityScores.sort((a, b) => a.complexity - b.complexity);
    const reducedChords = complexityScores.slice(0, targetSize);

    console.log(`  Target size ${targetSize}: Got ${reducedChords.length} chords`);
    const reductionRatio = reducedChords.length / originalChords.length;
    console.log(`    Reduction ratio: ${(reductionRatio * 100).toFixed(1)}%`);

    if (reducedChords.length > 0) {
      const avgComplexity =
        reducedChords.reduce((sum, r) => sum + r.complexity, 0) / reducedChords.length;
      console.log(`    Average complexity of preserved chords: ${avgComplexity.toFixed(2)}`);
    }
  });

  console.log();

  // Clean up chord objects
  originalChords.forEach((chord) => chord.free());
}

/**
 * Show sequence augmentation for ML training
 */
function demonstrateSequenceAugmentation(): void {
  console.log("=== Sequence Augmentation ===");

  // Original short sequence
  const originalMelody = [
    new MockNote(1, 4), // C4
    new MockNote(3, 4), // E4
    new MockNote(5, 4), // G4
    new MockNote(1, 5), // C5
  ];

  const originalChords = [
    new composer.WasmChord(1, 5), // C major
    new composer.WasmChord(5, 5), // G major
  ];

  console.log("Original sequences:");
  console.log(`  Melody: ${originalMelody.map((note) => note.toString())}`);
  console.log(`  Chords: ${originalChords.map((chord) => chord.toString())}`);
  console.log(`  Melody length: ${originalMelody.length} notes`);
  console.log(`  Chord length: ${originalChords.length} chords`);
  console.log();

  // Augmentation targets
  const minTokensTargets = [10, 20, 50];

  minTokensTargets.forEach((minTokens) => {
    console.log(`Augmenting to minimum ${minTokens} tokens:`);

    // Manual augmentation
    const melodyLen = originalMelody.length;
    const repetitionsNeeded = Math.ceil(minTokens / melodyLen);

    const augmentedMelody: string[] = [];
    const augmentedChords: string[] = [];

    for (let rep = 0; rep < repetitionsNeeded && augmentedMelody.length < minTokens; rep++) {
      // Add variation to avoid exact repetition
      originalMelody.forEach((note) => {
        if (augmentedMelody.length < minTokens) {
          if (rep > 0) {
            // Simple transposition variation
            const transposedDegree = ((note.scaleDegree + rep - 1) % 7) + 1;
            const transposedNote = new MockNote(transposedDegree, note.octave);
            augmentedMelody.push(transposedNote.toString());
            transposedNote.free();
          } else {
            augmentedMelody.push(note.toString());
          }
        }
      });

      originalChords.forEach((chord) => {
        if (augmentedChords.length < minTokens / 2) {
          if (rep > 0) {
            // Simple inversion variation
            try {
              const variedChord = chord.withInversion((chord.inversion + rep) % 3);
              augmentedChords.push(variedChord.toString());
              variedChord.free();
            } catch (error) {
              augmentedChords.push(chord.toString());
            }
          } else {
            augmentedChords.push(chord.toString());
          }
        }
      });
    }

    // Trim to target length
    const finalMelody = augmentedMelody.slice(0, minTokens);
    const finalChords = augmentedChords.slice(0, Math.floor(minTokens / 2));

    console.log(`  Augmented melody: ${finalMelody.length} tokens`);
    console.log(`  Augmented chords: ${finalChords.length} tokens`);
    console.log(`  Melody sample: ${finalMelody.slice(0, 8).join(", ")}...`);
    console.log(`  Chord sample: ${finalChords.slice(0, 4).join(", ")}...`);

    console.log();
  });

  // Clean up objects
  originalMelody.forEach((note) => note.free());
  originalChords.forEach((chord) => chord.free());
}

/**
 * Show batch processing for large ML datasets
 */
function demonstrateBatchProcessing(): void {
  console.log("=== Batch Processing for ML ===");

  // Create a large dataset simulation
  const batchSizes = [10, 50, 100];
  const totalSequences = 500;

  console.log(`Simulating batch processing for ${totalSequences} sequences:`);
  console.log();

  batchSizes.forEach((batchSize) => {
    console.log(`Batch size: ${batchSize}`);

    // Calculate batch statistics
    const numBatches = Math.ceil(totalSequences / batchSize);
    console.log(`  Number of batches: ${numBatches}`);

    // Simulate processing batches
    let processedSequences = 0;
    let totalTokens = 0;

    for (let batchIdx = 0; batchIdx < numBatches; batchIdx++) {
      // Simulate batch creation
      const sequencesInBatch = Math.min(batchSize, totalSequences - processedSequences);

      // Simulate token generation for batch
      let batchTokens = 0;
      for (let seqIdx = 0; seqIdx < sequencesInBatch; seqIdx++) {
        // Simulate sequence of varying length
        const sequenceLength = 8 + (seqIdx % 5); // 8-12 tokens per sequence
        batchTokens += sequenceLength;
      }

      totalTokens += batchTokens;
      processedSequences += sequencesInBatch;

      if (batchIdx < 3) {
        // Show first few batches
        console.log(
          `    Batch ${batchIdx + 1}: ${sequencesInBatch} sequences, ${batchTokens} tokens`
        );
      }
    }

    console.log(`  Total processed: ${processedSequences} sequences`);
    console.log(`  Total tokens: ${totalTokens}`);
    const avgTokens = totalTokens / processedSequences;
    console.log(`  Average tokens per sequence: ${avgTokens.toFixed(1)}`);
    console.log();
  });

  // Demonstrate memory-efficient processing
  console.log("Memory-efficient processing strategies:");
  console.log("  1. Stream processing: Process one batch at a time");
  console.log("  2. Token vocabulary sharing: Reuse token libraries across batches");
  console.log("  3. Binary serialization: Use 5-byte chord format for memory efficiency");
  console.log("  4. Lazy loading: Load data only when needed");
  console.log("  5. Parallel processing: Process multiple batches concurrently");
  console.log();
}

/**
 * Run all ML tokenization demonstration functions
 */
async function main(): Promise<void> {
  console.log("Composer Library - Advanced ML Tokenization Examples (TypeScript)");
  console.log("=".repeat(70));
  console.log();

  try {
    await initializeWasm();

    demonstrateTimelineReconstruction();
    demonstrateMidiLikeProcessing();
    demonstrateVocabularyOptimization();
    demonstrateSequenceAugmentation();
    demonstrateBatchProcessing();

    console.log("All ML tokenization examples completed!");
    console.log();
    console.log("Note: Some advanced ML features may require additional WASM");
    console.log("bindings to be fully functional. This example demonstrates the");
    console.log("intended usage patterns for machine learning applications.");
  } catch (error) {
    console.error("Error running ML tokenization examples:", error);
  }
}

// Run if executed directly
if (require.main === module) {
  main().catch(console.error);
}

// Export functions for potential reuse
export {
  demonstrateTimelineReconstruction,
  demonstrateMidiLikeProcessing,
  demonstrateVocabularyOptimization,
  demonstrateSequenceAugmentation,
  demonstrateBatchProcessing,
};
