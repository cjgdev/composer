//! Python bindings for chord serialization, tokenization, and data processing

use crate::error::ToPyResult;
use crate::{PyChord, PyScaleFingerprint};
use composer_serialization::{
    augment_with_repeated, deserialize_chord, deserialize_trie, detokenize_cluster,
    detokenize_midi_like, fast_hash, fold_hash, parse_duration_token, reconstruct_timeline,
    reduce_chord_vocab, scale40_decode, scale40_encode, serialize_chord, serialize_trie,
    tokenize_chord_as_raw, tokenize_duration, validate_binary_format, validate_chord_cluster_token,
    validate_duration_token, validate_octave_token, validate_raw_note_token, validate_token,
    ChordBinary, Note, Timeline, TokenLibrary, TrieNode, CHROMATIC_RANGE, OCTAVE_RANGE_MAX,
    OCTAVE_RANGE_MIN, TICKS_PER_BEAT,
};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};

// ============================================================================
// Basic Chord Serialization
// ============================================================================

/// Serialize a chord to 5-byte binary format
#[pyfunction]
pub fn serialize_chord_to_binary(chord: &PyChord, py: Python) -> PyResult<Py<PyBytes>> {
    let binary = serialize_chord(&chord.inner).to_py_result()?;
    let bytes = PyBytes::new(py, &binary);
    Ok(bytes.into())
}

/// Deserialize a chord from 5-byte binary format
#[pyfunction]
pub fn deserialize_chord_from_binary(data: &[u8]) -> PyResult<PyChord> {
    if data.len() != 5 {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected 5 bytes, got {}",
            data.len()
        )));
    }

    let mut binary: ChordBinary = [0; 5];
    binary.copy_from_slice(data);

    let chord = deserialize_chord(&binary).to_py_result()?;
    Ok(PyChord { inner: chord })
}

/// Convert chord to hexadecimal string representation
#[pyfunction]
pub fn chord_to_hex(chord: &PyChord) -> PyResult<String> {
    let binary = serialize_chord(&chord.inner).to_py_result()?;
    Ok(hex::encode(binary))
}

/// Create chord from hexadecimal string representation
#[pyfunction]
pub fn chord_from_hex(hex_string: &str) -> PyResult<PyChord> {
    let binary_data = hex::decode(hex_string).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Invalid hex string: {}", e))
    })?;

    if binary_data.len() != 5 {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected 5 bytes from hex, got {}",
            binary_data.len()
        )));
    }

    let mut binary: ChordBinary = [0; 5];
    binary.copy_from_slice(&binary_data);

    let chord = deserialize_chord(&binary).to_py_result()?;
    Ok(PyChord { inner: chord })
}

// ============================================================================
// Tokenization Classes and Functions
// ============================================================================

/// Python wrapper for Note
#[pyclass(name = "Note")]
#[derive(Clone)]
pub struct PyNote {
    pub inner: Note,
}

#[pymethods]
impl PyNote {
    #[new]
    #[pyo3(signature = (scale_degree, octave, is_rest=false))]
    fn new(scale_degree: u8, octave: u8, is_rest: bool) -> Self {
        PyNote {
            inner: Note {
                scale_degree,
                octave,
                is_rest,
            },
        }
    }

    #[getter]
    fn scale_degree(&self) -> u8 {
        self.inner.scale_degree
    }

    #[getter]
    fn octave(&self) -> u8 {
        self.inner.octave
    }

    #[getter]
    fn is_rest(&self) -> bool {
        self.inner.is_rest
    }

    fn __repr__(&self) -> String {
        if self.inner.is_rest {
            "Note(REST)".to_string()
        } else {
            format!(
                "Note(scale_degree={}, octave={})",
                self.inner.scale_degree, self.inner.octave
            )
        }
    }
}

/// Python wrapper for TokenLibrary
#[pyclass(name = "TokenLibrary")]
pub struct PyTokenLibrary {
    inner: TokenLibrary,
}

#[pymethods]
impl PyTokenLibrary {
    #[new]
    fn new() -> Self {
        PyTokenLibrary {
            inner: TokenLibrary::new(),
        }
    }

    fn add_chord_token(&mut self, token: String, chord_binary: &[u8]) -> PyResult<()> {
        if chord_binary.len() != 5 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Chord binary must be exactly 5 bytes",
            ));
        }
        self.inner.add_chord_token(token, chord_binary.to_vec());
        Ok(())
    }

    fn resolve_chord_token(&self, token: &str, py: Python) -> PyResult<Py<PyBytes>> {
        let binary = self.inner.resolve_chord_token(token).to_py_result()?;
        let bytes = PyBytes::new(py, &binary);
        Ok(bytes.into())
    }

    fn get_library_size(&self) -> usize {
        self.inner.get_library_size()
    }

    fn __len__(&self) -> usize {
        self.inner.get_library_size()
    }

    fn __repr__(&self) -> String {
        format!("TokenLibrary(size={})", self.inner.get_library_size())
    }
}

/// Python wrapper for Timeline
#[pyclass(name = "Timeline")]
#[derive(Clone)]
pub struct PyTimeline {
    inner: Timeline,
}

#[pymethods]
impl PyTimeline {
    #[getter]
    fn total_duration(&self) -> f64 {
        self.inner.total_duration
    }

    #[getter]
    fn event_count(&self) -> usize {
        self.inner.events.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "Timeline(duration={:.2}, events={})",
            self.inner.total_duration,
            self.inner.events.len()
        )
    }
}

/// Python wrapper for TrieNode
#[pyclass(name = "TrieNode")]
pub struct PyTrieNode {
    inner: TrieNode,
}

#[pymethods]
impl PyTrieNode {
    #[new]
    fn new() -> Self {
        PyTrieNode {
            inner: TrieNode::new(),
        }
    }

    fn add_pattern(&mut self, pattern: Vec<&[u8]>, id: u32) -> PyResult<()> {
        let pattern_vec: Vec<Vec<u8>> = pattern.into_iter().map(|p| p.to_vec()).collect();
        self.inner.add_pattern(&pattern_vec, id);
        Ok(())
    }

    fn search_patterns(&self, pattern: Vec<&[u8]>) -> Vec<u32> {
        let pattern_vec: Vec<Vec<u8>> = pattern.into_iter().map(|p| p.to_vec()).collect();
        self.inner.search_patterns(&pattern_vec)
    }

    #[getter]
    fn node_count(&self) -> u32 {
        self.inner.node_count
    }

    #[getter]
    fn id_list(&self) -> Vec<u32> {
        self.inner.id_list.clone()
    }

    #[getter]
    fn children_count(&self) -> usize {
        self.inner.children.len()
    }

    fn calculate_rank(&self, total_nodes: u32) -> u32 {
        self.inner.calculate_rank(total_nodes)
    }

    fn __repr__(&self) -> String {
        format!(
            "TrieNode(count={}, children={}, ids={})",
            self.inner.node_count,
            self.inner.children.len(),
            self.inner.id_list.len()
        )
    }
}

// ============================================================================
// Tokenization Functions
// ============================================================================

/// Tokenize duration to hex string
#[pyfunction]
pub fn py_tokenize_duration(duration: f64) -> String {
    tokenize_duration(duration)
}

/// Parse duration from token string
#[pyfunction]
pub fn py_parse_duration_token(token: &str) -> PyResult<f64> {
    parse_duration_token(token).to_py_result()
}

/// Tokenize chord as raw chromatic cluster
#[pyfunction]
pub fn py_tokenize_chord_as_raw(chord: &PyChord, scale: &PyScaleFingerprint) -> PyResult<String> {
    tokenize_chord_as_raw(&chord.inner, &scale.inner).to_py_result()
}

/// Detokenize cluster-based token string
#[pyfunction]
pub fn py_detokenize_cluster(
    token_string: &str,
    scale: &PyScaleFingerprint,
    py: Python,
) -> PyResult<Py<PyDict>> {
    let (chords, notes, duration) =
        detokenize_cluster(token_string, &scale.inner).to_py_result()?;

    let result = PyDict::new(py);

    let py_chords: Vec<PyObject> = chords
        .into_iter()
        .map(|c| PyChord { inner: c }.into_py(py))
        .collect();
    result.set_item("chords", PyList::new(py, py_chords))?;

    let py_notes: Vec<PyObject> = notes
        .into_iter()
        .map(|n| PyNote { inner: n }.into_py(py))
        .collect();
    result.set_item("notes", PyList::new(py, py_notes))?;

    result.set_item("duration", duration)?;

    Ok(result.into())
}

/// Detokenize MIDI-like token sequence
#[pyfunction]
pub fn py_detokenize_midi_like(
    tokens: Vec<String>,
    scale: &PyScaleFingerprint,
) -> PyResult<PyTimeline> {
    let timeline = detokenize_midi_like(&tokens, &scale.inner).to_py_result()?;
    Ok(PyTimeline { inner: timeline })
}

// ============================================================================
// Hash and Compression Functions
// ============================================================================

/// Fast hash function for data integrity
#[pyfunction]
pub fn py_fast_hash(data: &str) -> u32 {
    fast_hash(data)
}

/// Fold hash function for combining hashes
#[pyfunction]
pub fn py_fold_hash(existing_hash: u32, data: &str) -> u32 {
    fold_hash(existing_hash, data)
}

/// Scale fingerprint encoding
#[pyfunction]
pub fn py_scale40_encode(fingerprint: Vec<bool>) -> PyResult<String> {
    if fingerprint.len() != 12 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Fingerprint must be exactly 12 boolean values",
        ));
    }

    let mut array = [false; 12];
    array.copy_from_slice(&fingerprint);

    scale40_encode(&array).to_py_result()
}

/// Scale fingerprint decoding
#[pyfunction]
pub fn py_scale40_decode(encoded: &str) -> PyResult<Vec<bool>> {
    let array = scale40_decode(encoded).to_py_result()?;
    Ok(array.to_vec())
}

// ============================================================================
// Trie Serialization Functions
// ============================================================================

/// Serialize trie to binary format
#[pyfunction]
pub fn py_serialize_trie(trie: &PyTrieNode, py: Python) -> PyResult<Py<PyBytes>> {
    let binary = serialize_trie(&trie.inner).to_py_result()?;
    let bytes = PyBytes::new(py, &binary);
    Ok(bytes.into())
}

/// Deserialize trie from binary format
#[pyfunction]
pub fn py_deserialize_trie(data: &[u8], include_key_tonic: bool) -> PyResult<PyTrieNode> {
    let trie = deserialize_trie(data, include_key_tonic).to_py_result()?;
    Ok(PyTrieNode { inner: trie })
}

/// Validate binary format
#[pyfunction]
pub fn py_validate_binary_format(data: &[u8]) -> bool {
    validate_binary_format(data)
}

/// Reduce chord vocabulary for ML optimization
#[pyfunction]
pub fn py_reduce_chord_vocab(
    chords: Vec<&[u8]>,
    max_vocab: usize,
    py: Python,
) -> PyResult<Py<PyList>> {
    let chord_vecs: Vec<Vec<u8>> = chords.into_iter().map(|c| c.to_vec()).collect();
    let reduced = reduce_chord_vocab(&chord_vecs, max_vocab).to_py_result()?;

    let py_chords: Vec<PyObject> = reduced
        .into_iter()
        .map(|c| PyBytes::new(py, &c).into_py(py))
        .collect();

    let list = PyList::new(py, py_chords);
    Ok(list.into())
}

/// Augment sequence with repetition
#[pyfunction]
pub fn py_augment_with_repeated(sequence: Vec<String>, min_tokens: usize) -> Vec<String> {
    augment_with_repeated(sequence, min_tokens)
}

// ============================================================================
// Token Validation Functions
// ============================================================================

/// Validate any token format
#[pyfunction]
pub fn py_validate_token(token: &str) -> bool {
    validate_token(token)
}

/// Validate duration token format
#[pyfunction]
pub fn py_validate_duration_token(token: &str) -> bool {
    validate_duration_token(token)
}

/// Validate raw note token format
#[pyfunction]
pub fn py_validate_raw_note_token(token: &str) -> bool {
    validate_raw_note_token(token)
}

/// Validate octave token format
#[pyfunction]
pub fn py_validate_octave_token(token: &str) -> bool {
    validate_octave_token(token)
}

/// Validate chord cluster token format
#[pyfunction]
pub fn py_validate_chord_cluster_token(token: &str) -> bool {
    validate_chord_cluster_token(token)
}

// ============================================================================
// Constants
// ============================================================================

/// Get serialization constants
#[pyfunction]
pub fn get_serialization_constants(py: Python) -> PyResult<Py<PyDict>> {
    let constants = PyDict::new(py);

    constants.set_item("TICKS_PER_BEAT", TICKS_PER_BEAT)?;
    constants.set_item("OCTAVE_RANGE_MIN", OCTAVE_RANGE_MIN)?;
    constants.set_item("OCTAVE_RANGE_MAX", OCTAVE_RANGE_MAX)?;
    constants.set_item("CHROMATIC_RANGE", CHROMATIC_RANGE)?;

    Ok(constants.into())
}
