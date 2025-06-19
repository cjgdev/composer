Quick Start Guide
=================

Get up and running with Composer in minutes! This guide covers the essential features and common use cases.

🎵 **Basic Chord Operations**
-----------------------------

.. musical-example::

   import composer

   # Create basic chords
   c_major = composer.Chord.triad(1)      # I - Tonic
   a_minor = composer.Chord.triad(6)      # vi - Submediant  
   f_major = composer.Chord.triad(4)      # IV - Subdominant
   g_major = composer.Chord.triad(5)      # V - Dominant

   print(f"Pop progression: {c_major} - {a_minor} - {f_major} - {g_major}")

   # Create seventh chords
   c_maj7 = composer.Chord.seventh(1)     # Imaj7
   d_min7 = composer.Chord.seventh(2)     # ii7
   g_dom7 = composer.Chord.seventh(5)     # V7

   print(f"Jazz progression: {d_min7} - {g_dom7} - {c_maj7}")

🎼 **Scale Analysis**
--------------------

.. musical-example::

   # Work with different scales
   major_scale = composer.ScaleFingerprint.major()
   minor_scale = composer.ScaleFingerprint.minor()
   dorian_scale = composer.ScaleFingerprint.dorian()

   # Analyze chord in scale context
   chord = composer.Chord.seventh(5)
   degrees = composer.get_stable_scale_degrees(chord, major_scale)
   print(f"V7 contains scale degrees: {degrees}")

   # Get chord complexity
   complexity = composer.get_chord_complexity(chord, "major")
   print(f"V7 complexity score: {complexity}/10")

🎯 **Advanced Chord Construction**
---------------------------------

.. musical-example::

   # Create complex chords with alterations
   altered_dom = composer.Chord(5, 7)
   
   # Add alterations (these would be done via Rust methods if available)
   # altered_dom = altered_dom.with_alteration("b9").with_alteration("#11")
   
   # Create applied chords (V7/V)
   secondary_dom = composer.Chord(2, 7, applied=5)
   
   # Create borrowed chords
   borrowed_chord = composer.Chord(6, 5, borrowed="harmonic_minor")
   
   print(f"Altered dominant: {altered_dom}")
   print(f"Secondary dominant: {secondary_dom}")
   print(f"Borrowed chord: {borrowed_chord}")

🤖 **AI-Powered Features**
--------------------------

.. musical-example::

   # Initialize AI engine
   engine = composer.AiEngine()
   
   # Prepare training data (simplified example)
   training_patterns = [
       # ([chord1, chord2, chord3], "source_id", "key")
       ([composer.Chord.triad(1), composer.Chord.triad(6), 
         composer.Chord.triad(4), composer.Chord.triad(5)], 
        "pop-progression", "C"),
       ([composer.Chord.seventh(2), composer.Chord.seventh(5),
         composer.Chord.triad(1)], "jazz-ii-V-I", "C"),
   ]
   
   # Initialize with patterns
   engine.initialize(training_patterns)
   
   # Get chord suggestions
   context = [composer.Chord.triad(1), composer.Chord.triad(6)]
   following = [composer.Chord.triad(5)]
   suggestions = engine.get_magic_chord_solutions(context, following, "major", 5)
   
   print(f"AI suggests {len(suggestions)} chord options")

📊 **Binary Serialization**
---------------------------

.. musical-example::

   # Serialize chords to compact binary format
   chord = composer.Chord.seventh(5)
   
   # Convert to binary (5 bytes)
   binary_data = composer.serialize_chord_to_binary(chord)
   print(f"Binary size: {len(binary_data)} bytes")
   
   # Convert to hex string
   hex_string = composer.chord_to_hex(chord)
   print(f"Hex representation: {hex_string}")
   
   # Deserialize back
   restored_chord = composer.chord_from_hex(hex_string)
   print(f"Round-trip successful: {chord == restored_chord}")

🎹 **Roman Numeral Analysis**
----------------------------

.. musical-example::

   # Generate Roman numeral representations
   chord = composer.Chord.seventh(5)
   scale = composer.ScaleFingerprint.major()
   
   # Get Roman numeral graphic
   graphic = composer.get_relative_chord_graphic(chord, scale)
   print(f"Roman numeral: {graphic.symbol}")
   print(f"Quality: {graphic.quality}")
   print(f"Figured bass: {graphic.figured_bass}")

🔧 **Configuration and Constants**
---------------------------------

.. musical-example::

   # Access system constants
   constants = composer.get_configuration_constants()
   print(f"Version: {constants['APPLICATION_VERSION']}")
   print(f"Max memory: {constants['MEMORY_USAGE_MAX_MB']}MB")
   print(f"Chord lookup limit: {constants['CHORD_LOOKUP_MAX_MS']}ms")
   
   # Access built-in constants
   print(f"Ticks per beat: {composer.constants.TICKS_PER_BEAT}")
   print(f"Scale degrees: {composer.constants.SCALE_DEGREES}")

⚡ **Performance Tips**
----------------------

1. **Reuse Objects**: Create scales and engines once, reuse many times
   
   .. code-block:: python
   
      # Good: Create once
      major_scale = composer.ScaleFingerprint.major()
      
      # Use many times
      for chord in chord_progression:
          degrees = composer.get_stable_scale_degrees(chord, major_scale)

2. **Batch Operations**: Process multiple chords efficiently

   .. code-block:: python
   
      # Serialize multiple chords
      chords = [composer.Chord.triad(i) for i in range(1, 8)]
      binaries = [composer.serialize_chord_to_binary(c) for c in chords]

3. **AI Engine Initialization**: Initialize once per application

   .. code-block:: python
   
      # Initialize at startup
      engine = composer.AiEngine()
      engine.initialize(large_pattern_database)
      
      # Use throughout application lifecycle
      suggestions = engine.get_chord_suggestions(...)

🎨 **Common Patterns**
---------------------

**Chord Progression Analysis**:

.. code-block:: python

   def analyze_progression(chords, scale):
       """Analyze a chord progression."""
       results = []
       for chord in chords:
           degrees = composer.get_stable_scale_degrees(chord, scale)
           complexity = composer.get_chord_complexity(chord, scale.name)
           results.append({
               'chord': str(chord),
               'degrees': degrees,
               'complexity': complexity
           })
       return results

**Scale Comparison**:

.. code-block:: python

   def compare_scales(chord):
       """Compare how a chord functions in different scales."""
       scales = {
           'major': composer.ScaleFingerprint.major(),
           'minor': composer.ScaleFingerprint.minor(),
           'dorian': composer.ScaleFingerprint.dorian(),
       }
       
       for name, scale in scales.items():
           degrees = composer.get_stable_scale_degrees(chord, scale)
           print(f"{name}: {degrees}")

🚀 **Next Steps**
-----------------

Now that you've mastered the basics:

* :doc:`tutorial/index` - Dive deeper with comprehensive tutorials
* :doc:`examples/index` - Explore real-world examples
* :doc:`api/core` - Browse the complete API reference
* :doc:`advanced/performance` - Learn optimization techniques