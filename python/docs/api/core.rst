Core API Reference
==================

The core module provides fundamental music theory data structures and analysis functions.

Core Classes
------------

.. automodule:: composer
   :members: Chord, ScaleFingerprint, BorrowedScale
   :undoc-members:
   :show-inheritance:

Chord Class
~~~~~~~~~~~

.. autoclass:: composer.Chord
   :members:
   :undoc-members:
   :show-inheritance:

   The :class:`Chord` class is the fundamental data structure for representing musical chords
   with comprehensive support for extensions, alterations, inversions, and harmonic context.

   **Examples:**

   Creating basic chords:

   .. code-block:: python

      # Create a C major triad (I)
      tonic = Chord.triad(1)
      
      # Create a G dominant seventh (V7)
      dominant = Chord.seventh(5)
      
      # Create a complex altered chord
      altered = Chord(5, 7, inversion=1, applied=2)

   **Musical Properties:**

   - **Root**: Scale degree (1-7) where the chord is built
   - **Type**: Extension level (5=triad, 7=seventh, 9=ninth, 11=eleventh, 13=thirteenth)
   - **Inversion**: Bass note position (0=root, 1=first, 2=second, 3=third)
   - **Alterations**: Modified chord tones (♭5, ♯9, ♯11, etc.)
   - **Applied**: Secondary dominant/leading-tone relationships
   - **Borrowed**: Chords from parallel modes or related keys

ScaleFingerprint Class
~~~~~~~~~~~~~~~~~~~~~~

.. autoclass:: composer.ScaleFingerprint
   :members:
   :undoc-members:
   :show-inheritance:

   The :class:`ScaleFingerprint` class provides efficient scale representation using
   a 12-bit binary pattern for rapid analysis and comparison.

   **Examples:**

   Working with scales:

   .. code-block:: python

      # Create common scales
      major = ScaleFingerprint.major()
      minor = ScaleFingerprint.minor()
      dorian = ScaleFingerprint.dorian()
      
      # Custom scale from pattern
      blues = ScaleFingerprint([True, False, False, True, False, True, False, 
                               True, False, False, True, False])

   **Scale Properties:**

   - **Pattern**: 12-element boolean array representing chromatic degrees
   - **Chromatic Notes**: List of active semitone positions (0-11)
   - **Scale Degrees**: Diatonic degree mapping for the scale
   - **Mode Analysis**: Automatic detection of common modes

BorrowedScale Class
~~~~~~~~~~~~~~~~~~~

.. autoclass:: composer.BorrowedScale
   :members:
   :undoc-members:
   :show-inheritance:

   The :class:`BorrowedScale` class represents borrowed harmony relationships,
   enabling analysis of chords from parallel modes or related key areas.

   **Examples:**

   Borrowed chord analysis:

   .. code-block:: python

      # Chord borrowed from parallel minor
      borrowed = BorrowedScale("harmonic_minor")
      chord = Chord(6, 5, borrowed="harmonic_minor")  # ♭VI in major
      
      # Modal interchange
      dorian_borrowed = BorrowedScale("dorian")

Music Theory Functions
----------------------

Core Analysis
~~~~~~~~~~~~~

.. autofunction:: composer.get_stable_scale_degrees

   Analyzes which scale degrees a chord occupies within a specific key context.
   This is the primary function for harmonic analysis.

   **Parameters:**
      - **chord** (*Chord*): The chord to analyze
      - **scale** (*ScaleFingerprint*): The key/scale context

   **Returns:**
      - **List[str]**: Scale degrees with accidentals (e.g., ["1", "3", "♭5"])

   **Examples:**

   .. code-block:: python

      major_scale = ScaleFingerprint.major()
      
      # Analyze tonic triad
      tonic = Chord.triad(1)
      degrees = get_stable_scale_degrees(tonic, major_scale)
      # Returns: ["1", "3", "5"]
      
      # Analyze altered dominant
      altered_v = Chord(5, 7)  # With alterations added
      degrees = get_stable_scale_degrees(altered_v, major_scale)
      # Returns: ["5", "7", "2", "4", "♭6"]  # Including ♭9

.. autofunction:: composer.get_chord_complexity

   Calculates the harmonic complexity of a chord on a scale from 1.0 (simple) to 10.0 (very complex).

   **Parameters:**
      - **chord** (*Chord*): The chord to analyze
      - **scale_name** (*str*): Scale context name ("major", "minor", etc.)

   **Returns:**
      - **float**: Complexity score (1.0-10.0)

   **Complexity Factors:**
      - Number of chord tones (triads=1.0, thirteenths=6.0+)
      - Presence of alterations (+1.0 to +3.0 per alteration)
      - Inversion level (+0.5 per inversion)
      - Applied relationships (+1.0 to +2.0)
      - Borrowed harmony (+1.0 to +2.5)

   **Examples:**

   .. code-block:: python

      # Simple triad
      simple = Chord.triad(1)
      complexity = get_chord_complexity(simple, "major")
      # Returns: 1.0
      
      # Complex altered chord
      complex_chord = Chord(5, 13)  # V13 with alterations
      complexity = get_chord_complexity(complex_chord, "major")
      # Returns: 7.5+

.. autofunction:: composer.is_isotonal

   Determines if two chords are isotonal (harmonically equivalent) within a scale context.

   **Parameters:**
      - **chord1** (*Chord*): First chord
      - **chord2** (*Chord*): Second chord  
      - **scale** (*ScaleFingerprint*): Scale context for comparison

   **Returns:**
      - **bool**: True if chords contain the same scale degrees

   **Examples:**

   .. code-block:: python

      scale = ScaleFingerprint.major()
      
      # Different voicings of same harmony
      root_pos = Chord.triad(1)
      first_inv = Chord(1, 5, inversion=1)
      
      are_isotonal = is_isotonal(root_pos, first_inv, scale)
      # Returns: True (same scale degrees: 1, 3, 5)

Roman Numeral Analysis
~~~~~~~~~~~~~~~~~~~~~~

.. autofunction:: composer.get_relative_chord_graphic

   Generates Roman numeral notation for a chord within a scale context.

   **Parameters:**
      - **chord** (*Chord*): The chord to analyze
      - **scale** (*ScaleFingerprint*): The scale context

   **Returns:**
      - **ChordGraphic**: Roman numeral representation with quality and figured bass

   **Examples:**

   .. code-block:: python

      scale = ScaleFingerprint.major()
      
      # Tonic major triad
      tonic = Chord.triad(1)
      graphic = get_relative_chord_graphic(tonic, scale)
      print(graphic.symbol)        # "I"
      print(graphic.quality)       # "major"
      
      # Subdominant with seventh
      subdominant = Chord.seventh(4)
      graphic = get_relative_chord_graphic(subdominant, scale)
      print(graphic.symbol)        # "IV"
      print(graphic.figured_bass)  # "7"

ChordGraphic Class
~~~~~~~~~~~~~~~~~~

.. autoclass:: composer.PyChordGraphic
   :members:
   :undoc-members:
   :show-inheritance:

   Represents the Roman numeral analysis of a chord with complete harmonic annotation.

   **Properties:**
      - **symbol**: Base Roman numeral (I, ii, V7, etc.)
      - **quality**: Chord quality (major, minor, diminished, augmented)
      - **figured_bass**: Figured bass notation (6, 7, 6/4, etc.)
      - **applied**: Applied chord notation (V/V, vii°/vi, etc.)
      - **borrowed**: Borrowed chord indicators (♭VI, ♭VII, etc.)
      - **alterations**: Specific alterations (♭5, ♯9, ♯11, etc.)
      - **suspensions**: Suspension indicators (sus2, sus4, etc.)

Utility Functions
~~~~~~~~~~~~~~~~~

.. autofunction:: composer.analyze_harmonic_function

   Analyzes the harmonic function of a chord within tonal context.

.. autofunction:: composer.is_valid_tri_sub

   Validates tritone substitution relationships between chords.

.. autofunction:: composer.chord_letter_to_upper_case

   Converts chord letter notation to uppercase Roman numerals.

.. autofunction:: composer.chord_letter_to_lower_case

   Converts chord letter notation to lowercase Roman numerals.

Performance Notes
-----------------

The core functions are optimized for real-time performance:

- **Chord Analysis**: <1ms per chord (CHORD_LOOKUP_MAX_MS target)
- **Scale Operations**: <0.1ms for most operations  
- **Memory Usage**: <5MB for typical analysis sessions
- **Thread Safety**: All functions are thread-safe for concurrent use

The underlying Rust implementation provides:

- Zero-copy data structures where possible
- Optimized algorithms with O(1) or O(log n) complexity
- SIMD acceleration on supported platforms
- Memory pool allocation for reduced GC pressure

See Also
--------

- :doc:`ai` - AI-powered chord progression features
- :doc:`serialization` - Binary format and ML tokenization
- :doc:`../advanced/chord_theory` - Advanced harmonic theory concepts
- :doc:`../tutorial/chord_analysis` - Step-by-step analysis tutorial