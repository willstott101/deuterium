use pyo3::prelude::*;

mod iso;
mod mat4;
mod quat;
mod vec3;

#[pymodule]
/// A Python module wrapping the nalgebra crate to provide pythonic linear algebra
fn deuterium(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<mat4::Matrix4>()?;
    m.add_class::<vec3::Vector3>()?;
    m.add_class::<iso::Isometry3>()?;
    m.add_class::<quat::UnitQuaternion>()?;
    Ok(())
}
