use approx::AbsDiffEq;
use nalgebra::SMatrix;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::PyTuple;

type Matrix4d = SMatrix<f64, 4, 4>;

#[pyclass]
pub struct Matrix4(pub Matrix4d);

#[pymethods]
impl Matrix4 {
    #[staticmethod]
    fn identity() -> Matrix4 {
        return Matrix4(Matrix4d::identity());
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

    fn __mul__(&self, arg: &Matrix4) -> Matrix4 {
        Matrix4(self.0 * arg.0)
    }

    fn __imul__(&mut self, arg: &Matrix4) -> () {
        self.0 = self.0 * arg.0;
    }

    fn __len__(&self) -> usize {
        4
    }

    fn premultiply(&mut self, arg: &Matrix4) -> () {
        self.0 = arg.0 * self.0;
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
}
