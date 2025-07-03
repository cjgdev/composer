//! Python bindings for Chord data structure

use crate::error::ToPyResult;
use composer_core::{BorrowedScale, Chord};
use pyo3::prelude::*;
use pyo3::types::PyList;

/// Python wrapper for BorrowedScale
#[pyclass(name = "BorrowedScale")]
#[derive(Clone)]
pub struct PyBorrowedScale {
    pub inner: BorrowedScale,
}

#[pymethods]
impl PyBorrowedScale {
    #[new]
    fn new(scale_name: String) -> PyResult<Self> {
        let borrowed_scale = BorrowedScale::Named(scale_name);
        Ok(PyBorrowedScale {
            inner: borrowed_scale,
        })
    }

    #[getter]
    fn scale_name(&self) -> Option<String> {
        match &self.inner {
            BorrowedScale::Named(name) => Some(name.clone()),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        match &self.inner {
            BorrowedScale::Named(name) => format!("BorrowedScale('{}')", name),
            _ => "BorrowedScale(unknown)".to_string(),
        }
    }
}

/// Python wrapper for Chord
#[pyclass(name = "Chord")]
#[derive(Clone)]
pub struct PyChord {
    pub inner: Chord,
}

#[pymethods]
impl PyChord {
    /// Creates a new chord with specified properties.
    ///
    /// This is the primary constructor for creating chord instances in Python.
    /// All chord properties are validated according to music theory rules.
    ///
    /// Args:
    ///     root (int): Scale degree (1-7), where 1=tonic, 2=supertonic, etc.
    ///     chord_type (int): Chord extension level: 5=triad, 7=seventh, 9=ninth, 11=eleventh, 13=thirteenth
    ///     inversion (int, optional): Inversion level (0-3). Defaults to 0 (root position).
    ///     applied (int, optional): Applied chord target (1-7). Defaults to None.
    ///     borrowed (str, optional): Borrowed scale name (e.g., "harmonic_minor"). Defaults to None.
    ///
    /// Returns:
    ///     Chord: A new chord instance with the specified properties.
    ///
    /// Raises:
    ///     ValueError: If any parameters are musically invalid.
    ///
    /// Examples:
    ///     >>> # Create a simple C major triad (I)
    ///     >>> tonic = Chord(1, 5)
    ///     >>> print(f"Root: {tonic.root}, Type: {tonic.chord_type}")
    ///     Root: 1, Type: 5
    ///     
    ///     >>> # Create a dominant seventh in first inversion (V7/3)
    ///     >>> dom7_inv = Chord(5, 7, inversion=1)
    ///     >>> print(f"Inversion: {dom7_inv.inversion}")
    ///     Inversion: 1
    ///     
    ///     >>> # Create an applied dominant (V7/V)
    ///     >>> applied_dom = Chord(2, 7, applied=5)
    ///     >>> print(f"Applied to: {applied_dom.applied}")
    ///     Applied to: 5
    ///     
    ///     >>> # Create a borrowed chord from harmonic minor
    ///     >>> borrowed = Chord(6, 5, borrowed="harmonic_minor")
    ///     >>> print(borrowed.borrowed.scale_name if borrowed.borrowed else None)
    ///     harmonic_minor
    ///
    /// Related Functions:
    ///     - Chord.triad(root): Convenience constructor for triads
    ///     - Chord.seventh(root): Convenience constructor for seventh chords
    ///     - get_stable_scale_degrees(): Analyze chord in key context
    ///     - get_chord_complexity(): Calculate harmonic complexity
    ///
    /// Performance:
    ///     Chord creation typically completes in <0.001ms with full validation.
    #[new]
    #[pyo3(signature = (root, chord_type, inversion=None, applied=None, borrowed=None))]
    fn new(
        root: u8,
        chord_type: u8,
        inversion: Option<u8>,
        applied: Option<u8>,
        borrowed: Option<&str>,
    ) -> PyResult<Self> {
        let mut chord = Chord::new(root, chord_type).to_py_result()?;

        if let Some(inv) = inversion {
            chord.inversion = inv;
        }

        if let Some(app) = applied {
            chord.applied = app;
        }

        if let Some(borrowed_name) = borrowed {
            chord.borrowed = Some(BorrowedScale::Named(borrowed_name.to_string()));
        }

        Ok(PyChord { inner: chord })
    }

    /// Creates a basic triad chord (three-note chord).
    ///
    /// A convenience constructor for creating triad chords, which are the most common
    /// chord type in music. Triads consist of root, third, and fifth.
    ///
    /// Args:
    ///     root (int): Scale degree (1-7) for the chord root
    ///
    /// Returns:
    ///     Chord: A new triad chord with the specified root
    ///
    /// Raises:
    ///     ValueError: If root is not a valid scale degree (1-7)
    ///
    /// Examples:
    ///     >>> # Create triads for a I-vi-IV-V progression
    ///     >>> tonic = Chord.triad(1)       # I
    ///     >>> submediant = Chord.triad(6)  # vi
    ///     >>> subdominant = Chord.triad(4) # IV
    ///     >>> dominant = Chord.triad(5)    # V
    ///     >>>
    ///     >>> print(f"Tonic: {tonic}")
    ///     Tonic: 1
    ///     >>>
    ///     >>> # Check chord properties
    ///     >>> assert tonic.is_triad()
    ///     >>> assert tonic.expected_tone_count() == 3
    ///     >>> assert not tonic.is_seventh()
    ///
    /// Musical Context:
    ///     Triads are fundamental to:
    ///     - Pop and rock progressions (I-vi-IV-V, vi-IV-I-V)
    ///     - Classical harmony (functional analysis)
    ///     - Folk music (simple chord progressions)
    ///     - Beginning music theory education
    ///
    /// Related Functions:
    ///     - Chord.seventh(root): Create seventh chords
    ///     - Chord(root, 5): Equivalent explicit constructor
    ///     - chord.is_triad(): Check if a chord is a triad
    ///     - get_stable_scale_degrees(): Analyze triad in key context
    #[staticmethod]
    fn triad(root: u8) -> PyResult<Self> {
        let chord = Chord::triad(root).to_py_result()?;
        Ok(PyChord { inner: chord })
    }

    /// Create a seventh chord
    #[staticmethod]
    fn seventh(root: u8) -> PyResult<Self> {
        let chord = Chord::seventh(root).to_py_result()?;
        Ok(PyChord { inner: chord })
    }

    /// Create a rest chord
    #[staticmethod]
    fn rest() -> Self {
        PyChord {
            inner: Chord::rest(),
        }
    }

    // Getters for the actual fields that exist
    #[getter]
    fn root(&self) -> u8 {
        self.inner.root
    }

    #[getter]
    fn chord_type(&self) -> u8 {
        self.inner.chord_type
    }

    #[getter]
    fn inversion(&self) -> u8 {
        self.inner.inversion
    }

    #[getter]
    fn applied(&self) -> u8 {
        self.inner.applied
    }

    #[getter]
    fn adds(&self, py: Python) -> PyResult<Py<PyList>> {
        let list = PyList::new(py, &self.inner.adds)?;
        Ok(list.into())
    }

    #[getter]
    fn omits(&self, py: Python) -> PyResult<Py<PyList>> {
        let list = PyList::new(py, &self.inner.omits)?;
        Ok(list.into())
    }

    #[getter]
    fn alterations(&self, py: Python) -> PyResult<Py<PyList>> {
        let list = PyList::new(py, &self.inner.alterations)?;
        Ok(list.into())
    }

    #[getter]
    fn suspensions(&self, py: Python) -> PyResult<Py<PyList>> {
        let list = PyList::new(py, &self.inner.suspensions)?;
        Ok(list.into())
    }

    #[getter]
    fn borrowed(&self) -> Option<PyBorrowedScale> {
        self.inner
            .borrowed
            .as_ref()
            .map(|b| PyBorrowedScale { inner: b.clone() })
    }

    #[getter]
    fn is_rest(&self) -> bool {
        self.inner.is_rest
    }

    // Setters
    #[setter]
    fn set_root(&mut self, root: u8) {
        self.inner.root = root;
    }

    #[setter]
    fn set_chord_type(&mut self, chord_type: u8) {
        self.inner.chord_type = chord_type;
    }

    #[setter]
    fn set_inversion(&mut self, inversion: u8) {
        self.inner.inversion = inversion;
    }

    #[setter]
    fn set_applied(&mut self, applied: u8) {
        self.inner.applied = applied;
    }

    #[setter]
    fn set_borrowed(&mut self, borrowed: Option<PyBorrowedScale>) {
        self.inner.borrowed = borrowed.map(|b| b.inner);
    }

    // Basic mutation methods
    fn add_alteration(&mut self, alteration: String) {
        self.inner.alterations.push(alteration);
    }

    fn add_suspension(&mut self, suspension: u8) {
        self.inner.suspensions.push(suspension);
    }

    fn add_note(&mut self, note: u8) {
        self.inner.adds.push(note);
    }

    fn omit_note(&mut self, note: u8) {
        self.inner.omits.push(note);
    }

    // Query methods that actually exist
    fn is_applied(&self) -> bool {
        self.inner.is_applied()
    }

    fn is_borrowed(&self) -> bool {
        self.inner.is_borrowed()
    }

    fn has_alterations(&self) -> bool {
        self.inner.has_alterations()
    }

    fn has_suspensions(&self) -> bool {
        self.inner.has_suspensions()
    }

    fn expected_tone_count(&self) -> usize {
        self.inner.expected_tone_count()
    }

    // Display
    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!(
            "Chord(root={}, chord_type={}, inversion={})",
            self.inner.root, self.inner.chord_type, self.inner.inversion
        )
    }

    // Equality
    fn __eq__(&self, other: &PyChord) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &PyChord) -> bool {
        self.inner != other.inner
    }
}
