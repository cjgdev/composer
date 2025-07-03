//! Python bindings for scale data structures

use crate::error::ToPyResult;
use composer_core::ScaleFingerprint;
use pyo3::prelude::*;
use pyo3::types::PyList;

/// Python wrapper for ScaleFingerprint
#[pyclass(name = "ScaleFingerprint")]
#[derive(Clone)]
pub struct PyScaleFingerprint {
    pub inner: ScaleFingerprint,
}

#[pymethods]
impl PyScaleFingerprint {
    #[new]
    fn new(scale_degrees: Vec<u8>) -> PyResult<Self> {
        let fingerprint = ScaleFingerprint::from_slice(&scale_degrees).to_py_result()?;
        Ok(PyScaleFingerprint { inner: fingerprint })
    }

    /// Create a major scale fingerprint
    #[staticmethod]
    fn major() -> PyResult<Self> {
        let fingerprint = ScaleFingerprint::major_scale();
        Ok(PyScaleFingerprint { inner: fingerprint })
    }

    /// Create a minor scale fingerprint
    #[staticmethod]
    fn minor() -> PyResult<Self> {
        let fingerprint = ScaleFingerprint::minor_scale();
        Ok(PyScaleFingerprint { inner: fingerprint })
    }

    /// Create a dorian scale fingerprint
    #[staticmethod]
    fn dorian() -> PyResult<Self> {
        let fingerprint = ScaleFingerprint::dorian_scale();
        Ok(PyScaleFingerprint { inner: fingerprint })
    }

    /// Create a mixolydian scale fingerprint
    #[staticmethod]
    fn mixolydian() -> PyResult<Self> {
        let fingerprint = ScaleFingerprint::mixolydian_scale();
        Ok(PyScaleFingerprint { inner: fingerprint })
    }

    /// Create a chromatic scale fingerprint
    #[staticmethod]
    fn chromatic() -> PyResult<Self> {
        let fingerprint = ScaleFingerprint::chromatic_scale();
        Ok(PyScaleFingerprint { inner: fingerprint })
    }

    /// Get scale degrees as a list
    #[getter]
    fn scale_degrees(&self, py: Python) -> PyResult<Py<PyList>> {
        let degrees: Vec<u8> = self.inner.scale_degrees().into_iter().collect();
        let list = PyList::new(py, degrees)?;
        Ok(list.into())
    }

    /// Get chromatic notes as a list  
    #[getter]
    fn chromatic_notes(&self, py: Python) -> PyResult<Py<PyList>> {
        let notes: Vec<u8> = self.inner.chromatic_notes().into_iter().collect();
        let list = PyList::new(py, notes)?;
        Ok(list.into())
    }

    /// Check if scale contains a specific chromatic note
    fn contains_chromatic(&self, note: u8) -> bool {
        self.inner.contains_chromatic(note)
    }

    /// Convert scale degree to chromatic note
    fn scale_degree_to_chromatic(&self, degree: u8) -> Option<u8> {
        self.inner.scale_degree_to_chromatic(degree)
    }

    /// Convert chromatic note to scale degree
    fn chromatic_to_scale_degree(&self, note: u8) -> Option<u8> {
        self.inner.chromatic_to_scale_degree(note)
    }

    // Display
    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    fn __repr__(&self) -> String {
        let degrees: Vec<u8> = self.inner.scale_degrees().into_iter().collect();
        format!("ScaleFingerprint({:?})", degrees)
    }

    // Equality
    fn __eq__(&self, other: &PyScaleFingerprint) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &PyScaleFingerprint) -> bool {
        self.inner != other.inner
    }
}
