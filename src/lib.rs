use ndarray::{Array, Array1, ArrayView1, ArrayView2};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArrayDyn};
use pyo3::prelude::*;
use rayon::prelude::*;

const G: f64 = 6.673e-11;

fn point_gz(
    easting: f64,
    northing: f64,
    upward: f64,
    easting_p: f64,
    northing_p: f64,
    upward_p: f64,
    mass: f64,
) -> f64 {
    // Compute gz field of a single point source on single observation point
    let distance = ((easting - easting_p).powi(2)
        + (northing - northing_p).powi(2)
        + (upward - upward_p).powi(2))
    .sqrt();
    G * mass * (upward - upward_p) / distance.powi(3)
}

fn _points_gz(
    easting: ArrayView1<f64>,
    northing: ArrayView1<f64>,
    upward: ArrayView1<f64>,
    points: ArrayView2<f64>,
    masses: ArrayView1<f64>,
) -> Array1<f64> {
    // Compute gz generated by point sources on observation points
    let n_coords = easting.len();
    let n_points = masses.len();
    // Allocate results array
    let result: Vec<f64> = (0..n_coords)
        .into_par_iter()
        .map(|i| {
            let mut r = 0.0;
            for j in 0..n_points {
                r += point_gz(
                    easting[i],
                    northing[i],
                    upward[i],
                    points[[0, j]],
                    points[[1, j]],
                    points[[2, j]],
                    masses[j],
                )
            }
            r
        })
        .collect();
    Array::from_vec(result)
}

#[pyfunction]
fn points_gz<'py>(
    py: Python<'py>,
    easting: PyReadonlyArray1<f64>,
    northing: PyReadonlyArray1<f64>,
    upward: PyReadonlyArray1<f64>,
    points: PyReadonlyArray2<f64>,
    masses: PyReadonlyArray1<f64>,
) -> &'py PyArray1<f64> {
    // Compute g_z filed of point sources
    let easting = easting.as_array();
    let northing = northing.as_array();
    let upward = upward.as_array();
    let points = points.as_array();
    let masses = masses.as_array();
    let result = _points_gz(easting, northing, upward, points, masses);
    result.into_pyarray(py)
}

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
    m.add_function(wrap_pyfunction!(points_gz, m)?)?;

    Ok(())
}
