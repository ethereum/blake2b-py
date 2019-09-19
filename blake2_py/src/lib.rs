extern crate blake2;

use pyo3::exceptions::ValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::wrap_pyfunction;

use blake2::TFCompressArgs;

/// Convenience function for building python value errors.
fn value_error<V>(msg: String) -> PyResult<V> {
    Err(ValueError::py_err(msg))
}

/// extract_blake2b_parameters(input)
/// --
///
/// Extract parameters for the ``blake2b_compress`` function from a test
/// vector represented by a byte string.
///
/// Parameters
/// ----------
/// input : bytes, List[int]
///     A vector of 213 bytes representing the test vector.
///
/// Returns
/// ----------
/// out : (int, List[int], bytes, List[int], bool)
///     A tuple of parameters to pass to the ``blake2b_compress`` function.
#[pyfunction]
fn extract_blake2b_parameters(input: Vec<u8>) -> PyResult<TFCompressArgs> {
    let result = blake2::extract_blake2b_parameters(&input);

    match result {
        Err(msg) => Err(PyErr::new::<ValueError, _>(msg)),
        Ok(args) => Ok(args),
    }
}

/// blake2b_compress(num_rounds, h_starting_state, block, t_offset_counters,
///     final_block_flag)
/// --
///
/// Calculates a blake2b hash for the given message block.
///
/// Parameters
/// ----------
/// num_rounds : int
///     The number of rounds of mixing to occur during hashing.
/// h_starting_state : List[int]
///     A vector of 8 64-bit integers representing the starting state of the
///     hash function.
/// block : bytes, List[int]
///     A vector of 128 bytes representing the message block to be hashed.
/// t_offset_counters : List[int]
///     A vector of 2 64-bit integers representing the message byte offset at
///     the end of the current block.
/// final_block_flag : bool
///     A flag indicating the final block of the message.
///
/// Returns
/// -------
/// out : bytes
///     A vector of 64 bytes representing the blake2b hash of the input data.
#[pyfunction]
fn blake2b_compress(
    py: Python,
    num_rounds: usize,
    h_starting_state: Vec<u64>,
    block: Vec<u8>,
    t_offset_counters: Vec<u64>,
    final_block_flag: bool,
) -> PyResult<PyObject> {
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

    Ok(PyBytes::new(py, &result).into())
}

/// Functions for calculating blake2b hashes.
#[pymodule]
fn blake2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(extract_blake2b_parameters))?;
    m.add_wrapped(wrap_pyfunction!(blake2b_compress))?;

    Ok(())
}
