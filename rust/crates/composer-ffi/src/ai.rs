//! Python bindings for AI-powered features

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;
use std::sync::Arc;

use crate::error::ToPyResult;
use crate::{PyChord, PyScaleFingerprint};
use composer_ai::{
    AiEngine, AiEngineConfig, BassHarmonization, BassHarmonizationOptions, BassStyle,
    ChordProgressionSuggester, ChordProgressionTrie, ChordSuggestion, DifficultyAssessment,
    MusicalAnalyzer, ProgressionAnalysis, SkillLevel, SuggestionConfig, SuggestionContext,
};

/// Python wrapper for SuggestionContext
#[pyclass(name = "SuggestionContext")]
#[derive(Clone)]
pub struct PySuggestionContext {
    pub inner: SuggestionContext,
}

#[pymethods]
impl PySuggestionContext {
    #[new]
    #[pyo3(signature = (
        scale_fingerprint=None,
        position_in_progression=0.5,
        target_valence=0.0,
        complexity_preference=0.5,
        genre_weights=None,
        avoid_repetition_within=4
    ))]
    fn new(
        scale_fingerprint: Option<PyScaleFingerprint>,
        position_in_progression: f64,
        target_valence: f64,
        complexity_preference: f64,
        genre_weights: Option<HashMap<String, f64>>,
        avoid_repetition_within: usize,
    ) -> Self {
        let mut context = SuggestionContext::default();

        context.scale_fingerprint = scale_fingerprint.map(|s| s.inner);
        context.position_in_progression = position_in_progression;
        context.target_valence = target_valence;
        context.complexity_preference = complexity_preference;
        context.avoid_repetition_within = avoid_repetition_within;

        if let Some(weights) = genre_weights {
            context.genre_weights = weights;
        }

        PySuggestionContext { inner: context }
    }

    fn add_recent_chord(&mut self, chord: &PyChord) {
        self.inner.recent_chords.push(chord.inner.clone());
    }

    fn set_genre_weight(&mut self, genre: String, weight: f64) {
        self.inner.genre_weights.insert(genre, weight);
    }

    #[getter]
    fn position_in_progression(&self) -> f64 {
        self.inner.position_in_progression
    }

    #[getter]
    fn target_valence(&self) -> f64 {
        self.inner.target_valence
    }

    #[getter]
    fn complexity_preference(&self) -> f64 {
        self.inner.complexity_preference
    }
}

/// Python wrapper for SuggestionConfig
#[pyclass(name = "SuggestionConfig")]
#[derive(Clone)]
pub struct PySuggestionConfig {
    pub inner: SuggestionConfig,
}

#[pymethods]
impl PySuggestionConfig {
    #[new]
    #[pyo3(signature = (
        max_suggestions=None,
        min_confidence=None,
        search_depth=None,
        use_probabilistic=false,
        temperature=1.0,
        enable_context_weighting=true
    ))]
    fn new(
        max_suggestions: Option<usize>,
        min_confidence: Option<f64>,
        search_depth: Option<usize>,
        use_probabilistic: bool,
        temperature: f64,
        enable_context_weighting: bool,
    ) -> Self {
        let mut config = SuggestionConfig::default();

        if let Some(max) = max_suggestions {
            config.max_suggestions = max;
        }
        if let Some(min) = min_confidence {
            config.min_confidence = min;
        }
        if let Some(depth) = search_depth {
            config.search_depth = depth;
        }

        config.use_probabilistic = use_probabilistic;
        config.temperature = temperature;
        config.enable_context_weighting = enable_context_weighting;

        PySuggestionConfig { inner: config }
    }

    #[getter]
    fn max_suggestions(&self) -> usize {
        self.inner.max_suggestions
    }

    #[getter]
    fn min_confidence(&self) -> f64 {
        self.inner.min_confidence
    }

    #[getter]
    fn temperature(&self) -> f64 {
        self.inner.temperature
    }
}

/// Python wrapper for ChordSuggestion
#[pyclass(name = "ChordSuggestion")]
#[derive(Clone)]
pub struct PyChordSuggestion {
    pub inner: ChordSuggestion,
}

#[pymethods]
impl PyChordSuggestion {
    #[getter]
    fn chord(&self) -> PyChord {
        PyChord {
            inner: self.inner.chord.clone(),
        }
    }

    #[getter]
    fn confidence(&self) -> f64 {
        self.inner.confidence
    }

    #[getter]
    fn frequency_score(&self) -> f64 {
        self.inner.frequency_score
    }

    #[getter]
    fn context_score(&self) -> f64 {
        self.inner.context_score
    }

    #[getter]
    fn theory_score(&self) -> f64 {
        self.inner.theory_score
    }

    #[getter]
    fn weighted_score(&self) -> f64 {
        self.inner.weighted_score
    }

    #[getter]
    fn reasoning(&self) -> &str {
        &self.inner.reasoning
    }

    fn __repr__(&self) -> String {
        format!(
            "ChordSuggestion(chord={}, confidence={:.3}, score={:.3})",
            self.inner.chord, self.inner.confidence, self.inner.weighted_score
        )
    }
}

/// Python wrapper for DifficultyAssessment
#[pyclass(name = "DifficultyAssessment")]
#[derive(Clone)]
pub struct PyDifficultyAssessment {
    pub inner: DifficultyAssessment,
}

#[pymethods]
impl PyDifficultyAssessment {
    #[getter]
    fn overall_score(&self) -> f64 {
        self.inner.overall_score
    }

    #[getter]
    fn harmonic_complexity(&self) -> f64 {
        self.inner.harmonic_complexity
    }

    #[getter]
    fn rhythmic_complexity(&self) -> f64 {
        self.inner.rhythmic_complexity
    }

    #[getter]
    fn technical_complexity(&self) -> f64 {
        self.inner.technical_complexity
    }

    #[getter]
    fn melodic_complexity(&self) -> f64 {
        self.inner.melodic_complexity
    }

    #[getter]
    fn confidence(&self) -> f64 {
        self.inner.confidence
    }

    #[getter]
    fn skill_level(&self) -> String {
        match self.inner.skill_level {
            SkillLevel::Beginner => "Beginner".to_string(),
            SkillLevel::Intermediate => "Intermediate".to_string(),
            SkillLevel::Advanced => "Advanced".to_string(),
            SkillLevel::Expert => "Expert".to_string(),
        }
    }

    #[getter]
    fn unique_chords(&self) -> usize {
        self.inner.factors.unique_chords
    }

    #[getter]
    fn extended_harmonies(&self) -> usize {
        self.inner.factors.extended_harmonies
    }

    fn __repr__(&self) -> String {
        format!(
            "DifficultyAssessment(score={:.1}, skill_level='{}', confidence={:.3})",
            self.inner.overall_score,
            self.skill_level(),
            self.inner.confidence
        )
    }
}

/// Python wrapper for ProgressionAnalysis
#[pyclass(name = "ProgressionAnalysis")]
#[derive(Clone)]
pub struct PyProgressionAnalysis {
    pub inner: ProgressionAnalysis,
}

#[pymethods]
impl PyProgressionAnalysis {
    #[getter]
    fn voice_leading_quality(&self) -> f64 {
        self.inner.voice_leading_quality
    }

    #[getter]
    fn improvements(&self, py: Python) -> PyResult<Py<PyList>> {
        let list = PyList::new(py, &self.inner.improvements);
        Ok(list.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "ProgressionAnalysis(voice_leading={:.3}, improvements={})",
            self.inner.voice_leading_quality,
            self.inner.improvements.len()
        )
    }
}

/// Python wrapper for BassHarmonization
#[pyclass(name = "BassHarmonization")]
#[derive(Clone)]
pub struct PyBassHarmonization {
    pub inner: BassHarmonization,
}

#[pymethods]
impl PyBassHarmonization {
    #[getter]
    fn bass_notes(&self, py: Python) -> PyResult<Py<PyList>> {
        let list = PyList::new(py, &self.inner.bass_notes);
        Ok(list.into())
    }

    #[getter]
    fn rhythm(&self, py: Python) -> PyResult<Py<PyList>> {
        let list = PyList::new(py, &self.inner.rhythm);
        Ok(list.into())
    }

    #[getter]
    fn confidence(&self) -> f64 {
        self.inner.confidence
    }

    #[getter]
    fn style(&self) -> String {
        match self.inner.style {
            BassStyle::Root => "Root".to_string(),
            BassStyle::Alternating => "Alternating".to_string(),
            BassStyle::Walking => "Walking".to_string(),
            BassStyle::Arpeggiated => "Arpeggiated".to_string(),
            BassStyle::Rhythmic => "Rhythmic".to_string(),
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "BassHarmonization(style='{}', notes={}, confidence={:.3})",
            self.style(),
            self.inner.bass_notes.len(),
            self.inner.confidence
        )
    }
}

/// Python wrapper for the main AI Engine
#[pyclass(name = "AiEngine")]
pub struct PyAiEngine {
    inner: AiEngine,
}

#[pymethods]
impl PyAiEngine {
    #[new]
    #[pyo3(signature = (max_memory_mb=None, enable_monitoring=true))]
    fn new(max_memory_mb: Option<u32>, enable_monitoring: bool) -> Self {
        let mut config = AiEngineConfig::default();

        if let Some(memory) = max_memory_mb {
            config.max_memory_mb = memory;
        }
        config.enable_performance_monitoring = enable_monitoring;

        let engine = AiEngine::new(config);
        PyAiEngine { inner: engine }
    }

    /// Initialize the engine with training patterns
    fn initialize(
        &self,
        training_patterns: Vec<(Vec<PyChord>, String, Option<String>)>,
    ) -> PyResult<()> {
        let patterns: Vec<(Vec<composer_core::Chord>, String, Option<String>)> = training_patterns
            .into_iter()
            .map(|(chords, id, tonic)| {
                let rust_chords = chords.into_iter().map(|c| c.inner).collect();
                (rust_chords, id, tonic)
            })
            .collect();

        self.inner.initialize(patterns).to_py_result()
    }

    /// Check if engine is initialized
    fn is_initialized(&self) -> bool {
        self.inner.is_initialized()
    }

    /// Get chord progression suggestions
    fn get_chord_suggestions(
        &self,
        pattern: Vec<PyChord>,
        context: &PySuggestionContext,
        config: &PySuggestionConfig,
        py: Python,
    ) -> PyResult<Py<PyList>> {
        let rust_pattern: Vec<composer_core::Chord> =
            pattern.into_iter().map(|c| c.inner).collect();

        let suggestions = self
            .inner
            .get_chord_suggestions(&rust_pattern, &context.inner, &config.inner)
            .to_py_result()?;

        let py_suggestions: Vec<PyObject> = suggestions
            .into_iter()
            .map(|s| PyChordSuggestion { inner: s }.into_py(py))
            .collect();

        let list = PyList::new(py, py_suggestions);
        Ok(list.into())
    }

    /// Assess difficulty of a chord progression
    #[pyo3(signature = (progression, tempo_bpm=None, time_signature=None))]
    fn assess_difficulty(
        &self,
        progression: Vec<PyChord>,
        tempo_bpm: Option<f64>,
        time_signature: Option<(u8, u8)>,
    ) -> PyResult<PyDifficultyAssessment> {
        let rust_progression: Vec<composer_core::Chord> =
            progression.into_iter().map(|c| c.inner).collect();

        let assessment = self
            .inner
            .assess_difficulty(&rust_progression, tempo_bpm, time_signature)
            .to_py_result()?;

        Ok(PyDifficultyAssessment { inner: assessment })
    }

    /// Analyze chord progression patterns
    fn analyze_progression(&self, progression: Vec<PyChord>) -> PyResult<PyProgressionAnalysis> {
        let rust_progression: Vec<composer_core::Chord> =
            progression.into_iter().map(|c| c.inner).collect();

        let analysis = self
            .inner
            .analyze_progression(&rust_progression)
            .to_py_result()?;

        Ok(PyProgressionAnalysis { inner: analysis })
    }

    /// Generate bass line harmonization
    #[pyo3(signature = (progression, style="Root", complexity=0.5, enable_walking=false))]
    fn harmonize_bass_line(
        &self,
        progression: Vec<PyChord>,
        style: &str,
        complexity: f64,
        enable_walking: bool,
    ) -> PyResult<PyBassHarmonization> {
        let rust_progression: Vec<composer_core::Chord> =
            progression.into_iter().map(|c| c.inner).collect();

        let bass_style = match style {
            "Root" => BassStyle::Root,
            "Alternating" => BassStyle::Alternating,
            "Walking" => BassStyle::Walking,
            "Arpeggiated" => BassStyle::Arpeggiated,
            "Rhythmic" => BassStyle::Rhythmic,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Invalid bass style: {}",
                    style
                )))
            },
        };

        let options = BassHarmonizationOptions {
            style: bass_style,
            complexity,
            enable_walking,
            rhythm_pattern: None,
        };

        let harmonization = self
            .inner
            .harmonize_bass_line(&rust_progression, &options)
            .to_py_result()?;

        Ok(PyBassHarmonization {
            inner: harmonization,
        })
    }

    /// Add a training pattern to the engine
    fn add_training_pattern(
        &self,
        pattern: Vec<PyChord>,
        source_id: String,
        key_tonic: Option<String>,
    ) -> PyResult<()> {
        let rust_pattern: Vec<composer_core::Chord> =
            pattern.into_iter().map(|c| c.inner).collect();

        self.inner
            .add_training_pattern(&rust_pattern, source_id, key_tonic)
            .to_py_result()
    }

    /// Get engine performance metrics
    fn get_metrics(&self, py: Python) -> PyResult<Py<PyDict>> {
        let metrics = self.inner.get_metrics();
        let dict = PyDict::new(py);

        dict.set_item("total_requests", metrics.total_requests)?;
        dict.set_item("avg_response_time_ms", metrics.avg_response_time_ms)?;
        dict.set_item("memory_usage_bytes", metrics.memory_usage_bytes)?;
        dict.set_item("cache_hit_rate", metrics.cache_hit_rate)?;
        dict.set_item("total_patterns", metrics.total_patterns)?;
        dict.set_item("uptime_seconds", metrics.uptime_seconds)?;

        Ok(dict.into())
    }

    /// Clear all caches
    fn clear_caches(&self) {
        self.inner.clear_caches();
    }

    /// Validate memory usage
    fn validate_memory_usage(&self) -> PyResult<()> {
        self.inner.validate_memory_usage().to_py_result()
    }

    /// Get magic chord solutions using statistical weighting
    #[pyo3(signature = (previous_chords, following_chords, scale="major", limit=10))]
    fn get_magic_chord_solutions(
        &self,
        previous_chords: Vec<PyChord>,
        following_chords: Vec<PyChord>,
        scale: &str,
        limit: usize,
        py: Python,
    ) -> PyResult<Py<PyList>> {
        let rust_previous: Vec<composer_core::Chord> =
            previous_chords.into_iter().map(|c| c.inner).collect();
        let rust_following: Vec<composer_core::Chord> =
            following_chords.into_iter().map(|c| c.inner).collect();

        let suggestions = self
            .inner
            .get_magic_chord_solutions(&rust_previous, &rust_following, scale, limit)
            .to_py_result()?;

        let py_suggestions: Vec<PyObject> = suggestions
            .into_iter()
            .map(|s| PyChordSuggestion { inner: s }.into_py(py))
            .collect();

        let list = PyList::new(py, py_suggestions);
        Ok(list.into())
    }

    /// Get bass harmonization solutions
    #[pyo3(signature = (bass_note, scale="major", limit=5))]
    fn get_magic_bass_solutions(
        &self,
        bass_note: &str,
        scale: &str,
        limit: usize,
        py: Python,
    ) -> PyResult<Py<PyList>> {
        let suggestions = self
            .inner
            .get_magic_bass_solutions(bass_note, scale, limit)
            .to_py_result()?;

        let py_suggestions: Vec<PyObject> = suggestions
            .into_iter()
            .map(|s| PyChordSuggestion { inner: s }.into_py(py))
            .collect();

        let list = PyList::new(py, py_suggestions);
        Ok(list.into())
    }

    /// Get scale degree harmonization solutions
    #[pyo3(signature = (scale_degree_bits, scale="major", limit=5))]
    fn get_harmonize_by_sd_solutions(
        &self,
        scale_degree_bits: u32,
        scale: &str,
        limit: usize,
        py: Python,
    ) -> PyResult<Py<PyList>> {
        let suggestions = self
            .inner
            .get_harmonize_by_sd_solutions(scale_degree_bits, scale, limit)
            .to_py_result()?;

        let py_suggestions: Vec<PyObject> = suggestions
            .into_iter()
            .map(|s| PyChordSuggestion { inner: s }.into_py(py))
            .collect();

        let list = PyList::new(py, py_suggestions);
        Ok(list.into())
    }

    /// Get average suggestion generation time
    fn avg_suggestion_time_ms(&self) -> f64 {
        self.inner.get_metrics().avg_response_time_ms
    }

    /// Shutdown the engine
    fn shutdown(&self) -> PyResult<()> {
        self.inner.shutdown().to_py_result()
    }
}
