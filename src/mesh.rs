use industrial_robots::nalgebra::{try_convert, Matrix4};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn, PyUntypedArrayMethods};
use numpy::ndarray::ArrayD;
use pyo3::exceptions::PyValueError;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use crate::conversions::{array_to_faces, array_to_points3, array_to_vectors3};
use industrial_robots::micro_mesh::{mesh_to_bytes, bytes_to_mesh};
use crate::utility::Frame3;

#[pyfunction]
pub fn micro_serialize<'py>(
    vertices: PyReadonlyArrayDyn<'py, f64>,
    faces: PyReadonlyArrayDyn<'py, u32>,
) -> PyResult<Vec<u8>> {
    let vertices = array_to_points3(&vertices.as_array())?;
    let faces = array_to_faces(&faces.as_array())?;

    mesh_to_bytes(&vertices, &faces)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

pub fn mesh_to_numpy<'py>(
    py: Python<'py>,
    vertices: Vec<industrial_robots::Point3>,
    faces: Vec<[u32; 3]>,
) -> PyResult<(Bound<'py, PyArrayDyn<f64>>, Bound<'py, PyArrayDyn<u32>>)> {
    let mut vertices_array = ArrayD::zeros(vec![vertices.len(), 3]);
    for (i, vertex) in vertices.iter().enumerate() {
        vertices_array[[i, 0]] = vertex.x;
        vertices_array[[i, 1]] = vertex.y;
        vertices_array[[i, 2]] = vertex.z;
    }

    let mut faces_array = ArrayD::zeros(vec![faces.len(), 3]);
    for (i, face) in faces.iter().enumerate() {
        faces_array[[i, 0]] = face[0];
        faces_array[[i, 1]] = face[1];
        faces_array[[i, 2]] = face[2];
    }

    Ok((vertices_array.into_pyarray(py), faces_array.into_pyarray(py)))
}

#[pyfunction]
pub fn micro_deserialize<'py>(
    py: Python<'py>,
    data: Vec<u8>,
) -> PyResult<(Bound<'py, PyArrayDyn<f64>>, Bound<'py, PyArrayDyn<u32>>)> {
    let (vertices, faces) = bytes_to_mesh(&data)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    // let mut vertices_array = ArrayD::zeros(vec![vertices.len(), 3]);
    // for (i, vertex) in vertices.iter().enumerate() {
    //     vertices_array[[i, 0]] = vertex.x;
    //     vertices_array[[i, 1]] = vertex.y;
    //     vertices_array[[i, 2]] = vertex.z;
    // }
    //
    // let mut faces_array = ArrayD::zeros(vec![faces.len(), 3]);
    // for (i, face) in faces.iter().enumerate() {
    //     faces_array[[i, 0]] = face[0];
    //     faces_array[[i, 1]] = face[1];
    //     faces_array[[i, 2]] = face[2];
    // }
    //
    // Ok((vertices_array.into_pyarray(py), faces_array.into_pyarray(py)))

    mesh_to_numpy(py, vertices, faces)
}