use pyo3::prelude::*;

pub mod engine;

#[pymodule]
fn _engine(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<engine::FileDB>()?;
    Ok(())
}
