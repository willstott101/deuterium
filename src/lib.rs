use pyo3::prelude::*;

mod mat4;
mod vec3;

#[pymodule]
/// A Python module implemented in Rust.
fn deuterium(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<mat4::Matrix4>()?;
    m.add_class::<vec3::Vector3>()?;
    Ok(())
}
