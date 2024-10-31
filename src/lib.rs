use ndarray::{ArrayBase, Ix2, OwnedRepr};
use numpy::{PyArray2, ToPyArray};
use pyo3::exceptions;
use pyo3::prelude::*;
use std::path::Path;

pub mod rust_fn;

impl From<rust_fn::ReadError> for PyErr {
    fn from(err: rust_fn::ReadError) -> Self {
        match err {
            rust_fn::ReadError::Glob(e) => {
                PyErr::new::<exceptions::PyRuntimeError, _>(format!("{}", e))
            }
            rust_fn::ReadError::GlobPattern(e) => {
                PyErr::new::<exceptions::PyRuntimeError, _>(format!("{}", e))
            }
            rust_fn::ReadError::Io(e) => PyErr::new::<exceptions::PyIOError, _>(format!("{}", e)),
            rust_fn::ReadError::CSV(e) => PyErr::new::<exceptions::PyIOError, _>(format!("{}", e)),
            rust_fn::ReadError::ParseIntError(e) => {
                PyErr::new::<exceptions::PyRuntimeError, _>(format!("{}", e))
            }
            rust_fn::ReadError::ParseFloatError(e) => {
                PyErr::new::<exceptions::PyRuntimeError, _>(format!("{}", e))
            }
            rust_fn::ReadError::ShapeError(e) => {
                PyErr::new::<exceptions::PyIOError, _>(format!("{}", e))
            }
            rust_fn::ReadError::MiscError(e) => PyErr::new::<exceptions::PyRuntimeError, _>(e),
        }
    }
}

#[pyfunction]
fn read_layers<'py>(_py: Python<'py>, folder: &'py str) -> PyResult<Bound<'py, PyArray2<f64>>>
where
    ArrayBase<OwnedRepr<f64>, Ix2>: ToPyArray<Item = f64, Dim = Ix2>,
{
    let rs_result = rust_fn::read_layers(folder)?;
    let py_result = rs_result.to_pyarray_bound(_py);
    Ok(py_result)
}

#[pyfunction]
fn read_selected_layers<'py>(
    _py: Python<'py>,
    file_list: Vec<String>,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    let path_list = file_list
        .iter()
        .map(|x| Path::new(x).to_path_buf())
        .collect();
    let rs_result = rust_fn::read_selected_layers(path_list)?;
    let py_result = rs_result.to_pyarray_bound(_py);
    Ok(py_result)
}

#[pyfunction]
fn read_layer<'py>(_py: Python<'py>, file: String) -> PyResult<Bound<'py, PyArray2<f64>>> {
    let rs_result = rust_fn::read_layer(&file)?;
    let py_result = rs_result.to_pyarray_bound(_py);
    Ok(py_result)
}

#[pymodule]
fn read_aconity_layers(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_layers, m)?)?;
    m.add_function(wrap_pyfunction!(read_selected_layers, m)?)?;
    m.add_function(wrap_pyfunction!(read_layer, m)?)?;
    Ok(())
}
