use crate::quat::UnitQuaternion;
use crate::vec3::Vector3;
use approx::AbsDiffEq;
use nalgebra as na;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;

#[pyclass]
pub struct Isometry3(pub na::Isometry3<f64>);

#[pymethods]
impl Isometry3 {
    #[staticmethod]
    fn identity() -> Isometry3 {
        return Isometry3(na::Isometry3::identity());
    }

    #[staticmethod]
    fn from_translation(v: &Vector3) -> Isometry3 {
        return Isometry3(na::Isometry3::translation(v.0[0], v.0[1], v.0[2]));
    }

    fn __richcmp__(&self, py: Python, other: &Isometry3, op: CompareOp) -> Py<PyAny> {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn approx_equals(&self, arg: &Isometry3) -> bool {
        self.0.abs_diff_eq(&arg.0, 1e-08)
    }

    fn __mul__(&self, py: Python, arg: &PyAny) -> PyResult<PyObject> {
        let isor: PyResult<PyRef<Isometry3>> = arg.extract();
        if let Ok(iso) = isor {
            return Ok(Py::new(py, Isometry3(self.0 * iso.0))?.to_object(py));
        }
        let vecr: PyResult<PyRef<Vector3>> = arg.extract();
        if let Ok(vec) = vecr {
            return Ok(
                Py::new(py, Vector3::from_p3(&self.0.transform_point(&vec.as_p3())))?.to_object(py),
            );
        }
        Ok(py.NotImplemented())
    }

    fn __imul__(&mut self, arg: &Isometry3) -> () {
        self.0 = self.0 * arg.0;
    }

    fn __len__(&self) -> usize {
        4
    }

    fn premultiply(&mut self, arg: &Isometry3) -> () {
        self.0 = arg.0 * self.0;
    }

    fn invert(&mut self) -> () {
        self.0 = self.0.inverse();
    }

    fn inverse(&mut self) -> Isometry3 {
        Isometry3(self.0.inverse())
    }

    #[getter]
    fn get_translation(&self) -> Vector3 {
        Vector3::from_translation(&self.0.translation)
    }

    #[setter]
    fn set_translation(&mut self, v: PyRef<Vector3>) -> PyResult<()> {
        self.0.translation = v.as_translation();
        Ok(())
    }

    fn translate(&mut self, v: PyRef<Vector3>) -> () {
        self.0.translation *= v.as_translation();
    }

    #[getter]
    fn get_rotation(&self) -> UnitQuaternion {
        UnitQuaternion(self.0.rotation)
    }

    #[setter]
    fn set_rotation(&mut self, v: PyRef<UnitQuaternion>) -> PyResult<()> {
        self.0.rotation = v.0;
        Ok(())
    }
}
