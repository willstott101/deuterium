use approx::AbsDiffEq;
use nalgebra::SVector;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;

use crate::mat4;

pub type Vector3d = SVector<f64, 3>;
pub type Vector4d = SVector<f64, 4>;

#[pyclass(sequence)]
pub struct Vector3(pub Vector3d);

impl Vector3 {
    pub fn as_4(&self) -> Vector4d {
        Vector4d::new(self.0[0], self.0[1], self.0[2], 1.0)
    }

    pub fn from_4(v: &Vector4d) -> Vector3 {
        Vector3(Vector3d::new(v[0], v[1], v[2]))
    }
}

#[pymethods]
impl Vector3 {
    #[new]
    fn new(x: Option<f64>, y: Option<f64>, z: Option<f64>) -> Self {
        return Vector3(Vector3d::new(
            x.unwrap_or(0.0),
            y.unwrap_or(0.0),
            z.unwrap_or(0.0),
        ));
    }

    #[staticmethod]
    fn zero() -> Vector3 {
        return Vector3(Vector3d::new(0.0, 0.0, 0.0));
    }

    #[staticmethod]
    fn x() -> Vector3 {
        return Vector3(Vector3d::new(1.0, 0.0, 0.0));
    }

    #[staticmethod]
    fn y() -> Vector3 {
        return Vector3(Vector3d::new(0.0, 1.0, 0.0));
    }

    #[staticmethod]
    fn z() -> Vector3 {
        return Vector3(Vector3d::new(0.0, 0.0, 1.0));
    }

    fn __getitem__(&self, idx: isize) -> Result<f64, PyErr> {
        let i: usize = if idx < 0 && idx > -(self.0.len() as isize + 1) {
            (self.0.len() as isize + idx) as usize
        } else {
            match TryInto::<usize>::try_into(idx) {
                Err(_) => Err(PyIndexError::new_err(idx))?,
                Ok(ui) => ui,
            }
        };
        match self.0.get(i) {
            Some(v) => Ok(v.clone()),
            None => Err(PyIndexError::new_err(idx)),
        }
    }

    fn __setitem__(&mut self, idx: isize, value: f64) -> Result<(), PyErr> {
        let i: usize = if idx < 0 && idx > -(self.0.len() as isize + 1) {
            self.0.len() - idx as usize
        } else {
            let ui: usize = idx.try_into()?;
            ui
        };

        self.0[i] = value;
        Ok(())
    }

    #[classattr]
    const __contains__: Option<PyObject> = None;

    fn __richcmp__(&self, py: Python, other: &Vector3, op: CompareOp) -> Py<PyAny> {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn approx_equals(&self, v: &Vector3) -> bool {
        self.0.abs_diff_eq(&v.0, 1e-08)
    }

    fn __add__(&self, v: &Vector3) -> Vector3 {
        Vector3(self.0 + v.0)
    }

    fn __sub__(&self, v: &Vector3) -> Vector3 {
        Vector3(self.0 - v.0)
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn __iadd__(&mut self, v: &Vector3) -> () {
        self.0[0] += v.0[0];
        self.0[1] += v.0[1];
        self.0[2] += v.0[2];
    }

    fn __isub__(&mut self, v: &Vector3) -> () {
        self.0[0] -= v.0[0];
        self.0[1] -= v.0[1];
        self.0[2] -= v.0[2];
    }

    fn __mul__(&self, arg: f64) -> Vector3 {
        Vector3(Vector3d::new(
            self.0[0] * arg,
            self.0[1] * arg,
            self.0[2] * arg,
        ))
    }

    fn __imul__(&mut self, arg: f64) -> () {
        self.0[0] *= arg;
        self.0[1] *= arg;
        self.0[2] *= arg;
    }

    fn premultiply(&mut self, arg: &mat4::Matrix4) -> () {
        let v = arg.0 * self.as_4();
        self.0[0] = v[0];
        self.0[1] = v[1];
        self.0[2] = v[2];
    }

    fn __neg__(&self) -> Vector3 {
        Vector3(-self.0)
    }

    fn negate(&mut self) -> () {
        self.0 = -self.0;
    }

    fn tuple(&self) -> (f64, f64, f64) {
        return (self.0[0], self.0[1], self.0[2]);
    }

    fn list(&self) -> [f64; 3] {
        return [self.0[0], self.0[1], self.0[2]];
    }

    fn __repr__(&self) -> String {
        format!("Vector3({}, {}, {})", self.0[0], self.0[1], self.0[2])
    }
}
