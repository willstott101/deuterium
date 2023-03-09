use pyo3::prelude::*;

mod mat4;
mod vec3;
mod iso;

#[pymodule]
/// A Python module wrapping the nalgebra crate to provide pythonic linear algebra
fn deuterium(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<mat4::Matrix4>()?;
    m.add_class::<vec3::Vector3>()?;
    m.add_class::<iso::Isometry3>()?;
    Ok(())
}
