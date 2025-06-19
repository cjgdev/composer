AI Module Reference
===================

The AI module provides machine learning-powered musical intelligence features including chord progression suggestions, difficulty assessment, and pattern analysis.

AI Engine
---------

.. autoclass:: composer.AiEngine
   :members:
   :undoc-members:
   :show-inheritance:

   The :class:`AiEngine` is the central hub for all AI-powered musical features.
   It orchestrates pattern matching, chord progression suggestions, difficulty assessment,
   and advanced musical analysis using machine learning models and statistical techniques.

   **Architecture:**
      - **Pattern Storage**: Trie-based chord progression database
      - **Suggestion Engine**: Context-aware chord recommendation system  
      - **Difficulty Analyzer**: Statistical complexity assessment
      - **Performance Monitor**: Real-time metrics and optimization

   **Performance Characteristics:**
      - **Initialization**: 100-1000ms depending on training data size
      - **Chord Suggestions**: <50ms (CHORD_SUGGESTION_MAX_MS target)
      - **Difficulty Assessment**: <200ms (MUSIC_ANALYSIS_MAX_MS target)
      - **Memory Usage**: <150MB (MEMORY_USAGE_MAX_MB target)
      - **Concurrent Requests**: Up to 10 simultaneous

   **Examples:**

   Basic setup and usage:

   .. code-block:: python

      import composer

      # Create engine with default configuration
      engine = composer.AiEngine()

      # Prepare training patterns
      training_patterns = [
          ([Chord.triad(1), Chord.triad(6), Chord.triad(4), Chord.triad(5)], 
           "pop-progression-1", "C"),
          ([Chord.seventh(2), Chord.seventh(5), Chord.triad(1)], 
           "jazz-ii-V-I", "C"),
      ]

      # Initialize with training data
      engine.initialize(training_patterns)

      # Use AI features
      suggestions = engine.get_chord_suggestions(progression, context, config)
      assessment = engine.assess_difficulty(progression, tempo=120)

AI Engine Methods
~~~~~~~~~~~~~~~~~

.. automethod:: composer.AiEngine.__init__

   Creates a new AI engine instance with default configuration.

.. automethod:: composer.AiEngine.initialize

   Initializes the AI engine with training data patterns.

   **Parameters:**
      - **training_patterns** (*List[Tuple[List[Chord], str, Optional[str]]]*): 
        Training data as (progression, source_id, key) tuples

   **Returns:**
      - **None**: Raises exception on failure

   **Training Data Format:**
      Each pattern is a tuple containing:
      - **List[Chord]**: The chord progression pattern
      - **str**: Source identifier (e.g., "song-title", "exercise-1")  
      - **Optional[str]**: Optional key/tonic (e.g., "C", "Bb", "F#m")

   **Examples:**

   Loading common progressions:

   .. code-block:: python

      training_patterns = [
          # I-vi-IV-V progression (pop/rock staple)
          ([Chord.triad(1), Chord.triad(6), Chord.triad(4), Chord.triad(5)], 
           "pop-progression", "C"),
          
          # ii-V-I progression (jazz fundamental)
          ([Chord.seventh(2), Chord.seventh(5), Chord.triad(1)], 
           "jazz-ii-V-I", "C"),
          
          # Circle of fifths sequence
          ([Chord.seventh(6), Chord.seventh(2), Chord.seventh(5), Chord.triad(1)], 
           "circle-of-fifths", None),
      ]

      engine.initialize(training_patterns)

.. automethod:: composer.AiEngine.get_chord_suggestions

   Gets AI-powered chord progression suggestions based on musical context.

.. automethod:: composer.AiEngine.get_magic_chord_solutions

   Finds chords that connect given musical contexts using pattern matching.

   **Parameters:**
      - **previous_chords** (*List[Chord]*): Preceding harmonic context
      - **following_chords** (*List[Chord]*): Following harmonic context
      - **scale** (*str*): Scale context for analysis
      - **limit** (*int*): Maximum number of suggestions to return

   **Returns:**
      - **List[ChordSuggestion]**: Ranked chord suggestions with confidence scores

   **Algorithm:**
      The "Magic Chord" algorithm uses advanced pattern matching to find chords that:
      - Connect smoothly with preceding harmony (voice leading)
      - Prepare effectively for following harmony (harmonic function)
      - Maintain stylistic consistency (genre patterns)
      - Optimize harmonic rhythm and tension curves

   **Examples:**

   .. code-block:: python

      # Find connecting chords in a progression
      context = [Chord.triad(1), Chord.triad(6)]      # I - vi
      following = [Chord.triad(5)]                    # V
      
      suggestions = engine.get_magic_chord_solutions(context, following, "major", 5)
      
      for suggestion in suggestions:
          print(f"Chord: {suggestion.chord}")
          print(f"Confidence: {suggestion.confidence:.2f}")
          print(f"Reasoning: {suggestion.reasoning}")

.. automethod:: composer.AiEngine.get_magic_bass_solutions

   Generates bass line harmonization suggestions for a given bass note.

   **Parameters:**
      - **bass_note** (*str*): Target bass note (e.g., "C", "F#", "Bb")
      - **scale** (*str*): Scale context for harmonization
      - **limit** (*int*): Maximum number of suggestions

   **Returns:**
      - **List[BassHarmonization]**: Bass harmonization options

   **Examples:**

   .. code-block:: python

      # Harmonize a bass line
      bass_solutions = engine.get_magic_bass_solutions("F", "major", 3)
      
      for solution in bass_solutions:
          print(f"Bass harmonization: {solution.chord}")
          print(f"Style: {solution.style}")
          print(f"Confidence: {solution.confidence:.2f}")

.. automethod:: composer.AiEngine.get_harmonize_by_sd_solutions

   Harmonizes specific scale degrees with appropriate chord choices.

.. automethod:: composer.AiEngine.assess_difficulty

   Analyzes the performance difficulty of a chord progression.

   **Parameters:**
      - **progression** (*List[Chord]*): Chord progression to analyze
      - **tempo_bpm** (*Optional[float]*): Tempo in beats per minute
      - **time_signature** (*Optional[Tuple[int, int]]*): Time signature (numerator, denominator)

   **Returns:**
      - **DifficultyAssessment**: Comprehensive difficulty analysis

   **Difficulty Factors:**
      - **Harmonic Complexity**: Chord extensions, alterations, voice leading
      - **Rhythmic Complexity**: Tempo, time signature, harmonic rhythm
      - **Technical Demands**: Hand positions, fingerings, stretch requirements
      - **Musical Context**: Genre expectations, performance practice

   **Examples:**

   .. code-block:: python

      # Assess jazz progression difficulty
      progression = [
          Chord.seventh(2),                    # ii7
          Chord(5, 7).with_alteration("b9"),  # V7♭9 (if method available)
          Chord.seventh(1)                    # Imaj7
      ]
      
      assessment = engine.assess_difficulty(progression, tempo_bpm=180, 
                                          time_signature=(4, 4))
      
      print(f"Overall Score: {assessment.overall_score:.1f}/10")
      print(f"Skill Level: {assessment.skill_level}")
      print(f"Confidence: {assessment.confidence:.2f}")

.. automethod:: composer.AiEngine.is_initialized

   Checks if the engine has been initialized with training data.

.. automethod:: composer.AiEngine.get_metrics

   Returns performance metrics and usage statistics.

.. automethod:: composer.AiEngine.clear_caches

   Clears all internal caches to free memory.

.. automethod:: composer.AiEngine.shutdown

   Performs clean shutdown and resource cleanup.

AI Data Structures
------------------

SuggestionContext
~~~~~~~~~~~~~~~~~

.. autoclass:: composer.SuggestionContext
   :members:
   :undoc-members:
   :show-inheritance:

   Context information for AI chord suggestions including musical preferences,
   harmonic constraints, and stylistic guidelines.

   **Properties:**
      - **scale_fingerprint**: Current key/scale context
      - **position_in_progression**: Relative position (0.0-1.0)
      - **target_valence**: Emotional target (-1.0 to 1.0)
      - **complexity_preference**: Harmonic complexity preference (0.0-1.0)
      - **genre_weights**: Genre style preferences with weights
      - **avoid_repetition_within**: Chord repetition avoidance span
      - **recent_chords**: Recent harmonic history for context

SuggestionConfig
~~~~~~~~~~~~~~~~

.. autoclass:: composer.SuggestionConfig
   :members:
   :undoc-members:
   :show-inheritance:

   Configuration parameters for controlling AI suggestion behavior.

   **Properties:**
      - **max_suggestions**: Maximum number of suggestions to return
      - **min_confidence**: Minimum confidence threshold (0.0-1.0)
      - **search_depth**: Pattern search depth in the trie
      - **use_probabilistic**: Enable probabilistic vs deterministic selection
      - **temperature**: Temperature for probabilistic selection (0.0-2.0)
      - **enable_context_weighting**: Advanced context-aware weighting

ChordSuggestion
~~~~~~~~~~~~~~~

.. autoclass:: composer.ChordSuggestion
   :members:
   :undoc-members:
   :show-inheritance:

   A weighted chord suggestion result with confidence metrics and reasoning.

   **Properties:**
      - **chord**: The suggested chord
      - **confidence**: Overall suggestion confidence (0.0-1.0)
      - **frequency_score**: Pattern frequency score from training data
      - **context_score**: Contextual relevance score
      - **theory_score**: Music theory appropriateness score
      - **weighted_score**: Final weighted score combining all factors
      - **pattern_info**: Source pattern information
      - **reasoning**: Human-readable explanation of suggestion

DifficultyAssessment
~~~~~~~~~~~~~~~~~~~~

.. autoclass:: composer.DifficultyAssessment
   :members:
   :undoc-members:
   :show-inheritance:

   Comprehensive analysis of musical performance difficulty.

   **Properties:**
      - **overall_score**: Overall difficulty score (1.0-10.0)
      - **skill_level**: Recommended skill level (Beginner, Intermediate, Advanced, Expert)
      - **confidence**: Assessment confidence (0.0-1.0)
      - **component_scores**: Breakdown by difficulty factors
      - **recommendations**: Specific practice recommendations
      - **technical_demands**: Physical/technical challenge analysis
      - **musical_demands**: Musical/interpretive challenge analysis

BassHarmonization
~~~~~~~~~~~~~~~~~

.. autoclass:: composer.BassHarmonization
   :members:
   :undoc-members:
   :show-inheritance:

   Bass line harmonization result with stylistic information.

   **Properties:**
      - **bass_notes**: Generated bass note sequence
      - **rhythm**: Rhythm pattern in MIDI ticks
      - **confidence**: Harmonization confidence (0.0-1.0)
      - **style**: Bass line style (Root, Alternating, Walking, Arpeggiated, Rhythmic)

Advanced AI Features
--------------------

Pattern Analysis
~~~~~~~~~~~~~~~~

The AI engine includes sophisticated pattern analysis capabilities:

**Trie-Based Storage**: Efficient storage and retrieval of chord progressions using
prefix tree structures optimized for musical sequences.

**Context Weighting**: Advanced algorithms that consider:
   - Harmonic function and voice leading
   - Rhythmic context and metric placement
   - Genre-specific stylistic patterns
   - Historical precedence in training data

**Statistical Models**: Machine learning approaches including:
   - N-gram analysis for chord sequence probability
   - Bayesian inference for context-dependent suggestions
   - Clustering algorithms for style classification
   - Regression models for difficulty prediction

Performance Optimization
~~~~~~~~~~~~~~~~~~~~~~~~

The AI engine is optimized for real-time performance:

**Memory Management**:
   - Configurable memory limits and automatic garbage collection
   - Efficient pattern compression and deduplication
   - LRU caching for frequently accessed patterns

**Parallel Processing**:
   - Multi-threaded pattern search and analysis
   - Vectorized computations for batch operations
   - Lock-free data structures for concurrent access

**Profiling and Metrics**:
   - Built-in performance monitoring and profiling
   - Real-time metrics collection and reporting
   - Automatic performance tuning and optimization

Usage Patterns
--------------

**Interactive Composition**:

.. code-block:: python

   # Real-time composition assistance
   engine = AiEngine()
   engine.initialize(comprehensive_pattern_database)
   
   current_progression = []
   while composing:
       suggestions = engine.get_chord_suggestions(
           current_progression, context, config)
       # Present suggestions to user
       chosen_chord = user_selection(suggestions)
       current_progression.append(chosen_chord)

**Batch Analysis**:

.. code-block:: python

   # Analyze multiple progressions
   progressions = load_chord_progressions()
   
   results = []
   for progression in progressions:
       difficulty = engine.assess_difficulty(progression)
       complexity_scores = [get_chord_complexity(c, "major") for c in progression]
       results.append({
           'progression': progression,
           'difficulty': difficulty,
           'complexity': complexity_scores
       })

**Educational Applications**:

.. code-block:: python

   # Generate exercises by difficulty level
   def generate_exercises(target_difficulty, count=10):
       exercises = []
       while len(exercises) < count:
           # Generate progression
           progression = generate_random_progression()
           assessment = engine.assess_difficulty(progression)
           
           if abs(assessment.overall_score - target_difficulty) < 0.5:
               exercises.append((progression, assessment))
       
       return exercises

See Also
--------

- :doc:`core` - Core chord and scale analysis functions
- :doc:`serialization` - Pattern storage and ML tokenization
- :doc:`../advanced/ai_algorithms` - Detailed algorithm descriptions
- :doc:`../tutorial/ai_features` - Step-by-step AI tutorial