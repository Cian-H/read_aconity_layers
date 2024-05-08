use numpy::{IntoPyArray, PyArray2};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use pyo3::types::{PyList, PyString};
use std::path::Path;

pub mod rust_fn;

#[pymodule]
fn read_aconity_layers(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn read_layers<'py>(py: Python<'py>, folder: &PyString) -> &'py PyArray2<f64> {
        rust_fn::read_layers(folder.to_str().unwrap()).into_pyarray(py)
    }

    #[pyfn(m)]
    fn read_selected_layers<'py>(py: Python<'py>, file_list: &PyList) -> &'py PyArray2<f64> {
        rust_fn::read_selected_layers(
            file_list
                .iter()
                .map(|x| Path::new(&(*x).str().unwrap().to_string()).to_path_buf())
                .collect(),
        )
        .into_pyarray(py)
    }

    #[pyfn(m)]
    fn read_layer<'py>(py: Python<'py>, file: &PyString) -> &'py PyArray2<f64> {
        rust_fn::read_layer(file.to_str().unwrap()).into_pyarray(py)
    }

    Ok(())
}
