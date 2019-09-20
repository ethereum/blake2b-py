#![feature(test)]

mod blake2b;

use pyo3::exceptions::ValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::wrap_pyfunction;

type CompressArgs = (usize, Vec<u64>, Vec<u64>, Vec<u64>, bool);

/// decode_parameters(input)
/// --
///
/// Decode parameters for the ``compress`` function from the tightly packed
/// encoding in the byte sequence `input`.
///
/// Parameters
/// ----------
/// input : bytes, List[int]
///     A vector of 213 bytes representing the tightly encoded input.
///
/// Returns
/// ----------
/// out : (int, List[int], List[int], List[int], bool)
///     A tuple of parameters to pass to the ``compress`` function.
#[pyfunction]
fn decode_parameters(input: Vec<u8>) -> PyResult<CompressArgs> {
    let result = blake2b::decode_parameters(&input);

    match result {
        Err(msg) => Err(ValueError::py_err(msg)),
        Ok(args) => {
            let (rounds, state, block, offsets, flag) = args;
            Ok((
                rounds,
                state.to_vec(),
                block.to_vec(),
                offsets.to_vec(),
                flag,
            ))
        }
    }
}

fn checked_compress(
    rounds: usize,
    starting_state: &[u64],
    block: &[u64],
    offset_counters: &[u64],
    final_block_flag: bool,
) -> Result<[u8; 64], String> {
    if starting_state.len() != 8 {
        return Err(format!(
            "starting state vector must have length 8, got: {}",
            starting_state.len(),
        ));
    }
    if block.len() != 16 {
        return Err(format!(
            "block vector must have length 16, got: {}",
            block.len(),
        ));
    }
    if offset_counters.len() != 2 {
        return Err(format!(
            "offset counters vector must have length 2, got: {}",
            offset_counters.len(),
        ));
    }

    Ok(blake2b::F(
        rounds,
        starting_state,
        block,
        offset_counters,
        final_block_flag,
    ))
}

/// compress(rounds, starting_state, block, offset_counters, final_block_flag)
/// --
///
/// Calculates a blake2b hash for the given message block.
///
/// Parameters
/// ----------
/// rounds : int
///     The number of rounds of mixing to occur during hashing.
/// starting_state : List[int]
///     A vector of 8 64-bit integers representing the starting state of the
///     hash function.
/// block : List[int]
///     A vector of 16 64-bit integers representing the message block to be hashed.
/// offset_counters : List[int]
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
fn compress(
    py: Python,
    rounds: usize,
    starting_state: Vec<u64>,
    block: Vec<u64>,
    offset_counters: Vec<u64>,
    final_block_flag: bool,
) -> PyResult<PyObject> {
    let result = checked_compress(
        rounds,
        &starting_state,
        &block,
        &offset_counters,
        final_block_flag,
    );

    match result {
        Err(msg) => Err(ValueError::py_err(msg)),
        Ok(ok) => Ok(PyBytes::new(py, &ok).into()),
    }
}

fn _decode_and_compress(input: Vec<u8>) -> Result<[u8; 64], String> {
    let (r, h, m, t, f) = blake2b::decode_parameters(&input)?;
    checked_compress(r, &h, &m, &t, f)
}

/// decode_and_compress(input)
/// --
///
/// Calculates a blake2b hash for the tightly encoded input given in the byte
/// sequence `input`.
///
/// Parameters
/// ----------
/// input : bytes, List[int]
///     A vector of 213 bytes representing the tightly encoded input.
///
/// Returns
/// -------
/// out : bytes
///     A vector of 64 bytes representing the blake2b hash of the input data.
#[pyfunction]
fn decode_and_compress(py: Python, input: Vec<u8>) -> PyResult<PyObject> {
    let result = _decode_and_compress(input);

    match result {
        Err(msg) => Err(ValueError::py_err(msg)),
        Ok(ok) => Ok(PyBytes::new(py, &ok).into()),
    }
}

/// Functions for calculating blake2b hashes.
#[pymodule]
fn blake2b(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(decode_parameters))?;
    m.add_wrapped(wrap_pyfunction!(compress))?;
    m.add_wrapped(wrap_pyfunction!(decode_and_compress))?;

    Ok(())
}
