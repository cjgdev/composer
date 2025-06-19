//! Python bindings for music theory functions

use crate::error::ToPyResult;
use crate::{PyChord, PyScaleFingerprint};
use composer_core::theory;
use pyo3::prelude::*;

/// Python wrapper for RomanNumeralGraphic
#[pyclass]
#[derive(Clone)]
pub struct PyChordGraphic {
    inner: composer_core::roman::RomanNumeralGraphic,
}

#[pymethods]
impl PyChordGraphic {
    /// Roman numeral symbol (I, ii, V7, etc.)
    #[getter]
    fn symbol(&self) -> String {
        self.inner.symbol.clone()
    }

    /// Figured bass notation (6, 65, 42, etc.)
    #[getter]
    fn figured_bass(&self) -> String {
        self.inner.figured_bass.clone()
    }

    /// Quality symbols (°, ø, +, maj, m)
    #[getter]
    fn quality(&self) -> String {
        self.inner.quality.clone()
    }

    /// Applied notation (/V, /vi, etc.)
    #[getter]
    fn applied(&self) -> String {
        self.inner.applied.clone()
    }

    /// Borrowed chord indication
    #[getter]
    fn borrowed(&self) -> String {
        self.inner.borrowed.clone()
    }

    /// Visible alterations
    #[getter]
    fn alterations(&self) -> Vec<String> {
        self.inner.alterations.iter().cloned().collect()
    }

    /// Suspension notations
    #[getter]
    fn suspensions(&self) -> Vec<String> {
        self.inner.suspensions.iter().cloned().collect()
    }

    /// Add tone notations
    #[getter]
    fn adds(&self) -> Vec<String> {
        self.inner.adds.iter().cloned().collect()
    }

    /// Omit tone notations
    #[getter]
    fn omits(&self) -> Vec<String> {
        self.inner.omits.iter().cloned().collect()
    }

    fn __str__(&self) -> String {
        format!("{}{}", self.inner.symbol, self.inner.figured_bass)
    }

    fn __repr__(&self) -> String {
        format!(
            "ChordGraphic(symbol='{}', figured_bass='{}', quality='{}')",
            self.inner.symbol, self.inner.figured_bass, self.inner.quality
        )
    }
}

/// Python wrapper for ScaleDegreeResult
#[pyclass]
#[derive(Clone)]
pub struct PyRelativeScaleDegrees {
    inner: composer_core::theory::ScaleDegreeResult,
}

#[pymethods]
impl PyRelativeScaleDegrees {
    /// Scale degree numbers (1-7)
    #[getter]
    fn sd_numbers(&self) -> Vec<u8> {
        self.inner.sd_numbers.clone()
    }

    /// Accidentals for each scale degree ("", "b", "#")
    #[getter]
    fn sd_accs(&self) -> Vec<String> {
        self.inner.sd_accs.clone()
    }

    fn __str__(&self) -> String {
        let degrees_with_accs: Vec<String> = self
            .inner
            .sd_numbers
            .iter()
            .zip(self.inner.sd_accs.iter())
            .map(|(num, acc)| format!("{}{}", acc, num))
            .collect();
        format!("[{}]", degrees_with_accs.join(", "))
    }

    fn __repr__(&self) -> String {
        format!(
            "RelativeScaleDegrees(numbers={:?}, accidentals={:?})",
            self.inner.sd_numbers, self.inner.sd_accs
        )
    }
}

/// Calculate chord complexity score
#[pyfunction]
pub fn get_chord_complexity(chord: &PyChord) -> PyResult<f64> {
    theory::get_chord_complexity(&chord.inner, "major").to_py_result()
}

/// Generate complete Roman numeral representation
#[pyfunction]
pub fn get_relative_chord_graphic(
    chord: &PyChord,
    scale: &PyScaleFingerprint,
) -> PyResult<PyChordGraphic> {
    let graphic = theory::get_relative_chord_graphic(&chord.inner, &scale.inner).to_py_result()?;
    Ok(PyChordGraphic { inner: graphic })
}

/// Calculate stable scale degrees for a chord in scale context
#[pyfunction]
pub fn get_stable_scale_degrees(
    chord: &PyChord,
    scale: &PyScaleFingerprint,
) -> PyResult<Vec<String>> {
    theory::get_stable_scale_degrees(&chord.inner, &scale.inner).to_py_result()
}

/// Calculate scale degrees relative to chord root
#[pyfunction]
pub fn get_relative_scale_degrees(chord: &PyChord) -> PyResult<PyRelativeScaleDegrees> {
    let result = theory::get_relative_scale_degrees(&chord.inner).to_py_result()?;
    Ok(PyRelativeScaleDegrees { inner: result })
}

/// Validate tritone substitution eligibility
#[pyfunction]
pub fn is_valid_tri_sub(chord: &PyChord, scale_type: &str) -> bool {
    theory::is_valid_tri_sub(&chord.inner, scale_type)
}

/// Check if two chords are harmonically equivalent (isotonal)
#[pyfunction]
pub fn is_isotonal(
    chord1: &PyChord,
    chord2: &PyChord,
    scale: &PyScaleFingerprint,
) -> PyResult<bool> {
    // Get stable scale degrees for both chords
    let degrees1 = theory::get_stable_scale_degrees(&chord1.inner, &scale.inner).to_py_result()?;
    let degrees2 = theory::get_stable_scale_degrees(&chord2.inner, &scale.inner).to_py_result()?;

    // Check if they have the same scale degrees (isotonal)
    Ok(degrees1 == degrees2)
}

/// Analyze harmonic function of a chord
#[pyfunction]
pub fn analyze_harmonic_function(chord: &PyChord, _scale_type: &str) -> String {
    match chord.inner.root {
        1 | 6 | 3 => "Tonic".to_string(),   // I, vi, iii
        4 | 2 => "Predominant".to_string(), // IV, ii
        5 | 7 => "Dominant".to_string(),    // V, vii°
        _ => "Chromatic".to_string(),
    }
}

/// Convert chord letter to lowercase while preserving accidentals
#[pyfunction]
pub fn chord_letter_to_lower_case(note_string: &str) -> String {
    theory::chord_letter_to_lower_case(note_string)
}

/// Convert chord letter to uppercase while preserving accidentals
#[pyfunction]
pub fn chord_letter_to_upper_case(note_string: &str) -> String {
    theory::chord_letter_to_upper_case(note_string)
}
