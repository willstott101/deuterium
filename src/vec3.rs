use crate::iso::Isometry3;
use crate::mat4::Matrix4;
use crate::quat::UnitQuaternion;
use approx::AbsDiffEq;
use nalgebra as na;
use pyo3::exceptions::{PyIndexError, PyTypeError};
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
// use pyo3::types::PySequence;

use crate::mat4;

#[pyclass]
#[derive(Clone)]
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

    fn approx_equals(&self, other: &Vector3) -> bool {
        self.0.abs_diff_eq(&other.0, 1e-08)
    }

    fn __add__(&self, other: &Vector3) -> Vector3 {
        Vector3(self.0 + other.0)
    }

    fn __sub__(&self, other: &Vector3) -> Vector3 {
        Vector3(self.0 - other.0)
    }

    #[staticmethod]
    fn __len__() -> usize {
        3
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

    fn __iadd__(&mut self, other: &Vector3) -> () {
        self.0 += other.0;
    }

    fn __isub__(&mut self, other: &Vector3) -> () {
        self.0 -= other.0;
    }

    fn __mul__(&self, arg: &PyAny) -> PyResult<Vector3> {
        let scalarr: PyResult<f64> = arg.extract();
        if let Ok(scalar) = scalarr {
            return Ok(Vector3(self.0 * scalar));
        }
        let vecr: PyResult<PyRef<Vector3>> = arg.extract();
        if let Ok(vec) = vecr {
            return Ok(Vector3(na::Vector3::new(self.0.x * vec.0.x, self.0.y * vec.0.y, self.0.z * vec.0.z)));
        }
        return Err(PyTypeError::new_err(format!(
            "Cannot multiply a Vector3 by {}",
            arg.get_type().name().unwrap_or("?")
        )));
    }

    fn __imul__(&mut self, arg: &PyAny) -> PyResult<()> {
        let r = self.__mul__(arg)?;
        self.0 = r.0;
        Ok(())
    }

    fn __truediv__(&self, arg: f64) -> Vector3 {
        Vector3(self.0 / arg)
    }

    fn __itruediv__(&mut self, arg: f64) -> () {
        self.0 /= arg;
    }

    fn premultiply(&mut self, other: &mat4::Matrix4) -> () {
        let v = other.0 * self.as_4();
        self.0[0] = v[0];
        self.0[1] = v[1];
        self.0[2] = v[2];
    }

    fn cross(&self, other: PyRef<Vector3>) -> Vector3 {
        Vector3(self.0.cross(&other.0))
    }

    fn dot(&self, other: PyRef<Vector3>) -> f64 {
        self.0.dot(&other.0)
    }

    fn angle_between(&self, other: PyRef<Vector3>) -> f64 {
        self.0.angle(&other.0)
    }

    /// Projects the current vector onto the given `other` vector.
    ///
    /// The resulting vector represents the component of the current vector that lies
    /// along the direction of the `other` vector. If the `other` vector is already
    /// normalized (i.e., it has a length of 1), the magnitude of the resulting vector
    /// will be equal to the length of the projection. If the `other` vector is not
    /// normalized, the magnitude of the resulting vector will be scaled by the length
    /// of the `other` vector.
    ///
    /// It is not necessary to normalize the input vectors before calling this method,
    /// but doing so can make it easier to interpret the resulting vector's magnitude
    /// in certain applications.
    fn projected_onto(&self, other: PyRef<Vector3>) -> Vector3 {
        let scalar_proj = self.0.dot(&other.0) / other.0.magnitude_squared();
        Vector3(other.0 * scalar_proj)
    }

    /// Performs projected_onto in-place
    fn project_onto(&mut self, other: PyRef<Vector3>) -> () {
        let scalar_proj = self.0.dot(&other.0) / other.0.magnitude_squared();
        self.0 = other.0 * scalar_proj;
    }

    /// Returns a new Vector3 object which lies 't' position along the
    /// line from self to other where a 't' of 1 is at the same position as
    /// other and a't' of 0 is at the same position as self.
    fn lerp(&self, other: PyRef<Vector3>, t: f64) -> Vector3 {
        Vector3(self.0.lerp(&other.0, t))
    }

    fn transformed(&self, arg: &PyAny) -> PyResult<Vector3> {
        let isor: PyResult<PyRef<Isometry3>> = arg.extract();
        if let Ok(iso) = isor {
            // For now recommending to use Vector3.transformed(Isometry3.rotation()) for transform_vector
            return Ok(Vector3::from_p3(&iso.0.transform_point(&self.as_p3())));
        }
        let matr: PyResult<PyRef<Matrix4>> = arg.extract();
        if let Ok(mat) = matr {
            // For now recommending to use Vector3.transformed(Matrix4.rotation()) for transform_vector
            return Ok(Vector3::from_p3(&mat.0.transform_point(&self.as_p3())));
        }
        let quatr: PyResult<PyRef<UnitQuaternion>> = arg.extract();
        if let Ok(quat) = quatr {
            return Ok(Vector3(quat.0.transform_vector(&self.0)));
        }
        return Err(PyTypeError::new_err(format!(
            "Cannot transform a Vector3 by {}",
            arg.get_type().name().unwrap_or("?")
        )));
    }

    fn transform(&mut self, arg: &PyAny) -> PyResult<()> {
        let t = self.transformed(arg)?;
        self.0 = t.0;
        Ok(())
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
