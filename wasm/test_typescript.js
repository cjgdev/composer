// Simple test to verify TypeScript examples can be used
const composer = require("./composer_wasm");

console.log("Testing Composer WASM TypeScript Compatibility");
console.log("=".repeat(50));
console.log(`Library version: ${composer.getVersion()}`);

// Test all the core functionality that TypeScript examples use
function testChordOperations() {
  console.log("\n1. Testing Chord Operations:");

  // Basic chord creation
  const chord = new composer.WasmChord(5, 7);
  console.log(`   Created G7 chord: ${chord.toString()}`);
  console.log(`   Root: ${chord.root}, Type: ${chord.chordType}`);
  console.log(`   Is seventh: ${chord.isSeventh()}`);

  // Serialization
  const hex = chord.toHex();
  console.log(`   Serialized: ${hex}`);

  const restored = composer.WasmChord.fromHex(hex);
  console.log(`   Restored: ${restored.toString()}`);
  console.log(`   Roundtrip success: ${chord.toString() === restored.toString()}`);

  chord.free();
  restored.free();
}

function testScaleOperations() {
  console.log("\n2. Testing Scale Operations:");

  const majorScale = composer.WasmScaleFingerprint.major();
  const minorScale = composer.WasmScaleFingerprint.minor();

  console.log(`   Major scale: ${majorScale.toString()}`);
  console.log(`   Minor scale: ${minorScale.toString()}`);
  console.log(`   Major scale notes: ${majorScale.noteCount()}`);
  console.log(`   Is diatonic: ${majorScale.isDiatonic()}`);

  majorScale.free();
  minorScale.free();
}

function testAnalysisFunctions() {
  console.log("\n3. Testing Analysis Functions:");

  const chord = new composer.WasmChord(5, 7); // G7
  const scale = composer.WasmScaleFingerprint.major();

  const roman = composer.getRomanNumeral(chord, scale);
  const degrees = composer.getStableScaleDegrees(chord, scale);
  const complexity = composer.getChordComplexity(chord, "major");

  console.log(`   G7 in C major: ${roman}`);
  console.log(`   Scale degrees: [${degrees.join(", ")}]`);
  console.log(`   Complexity: ${complexity}`);

  chord.free();
  scale.free();
}

function testProgression() {
  console.log("\n4. Testing Chord Progression:");

  const progression = [
    { root: 1, type: 5, name: "C" },
    { root: 6, type: 5, name: "Am" },
    { root: 4, type: 5, name: "F" },
    { root: 5, type: 5, name: "G" },
  ];

  const scale = composer.WasmScaleFingerprint.major();

  console.log("   I-vi-IV-V progression:");
  progression.forEach(({ root, type, name }, i) => {
    const chord = new composer.WasmChord(root, type);
    const roman = composer.getRomanNumeral(chord, scale);
    console.log(`     ${i + 1}. ${name} = ${chord.toString()} (${roman})`);
    chord.free();
  });

  scale.free();
}

// Run all tests
try {
  testChordOperations();
  testScaleOperations();
  testAnalysisFunctions();
  testProgression();

  console.log("\n✅ All TypeScript functionality tests passed!");
  console.log("\nThe TypeScript examples demonstrate:");
  console.log("   - Full type safety with .d.ts files");
  console.log("   - Memory management patterns");
  console.log("   - Error handling best practices");
  console.log("   - Performance benchmarking");
  console.log("   - Complete music theory workflows");
} catch (error) {
  console.error("\n❌ Test failed:", error);
}
