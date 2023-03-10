use approx::AbsDiffEq;
use nalgebra as na;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;

use crate::vec3::Vector3;

#[pyclass(sequence)]
pub struct UnitQuaternion(pub na::UnitQuaternion<f64>);

#[pymethods]
impl UnitQuaternion {
    #[new]
    fn new() -> Self {
        return UnitQuaternion(na::UnitQuaternion::identity());
    }

    #[staticmethod]
    fn from_axis_angle(v: &Vector3, a: f64) -> UnitQuaternion {
        let vn = na::Unit::new_normalize(v.0);
        return UnitQuaternion(na::UnitQuaternion::from_axis_angle(&vn, a));
    }

    #[staticmethod]
    fn from_scaled_axis(v: &Vector3) -> UnitQuaternion {
        return UnitQuaternion(na::UnitQuaternion::from_scaled_axis(v.0));
    }

    fn __getitem__(&self, idx: isize) -> Result<f64, PyErr> {
        let i: usize = if idx < 0 && idx > -(self.__len__() as isize + 1) {
            (self.__len__() as isize + idx) as usize
        } else {
            match TryInto::<usize>::try_into(idx) {
                Err(_) => Err(PyIndexError::new_err(idx))?,
                Ok(ui) => ui,
            }
        };
        match self.0.coords.get(i) {
            Some(v) => Ok(v.clone()),
            None => Err(PyIndexError::new_err(idx)),
        }
    }

    // fn __setitem__(&mut self, idx: isize, value: f64) -> Result<(), PyErr> {
    //     let i: usize = if idx < 0 && idx > -(self.__len__() as isize + 1) {
    //         self.__len__() - idx as usize
    //     } else {
    //         let ui: usize = idx.try_into()?;
    //         ui
    //     };

    //     self.0[i] = value;
    //     Ok(())
    // }

    #[classattr]
    const __contains__: Option<PyObject> = None;

    fn __richcmp__(&self, py: Python, other: &UnitQuaternion, op: CompareOp) -> Py<PyAny> {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn approx_equals(&self, v: &UnitQuaternion) -> bool {
        self.0.abs_diff_eq(&v.0, 1e-08)
    }

    fn __len__(&self) -> usize {
        4
    }

    fn __mul__(&self, py: Python, arg: &PyAny) -> PyResult<PyObject> {
        let quatr: PyResult<PyRef<UnitQuaternion>> = arg.extract();
        if let Ok(quat) = quatr {
            return Ok(Py::new(py, UnitQuaternion(self.0 * quat.0))?.to_object(py));
        }
        let vecr: PyResult<PyRef<Vector3>> = arg.extract();
        if let Ok(vec) = vecr {
            return Ok(
                Py::new(py, Vector3::from_p3(&self.0.transform_point(&vec.as_p3())))?.to_object(py),
            );
        }
        Ok(py.NotImplemented())
    }

    fn __imul__(&mut self, arg: PyRef<UnitQuaternion>) -> () {
        self.0 = self.0 * arg.0;
    }

    fn premultiply(&mut self, arg: PyRef<UnitQuaternion>) -> () {
        self.0 = arg.0 * self.0;
    }

    fn inverse(&self) -> UnitQuaternion {
        UnitQuaternion(self.0.conjugate())
    }

    fn invert(&mut self) -> () {
        self.0 = self.0.conjugate();
    }

    fn tuple(&self) -> (f64, f64, f64, f64) {
        (self.0[0], self.0[1], self.0[2], self.0[3])
    }

    fn list(&self) -> [f64; 4] {
        [self.0[0], self.0[1], self.0[2], self.0[3]]
    }

    fn __repr__(&self) -> String {
        format!(
            "UnitQuaternion({}, {}, {}, {}",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}
