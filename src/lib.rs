use nalgebra::SMatrix;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

type Matrix4d = SMatrix<f64, 4, 4>;

#[pyclass]
struct Matrix4 {
    m: Matrix4d,
}

#[pymethods]
impl Matrix4 {
    #[staticmethod]
    fn identity() -> Matrix4 {
        return Matrix4 {
            m: Matrix4d::identity(),
        };
    }

    fn __getitem__(&self, py: Python, arg: &PyAny) -> Result<Py<PyAny>, PyErr> {
        let idx: Result<(usize, usize), PyErr> = arg.extract();
        match idx {
            Err(_) => {
                let i: Result<usize, PyErr> = arg.extract();
                match i {
                    Ok(i_int) => Ok(PyTuple::new(py, self.m.row(i_int).iter()).into()),
                    Err(e) => Err(e),
                }
            }
            Ok(pair) => Ok(self.m[pair].to_object(py)),
        }
    }

    fn __setitem__(&mut self, idx: (usize, usize), value: f64) -> () {
        self.m[idx] = value;
    }

    fn __mul__(&self, arg: &Matrix4) -> Matrix4 {
        Matrix4 { m: self.m * arg.m }
    }

    fn __imul__(&mut self, arg: &Matrix4) -> () {
        self.m = self.m * arg.m;
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

#[pymodule]
/// A Python module implemented in Rust.
fn deuterium(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Matrix4>()?;
    Ok(())
}
