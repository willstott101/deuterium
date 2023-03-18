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
    fn identity() -> Self {
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

    #[staticmethod]
    fn from_euler(roll: f64, pitch: f64, yaw: f64) -> UnitQuaternion {
        UnitQuaternion(na::UnitQuaternion::from_euler_angles(roll, pitch, yaw))
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

    /// Performs a spherical linear interpolation (slerp) between `self` and `other` unit quaternions.
    /// The interpolation parameter `t` ranges from 0 to 1, where 0 corresponds to `self` and 1 corresponds to `other`.
    /// When t is in-between 0 and 1, it returns a smoothly interpolated quaternion between `self` and `other`.
    fn slerp(&self, other: PyRef<UnitQuaternion>, t: f64) -> UnitQuaternion {
        UnitQuaternion(self.0.slerp(&other.0, t))
    }

    /// Returns the angle of rotation (in radians) represented by the UnitQuaternion.
    ///
    /// Axis-angle representation is a way to describe the orientation of an object
    /// in 3D space. It consists of a unit vector (the axis) and an angle. The object
    /// is rotated around the axis by the specified angle. The angle is expressed in
    /// radians and lies within the range [0, π].
    ///
    /// Note: The returned angle might be the negation of the actual angle if the
    /// quaternion represents the same rotation with an inverted axis.
    fn angle(&self) -> f64 {
        self.0.angle()
    }

    /// Returns the normalized axis of rotation represented by the UnitQuaternion.
    ///
    /// Axis-angle representation is a way to describe the orientation of an object
    /// in 3D space. It consists of a unit vector (the axis) and an angle. The object
    /// is rotated around the axis by the specified angle. The axis vector is
    /// normalized to ensure that its magnitude is 1.
    ///
    /// Note: The returned axis might be the negation of the actual axis if the
    /// quaternion represents the same rotation with an inverted angle.
    ///
    /// Note: The axis might be None or undefined in the case of a zero rotation,
    /// which is when the angle of rotation is exactly 0 or a multiple of 2π.
    fn axis(&self) -> Option<Vector3> {
        match self.0.axis() {
            Some(a) => Some(Vector3(*a)),
            None => None,
        }
    }

    /// Returns this rotation and (roll, pitch, yaw) euler angles.
    fn euler(&self) -> (f64, f64, f64) {
        self.0.euler_angles()
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
