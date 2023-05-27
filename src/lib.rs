use pyo3::prelude::*;
use industrial_robots::robot::FanucLrMate200id;
use pyo3::types::PyList;
use numpy::ndarray::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn fanuc_fk(j1: f64, j2: f64, j3: f64, j4: f64, j5: f64, j6: f64) -> PyResult<Vec<f64>> {
    let mut robot = FanucLrMate200id::new();
    robot.set_joints(&[j1, j2, j3, j4, j5, j6]);
    let pose = robot.end_pose();
    let mat = pose.to_matrix();
    let py_list = mat.iter().map(|x| x.to_owned()).collect::<Vec<f64>>();
    Ok(py_list)
}


/// A Python module implemented in Rust.
#[pymodule]
fn industrial_robots_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fanuc_fk, m)?)?;
    Ok(())
}
