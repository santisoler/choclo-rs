use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1, PyReadonlyArrayDyn};
use pyo3::prelude::*;

// const G: f64 = 1e-6;

// fn point_gz(
//     easting: f64,
//     northing: f64,
//     upward: f64,
//     easting_p: f64,
//     northing_p: f64,
//     upward_p: f64,
//     mass: f64,
// ) -> f64 {
//     // Compute gz field of a single point source on single observation point
//     let distance = (easting - easting_p).powi(2)
//         + (northing - northing_p).powi(2)
//         + (upward - upward_p).powi(2);
//     G * mass * (upward - upward_p) / distance
// }
//
// fn points_gz(
//     easting: PyReadOnlyArray1<f64>,
//     northing: PyReadOnlyArray1<f64>,
//     upward: PyReadOnlyArray1<f64>,
//     points: PyReadOnlyArray1<f64>,
//     masses: PyReadOnlyArray2<f64>,
// ) -> ArrayD<f64> {
//     // compute g_z filed of point sources
//     let easting = easting.as_array();
//     let northing = northing.as_array();
//     let upward = upward.as_array();
//     let n_coords = &easting.len();
//     let n_points = &points.len();
//     &easting + &northing
// }

#[pyfunction]
fn max<'py>(_py: Python<'py>, x: PyReadonlyArrayDyn<f64>) -> f64 {
    // Compute max value of an array
    let array = x.as_array();
    let mut max = &array[0];
    for element in array.iter() {
        if element > max {
            max = element;
        }
    }
    *max
}

#[pyfunction]
fn increment_by_one<'py>(py: Python<'py>, x: PyReadonlyArray1<f64>) -> &'py PyArray1<f64> {
    // Return array whose elements had been incremented by one
    let x = x.as_array();
    let result = x.map(|i| i + 1.0);
    result.into_pyarray(py)
}

#[pyfunction]
fn add<'py>(
    _py: Python<'py>,
    x: PyReadonlyArray1<f64>,
    y: PyReadonlyArray1<f64>,
    result: &PyArray1<f64>,
) {
    // Add the element-wise sum of two arrays into a running result array
    let x = x.as_array();
    let y = y.as_array();
    let mut result = unsafe { result.as_array_mut() };
    for i in 0..x.len() {
        result[i] += x[i] + y[i]
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn choclors(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(max, m)?)?;
    m.add_function(wrap_pyfunction!(increment_by_one, m)?)?;

    Ok(())
}
