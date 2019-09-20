from hypothesis import (
    given,
    settings,
    strategies as st,
)

import blake2
from . import reference_impl

u8 = st.integers(min_value=0, max_value=2 ** 8 - 1)
u64 = st.integers(min_value=0, max_value=2 ** 32 - 1)

rounds = u8
starting_states = st.lists(u64, min_size=8, max_size=8)
blocks = st.binary(min_size=128, max_size=128)
offset_counters = st.lists(u64, min_size=2, max_size=2)
final_block_flags = st.booleans()


@settings(
    max_examples=2,
)
@given(
    rounds,
    starting_states,
    blocks,
    offset_counters,
    final_block_flags,
)
def test_equivalence_with_python_impl(
        rounds,
        starting_state,
        block,
        offset_counter,
        final_block_flag,
):
    python_result = reference_impl.blake2b_compress(
        rounds,
        starting_state,
        block,
        offset_counter,
        final_block_flag,
    )
    rust_result = blake2.blake2b_compress(
        rounds,
        starting_state,
        block,
        offset_counter,
        final_block_flag,
    )
    assert python_result == rust_result
