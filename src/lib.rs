use numpy::{IntoPyArray, PyArray2};
use pyo3::exceptions;
use pyo3::prelude::{pymodule, PyErr, PyModule, PyResult, Python};
use pyo3::types::{PyList, PyString};
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
            rust_fn::ReadError::NdarrayCSV(e) => {
                PyErr::new::<exceptions::PyIOError, _>(format!("{}", e))
            }
            rust_fn::ReadError::ParseFloatError(e) => {
                PyErr::new::<exceptions::PyRuntimeError, _>(format!("{}", e))
            }
            rust_fn::ReadError::MiscError(e) => {
                PyErr::new::<exceptions::PyRuntimeError, _>(format!("{}", e))
            }
        }
    }
}

#[pymodule]
fn read_aconity_layers(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn read_layers<'py>(py: Python<'py>, folder: &PyString) -> PyResult<&'py PyArray2<f64>> {
        Ok(rust_fn::read_layers(folder.to_str().unwrap())?.into_pyarray(py))
    }

    #[pyfn(m)]
    fn read_selected_layers<'py>(
        py: Python<'py>,
        file_list: &PyList,
    ) -> PyResult<&'py PyArray2<f64>> {
        Ok(rust_fn::read_selected_layers(
            file_list
                .iter()
                .map(|x| Path::new(&(*x).str().unwrap().to_string()).to_path_buf())
                .collect(),
        )?
        .into_pyarray(py))
    }

    #[pyfn(m)]
    fn read_layer<'py>(py: Python<'py>, file: &PyString) -> PyResult<&'py PyArray2<f64>> {
        Ok(rust_fn::read_layer(file.to_str().unwrap())?.into_pyarray(py))
    }

    Ok(())
}
