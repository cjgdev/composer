//! Error handling for Python bindings

use composer_ai::AiError;
use composer_core::ChordTheoryError;
use composer_serialization::SerializationError;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

/// Custom exception for Composer errors
#[pyclass(extends=pyo3::exceptions::PyException)]
pub struct PyComposerError;

#[pymethods]
impl PyComposerError {
    #[new]
    fn new(_message: String) -> Self {
        PyComposerError
    }
}

/// Convert Rust errors to Python exceptions
pub trait ToPyResult<T> {
    fn to_py_result(self) -> PyResult<T>;
}

impl<T> ToPyResult<T> for Result<T, ChordTheoryError> {
    fn to_py_result(self) -> PyResult<T> {
        self.map_err(|e| PyValueError::new_err(format!("Chord theory error: {}", e)))
    }
}

impl<T> ToPyResult<T> for Result<T, SerializationError> {
    fn to_py_result(self) -> PyResult<T> {
        self.map_err(|e| PyRuntimeError::new_err(format!("Serialization error: {}", e)))
    }
}

impl<T> ToPyResult<T> for Result<T, AiError> {
    fn to_py_result(self) -> PyResult<T> {
        self.map_err(|e| match e.severity() {
            composer_ai::Severity::Critical => {
                PyRuntimeError::new_err(format!("AI engine error: {}", e))
            },
            _ => PyValueError::new_err(format!("AI error: {}", e)),
        })
    }
}
