mod utility;

use pyo3::prelude::*;
use industrial_robots::{Isometry3, Matrix4, Vector3, try_convert};
use pyo3::types::PyList;
use numpy::ndarray::prelude::*;
use utility::{fanuc_with_joints, to_mat4, from_mat4};


#[pyfunction]
fn fanuc_fk(joints: Vec<f64>) -> PyResult<Vec<f64>> {
    let mut robot = fanuc_with_joints(joints);
    let pose = robot.end_pose();
    Ok(from_mat4(pose.to_matrix()))
}

#[pyfunction]
fn fanuc_ik(target_pose: Vec<f64>, starting_joints: Vec<f64>) -> PyResult<Vec<f64>> {
    let mut robot = fanuc_with_joints(starting_joints);

    let pose = to_mat4(target_pose);
    let target: Isometry3<f64> = try_convert(pose).unwrap();
    println!("target: {:?}", target);
    println!("target: {:?}", target);

    if let Some(joints) = robot.find_joints(&target) {
        let py_list = joints.iter().map(|x| x.to_owned()).collect::<Vec<f64>>();
        Ok(py_list)
    }
    else {
        Err(pyo3::exceptions::PyValueError::new_err("No solution found"))
    }
}


#[pymodule]
fn industrial_robots_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fanuc_fk, m)?)?;
    m.add_function(wrap_pyfunction!(fanuc_ik, m)?)?;
    Ok(())
}
