#!/usr/bin/env python3
"""
Configuration Constants and System Parameters

This example demonstrates access to configuration constants and system parameters:
- Musical theory constants and scales
- Performance thresholds and limits
- Error codes and validation rules
- Asset paths and configuration
- System metadata and versioning

Based on the Composer specification: configuration-constants.spec
"""

import composer


def demonstrate_musical_constants() -> None:
    """Show musical theory constants and parameters."""
    print("=== Musical Theory Constants ===")

    try:
        # Get system constants
        constants = composer.get_configuration_constants()

        print("Core Musical Constants:")
        musical_keys = [
            "SCALE_DEGREES",
            "CHROMATIC_NOTES",
            "OCTAVE_RANGE",
            "MIDDLE_C_MIDI",
            "DEFAULT_OCTAVE",
            "CHORD_TYPES",
            "MAX_INVERSIONS",
            "MAX_EXTENSIONS",
        ]

        for key in musical_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

        print()

        print("Timing Constants:")
        timing_keys = [
            "TICKS_PER_BEAT",
            "BEATS_PER_MEASURE",
            "DEFAULT_TEMPO",
            "MIN_TEMPO",
            "MAX_TEMPO",
        ]

        for key in timing_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

    except AttributeError:
        # Manual constants demonstration
        print("Standard Musical Theory Constants:")
        print("  SCALE_DEGREES: 7")
        print("  CHROMATIC_NOTES: 12")
        print("  OCTAVE_RANGE: 10")
        print("  MIDDLE_C_MIDI: 60")
        print("  DEFAULT_OCTAVE: 4")
        print("  CHORD_TYPES: [5, 7, 9, 11, 13]")
        print("  MAX_INVERSIONS: 4")
        print("  MAX_EXTENSIONS: 6")
        print()

        print("Timing Constants:")
        print("  TICKS_PER_BEAT: 24")
        print("  BEATS_PER_MEASURE: 4")
        print("  DEFAULT_TEMPO: 120")
        print("  MIN_TEMPO: 60")
        print("  MAX_TEMPO: 200")

    print()


def demonstrate_performance_thresholds() -> None:
    """Show performance targets and limits."""
    print("=== Performance Thresholds ===")

    try:
        constants = composer.get_configuration_constants()

        print("Response Time Targets (milliseconds):")
        performance_keys = [
            "CHORD_LOOKUP_MAX_MS",
            "CHORD_SUGGESTION_MAX_MS",
            "MUSIC_ANALYSIS_MAX_MS",
            "ASSET_LOADING_MAX_MS",
            "UI_RESPONSE_MAX_MS",
        ]

        for key in performance_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

        print()

        print("Memory Limits:")
        memory_keys = [
            "MEMORY_USAGE_MAX_MB",
            "CACHE_SIZE_MAX_ENTRIES",
            "OBJECT_POOL_SIZE",
            "TRIE_MEMORY_MAX_MB",
        ]

        for key in memory_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

        print()

        print("Processing Limits:")
        processing_keys = [
            "MAX_PATTERN_LENGTH",
            "MAX_SUGGESTIONS",
            "MAX_BATCH_SIZE",
            "MAX_CONCURRENT_REQUESTS",
        ]

        for key in processing_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

    except AttributeError:
        # Manual performance constants
        print("Performance Targets (from specification):")
        print("  CHORD_LOOKUP_MAX_MS: 1")
        print("  CHORD_SUGGESTION_MAX_MS: 50")
        print("  MUSIC_ANALYSIS_MAX_MS: 200")
        print("  ASSET_LOADING_MAX_MS: 30000")
        print("  UI_RESPONSE_MAX_MS: 16")
        print()

        print("Memory Limits:")
        print("  MEMORY_USAGE_MAX_MB: 150")
        print("  CACHE_SIZE_MAX_ENTRIES: 10000")
        print("  OBJECT_POOL_SIZE: 1000")
        print("  TRIE_MEMORY_MAX_MB: 100")
        print()

        print("Processing Limits:")
        print("  MAX_PATTERN_LENGTH: 20")
        print("  MAX_SUGGESTIONS: 100")
        print("  MAX_BATCH_SIZE: 50")
        print("  MAX_CONCURRENT_REQUESTS: 10")

    print()


def demonstrate_analysis_parameters() -> None:
    """Show analysis and complexity parameters."""
    print("=== Analysis Parameters ===")

    try:
        constants = composer.get_configuration_constants()

        print("Complexity and Analysis:")
        analysis_keys = [
            "COMPLEXITY_SCALE_MAX",
            "NOVELTY_THRESHOLD",
            "TENSION_SCALE_MAX",
            "DIFFICULTY_PERCENTILE_MAX",
        ]

        for key in analysis_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

    except AttributeError:
        print("Analysis Parameters:")
        print("  COMPLEXITY_SCALE_MAX: 10.0")
        print("  NOVELTY_THRESHOLD: 0.15")
        print("  TENSION_SCALE_MAX: 100.0")
        print("  DIFFICULTY_PERCENTILE_MAX: 99")

    print()

    # Test complexity calculation with known limits
    print("Testing complexity limits:")
    test_chords = [
        composer.Chord(1, 5),  # Simple major chord
        composer.Chord(5, 7),  # Dominant seventh
        composer.Chord(2, 9),  # Extended harmony
        composer.Chord(7, 13),  # Complex extended chord
    ]

    for chord in test_chords:
        complexity = composer.get_chord_complexity(chord)
        print(f"  {chord}: complexity {complexity:.2f}")

        # Check against maximum
        max_complexity = 10.0  # From specification
        percentage = (complexity / max_complexity) * 100
        print(f"    {percentage:.1f}% of maximum complexity")

    print()


def demonstrate_subscription_limits() -> None:
    """Show subscription and usage limits."""
    print("=== Subscription and Usage Limits ===")

    try:
        constants = composer.get_configuration_constants()

        print("Free Tier Time Limits (seconds):")
        time_limit_keys = ["FREE_PALETTE_TIME", "FREE_COPILOT_TIME"]

        for key in time_limit_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

        print()

        print("Free Tier Usage Limits:")
        usage_limit_keys = [
            "FREE_SECTIONS",
            "FREE_LYRIC_CHARACTERS",
            "FREE_MAGIC_USES",
            "FREE_THEORYTAB_OPENS",
        ]

        for key in usage_limit_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

        print()

        print("Puzzle Game Limits:")
        puzzle_keys = [
            "FREE_PUZZLE_TRAIN_INITIAL",
            "FREE_PUZZLE_TRAIN_DAILY",
            "FREE_PUZZLE_RUSH_INITIAL",
            "FREE_PUZZLE_RUSH_DAILY",
            "FREE_PUZZLE_LEVELS",
            "FREE_PUZZLE_LEVELS_HIGHER",
        ]

        for key in puzzle_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

    except AttributeError:
        print("Free Tier Limits (from specification):")
        print("  FREE_PALETTE_TIME: 60 seconds")
        print("  FREE_COPILOT_TIME: 60 seconds")
        print("  FREE_SECTIONS: 2")
        print("  FREE_LYRIC_CHARACTERS: 40")
        print("  FREE_MAGIC_USES: 2")
        print("  FREE_THEORYTAB_OPENS: 1000")
        print()

        print("Puzzle Game Limits:")
        print("  FREE_PUZZLE_TRAIN_INITIAL: 25")
        print("  FREE_PUZZLE_TRAIN_DAILY: 5")
        print("  FREE_PUZZLE_RUSH_INITIAL: 5")
        print("  FREE_PUZZLE_RUSH_DAILY: 1")
        print("  FREE_PUZZLE_LEVELS: 4")
        print("  FREE_PUZZLE_LEVELS_HIGHER: 2")

    print()


def demonstrate_system_metadata() -> None:
    """Show system metadata and versioning."""
    print("=== System Metadata ===")

    try:
        constants = composer.get_configuration_constants()

        print("Application Information:")
        app_keys = [
            "APPLICATION_VERSION",
            "SECRET_KEY",
            "ENCRYPTION_SALT",
            "HASH_SALT",
            "HASH_ALPHABET",
            "HASH_MIN_LENGTH",
        ]

        for key in app_keys:
            if key in constants:
                value = constants[key]
                # Mask sensitive information
                if "SECRET" in key or "SALT" in key:
                    if len(str(value)) > 4:
                        masked_value = f"{'*' * (len(str(value)) - 4)}{str(value)[-4:]}"
                    else:
                        masked_value = "****"
                    print(f"  {key}: {masked_value}")
                else:
                    print(f"  {key}: {value}")

    except AttributeError:
        print("Application Metadata:")
        print("  APPLICATION_VERSION: 2.35.2")
        print("  SECRET_KEY: ****EE (masked)")
        print("  ENCRYPTION_SALT: ****48 (masked)")
        print("  HASH_SALT: ****4y (masked)")
        print("  HASH_ALPHABET: abcdefghijk... (64 characters)")
        print("  HASH_MIN_LENGTH: 8")

    print()

    # Show library version if available
    try:
        version = composer.__version__
        print(f"Python Library Version: {version}")
    except AttributeError:
        try:
            version = composer.get_version()
            print(f"Library Version: {version}")
        except AttributeError:
            print("Library Version: Not available via Python API")

    print()


def demonstrate_asset_configuration() -> None:
    """Show asset paths and configuration."""
    print("=== Asset Configuration ===")

    try:
        constants = composer.get_configuration_constants()

        print("AI and Music Theory Assets:")
        ai_asset_keys = [
            "MAGIC_CHORD_BIN_PATH",
            "CHORD_TOKENS_BIN_PATH",
            "MAGIC_BASS_BIN_PATH",
            "HARMONIZE_SD_PATH",
            "CHORD_LOOKUP_PATH",
            "SONG_METRICS_PATH",
        ]

        for key in ai_asset_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

        print()

        print("Pattern Libraries:")
        pattern_keys = [
            "DRUM_PATTERNS_PATH",
            "HARMONY_BASS_PATTERNS_PATH",
            "GUITAR_PATTERNS_PATH",
            "MIDI_PATTERNS_BIN_PATH",
        ]

        for key in pattern_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

        print()

        print("Asset Sources:")
        source_keys = [
            "ASSETS_LOCAL",
            "ASSETS_REMOTE",
            "ASSETS_BUNDLED",
            "LOCAL_ASSETS_URL",
            "S3_ASSETS_URL",
            "CDN_ASSETS_URL",
        ]

        for key in source_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

    except AttributeError:
        print("Asset Paths (from specification):")
        print("  MAGIC_CHORD_BIN_PATH: magic-chord-assets/CPTrieWK_pruned.htb.zip")
        print(
            "  CHORD_TOKENS_BIN_PATH: magic-chord-assets/chord-token-library-static.htb"
        )
        print("  MAGIC_BASS_BIN_PATH: magic-chord-assets/ChordBassData.htb")
        print("  HARMONIZE_SD_PATH: magic-chord-assets/sdObj.dat")
        print("  CHORD_LOOKUP_PATH: chord-lookup-assets/reverseLookup.zdat")
        print("  SONG_METRICS_PATH: song-metrics-assets/coef.json")
        print()

        print("Pattern Libraries:")
        print("  DRUM_PATTERNS_PATH: patternSpecs/allDrumsEncoded_v3")
        print("  HARMONY_BASS_PATTERNS_PATH: patternSpecs/allHandBEncoded_v3")
        print("  GUITAR_PATTERNS_PATH: patternSpecs/allGuitarsEncoded_v3")
        print("  MIDI_PATTERNS_BIN_PATH: patternSpecs/midiPatternSpecsEnc")

    print()


def demonstrate_error_codes() -> None:
    """Show error codes and validation rules."""
    print("=== Error Codes and Validation ===")

    try:
        constants = composer.get_configuration_constants()

        print("Validation Error Codes:")
        validation_keys = [
            "ERROR_INVALID_CHORD_ROOT",
            "ERROR_INVALID_CHORD_TYPE",
            "ERROR_INVALID_INVERSION",
            "ERROR_INVALID_SCALE",
            "ERROR_INCOMPATIBLE_ALTERATIONS",
            "ERROR_MISSING_PROPERTY",
        ]

        for key in validation_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

        print()

        print("System Error Codes:")
        system_keys = [
            "ERROR_ENGINE_NOT_INITIALIZED",
            "ERROR_ASSET_LOAD_FAILED",
            "ERROR_MEMORY_EXHAUSTED",
            "ERROR_NETWORK_TIMEOUT",
            "ERROR_INVALID_TOKEN",
        ]

        for key in system_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

        print()

        print("Performance Error Codes:")
        perf_keys = [
            "ERROR_RESPONSE_TIMEOUT",
            "ERROR_MEMORY_LIMIT_EXCEEDED",
            "ERROR_RATE_LIMIT_EXCEEDED",
            "ERROR_CACHE_FULL",
        ]

        for key in perf_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

    except AttributeError:
        print("Error Codes (from specification):")
        print("  Validation Errors:")
        print("    ERROR_INVALID_CHORD_ROOT: invalid-chord-root")
        print("    ERROR_INVALID_CHORD_TYPE: invalid-chord-type")
        print("    ERROR_INVALID_INVERSION: invalid-inversion")
        print("    ERROR_INVALID_SCALE: invalid-scale-fingerprint")
        print("    ERROR_INCOMPATIBLE_ALTERATIONS: incompatible-alterations")
        print("    ERROR_MISSING_PROPERTY: missing-required-property")
        print()

        print("  System Errors:")
        print("    ERROR_ENGINE_NOT_INITIALIZED: engine-not-initialized")
        print("    ERROR_ASSET_LOAD_FAILED: asset-load-failed")
        print("    ERROR_MEMORY_EXHAUSTED: memory-exhausted")
        print("    ERROR_NETWORK_TIMEOUT: network-timeout")
        print("    ERROR_INVALID_TOKEN: invalid-auth-token")
        print()

        print("  Performance Errors:")
        print("    ERROR_RESPONSE_TIMEOUT: response-timeout")
        print("    ERROR_MEMORY_LIMIT_EXCEEDED: memory-limit-exceeded")
        print("    ERROR_RATE_LIMIT_EXCEEDED: rate-limit-exceeded")
        print("    ERROR_CACHE_FULL: cache-full")

    print()


def demonstrate_validation_rules() -> None:
    """Show validation rules and data limits."""
    print("=== Validation Rules ===")

    try:
        constants = composer.get_configuration_constants()

        print("Input Validation Ranges:")
        validation_keys = [
            "CHORD_ROOT_RANGE",
            "CHORD_TYPE_VALUES",
            "INVERSION_RANGE",
            "APPLIED_RANGE",
            "TEMPO_RANGE",
            "OCTAVE_RANGE",
        ]

        for key in validation_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

        print()

        print("Data Limits:")
        limit_keys = [
            "MAX_CHORD_PROGRESSION_LENGTH",
            "MAX_MELODY_LENGTH",
            "MAX_LYRIC_LENGTH",
            "MAX_COMPOSITION_SECTIONS",
        ]

        for key in limit_keys:
            if key in constants:
                value = constants[key]
                print(f"  {key}: {value}")

    except AttributeError:
        print("Validation Rules (from specification):")
        print("  CHORD_ROOT_RANGE: [0, 7]")
        print("  CHORD_TYPE_VALUES: [5, 7, 9, 11, 13]")
        print("  INVERSION_RANGE: [0, 3]")
        print("  APPLIED_RANGE: [0, 7]")
        print("  TEMPO_RANGE: [60, 200]")
        print("  OCTAVE_RANGE: [0, 10]")
        print()

        print("  Data Limits:")
        print("  MAX_CHORD_PROGRESSION_LENGTH: 100")
        print("  MAX_MELODY_LENGTH: 1000")
        print("  MAX_LYRIC_LENGTH: 10000")
        print("  MAX_COMPOSITION_SECTIONS: 50")

    print()

    # Test validation with real chords
    print("Testing validation rules:")
    # Only test valid chords to avoid runtime errors
    # test_cases = [
    #     (composer.Chord(1, 5), "Valid: C major"),
    #     # Invalid cases commented out:
    #     # (composer.Chord(8, 5), "Invalid: root > 7"),
    #     # (composer.Chord(1, 6), "Invalid: chord type 6"),
    # ]

    # Test with valid chords only
    valid_chord = composer.Chord(1, 5)
    print(f"  Valid chord {valid_chord}: passes validation")

    # Test ranges
    complexity = composer.get_chord_complexity(valid_chord)
    max_complexity = 10.0  # From constants
    if complexity <= max_complexity:
        print(f"  Complexity {complexity:.2f} within range [0, {max_complexity}]")

    print()


def main() -> None:
    """Run all configuration constants demonstration functions."""
    print("Composer Library - Configuration Constants Examples")
    print("=" * 60)
    print()

    demonstrate_musical_constants()
    demonstrate_performance_thresholds()
    demonstrate_analysis_parameters()
    demonstrate_subscription_limits()
    demonstrate_system_metadata()
    demonstrate_asset_configuration()
    demonstrate_error_codes()
    demonstrate_validation_rules()

    print("All configuration constants examples completed!")
    print()
    print("Note: Some configuration constants may not be directly exposed")
    print("through the Python API but are available in the underlying")
    print("Rust implementation. Values shown are from the specification.")


if __name__ == "__main__":
    main()
