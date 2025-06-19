#!/usr/bin/env python3
"""
AI-Powered Chord Suggestions

This example demonstrates the AI engine for chord progression suggestions:
- Initializing the AI engine with training data
- Context-aware chord suggestions
- Magic chord solutions using statistical weighting
- Bass harmonization algorithms
- Scale degree harmonization

Based on the Composer specification: ai-powered-features.spec
"""

import composer

# Type aliases for clarity
AiEngine = composer.AiEngine  # Replace with actual type when available
SuggestionContext = dict[str, str]  # Simplified type for context


def create_sample_training_data() -> list[tuple]:
    """Create sample chord progressions for training the AI engine."""
    print("=== Creating Training Data ===")

    # Common chord progressions in various keys
    progressions = [
        # I-V-vi-IV progression in C major
        (
            [
                composer.Chord(1, 5),
                composer.Chord(5, 5),
                composer.Chord(6, 5),
                composer.Chord(4, 5),
            ],
            "pop_progression_1",
            "C",
        ),
        # ii-V-I progression in C major
        (
            [composer.Chord(2, 7), composer.Chord(5, 7), composer.Chord(1, 5)],
            "jazz_ii_v_i",
            "C",
        ),
        # I-vi-IV-V progression in C major
        (
            [
                composer.Chord(1, 5),
                composer.Chord(6, 5),
                composer.Chord(4, 5),
                composer.Chord(5, 5),
            ],
            "classic_progression",
            "C",
        ),
        # Minor progression: i-VII-VI-VII in A minor
        (
            [
                composer.Chord(1, 5),
                composer.Chord(7, 5),
                composer.Chord(6, 5),
                composer.Chord(7, 5),
            ],
            "minor_progression",
            "Am",
        ),
        # Jazz progression: IM7-VI7-ii7-V7 in C major
        (
            [
                composer.Chord(1, 7),
                composer.Chord(6, 7),
                composer.Chord(2, 7),
                composer.Chord(5, 7),
            ],
            "jazz_circle",
            "C",
        ),
    ]

    print(f"Created {len(progressions)} training progressions")
    for i, (chords, name, key) in enumerate(progressions, 1):
        print(f"  {i}. {name} in {key}: {len(chords)} chords")

    print()
    return progressions


def demonstrate_ai_engine_setup() -> AiEngine:
    """Initialize and configure the AI engine."""
    print("=== AI Engine Setup ===")

    # Create AI engine with custom configuration
    engine = composer.AiEngine(max_memory_mb=64, enable_monitoring=True)
    print(f"AI Engine created. Initialized: {engine.is_initialized()}")

    # Initialize with training data
    training_data = create_sample_training_data()
    engine.initialize(training_data)
    print(f"Engine initialized: {engine.is_initialized()}")

    # Get initial metrics
    metrics = engine.get_metrics()
    print("Initial metrics:")
    print(f"  Total patterns: {metrics['total_patterns']}")
    print(f"  Memory usage: {metrics['memory_usage_bytes']} bytes")
    print(f"  Uptime: {metrics['uptime_seconds']} seconds")
    print()

    return engine


def demonstrate_suggestion_context() -> SuggestionContext:
    """Show how to create and configure suggestion contexts."""
    print("=== Suggestion Context Configuration ===")

    # Create a C major scale for context
    major_scale = [
        True,
        False,
        True,
        False,
        True,
        True,
        False,
        True,
        False,
        True,
        False,
        True,
    ]
    scale = composer.ScaleFingerprint(major_scale)

    # Create suggestion context with various parameters
    context = composer.SuggestionContext(
        scale_fingerprint=scale,
        position_in_progression=0.75,  # Near end of progression
        target_valence=0.6,  # Slightly positive
        complexity_preference=0.4,  # Moderate complexity
        avoid_repetition_within=4,
    )

    print("Suggestion context created:")
    print(f"  Position in progression: {context.position_in_progression}")
    print(f"  Target valence: {context.target_valence}")
    print(f"  Complexity preference: {context.complexity_preference}")

    # Add recent chords to context
    recent_chords = [
        composer.Chord(1, 5),  # C major
        composer.Chord(5, 5),  # F major
        composer.Chord(7, 5),  # G major
    ]

    for chord in recent_chords:
        context.add_recent_chord(chord)

    print(f"Added {len(recent_chords)} recent chords to context")

    # Set genre weights
    context.set_genre_weight("pop", 0.7)
    context.set_genre_weight("jazz", 0.3)
    context.set_genre_weight("classical", 0.2)

    print("Genre weights configured")
    print()

    return context


def demonstrate_chord_suggestions(engine: AiEngine, context: SuggestionContext) -> None:
    """Get and analyze chord progression suggestions."""
    print("=== Chord Progression Suggestions ===")

    # Create a partial progression
    partial_progression = [
        composer.Chord(1, 5),  # C major
        composer.Chord(6, 7),  # Am7
    ]

    print("Current progression:")
    for i, chord in enumerate(partial_progression, 1):
        print(f"  {i}. {chord}")
    print()

    # Configure suggestion parameters
    config = composer.SuggestionConfig(
        max_suggestions=8,
        min_confidence=0.3,
        search_depth=3,
        use_probabilistic=True,
        temperature=0.8,
        enable_context_weighting=True,
    )

    print("Suggestion config:")
    print(f"  Max suggestions: {config.max_suggestions}")
    print(f"  Min confidence: {config.min_confidence}")
    print(f"  Temperature: {config.temperature}")
    print()

    # Get suggestions
    try:
        suggestions = engine.get_chord_suggestions(partial_progression, context, config)

        print(f"Generated {len(suggestions)} suggestions:")
        for i, suggestion in enumerate(suggestions, 1):
            print(f"  {i}. {suggestion.chord}")
            print(f"     Confidence: {suggestion.confidence:.3f}")
            print(f"     Frequency score: {suggestion.frequency_score:.3f}")
            print(f"     Context score: {suggestion.context_score:.3f}")
            print(f"     Theory score: {suggestion.theory_score:.3f}")
            print(f"     Weighted score: {suggestion.weighted_score:.3f}")
            print(f"     Reasoning: {suggestion.reasoning}")
            print()

    except Exception as e:
        print(f"Error getting suggestions: {e}")

    print()


def demonstrate_magic_chord_solutions(engine: AiEngine) -> None:
    """Show magic chord solutions using statistical weighting."""
    print("=== Magic Chord Solutions ===")

    # Define previous and following chords
    previous_chords = [
        composer.Chord(1, 5),  # C major
        composer.Chord(5, 5),  # F major
    ]

    following_chords = [
        composer.Chord(1, 5),  # C major (back to tonic)
    ]

    print("Finding magic chord between:")
    print(f"  Previous: {[str(c) for c in previous_chords]}")
    print(f"  Following: {[str(c) for c in following_chords]}")
    print()

    try:
        magic_solutions = engine.get_magic_chord_solutions(
            previous_chords, following_chords, "major", limit=5
        )

        print(f"Magic chord solutions ({len(magic_solutions)}):")
        for i, solution in enumerate(magic_solutions, 1):
            print(f"  {i}. {solution.chord}")
            print(f"     Confidence: {solution.confidence:.3f}")
            print(f"     Score: {solution.weighted_score:.3f}")
            print(f"     Reasoning: {solution.reasoning}")
            print()

    except Exception as e:
        print(f"Error getting magic solutions: {e}")


def demonstrate_bass_harmonization(engine: AiEngine) -> None:
    """Show bass harmonization and bass line solutions."""
    print("=== Bass Harmonization ===")

    # Create a chord progression for bass harmonization
    progression = [
        composer.Chord(1, 5),  # C major
        composer.Chord(6, 7),  # Am7
        composer.Chord(4, 5),  # F major
        composer.Chord(5, 5),  # G major
    ]

    print("Progression for bass harmonization:")
    for i, chord in enumerate(progression, 1):
        print(f"  {i}. {chord}")
    print()

    # Test different bass styles
    bass_styles = ["Root", "Alternating", "Walking", "Arpeggiated", "Rhythmic"]

    for style in bass_styles:
        try:
            harmonization = engine.harmonize_bass_line(
                progression,
                style=style,
                complexity=0.6,
                enable_walking=(style == "Walking"),
            )

            print(f"{style} bass style:")
            print(f"  Bass notes: {harmonization.bass_notes}")
            print(f"  Rhythm: {harmonization.rhythm}")
            print(f"  Confidence: {harmonization.confidence:.3f}")
            print(f"  Style: {harmonization.style}")
            print()

        except Exception as e:
            print(f"Error with {style} bass: {e}")

    # Demonstrate magic bass solutions
    print("Magic Bass Solutions:")
    try:
        bass_solutions = engine.get_magic_bass_solutions("C", "major", limit=3)

        for i, solution in enumerate(bass_solutions, 1):
            print(f"  {i}. {solution.chord} for C bass")
            print(f"     Confidence: {solution.confidence:.3f}")
            print(f"     Reasoning: {solution.reasoning}")
            print()

    except Exception as e:
        print(f"Error getting bass solutions: {e}")


def demonstrate_scale_degree_harmonization(engine: AiEngine) -> None:
    """Show scale degree harmonization functionality."""
    print("=== Scale Degree Harmonization ===")

    # Test different scale degree combinations (as bit patterns)
    scale_patterns = [
        (0b000000001001, "Root and Fifth"),  # 1st and 5th degrees
        (0b000100001001, "Major Triad"),  # 1st, 3rd, 5th degrees
        (0b001100001001, "Seventh Chord"),  # 1st, 3rd, 5th, 7th degrees
        (0b000010001001, "Sus2"),  # 1st, 2nd, 5th degrees
        (0b000001001001, "Sus4"),  # 1st, 4th, 5th degrees
    ]

    for bits, description in scale_patterns:
        print(f"\nHarmonizing {description} (bits: {bits:012b}):")

        try:
            solutions = engine.get_harmonize_by_sd_solutions(bits, "major", limit=3)

            for i, solution in enumerate(solutions, 1):
                print(f"  {i}. {solution.chord}")
                print(f"     Confidence: {solution.confidence:.3f}")
                print(f"     Score: {solution.weighted_score:.3f}")
                if solution.reasoning:
                    print(f"     Reasoning: {solution.reasoning}")

            if not solutions:
                print("  No solutions found")

        except Exception as e:
            print(f"  Error: {e}")


def demonstrate_difficulty_assessment(engine: AiEngine) -> None:
    """Show difficulty assessment for chord progressions."""
    print("=== Difficulty Assessment ===")

    # Test progressions with different complexity levels
    progressions = [
        # Simple progression
        (
            [composer.Chord(1, 5), composer.Chord(5, 5), composer.Chord(1, 5)],
            "Simple I-V-I",
        ),
        # Moderate progression
        (
            [
                composer.Chord(1, 5),
                composer.Chord(6, 7),
                composer.Chord(4, 5),
                composer.Chord(5, 5),
            ],
            "Pop progression",
        ),
        # Complex jazz progression
        (
            [
                composer.Chord(1, 7),
                composer.Chord(6, 7),
                composer.Chord(2, 7),
                composer.Chord(5, 7),
            ],
            "Jazz progression",
        ),
    ]

    for chords, name in progressions:
        print(f"\nAssessing: {name}")
        print(f"Chords: {[str(c) for c in chords]}")

        try:
            assessment = engine.assess_difficulty(
                chords, tempo_bpm=120.0, time_signature=(4, 4)
            )

            print(f"  Overall score: {assessment.overall_score:.1f}/10")
            print(f"  Skill level: {assessment.skill_level}")
            print(f"  Confidence: {assessment.confidence:.3f}")
            print(f"  Harmonic complexity: {assessment.harmonic_complexity:.2f}")
            print(f"  Rhythmic complexity: {assessment.rhythmic_complexity:.2f}")
            print(f"  Technical complexity: {assessment.technical_complexity:.2f}")
            print(f"  Melodic complexity: {assessment.melodic_complexity:.2f}")
            print(f"  Unique chords: {assessment.unique_chords}")
            print(f"  Extended harmonies: {assessment.extended_harmonies}")

        except Exception as e:
            print(f"  Error: {e}")


def demonstrate_performance_metrics(engine: AiEngine) -> None:
    """Show AI engine performance monitoring."""
    print("=== Performance Metrics ===")

    # Get current metrics
    metrics = engine.get_metrics()

    print("Current engine metrics:")
    print(f"  Total requests: {metrics['total_requests']}")
    print(f"  Average response time: {metrics['avg_response_time_ms']:.2f} ms")
    print(f"  Memory usage: {metrics['memory_usage_bytes']:,} bytes")
    print(f"  Cache hit rate: {metrics['cache_hit_rate']:.1%}")
    print(f"  Total patterns: {metrics['total_patterns']}")
    print(f"  Uptime: {metrics['uptime_seconds']} seconds")

    # Test average suggestion time
    avg_time = engine.avg_suggestion_time_ms()
    print(f"  Average suggestion time: {avg_time:.2f} ms")

    # Memory validation
    try:
        engine.validate_memory_usage()
        print("✓ Memory usage within limits")
    except Exception as e:
        print(f"✗ Memory validation failed: {e}")

    print()


def main() -> None:
    """Run all AI suggestion demonstration functions."""
    print("Composer Library - AI-Powered Chord Suggestions")
    print("=" * 60)
    print()

    # Setup
    engine = demonstrate_ai_engine_setup()
    context = demonstrate_suggestion_context()

    # Main functionality
    demonstrate_chord_suggestions(engine, context)
    demonstrate_magic_chord_solutions(engine)
    demonstrate_bass_harmonization(engine)
    demonstrate_scale_degree_harmonization(engine)
    demonstrate_difficulty_assessment(engine)
    demonstrate_performance_metrics(engine)

    # Cleanup
    print("=== Engine Shutdown ===")
    engine.clear_caches()
    engine.shutdown()
    print("AI engine shut down successfully")

    print("\nAll AI examples completed successfully!")


if __name__ == "__main__":
    main()
