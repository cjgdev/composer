/**
 * Node.js example for Composer WASM bindings
 *
 * This example demonstrates how to use the Composer library in a Node.js environment.
 */

const composer = require("../composer_wasm");

async function main() {
  console.log("Composer WASM Node.js Example\n");
  console.log(`Library version: ${composer.getVersion()}\n`);

  try {
    // Create a basic chord
    console.log("Creating a G7 chord...");
    const chord = new composer.WasmChord(5, 7); // G7 chord (root=5, type=7)
    console.log(`Chord created: ${chord.toString()}`);

    // Analyze chord properties
    console.log(`Root note: ${chord.root}`);
    console.log(`Chord type: ${chord.chordType}`);
    console.log(`Inversion: ${chord.inversion}`);
    console.log(`Is rest: ${chord.isRest}`);
    console.log(`Is triad: ${chord.isTriad()}`);
    console.log(`Is seventh: ${chord.isSeventh()}`);

    // Work with scales
    console.log("\nWorking with scales...");
    const majorScale = composer.WasmScaleFingerprint.major();
    console.log(`Major scale note count: ${majorScale.noteCount()}`);
    console.log(`Is diatonic: ${majorScale.isDiatonic()}`);
    console.log(`Scale fingerprint: ${majorScale.toString()}`);

    // Get scale degrees for the chord
    const scaleDegrees = composer.getStableScaleDegrees(chord, majorScale);
    console.log(`G7 chord scale degrees in C major: [${scaleDegrees.join(", ")}]`);

    // Get Roman numeral representation
    const romanNumeral = composer.getRomanNumeral(chord, majorScale);
    console.log(`Roman numeral: ${romanNumeral}`);

    // Get chord complexity
    const complexity = composer.getChordComplexity(chord, "major");
    console.log(`Chord complexity: ${complexity}`);

    // Binary serialization
    console.log("\nTesting serialization...");
    const hexString = chord.toHex();
    console.log(`Serialized chord (hex): ${hexString}`);

    const deserialized = composer.WasmChord.fromHex(hexString);
    console.log(`Deserialized chord: ${deserialized.toString()}`);

    // Verify roundtrip
    const match = chord.toString() === deserialized.toString();
    console.log(`Serialization roundtrip: ${match ? "SUCCESS" : "FAILED"}`);

    // Test different chord types
    console.log("\nTesting different chord types...");
    const cmajor = new composer.WasmChord(1, 5); // C major triad
    const dminor7 = new composer.WasmChord(2, 7); // D minor 7
    const etriad = composer.WasmChord.triad(4); // E triad
    const fseventh = composer.WasmChord.seventh(6); // F seventh
    const rest = composer.WasmChord.rest(); // Rest chord

    console.log(`C major: ${cmajor.toString()}`);
    console.log(`D minor 7: ${dminor7.toString()}`);
    console.log(`E triad: ${etriad.toString()}`);
    console.log(`F seventh: ${fseventh.toString()}`);
    console.log(`Rest chord: ${rest.toString()} (is rest: ${rest.isRest})`);

    // Test chord alterations
    console.log("\nTesting chord alterations...");
    const chordWithAlteration = cmajor.withAlteration("#5");
    console.log(`C major with #5: ${chordWithAlteration.toString()}`);

    const chordWithInversion = cmajor.withInversion(1);
    console.log(`C major 1st inversion: ${chordWithInversion.toString()}`);

    // Test different scales
    console.log("\nTesting different scales...");
    const minorScale = composer.WasmScaleFingerprint.minor();
    const harmonicMinor = composer.WasmScaleFingerprint.harmonicMinor();

    console.log(`Natural minor scale: ${minorScale.toString()}`);
    console.log(`Harmonic minor scale: ${harmonicMinor.toString()}`);

    // Custom scale from array
    const customScale = composer.WasmScaleFingerprint.fromArray(
      new Uint8Array([1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1])
    );
    console.log(`Custom scale: ${customScale.toString()}`);

    console.log("\nExample completed successfully!");

    // Clean up memory
    chord.free();
    deserialized.free();
    cmajor.free();
    dminor7.free();
    etriad.free();
    fseventh.free();
    rest.free();
    chordWithAlteration.free();
    chordWithInversion.free();
    majorScale.free();
    minorScale.free();
    harmonicMinor.free();
    customScale.free();
  } catch (error) {
    console.error("Error in example:", error);
  }
}

main();
