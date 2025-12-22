//pub mod raw;
//pub mod wrapped;

pub mod level;
pub mod sub_blocks;
pub mod types;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyo3::pymodule]
pub fn lotrc(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_submodule(&level::init(py)?)?;
    m.add_submodule(&sub_blocks::init(py)?)?;
    m.add_submodule(&types::init(py)?)?;
    Ok(())
}
