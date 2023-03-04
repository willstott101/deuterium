use nalgebra::SVector;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::pyclass::CompareOp;
use approx::AbsDiffEq;

type Vector3d = SVector<f64, 3>;

#[pyclass]
pub struct Vector3 {
    v: Vector3d,
}

#[pymethods]
impl Vector3 {
    #[staticmethod]
    fn zero() -> Vector3 {
        return Vector3 {
            v: Vector3d::new(0.0, 0.0, 0.0),
        };
    }

    #[staticmethod]
    fn x() -> Vector3 {
        return Vector3 {
            v: Vector3d::new(1.0, 0.0, 0.0),
        };
    }

    #[staticmethod]
    fn y() -> Vector3 {
        return Vector3 {
            v: Vector3d::new(0.0, 1.0, 0.0),
        };
    }

    #[staticmethod]
    fn z() -> Vector3 {
        return Vector3 {
            v: Vector3d::new(0.0, 0.0, 1.0),
        };
    }

    fn __getitem__(&self, py: Python, idx: usize) -> Result<Py<PyAny>, PyErr> {
        match self.v.get(idx) {
            Some(v) => Ok(v.into_py(py)),
            None => Err(PyIndexError::new_err(idx)),
        }
    }

    fn __setitem__(&mut self, idx: i32, value: f64) -> Result<(), PyErr>  {
        if idx < 0 && idx > -4 {
            idx = 3 - idx;
        } else if idx < 0 || idx > 2 {
            return Err(PyIndexError::new_err(idx))
        }
        self.v[idx] = value;
        Ok(())
    }

    fn __richcmp__(&self, py: Python, other: &Vector3, op: CompareOp) -> Py<PyAny> {
        match op {
          CompareOp::Eq => (self.v == other.v).into_py(py),
          CompareOp::Ne => (self.v != other.v).into_py(py),
          _ => py.NotImplemented(),
        }
    }

    fn approx_equals(&self, arg: &Vector3) -> bool {
        self.v.abs_diff_eq(&arg.v, 1e-08)
    }

    fn __mul__(&self, arg: &Matrix4) -> Matrix4 {
        Matrix4 { m: self.m * arg.m }
    }

    fn __imul__(&mut self, arg: &Matrix4) -> () {
        self.m = self.m * arg.m;
    }

    fn premultiply(&mut self, arg: &Matrix4) -> () {
        self.m = arg.m * self.m;
    }

    fn tuple(
        &self,
    ) -> (
        (f64, f64, f64, f64),
        (f64, f64, f64, f64),
        (f64, f64, f64, f64),
        (f64, f64, f64, f64),
    ) {
        return (
            (
                self.m[(0, 0)],
                self.m[(0, 1)],
                self.m[(0, 2)],
                self.m[(0, 3)],
            ),
            (
                self.m[(1, 0)],
                self.m[(1, 1)],
                self.m[(1, 2)],
                self.m[(1, 3)],
            ),
            (
                self.m[(2, 0)],
                self.m[(2, 1)],
                self.m[(2, 2)],
                self.m[(2, 3)],
            ),
            (
                self.m[(3, 0)],
                self.m[(3, 1)],
                self.m[(3, 2)],
                self.m[(3, 3)],
            ),
        );
    }

    fn list(&self) -> [[f64; 4]; 4] {
        return [
            [
                self.m[(0, 0)],
                self.m[(0, 1)],
                self.m[(0, 2)],
                self.m[(0, 3)],
            ],
            [
                self.m[(1, 0)],
                self.m[(1, 1)],
                self.m[(1, 2)],
                self.m[(1, 3)],
            ],
            [
                self.m[(2, 0)],
                self.m[(2, 1)],
                self.m[(2, 2)],
                self.m[(2, 3)],
            ],
            [
                self.m[(3, 0)],
                self.m[(3, 1)],
                self.m[(3, 2)],
                self.m[(3, 3)],
            ],
        ];
    }
}