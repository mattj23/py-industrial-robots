use crate::mesh::mesh_to_numpy;
use crate::utility::Frame3;
use numpy::{IntoPyArray, PyArrayDyn, PyUntypedArrayMethods};
use numpy::ndarray::ArrayD;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

enum CrxType {
    Crx5ia,
    Crx10ia,
}

#[pyclass]
pub struct Crx {
    inner: industrial_robots::fanuc::Crx,
    crx_type: CrxType,
}

impl Crx {
    pub fn get_inner(&self) -> &industrial_robots::fanuc::Crx {
        &self.inner
    }
}

#[pymethods]
impl Crx {
    fn get_meshes<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Vec<(Bound<'py, PyArrayDyn<f64>>, Bound<'py, PyArrayDyn<u32>>)>> {
        let mut result = Vec::new();
        let meshes = match self.crx_type {
            CrxType::Crx5ia => Ok(industrial_robots::fanuc::crx5ia_mesh()),
            CrxType::Crx10ia => Ok(industrial_robots::fanuc::crx10ia_mesh()),
            _ => Err(PyValueError::new_err("Unknown CRX type")),
        }?;

        for (vertices, faces) in meshes {
            result.push(mesh_to_numpy(py, vertices, faces)?);
        }

        Ok(result)
    }

    #[staticmethod]
    fn new_5ia() -> Self {
        Self {
            inner: industrial_robots::fanuc::Crx::new_5ia(),
            crx_type: CrxType::Crx5ia,
        }
    }

    #[staticmethod]
    fn new_10ia() -> Self {
        Self {
            inner: industrial_robots::fanuc::Crx::new_10ia(),
            crx_type: CrxType::Crx10ia,
        }
    }
    
    // fn o4_circle<'py>(&self, py: Python<'py>, end_frame: &Frame3) -> Bound<'py, PyArrayDyn<f64>> {
    //     let points = self.inner.o4_circle(end_frame.get_inner());
    //
    //     let mut result = ArrayD::zeros(vec![points.len(), 3]);
    //     for (i, point) in points.iter().enumerate() {
    //         result[[i, 0]] = point.x;
    //         result[[i, 1]] = point.y;
    //         result[[i, 2]] = point.z;
    //     }
    //
    //     result.into_pyarray(py)
    // }
    
    // fn ik(&self, frame: &Frame3) {
    //     self.inner.ik(&frame.get_inner())
    // }

    fn fk(&self, joints: Vec<f64>) -> PyResult<Frame3> {
        if joints.len() != 6 {
            return Err(PyValueError::new_err("Expected 6 joint angles"));
        }
        let joints: [f64; 6] = [
            joints[0], joints[1], joints[2], joints[3], joints[4], joints[5],
        ];
        let frame = self.inner.fk(&joints);
        Ok(Frame3::from_inner(frame))
    }

    fn fk_all(
        &self,
        joints: Vec<f64>,
    ) -> PyResult<Vec<Frame3>> {
        if joints.len() != 6 {
            return Err(PyValueError::new_err("Expected 6 joint angles"));
        }
        let joints: [f64; 6] = [
            joints[0], joints[1], joints[2], joints[3], joints[4], joints[5],
        ];
        let results = self.inner.fk_all(&joints);

        // Convert the results to a Vec<Frame3>
        Ok(results.map(|f| Frame3::from_inner(f)).to_vec())
    }
}
