extern crate blake2;

use pyo3::exceptions::ValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use blake2::TFCompressArgs;

/// Convenience function for building python value errors.
fn value_error<V>(msg: String) -> PyResult<V> {
    Err(ValueError::py_err(msg))
}

/// Extract `blake2b_compress` parameters from a test vector represented by a byte string.
#[pyfunction]
fn extract_blake2b_parameters(input: Vec<u8>) -> PyResult<TFCompressArgs> {
    let result = blake2::extract_blake2b_parameters(&input);

    match result {
        Err(msg) => Err(PyErr::new::<ValueError, _>(msg)),
        Ok(args) => Ok(args),
    }
}

/// Calculate a blake2b hash for the given message block.
#[pyfunction]
fn blake2b_compress(
    num_rounds: usize,
    h_starting_state: Vec<u64>,
    block: Vec<u8>,
    t_offset_counters: Vec<u64>,
    final_block_flag: bool,
) -> PyResult<Vec<u8>> {
    if h_starting_state.len() != 8 {
        return value_error(format!(
            "starting state vector must have length 8, got: {}",
            h_starting_state.len(),
        ));
    }
    if block.len() != 128 {
        return value_error(format!(
            "block vector must have length 128, got: {}",
            block.len(),
        ));
    }
    if t_offset_counters.len() != 2 {
        return value_error(format!(
            "offset counters vector must have length 2, got: {}",
            t_offset_counters.len(),
        ));
    }

    let result = blake2::blake2b_compress(
        num_rounds,
        &h_starting_state,
        &block,
        &t_offset_counters,
        final_block_flag,
    );

    Ok(result.to_vec())
}

#[pymodule]
fn blake2_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(extract_blake2b_parameters))?;
    m.add_wrapped(wrap_pyfunction!(blake2b_compress))?;

    Ok(())
}
