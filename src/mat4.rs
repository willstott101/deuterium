use crate::vec3::Vector3;
use approx::AbsDiffEq;
use nalgebra as na;
use nalgebra::SMatrix;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::PyTuple;

pub type Matrix4d = SMatrix<f64, 4, 4>;

#[pyclass]
pub struct Matrix4(pub Matrix4d);

#[pymethods]
impl Matrix4 {
    #[staticmethod]
    fn identity() -> Matrix4 {
        return Matrix4(Matrix4d::identity());
    }

    #[staticmethod]
    fn from_translation(v: &Vector3) -> Matrix4 {
        let mut m = Matrix4d::identity();
        m[(0, 3)] = v.0[0];
        m[(1, 3)] = v.0[1];
        m[(2, 3)] = v.0[2];
        return Matrix4(m);
    }

    fn __getitem__(&self, py: Python, arg: &PyAny) -> Result<Py<PyAny>, PyErr> {
        let idx: Result<(isize, isize), PyErr> = arg.extract();
        match idx {
            Err(_) => {
                let i: Result<isize, PyErr> = arg.extract();
                match i {
                    Ok(i_int) => {
                        if i_int > 3 || i_int < 0 {
                            Err(PyIndexError::new_err(i_int))?;
                        }
                        Ok(PyTuple::new(py, self.0.row(i_int as usize).iter()).into())
                    }
                    Err(e) => Err(e),
                }
            }
            Ok(pair) => {
                if pair.0 < 0 || pair.1 < 0 || pair.0 > 3 || pair.1 > 3 {
                    Err(PyIndexError::new_err(pair))
                } else {
                    Ok(self.0[(pair.0 as usize, pair.1 as usize)].to_object(py))
                }
            }
        }
    }

    fn __setitem__(&mut self, idx: (usize, usize), value: f64) -> () {
        self.0[idx] = value;
    }

    fn __richcmp__(&self, py: Python, other: &Matrix4, op: CompareOp) -> Py<PyAny> {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn approx_equals(&self, arg: &Matrix4) -> bool {
        self.0.abs_diff_eq(&arg.0, 1e-08)
    }

    fn __mul__(&self, py: Python, arg: &PyAny) -> PyResult<PyObject> {
        let matr: PyResult<PyRef<Matrix4>> = arg.extract();
        if let Ok(mat) = matr {
            return Ok(Py::new(py, Matrix4(self.0 * mat.0))?.to_object(py));
        }
        let vecr: PyResult<PyRef<Vector3>> = arg.extract();
        if let Ok(vec) = vecr {
            return Ok(Py::new(py, Vector3::from_4(&(self.0 * vec.as_4())))?.to_object(py));
        }
        Ok(py.NotImplemented())
    }

    fn __imul__(&mut self, arg: &Matrix4) -> () {
        self.0 = self.0 * arg.0;
    }

    #[staticmethod]
    fn __len__() -> usize {
        4
    }

    fn premultiply(&mut self, arg: &Matrix4) -> () {
        self.0 = arg.0 * self.0;
    }

    // fn invert(&mut self) -> PyResult<()> {
    //     // TODO: Should this Matrix be an nalgebra Projective?
    //     self.0 = self.0.pseudo_inverse(0.00001)?;
    //     Ok(());
    // }

    fn transposed(&self) -> Matrix4 {
        Matrix4(self.0.transpose())
    }

    #[getter]
    fn get_translation(&self) -> Vector3 {
        Vector3(na::Vector3::new(
            self.0[(0, 3)],
            self.0[(1, 3)],
            self.0[(2, 3)],
        ))
    }

    #[setter]
    fn set_translation(&mut self, v: PyRef<Vector3>) -> PyResult<()> {
        self.0[(0, 3)] = v.0[0];
        self.0[(1, 3)] = v.0[1];
        self.0[(2, 3)] = v.0[2];
        Ok(())
    }

    fn translate(&mut self, v: PyRef<Vector3>) -> () {
        self.0[(0, 3)] += v.0[0];
        self.0[(1, 3)] += v.0[1];
        self.0[(2, 3)] += v.0[2];
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
                self.0[(0, 0)],
                self.0[(0, 1)],
                self.0[(0, 2)],
                self.0[(0, 3)],
            ),
            (
                self.0[(1, 0)],
                self.0[(1, 1)],
                self.0[(1, 2)],
                self.0[(1, 3)],
            ),
            (
                self.0[(2, 0)],
                self.0[(2, 1)],
                self.0[(2, 2)],
                self.0[(2, 3)],
            ),
            (
                self.0[(3, 0)],
                self.0[(3, 1)],
                self.0[(3, 2)],
                self.0[(3, 3)],
            ),
        );
    }

    fn list(&self) -> [[f64; 4]; 4] {
        return [
            [
                self.0[(0, 0)],
                self.0[(0, 1)],
                self.0[(0, 2)],
                self.0[(0, 3)],
            ],
            [
                self.0[(1, 0)],
                self.0[(1, 1)],
                self.0[(1, 2)],
                self.0[(1, 3)],
            ],
            [
                self.0[(2, 0)],
                self.0[(2, 1)],
                self.0[(2, 2)],
                self.0[(2, 3)],
            ],
            [
                self.0[(3, 0)],
                self.0[(3, 1)],
                self.0[(3, 2)],
                self.0[(3, 3)],
            ],
        ];
    }

    fn __repr__(&self) -> String {
        use std::cmp::max;

        let mut s00 = format!("{:?}", self.0[(0, 0)]);
        let mut s01 = format!("{:?}", self.0[(0, 1)]);
        let mut s02 = format!("{:?}", self.0[(0, 2)]);
        let mut s03 = format!("{:?}", self.0[(0, 3)]);
        let mut s10 = format!("{:?}", self.0[(1, 0)]);
        let mut s11 = format!("{:?}", self.0[(1, 1)]);
        let mut s12 = format!("{:?}", self.0[(1, 2)]);
        let mut s13 = format!("{:?}", self.0[(1, 3)]);
        let mut s20 = format!("{:?}", self.0[(2, 0)]);
        let mut s21 = format!("{:?}", self.0[(2, 1)]);
        let mut s22 = format!("{:?}", self.0[(2, 2)]);
        let mut s23 = format!("{:?}", self.0[(2, 3)]);
        let mut s30 = format!("{:?}", self.0[(3, 0)]);
        let mut s31 = format!("{:?}", self.0[(3, 1)]);
        let mut s32 = format!("{:?}", self.0[(3, 2)]);
        let mut s33 = format!("{:?}", self.0[(3, 3)]);

        let len_col_0 = max(s00.len(), max(s10.len(), max(s20.len(), s30.len())));
        let len_col_1 = max(s01.len(), max(s11.len(), max(s21.len(), s31.len())));
        let len_col_2 = max(s02.len(), max(s12.len(), max(s22.len(), s32.len())));
        let len_col_3 = max(s03.len(), max(s13.len(), max(s23.len(), s33.len())));

        s00 = format!("{:width$}", s00, width = len_col_0);
        s01 = format!("{:width$}", s01, width = len_col_1);
        s02 = format!("{:width$}", s02, width = len_col_2);
        s03 = format!("{:width$}", s03, width = len_col_3);
        s10 = format!("{:width$}", s10, width = len_col_0);
        s11 = format!("{:width$}", s11, width = len_col_1);
        s12 = format!("{:width$}", s12, width = len_col_2);
        s13 = format!("{:width$}", s13, width = len_col_3);
        s20 = format!("{:width$}", s20, width = len_col_0);
        s21 = format!("{:width$}", s21, width = len_col_1);
        s22 = format!("{:width$}", s22, width = len_col_2);
        s23 = format!("{:width$}", s23, width = len_col_3);
        s30 = format!("{:width$}", s30, width = len_col_0);
        s31 = format!("{:width$}", s31, width = len_col_1);
        s32 = format!("{:width$}", s32, width = len_col_2);
        s33 = format!("{:width$}", s33, width = len_col_3);

        format!("Matrix4<{}, {}, {}, {},\n        {}, {}, {}, {},\n        {}, {}, {}, {},\n        {}, {}, {}, {}>", s00,
s01,
s02,
s03,
s10,
s11,
s12,
s13,
s20,
s21,
s22,
s23,
s30,
s31,
s32,
s33)
    }
}
