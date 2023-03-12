use approx::AbsDiffEq;
use nalgebra as na;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
// use pyo3::types::PySequence;

use crate::mat4;

#[pyclass]
pub struct Vector3(pub na::Vector3<f64>);

impl Vector3 {
    pub fn as_4(&self) -> na::Vector4<f64> {
        na::Vector4::new(self.0[0], self.0[1], self.0[2], 1.0)
    }

    pub fn from_4(v: &na::Vector4<f64>) -> Vector3 {
        Vector3(na::Vector3::new(v[0], v[1], v[2]))
    }

    pub fn as_p3(&self) -> na::Point3<f64> {
        na::Point3::new(self.0[0], self.0[1], self.0[2])
    }

    pub fn from_p3(v: &na::Point3<f64>) -> Vector3 {
        Vector3(na::Vector3::new(v[0], v[1], v[2]))
    }

    pub fn as_translation(&self) -> na::Translation3<f64> {
        na::Translation3::new(self.0[0], self.0[1], self.0[2])
    }

    pub fn from_translation(v: &na::Translation3<f64>) -> Vector3 {
        Vector3(na::Vector3::new(v.x, v.y, v.z))
    }
}

#[pymethods]
impl Vector3 {
    #[new]
    fn new(x: Option<f64>, y: Option<f64>, z: Option<f64>) -> Self {
        Vector3(na::Vector3::new(
            x.unwrap_or(0.0),
            y.unwrap_or(0.0),
            z.unwrap_or(0.0),
        ))
    }

    // Slower than Vector3(*arr[1:4])... not worth it
    // #[staticmethod]
    // fn from_seq(v: &PySequence, offset: Option<usize>) -> PyResult<Self> {
    //     let off = offset.unwrap_or(0);
    //     let x: f64 = v.get_item(off)?.extract()?;
    //     let y: f64 = v.get_item(off + 1)?.extract()?;
    //     let z: f64 = v.get_item(off + 2)?.extract()?;
    //     Ok(Vector3(na::Vector3::new(x, y, z)))
    // }

    #[getter]
    fn get_x(&self) -> f64 {
        self.0.x
    }

    #[setter]
    fn set_x(&mut self, arg: f64) -> () {
        self.0.x = arg;
    }

    #[getter]
    fn get_y(&self) -> f64 {
        self.0.y
    }

    #[setter]
    fn set_y(&mut self, arg: f64) -> () {
        self.0.y = arg;
    }

    #[getter]
    fn get_z(&self) -> f64 {
        self.0.z
    }

    #[setter]
    fn set_z(&mut self, arg: f64) -> () {
        self.0.z = arg;
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

    fn length(&self) -> f64 {
        self.0.magnitude()
    }

    fn length_squared(&self) -> f64 {
        self.0.magnitude_squared()
    }

    fn normalize(&mut self) -> PyResult<()> {
        self.0.normalize_mut();
        Ok(())
    }

    fn normalized(&self) -> PyResult<Vector3> {
        Ok(Vector3(self.0.normalize()))
    }

    fn distance_to(&self, other: PyRef<Vector3>) -> f64 {
        (other.0 - self.0).magnitude()
    }

    fn distance_to_squared(&self, other: PyRef<Vector3>) -> f64 {
        (other.0 - self.0).magnitude_squared()
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
        Vector3(self.0 * arg)
    }

    fn __imul__(&mut self, arg: f64) -> () {
        self.0 *= arg;
    }

    fn __truediv__(&self, arg: f64) -> Vector3 {
        Vector3(self.0 / arg)
    }

    fn __itruediv__(&mut self, arg: f64) -> () {
        self.0 /= arg;
    }

    fn premultiply(&self, arg: &mat4::Matrix4) -> Vector3 {
        let v = arg.0 * self.as_4();
        Vector3(na::Vector3::new(v[0], v[1], v[2]))
    }

    fn cross(&self, v: PyRef<Vector3>) -> Vector3 {
        let x = self.0.y * v.0.z - self.0.z * v.0.y;
        let y = self.0.z * v.0.x - self.0.x * v.0.z;
        let z = self.0.x * v.0.y - self.0.y * v.0.x;
        Vector3(na::Vector3::new(x, y, z))
        // Vector3(v.0.cross(&v.0))
    }

    fn dot(&self, v: PyRef<Vector3>) -> f64 {
        self.0.x * v.0.x + self.0.y * v.0.y + self.0.z * v.0.z
        // v.0.dot(&v.0)
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
