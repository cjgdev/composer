#!/usr/bin/env python3
"""
Complete Composer Workflow Example

This comprehensive example demonstrates a full workflow using all major
Composer library features:
- Creating and analyzing chord progressions
- AI-powered suggestions and harmonization
- Difficulty assessment and performance optimization
- Serialization for storage and ML applications
- Real-world music composition scenarios

Based on all Composer specifications and test cases.
"""

from typing import Any

import composer


class ComposerWorkflow:
    """Complete workflow manager for the Composer library."""

    def __init__(self) -> None:
        """Initialize the workflow with AI engine and configurations."""
        print("=== Initializing Composer Workflow ===")

        # Initialize AI engine
        self.ai_engine = composer.AiEngine(max_memory_mb=128, enable_monitoring=True)
        self.is_initialized = False

        # Create standard scales
        self.major_scale = composer.ScaleFingerprint(
            [
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
        )

        self.minor_scale = composer.ScaleFingerprint(
            [
                True,
                False,
                True,
                True,
                False,
                True,
                False,
                True,
                True,
                False,
                True,
                False,
            ]
        )

        # Token library for serialization
        self.token_library = composer.TokenLibrary()

        print("✓ Workflow initialized")
        print()

    def setup_training_data(self) -> None:
        """Initialize AI engine with comprehensive training data."""
        print("=== Setting Up Training Data ===")

        # Create diverse training progressions
        training_data = [
            # Pop progressions
            (
                [
                    composer.Chord(1, 5),
                    composer.Chord(2, 7),
                    composer.Chord(5, 5),
                    composer.Chord(7, 5),
                ],
                "pop_1",
                "C",
            ),
            (
                [
                    composer.Chord(1, 5),
                    composer.Chord(5, 5),
                    composer.Chord(7, 5),
                    composer.Chord(1, 5),
                ],
                "pop_2",
                "C",
            ),
            # Jazz progressions
            (
                [
                    composer.Chord(1, 7),
                    composer.Chord(2, 7),
                    composer.Chord(2, 7),
                    composer.Chord(7, 9),
                ],
                "jazz_1",
                "C",
            ),
            (
                [
                    composer.Chord(5, 7),
                    composer.Chord(2, 9),
                    composer.Chord(7, 7),
                    composer.Chord(1, 9),
                ],
                "jazz_2",
                "F",
            ),
            # Classical progressions
            (
                [
                    composer.Chord(1, 5),
                    composer.Chord(2, 7),
                    composer.Chord(7, 5),
                    composer.Chord(1, 5),
                ],
                "classical_1",
                "C",
            ),
            # Minor progressions
            (
                [
                    composer.Chord(2, 7),
                    composer.Chord(1, 5),
                    composer.Chord(5, 5),
                    composer.Chord(7, 5),
                ],
                "minor_1",
                "Am",
            ),
            # Extended harmonies
            (
                [
                    composer.Chord(1, 7),
                    composer.Chord(4, 7),
                    composer.Chord(7, 9),
                    composer.Chord(1, 7),
                ],
                "extended_1",
                "C",
            ),
        ]

        # Initialize engine
        self.ai_engine.initialize(training_data)
        self.is_initialized = True

        print(f"✓ Training data loaded: {len(training_data)} progressions")
        print(f"✓ AI engine initialized: {self.ai_engine.is_initialized()}")

        # Add chord tokens to library
        for progression, name, _key in training_data:
            for i, chord in enumerate(progression):
                binary_data = composer.serialize_chord_to_binary(chord)
                token_name = f"{name}_chord_{i}"
                self.token_library.add_chord_token(token_name, binary_data)

        print(f"✓ Token library populated: {len(self.token_library)} tokens")
        print()

    def analyze_chord_progression(
        self, progression: list[composer.Chord], name: str = "Progression"
    ) -> dict[str, Any]:
        """Comprehensive analysis of a chord progression."""
        print(f"=== Analyzing {name} ===")

        analysis = {
            "name": name,
            "chords": progression,
            "chord_count": len(progression),
            "complexity_scores": [],
            "difficulty_assessment": None,
            "progression_analysis": None,
            "serialization": {"binary_data": [], "hex_strings": [], "total_bytes": 0},
        }

        # Individual chord analysis
        print("Chord Analysis:")
        for i, chord in enumerate(progression, 1):
            complexity = composer.get_chord_complexity(chord)
            analysis["complexity_scores"].append(complexity)

            # Serialization
            binary_data = composer.serialize_chord_to_binary(chord)
            hex_string = composer.chord_to_hex(chord)
            analysis["serialization"]["binary_data"].append(binary_data)
            analysis["serialization"]["hex_strings"].append(hex_string)
            analysis["serialization"]["total_bytes"] += len(binary_data)

            print(f"  {i}. {chord}")
            print(f"     Complexity: {complexity:.2f}")
            print(f"     Binary: {len(binary_data)} bytes")
            print(f"     Hex: {hex_string}")

        # Overall complexity
        avg_complexity = sum(analysis["complexity_scores"]) / len(
            analysis["complexity_scores"]
        )
        max_complexity = max(analysis["complexity_scores"])

        print("\nOverall Complexity:")
        print(f"  Average: {avg_complexity:.2f}")
        print(f"  Maximum: {max_complexity:.2f}")
        print(
            f"  Total serialized size: {analysis['serialization']['total_bytes']} bytes"
        )

        # AI-powered difficulty assessment
        if self.is_initialized:
            try:
                difficulty = self.ai_engine.assess_difficulty(
                    progression, tempo_bpm=120.0, time_signature=(4, 4)
                )
                analysis["difficulty_assessment"] = difficulty

                print("\nDifficulty Assessment:")
                print(f"  Overall score: {difficulty.overall_score:.1f}/10")
                print(f"  Skill level: {difficulty.skill_level}")
                print(f"  Confidence: {difficulty.confidence:.3f}")
                print(f"  Harmonic complexity: {difficulty.harmonic_complexity:.2f}")
                print(f"  Unique chords: {difficulty.unique_chords}")

                # Progression analysis
                prog_analysis = self.ai_engine.analyze_progression(progression)
                analysis["progression_analysis"] = prog_analysis

                print(
                    f"  Voice leading quality: "
                    f"{prog_analysis.voice_leading_quality:.3f}"
                )

            except Exception as e:
                print(f"  AI analysis error: {e}")

        print()
        return analysis

    def generate_suggestions(
        self,
        partial_progression: list[composer.Chord],
        context_params: dict[str, Any] = None,
    ) -> list[composer.ChordSuggestion]:
        """Generate AI-powered chord suggestions."""
        print("=== Generating AI Suggestions ===")

        if not self.is_initialized:
            print("✗ AI engine not initialized")
            return []

        # Create suggestion context
        context_params = context_params or {}
        context = composer.SuggestionContext(
            scale_fingerprint=self.major_scale,
            position_in_progression=context_params.get("position", 0.75),
            target_valence=context_params.get("valence", 0.5),
            complexity_preference=context_params.get("complexity", 0.5),
            avoid_repetition_within=context_params.get("avoid_repetition", 4),
        )

        # Add recent chords to context
        for chord in partial_progression:
            context.add_recent_chord(chord)

        # Set genre weights
        for genre, weight in context_params.get("genres", {}).items():
            context.set_genre_weight(genre, weight)

        # Configuration
        config = composer.SuggestionConfig(
            max_suggestions=6,
            min_confidence=0.2,
            search_depth=3,
            use_probabilistic=True,
            temperature=0.7,
            enable_context_weighting=True,
        )

        print("Current progression:")
        for i, chord in enumerate(partial_progression, 1):
            print(f"  {i}. {chord}")

        try:
            suggestions = self.ai_engine.get_chord_suggestions(
                partial_progression, context, config
            )

            print(f"\nGenerated {len(suggestions)} suggestions:")
            for i, suggestion in enumerate(suggestions, 1):
                print(f"  {i}. {suggestion.chord}")
                print(f"     Confidence: {suggestion.confidence:.3f}")
                print(f"     Weighted score: {suggestion.weighted_score:.3f}")
                if suggestion.reasoning:
                    print(f"     Reasoning: {suggestion.reasoning}")

            print()
            return suggestions

        except Exception as e:
            print(f"Error generating suggestions: {e}")
            return []

    def create_bass_harmonization(
        self, progression: list[composer.Chord], style: str = "Walking"
    ) -> composer.BassHarmonization:
        """Create bass line harmonization for progression."""
        print(f"=== Creating {style} Bass Harmonization ===")

        if not self.is_initialized:
            print("✗ AI engine not initialized")
            return None

        print("Progression:")
        for i, chord in enumerate(progression, 1):
            print(f"  {i}. {chord}")

        try:
            harmonization = self.ai_engine.harmonize_bass_line(
                progression,
                style=style,
                complexity=0.7,
                enable_walking=(style.lower() == "walking"),
            )

            print(f"\n{style} Bass Line:")
            print(f"  Style: {harmonization.style}")
            print(f"  Bass notes: {harmonization.bass_notes}")
            print(f"  Rhythm pattern: {harmonization.rhythm}")
            print(f"  Confidence: {harmonization.confidence:.3f}")

            # Convert bass notes to note names for readability
            note_names = [
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
            ]
            bass_note_names = [
                note_names[note % 12] for note in harmonization.bass_notes
            ]
            print(f"  Bass note names: {bass_note_names}")

            print()
            return harmonization

        except Exception as e:
            print(f"Error creating bass harmonization: {e}")
            return None

    def demonstrate_magic_solutions(self) -> None:
        """Demonstrate magic chord and bass solutions."""
        print("=== Magic Solutions Demonstration ===")

        if not self.is_initialized:
            print("✗ AI engine not initialized")
            return

        # Magic chord solutions
        print("Magic Chord Solutions:")
        previous = [composer.Chord(1, 5), composer.Chord(6, 7)]  # C major, Am7
        following = [composer.Chord(1, 5)]  # Back to C major

        print(
            f"  Between: {[str(c) for c in previous]} -> ? -> "
            f"{[str(c) for c in following]}"
        )

        try:
            magic_chords = self.ai_engine.get_magic_chord_solutions(
                previous, following, "major", limit=3
            )

            for i, solution in enumerate(magic_chords, 1):
                print(
                    f"    {i}. {solution.chord} (confidence: {solution.confidence:.3f})"
                )

        except Exception as e:
            print(f"  Error: {e}")

        print()

        # Magic bass solutions
        print("Magic Bass Solutions:")
        bass_notes = ["C", "F", "G"]

        for bass_note in bass_notes:
            print(f"  For {bass_note} bass:")
            try:
                bass_solutions = self.ai_engine.get_magic_bass_solutions(
                    bass_note, "major", limit=2
                )

                for i, solution in enumerate(bass_solutions, 1):
                    print(
                        f"    {i}. {solution.chord} "
                        f"(confidence: {solution.confidence:.3f})"
                    )

            except Exception as e:
                print(f"    Error: {e}")

        print()

    def performance_analysis(self) -> dict[str, Any]:
        """Analyze engine performance and optimization."""
        print("=== Performance Analysis ===")

        if not self.is_initialized:
            print("✗ AI engine not initialized")
            return {}

        # Get current metrics
        metrics = self.ai_engine.get_metrics()

        print("Engine Metrics:")
        print(f"  Total requests: {metrics['total_requests']}")
        print(f"  Average response time: {metrics['avg_response_time_ms']:.2f} ms")
        print(f"  Memory usage: {metrics['memory_usage_bytes']:,} bytes")
        print(f"  Cache hit rate: {metrics['cache_hit_rate']:.1%}")
        print(f"  Total patterns: {metrics['total_patterns']}")
        print(f"  Uptime: {metrics['uptime_seconds']} seconds")

        # Test suggestion timing
        avg_time = self.ai_engine.avg_suggestion_time_ms()
        print(f"  Average suggestion time: {avg_time:.2f} ms")

        # Memory validation
        try:
            self.ai_engine.validate_memory_usage()
            print("  ✓ Memory usage within limits")
        except Exception as e:
            print(f"  ✗ Memory validation failed: {e}")

        # Performance benchmarks (targets from specification)
        performance_analysis = {
            "response_time_ms": metrics["avg_response_time_ms"],
            "memory_usage_mb": metrics["memory_usage_bytes"] / (1024 * 1024),
            "cache_efficiency": metrics["cache_hit_rate"],
            "meets_targets": {
                "response_time": metrics["avg_response_time_ms"] < 1.0,  # < 1ms target
                "memory_usage": (metrics["memory_usage_bytes"] / (1024 * 1024))
                < 150,  # < 150MB
                "suggestion_time": avg_time < 1.0,  # < 1ms target
            },
        }

        print("\nPerformance Targets:")
        for target, meets in performance_analysis["meets_targets"].items():
            status = "✓" if meets else "✗"
            print(f"  {status} {target.replace('_', ' ').title()}")

        print()
        return performance_analysis

    def export_for_ml(
        self, progressions: list[tuple[list[composer.Chord], str]]
    ) -> dict[str, Any]:
        """Export data in ML-ready formats."""
        print("=== ML Export Preparation ===")

        ml_export = {
            "tokenized_progressions": [],
            "chord_vocabulary": [],
            "binary_patterns": [],
            "hash_signatures": [],
            "trie_data": None,
        }

        # Collect all unique chords (use chord hex for uniqueness)
        chord_hex_set = set()
        chord_map = {}
        for progression, _ in progressions:
            for chord in progression:
                hex_key = composer.chord_to_hex(chord)
                if hex_key not in chord_hex_set:
                    chord_hex_set.add(hex_key)
                    chord_map[hex_key] = chord

        all_chords = list(chord_map.values())

        # Create chord vocabulary
        chord_binaries = []
        for chord in all_chords:
            binary = composer.serialize_chord_to_binary(chord)
            chord_binaries.append(binary)
            ml_export["chord_vocabulary"].append(
                {
                    "chord": str(chord),
                    "binary": binary,
                    "hex": composer.chord_to_hex(chord),
                }
            )

        print(f"Chord vocabulary: {len(ml_export['chord_vocabulary'])} unique chords")

        # Reduce vocabulary if needed (ML optimization)
        if len(chord_binaries) > 50:
            try:
                reduced_vocab = composer.py_reduce_chord_vocab(
                    chord_binaries, max_vocab=50
                )
                print(f"Reduced vocabulary: {len(reduced_vocab)} chords")
            except Exception as e:
                print(f"Vocabulary reduction error: {e}")

        # Tokenize progressions
        for progression, name in progressions:
            tokenized = []
            pattern_hash = 0

            for chord in progression:
                # Create token representation
                token = composer.chord_to_hex(chord)
                tokenized.append(token)

                # Update hash signature
                pattern_hash = composer.py_fold_hash(pattern_hash, token)

            ml_export["tokenized_progressions"].append(
                {"name": name, "tokens": tokenized, "hash": pattern_hash}
            )

            # Add binary pattern
            binary_pattern = [
                composer.serialize_chord_to_binary(c) for c in progression
            ]
            ml_export["binary_patterns"].append(binary_pattern)
            ml_export["hash_signatures"].append(pattern_hash)

        print(f"Tokenized progressions: {len(ml_export['tokenized_progressions'])}")

        # Create trie for pattern storage
        trie = composer.TrieNode()
        for i, (progression, _) in enumerate(progressions):
            pattern = [composer.serialize_chord_to_binary(c) for c in progression]
            trie.add_pattern(pattern, i)

        # Serialize trie
        try:
            trie_binary = composer.py_serialize_trie(trie)
            ml_export["trie_data"] = {
                "binary_size": len(trie_binary),
                "node_count": trie.node_count,
                "serialized": trie_binary,
            }
            print(f"Trie serialized: {len(trie_binary)} bytes, {trie.node_count} nodes")
        except Exception as e:
            print(f"Trie serialization error: {e}")

        print()
        return ml_export


def demonstrate_complete_workflow() -> None:
    """Run a complete workflow demonstration."""
    print("Composer Library - Complete Workflow Demonstration")
    print("=" * 65)
    print()

    # Initialize workflow
    workflow = ComposerWorkflow()
    workflow.setup_training_data()

    # Create test progressions
    test_progressions = [
        # Simple pop progression
        (
            [
                composer.Chord(1, 5),
                composer.Chord(6, 7),
                composer.Chord(5, 5),
                composer.Chord(7, 5),
            ],
            "Pop Progression",
        ),
        # Jazz progression
        (
            [composer.Chord(1, 7), composer.Chord(4, 7), composer.Chord(7, 9)],
            "Jazz Progression",
        ),
        # Classical progression
        (
            [
                composer.Chord(1, 5),
                composer.Chord(2, 7),
                composer.Chord(7, 5),
                composer.Chord(1, 5),
            ],
            "Classical Progression",
        ),
    ]

    # Analyze each progression
    analyses = []
    for progression, name in test_progressions:
        analysis = workflow.analyze_chord_progression(progression, name)
        analyses.append(analysis)

    # Generate suggestions for partial progressions
    partial_progression = [composer.Chord(1, 5), composer.Chord(6, 7)]
    context_params = {
        "position": 0.6,
        "valence": 0.4,
        "complexity": 0.7,
        "genres": {"pop": 0.8, "jazz": 0.4},
    }

    suggestions = workflow.generate_suggestions(partial_progression, context_params)

    # Create bass harmonizations
    for progression, _name in test_progressions[:2]:  # First two progressions
        for style in ["Root", "Walking", "Alternating"]:
            workflow.create_bass_harmonization(progression, style)

    # Demonstrate magic solutions
    workflow.demonstrate_magic_solutions()

    # Performance analysis
    performance = workflow.performance_analysis()

    # ML export
    ml_data = workflow.export_for_ml(test_progressions)

    # Summary report
    print("=== Workflow Summary ===")
    print(f"Progressions analyzed: {len(analyses)}")
    print(f"Suggestions generated: {len(suggestions)}")
    print(
        f"Performance meets targets: "
        f"{sum(performance.get('meets_targets', {}).values())}/3"
    )
    print(f"ML export ready: {len(ml_data['tokenized_progressions'])} progressions")

    # Cleanup
    print("\n=== Cleanup ===")
    workflow.ai_engine.clear_caches()
    workflow.ai_engine.shutdown()
    print("✓ Workflow completed successfully")


if __name__ == "__main__":
    demonstrate_complete_workflow()
