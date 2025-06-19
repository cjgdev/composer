#!/usr/bin/env python3
"""
Advanced ML Tokenization and Optimization

This example demonstrates advanced tokenization and ML features:
- Timeline reconstruction from tokenized data
- MIDI-like token processing and conversion
- Cluster-based detokenization algorithms
- ML vocabulary optimization techniques
- Batch processing for large datasets
- Performance optimization for ML pipelines

Based on the Composer specification: data-processing-serialization.spec
"""

import composer


def demonstrate_timeline_reconstruction() -> None:
    """Show timeline reconstruction from tokenized musical data."""
    print("=== Timeline Reconstruction ===")

    # Create a musical timeline with notes and chords
    timeline_events = [
        {"type": "chord", "chord": composer.Chord(1, 5), "beat": 0.0, "duration": 2.0},
        {"type": "note", "note": composer.Note(1, 4), "beat": 0.0, "duration": 1.0},
        {"type": "note", "note": composer.Note(3, 4), "beat": 1.0, "duration": 1.0},
        {"type": "chord", "chord": composer.Chord(5, 5), "beat": 2.0, "duration": 2.0},
        {"type": "note", "note": composer.Note(5, 4), "beat": 2.0, "duration": 0.5},
        {"type": "rest", "beat": 2.5, "duration": 0.5},
        {"type": "note", "note": composer.Note(3, 4), "beat": 3.0, "duration": 1.0},
    ]

    major_scale = composer.ScaleFingerprint.major()

    print("Original timeline:")
    for i, event in enumerate(timeline_events):
        print(f"  {i + 1}. Beat {event['beat']}: {event['type']}")
        if event["type"] == "chord":
            print(f"     Chord: {event['chord']}")
        elif event["type"] == "note":
            print(f"     Note: {event['note']}")
        print(f"     Duration: {event['duration']} beats")
    print()

    # Tokenize the timeline
    print("Tokenizing timeline:")
    tokens = []

    for event in timeline_events:
        # Add duration token
        duration_token = composer.py_tokenize_duration(event["duration"])
        tokens.append(duration_token)
        print(f"  Duration: {duration_token}")

        if event["type"] == "chord":
            try:
                chord_token = composer.py_tokenize_chord_as_raw(
                    event["chord"], major_scale
                )
                tokens.append(chord_token)
                print(f"  Chord: {chord_token}")
            except AttributeError:
                # Fallback tokenization
                hex_token = composer.chord_to_hex(event["chord"])
                tokens.append(hex_token)
                print(f"  Chord (hex): {hex_token}")

        elif event["type"] == "note":
            try:
                note_token = composer.py_tokenize_note_as_raw(
                    event["note"], major_scale
                )
                tokens.append(note_token)
                print(f"  Note: {note_token}")
            except AttributeError:
                # Manual note tokenization
                chromatic = (event["note"].scale_degree - 1) % 12
                octave = event["note"].octave
                note_token = f"R_{chromatic:X} O_{octave}"
                tokens.append(note_token)
                print(f"  Note (manual): {note_token}")

        elif event["type"] == "rest":
            tokens.append("NOTE-REST")
            print("  Rest: NOTE-REST")

    print(f"\nTotal tokens generated: {len(tokens)}")
    print()

    # Reconstruct timeline from tokens
    print("Reconstructing timeline from tokens:")
    try:
        reconstructed = composer.py_reconstruct_timeline(tokens, major_scale)

        print(f"Reconstructed {len(reconstructed)} events:")
        for i, event in enumerate(reconstructed):
            print(f"  {i + 1}. {event}")

    except AttributeError:
        # Manual reconstruction demonstration
        print("Manual token parsing:")
        current_beat = 0.0
        # Track parsed events
        # reconstructed_events = []  # Currently unused

        i = 0
        while i < len(tokens):
            token = tokens[i]

            # Parse duration
            if token.startswith("D_"):
                try:
                    duration = composer.py_parse_duration_token(token)
                    print(f"  Duration: {duration} beats")

                    # Get next token for content
                    if i + 1 < len(tokens):
                        content_token = tokens[i + 1]
                        if content_token.startswith("R_"):
                            print(f"  Note content: {content_token}")
                        elif "NOTE-REST" in content_token:
                            print(f"  Rest content: {content_token}")
                        else:
                            print(f"  Chord content: {content_token}")
                        i += 1

                    current_beat += duration

                except AttributeError:
                    print(f"  Unrecognized token: {token}")

            i += 1


def demonstrate_midi_like_processing() -> None:
    """Show MIDI-like token processing."""
    print("=== MIDI-like Token Processing ===")

    # Create MIDI-like token sequence
    midi_like_tokens = [
        "NOTE-60-ON",  # C4 on
        "DELAY-24",  # Quarter note delay (24 ticks)
        "NOTE-60-OFF",  # C4 off
        "NOTE-64-ON",  # E4 on
        "DELAY-24",  # Quarter note delay
        "NOTE-64-OFF",  # E4 off
        "NOTE-67-ON",  # G4 on
        "DELAY-48",  # Half note delay (48 ticks)
        "NOTE-67-OFF",  # G4 off
        "CHORD-60-64-67-ON",  # C major chord on
        "DELAY-96",  # Whole note delay (96 ticks)
        "CHORD-60-64-67-OFF",  # C major chord off
    ]

    major_scale = composer.ScaleFingerprint.major()

    print("MIDI-like token sequence:")
    for i, token in enumerate(midi_like_tokens):
        print(f"  {i + 1}. {token}")
    print()

    # Process MIDI-like tokens
    print("Processing MIDI-like tokens:")
    try:
        processed = composer.py_detokenize_midi_like(midi_like_tokens, major_scale)

        print("Processed timeline:")
        print(f"  Timeline object: {processed}")
        print(f"  Timeline type: {type(processed)}")
        print(f"  Timeline beats: {processed.beats}")
        print(f"  Timeline events: {len(processed.events)}")

        for event in processed.events:
            print(f"    {event}")

    except AttributeError:
        # Manual MIDI-like processing
        print("Manual MIDI-like processing:")
        current_time = 0
        active_notes = {}
        # Track parsed events
        # events = []  # Currently unused

        for token in midi_like_tokens:
            if token.startswith("NOTE-") and token.endswith("-ON"):
                # Extract MIDI note number
                midi_num = int(token.split("-")[1])
                note_name = [
                    "C",
                    "C#",
                    "D",
                    "D#",
                    "E",
                    "F",
                    "F#",
                    "G",
                    "G#",
                    "A",
                    "A#",
                    "B",
                ][midi_num % 12]
                octave = midi_num // 12 - 1

                note_info = f"{note_name}{octave} (MIDI {midi_num})"
                print(f"  Time {current_time}: Note ON - {note_info}")
                active_notes[midi_num] = current_time

            elif token.startswith("NOTE-") and token.endswith("-OFF"):
                midi_num = int(token.split("-")[1])
                if midi_num in active_notes:
                    start_time = active_notes.pop(midi_num)
                    duration = current_time - start_time
                    note_name = [
                        "C",
                        "C#",
                        "D",
                        "D#",
                        "E",
                        "F",
                        "F#",
                        "G",
                        "G#",
                        "A",
                        "A#",
                        "B",
                    ][midi_num % 12]
                    octave = midi_num // 12 - 1

                    note_info = f"{note_name}{octave} (duration: {duration} ticks)"
                    print(f"  Time {current_time}: Note OFF - {note_info}")

            elif token.startswith("CHORD-") and token.endswith("-ON"):
                # Extract chord MIDI numbers
                midi_nums = token.replace("CHORD-", "").replace("-ON", "").split("-")
                midi_nums = [int(n) for n in midi_nums]

                note_names = []
                for midi_num in midi_nums:
                    note_name = [
                        "C",
                        "C#",
                        "D",
                        "D#",
                        "E",
                        "F",
                        "F#",
                        "G",
                        "G#",
                        "A",
                        "A#",
                        "B",
                    ][midi_num % 12]
                    octave = midi_num // 12 - 1
                    note_names.append(f"{note_name}{octave}")

                print(f"  Time {current_time}: Chord ON - {', '.join(note_names)}")

            elif token.startswith("DELAY-"):
                delay_ticks = int(token.split("-")[1])
                current_time += delay_ticks
                print(f"  Advance time by {delay_ticks} ticks to {current_time}")

    print()


def demonstrate_cluster_detokenization() -> None:
    """Show cluster-based detokenization of complex musical data."""
    print("=== Cluster-based Detokenization ===")

    # Create XML-like cluster format (based on specification)
    cluster_xml = """
    <CHORD>D_48 R_0-R_4-R_7</CHORD>
    <NOTES>D_24 R_0 O_4</NOTES>
    <NOTES>D_24 R_4 O_4</NOTES>
    <CHORD>D_48 R_7-R_11-R_2</CHORD>
    <NOTE>D_96 NOTE-REST</NOTE>
    <CHORD>D_24 R_0-R_4-R_7</CHORD>
    """

    major_scale = composer.ScaleFingerprint.major()

    print("Cluster XML format:")
    print(cluster_xml)
    print()

    print("Detokenizing clusters:")
    try:
        detokenized = composer.py_detokenize_cluster(cluster_xml, major_scale)

        print("Detokenization results:")
        print(f"  Result type: {type(detokenized)}")
        print(f"  Result: {detokenized}")

        if hasattr(detokenized, "chords"):
            print(f"  Chords: {len(detokenized.chords)}")
            for i, chord in enumerate(detokenized.chords):
                print(f"    Chord {i + 1}: {chord}")

        if hasattr(detokenized, "notes"):
            print(f"  Notes: {len(detokenized.notes)}")
            for i, note in enumerate(detokenized.notes):
                print(f"    Note {i + 1}: {note}")

    except AttributeError:
        # Manual cluster parsing
        print("Manual cluster parsing:")

        # Extract chord clusters
        import re

        chord_matches = re.findall(r"<CHORD>(.*?)</CHORD>", cluster_xml)
        note_matches = re.findall(r"<NOTE[S]?>(.*?)</NOTE[S]?>", cluster_xml)

        print(f"Found {len(chord_matches)} chord clusters:")
        for i, match in enumerate(chord_matches):
            print(f"  {i + 1}. {match}")

            # Parse duration and notes
            parts = match.strip().split()
            if parts:
                duration_part = parts[0]
                if duration_part.startswith("D_"):
                    try:
                        duration = composer.py_parse_duration_token(duration_part)
                        print(f"     Duration: {duration} beats")
                    except Exception:
                        print(f"     Duration token: {duration_part}")

                # Parse chord notes
                if len(parts) > 1:
                    notes_part = parts[1]
                    if "-" in notes_part:
                        raw_notes = notes_part.split("-")
                        print(f"     Raw notes: {raw_notes}")

        print(f"\nFound {len(note_matches)} note elements:")
        for i, match in enumerate(note_matches):
            print(f"  {i + 1}. {match}")

    print()


def demonstrate_vocabulary_optimization() -> None:
    """Show ML vocabulary optimization techniques."""
    print("=== ML Vocabulary Optimization ===")

    # Create a large vocabulary of chords
    original_chords = []
    chord_types = [5, 7, 9, 11, 13]
    roots = range(1, 8)

    print("Creating large chord vocabulary:")
    for root in roots:
        for chord_type in chord_types:
            for inversion in range(3):  # 0, 1, 2
                chord = composer.Chord(root, chord_type, inversion=inversion)
                original_chords.append(chord)

    print(f"Original vocabulary: {len(original_chords)} chords")

    # Convert to binary for ML processing
    chord_binaries = []
    for chord in original_chords:
        binary = composer.serialize_chord_to_binary(chord)
        chord_binaries.append(binary)

    print(f"Binary representations: {len(chord_binaries)} entries")
    print(f"Total size: {sum(len(b) for b in chord_binaries)} bytes")
    print()

    # Vocabulary reduction
    print("Vocabulary reduction:")
    target_sizes = [50, 25, 10]

    for target_size in target_sizes:
        try:
            reduced_vocab = composer.py_reduce_chord_vocab(
                chord_binaries.copy(), max_vocab=target_size
            )

            print(f"  Target size {target_size}: Got {len(reduced_vocab)} chords")
            reduction_ratio = len(reduced_vocab) / len(chord_binaries)
            print(f"    Reduction ratio: {reduction_ratio:.1%}")

            # Analyze preserved chords
            preserved_complexity = []
            for binary in reduced_vocab:
                try:
                    chord = composer.deserialize_chord_from_binary(binary)
                    complexity = composer.get_chord_complexity(chord)
                    preserved_complexity.append(complexity)
                except Exception:
                    pass

            if preserved_complexity:
                avg_complexity = sum(preserved_complexity) / len(preserved_complexity)
                avg_msg = (
                    f"Average complexity of preserved chords: {avg_complexity:.2f}"
                )
                print(f"    {avg_msg}")

        except AttributeError:
            # Manual vocabulary reduction simulation
            reduction_msg = f"Would reduce from {len(chord_binaries)}"
            print(f"  Target size {target_size}: {reduction_msg}")

            # Simple frequency-based reduction
            complexity_scores = []
            for chord in original_chords:
                complexity = composer.get_chord_complexity(chord)
                complexity_scores.append((chord, complexity))

            # Sort by complexity (keep simpler chords)
            complexity_scores.sort(key=lambda x: x[1])
            reduced_chords = [chord for chord, _ in complexity_scores[:target_size]]

            print(f"    Simulated reduction: {len(reduced_chords)} chords")
            avg_complexity = (
                sum(complexity for _, complexity in complexity_scores[:target_size])
                / target_size
            )
            print(f"    Average complexity: {avg_complexity:.2f}")

    print()


def demonstrate_sequence_augmentation() -> None:
    """Show sequence augmentation for ML training."""
    print("=== Sequence Augmentation ===")

    # Original short sequence
    original_melody = [
        composer.Note(1, 4),  # C4
        composer.Note(3, 4),  # E4
        composer.Note(5, 4),  # G4
        composer.Note(1, 5),  # C5
    ]

    original_chords = [
        composer.Chord(1, 5),  # C major
        composer.Chord(5, 5),  # G major
    ]

    print("Original sequences:")
    print(f"  Melody: {[str(note) for note in original_melody]}")
    print(f"  Chords: {[str(chord) for chord in original_chords]}")
    print(f"  Melody length: {len(original_melody)} notes")
    print(f"  Chord length: {len(original_chords)} chords")
    print()

    # Augmentation targets
    min_tokens_targets = [10, 20, 50]

    for min_tokens in min_tokens_targets:
        print(f"Augmenting to minimum {min_tokens} tokens:")

        try:
            # Augment melody
            augmented_melody = composer.py_augment_with_repeated(
                [str(note) for note in original_melody], min_tokens
            )

            # Augment chords
            augmented_chords = composer.py_augment_with_repeated(
                [str(chord) for chord in original_chords],
                min_tokens // 2,  # Chords typically change less frequently
            )

            print(f"  Augmented melody: {len(augmented_melody)} tokens")
            print(f"  Augmented chords: {len(augmented_chords)} tokens")
            print(f"  Melody sample: {augmented_melody[:8]}...")
            print(f"  Chord sample: {augmented_chords[:4]}...")

        except AttributeError:
            # Manual augmentation
            melody_len = len(original_melody)
            repetitions_needed = (min_tokens + melody_len - 1) // melody_len

            augmented_melody = []
            augmented_chords = []

            for rep in range(repetitions_needed):
                # Add variation to avoid exact repetition
                for note in original_melody:
                    # Simple transposition variation
                    if rep > 0:
                        transposed_note = composer.Note(
                            (note.scale_degree + rep - 1) % 7 + 1, note.octave
                        )
                        augmented_melody.append(str(transposed_note))
                    else:
                        augmented_melody.append(str(note))

                for chord in original_chords:
                    if rep > 0:
                        # Simple inversion variation
                        varied_chord = composer.Chord(
                            chord.root,
                            chord.chord_type,
                            inversion=(chord.inversion + rep) % 3,
                        )
                        augmented_chords.append(str(varied_chord))
                    else:
                        augmented_chords.append(str(chord))

            # Trim to target length
            augmented_melody = augmented_melody[:min_tokens]
            augmented_chords = augmented_chords[: min_tokens // 2]

            print(f"  Manual augmented melody: {len(augmented_melody)} tokens")
            print(f"  Manual augmented chords: {len(augmented_chords)} tokens")

        print()


def demonstrate_batch_processing() -> None:
    """Show batch processing for large ML datasets."""
    print("=== Batch Processing for ML ===")

    # Create a large dataset simulation
    batch_sizes = [10, 50, 100]
    total_sequences = 500

    print(f"Simulating batch processing for {total_sequences} sequences:")
    print()

    for batch_size in batch_sizes:
        print(f"Batch size: {batch_size}")

        # Calculate batch statistics
        num_batches = (total_sequences + batch_size - 1) // batch_size
        print(f"  Number of batches: {num_batches}")

        # Simulate processing batches
        processed_sequences = 0
        total_tokens = 0

        for batch_idx in range(num_batches):
            # Simulate batch creation
            sequences_in_batch = min(batch_size, total_sequences - processed_sequences)

            # Simulate token generation for batch
            batch_tokens = 0
            for seq_idx in range(sequences_in_batch):
                # Simulate sequence of varying length
                sequence_length = 8 + (seq_idx % 5)  # 8-12 tokens per sequence
                batch_tokens += sequence_length

            total_tokens += batch_tokens
            processed_sequences += sequences_in_batch

            if batch_idx < 3:  # Show first few batches
                batch_info = f"{sequences_in_batch} sequences, {batch_tokens} tokens"
                print(f"    Batch {batch_idx + 1}: {batch_info}")

        print(f"  Total processed: {processed_sequences} sequences")
        print(f"  Total tokens: {total_tokens}")
        avg_tokens = total_tokens / processed_sequences
        print(f"  Average tokens per sequence: {avg_tokens:.1f}")
        print()

    # Demonstrate memory-efficient processing
    print("Memory-efficient processing strategies:")
    print("  1. Stream processing: Process one batch at a time")
    print("  2. Token vocabulary sharing: Reuse token libraries across batches")
    print("  3. Binary serialization: Use 5-byte chord format for memory efficiency")
    print("  4. Lazy loading: Load data only when needed")
    print("  5. Parallel processing: Process multiple batches concurrently")
    print()


def main() -> None:
    """Run all ML tokenization demonstration functions."""
    print("Composer Library - Advanced ML Tokenization Examples")
    print("=" * 65)
    print()

    demonstrate_timeline_reconstruction()
    demonstrate_midi_like_processing()
    demonstrate_cluster_detokenization()
    demonstrate_vocabulary_optimization()
    demonstrate_sequence_augmentation()
    demonstrate_batch_processing()

    print("All ML tokenization examples completed!")
    print()
    print("Note: Some advanced ML features may require additional Python")
    print("bindings to be fully functional. This example demonstrates the")
    print("intended usage patterns for machine learning applications.")


if __name__ == "__main__":
    main()
