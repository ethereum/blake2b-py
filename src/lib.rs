extern crate pyo3;

use std::convert::TryInto;

use pyo3::prelude::*;

const SIGMA_SCHEDULE_LEN: usize = 10;
const SIGMA_SCHEDULE: [[usize; 16]; SIGMA_SCHEDULE_LEN] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
];

const WORDBITS: u8 = 64;
const MASKBITS: u64 = u64::max_value();

const IV: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

const ROT1: u8 = 32;
const ROT2: u8 = 24;
const ROT3: u8 = 16;
const ROT4: u8 = 63;

const WB_ROT1: u8 = WORDBITS - ROT1;
const WB_ROT2: u8 = WORDBITS - ROT2;
const WB_ROT3: u8 = WORDBITS - ROT3;
const WB_ROT4: u8 = WORDBITS - ROT4;

#[inline]
fn block_to_16_le_words(input: &[u8]) -> [u64; 16] {
    [
        u64::from_le_bytes((&input[..8]).try_into().unwrap()),
        u64::from_le_bytes((&input[8..16]).try_into().unwrap()),
        u64::from_le_bytes((&input[16..24]).try_into().unwrap()),
        u64::from_le_bytes((&input[24..32]).try_into().unwrap()),
        u64::from_le_bytes((&input[32..40]).try_into().unwrap()),
        u64::from_le_bytes((&input[40..48]).try_into().unwrap()),
        u64::from_le_bytes((&input[48..56]).try_into().unwrap()),
        u64::from_le_bytes((&input[56..64]).try_into().unwrap()),
        u64::from_le_bytes((&input[64..72]).try_into().unwrap()),
        u64::from_le_bytes((&input[72..80]).try_into().unwrap()),
        u64::from_le_bytes((&input[80..88]).try_into().unwrap()),
        u64::from_le_bytes((&input[88..96]).try_into().unwrap()),
        u64::from_le_bytes((&input[96..104]).try_into().unwrap()),
        u64::from_le_bytes((&input[104..112]).try_into().unwrap()),
        u64::from_le_bytes((&input[112..120]).try_into().unwrap()),
        u64::from_le_bytes((&input[120..128]).try_into().unwrap()),
    ]
}

pub fn blake2b_compress(
    num_rounds: usize,
    h_starting_state: (u64, u64, u64, u64, u64, u64, u64, u64),
    block: &[u8],
    t_offset_counters: (u64, u64),
    final_block_flag: bool,
) -> [u8; 64] {
    let m = block_to_16_le_words(block);

    let mut v = [
        h_starting_state.0,          // 0
        h_starting_state.1,          // 1
        h_starting_state.2,          // 2
        h_starting_state.3,          // 3
        h_starting_state.4,          // 4
        h_starting_state.5,          // 5
        h_starting_state.6,          // 6
        h_starting_state.7,          // 7
        IV[0],                       // 8
        IV[1],                       // 9
        IV[2],                       // 10
        IV[3],                       // 11
        t_offset_counters.0 ^ IV[4], // 12
        t_offset_counters.1 ^ IV[5], // 13
        if final_block_flag {
            MASKBITS ^ IV[6]
        } else {
            IV[6]
        }, // 14
        IV[7],                       // 15
    ];

    macro_rules! blake2b_G {
        ($v:ident, $a:expr, $b:expr, $c:expr, $d:expr, $msri2:ident, $msri21:ident) => {{
            let mut va = $v[$a];
            let mut vb = $v[$b];
            let mut vc = $v[$c];
            let mut vd = $v[$d];
            va = (va + vb + $msri2) & MASKBITS;
            let mut w = vd ^ va;
            vd = (w >> ROT1) | (w << (WB_ROT1)) & MASKBITS;
            vc = (vc + vd) & MASKBITS;
            w = vb ^ vc;
            vb = (w >> ROT2) | (w << (WB_ROT2)) & MASKBITS;
            va = (va + vb + $msri21) & MASKBITS;
            w = vd ^ va;
            vd = (w >> ROT3) | (w << (WB_ROT3)) & MASKBITS;
            vc = (vc + vd) & MASKBITS;
            w = vb ^ vc;
            vb = (w >> ROT4) | (w << (WB_ROT4)) & MASKBITS;
            $v[$a] = va;
            $v[$b] = vb;
            $v[$c] = vc;
            $v[$d] = vd;
        }};
    }

    for r in 0..num_rounds {
        let sr = &SIGMA_SCHEDULE[r % SIGMA_SCHEDULE_LEN];
        let msri2 = m[sr[0]];
        let msri21 = m[sr[1]];
        blake2b_G!(v, 0, 4, 8, 12, msri2, msri21);
        let msri2 = m[sr[2]];
        let msri21 = m[sr[3]];
        blake2b_G!(v, 1, 5, 9, 13, msri2, msri21);
        let msri2 = m[sr[4]];
        let msri21 = m[sr[5]];
        blake2b_G!(v, 2, 6, 10, 14, msri2, msri21);
        let msri2 = m[sr[6]];
        let msri21 = m[sr[7]];
        blake2b_G!(v, 3, 7, 11, 15, msri2, msri21);
        let msri2 = m[sr[8]];
        let msri21 = m[sr[9]];
        blake2b_G!(v, 0, 5, 10, 15, msri2, msri21);
        let msri2 = m[sr[10]];
        let msri21 = m[sr[11]];
        blake2b_G!(v, 1, 6, 11, 12, msri2, msri21);
        let msri2 = m[sr[12]];
        let msri21 = m[sr[13]];
        blake2b_G!(v, 2, 7, 8, 13, msri2, msri21);
        let msri2 = m[sr[14]];
        let msri21 = m[sr[15]];
        blake2b_G!(v, 3, 4, 9, 14, msri2, msri21);
    }

    let result_message_word_bytes = [
        (h_starting_state.0 ^ v[0] ^ v[8]).to_le_bytes(),
        (h_starting_state.1 ^ v[1] ^ v[9]).to_le_bytes(),
        (h_starting_state.2 ^ v[2] ^ v[10]).to_le_bytes(),
        (h_starting_state.3 ^ v[3] ^ v[11]).to_le_bytes(),
        (h_starting_state.4 ^ v[4] ^ v[12]).to_le_bytes(),
        (h_starting_state.5 ^ v[5] ^ v[13]).to_le_bytes(),
        (h_starting_state.6 ^ v[6] ^ v[14]).to_le_bytes(),
        (h_starting_state.7 ^ v[7] ^ v[15]).to_le_bytes(),
    ];
    let mut result = [0u8; 64];
    for (i, word_bytes) in result_message_word_bytes.into_iter().enumerate() {
        for (j, x) in word_bytes.into_iter().enumerate() {
            result[i * 8 + j] = *x;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    extern crate hex;

    use super::*;

    type TFCompressArgs = (usize, Vec<u64>, Vec<u8>, Vec<u64>, bool);

    #[inline]
    fn u32_from_be(input: &[u8]) -> u32 {
        u32::from_be_bytes(input.try_into().unwrap())
    }

    #[inline]
    fn u64_from_le(input: &[u8]) -> u64 {
        u64::from_le_bytes(input.try_into().unwrap())
    }

    fn extract_blake2b_parameters(input: &[u8]) -> Result<TFCompressArgs, String> {
        if input.len() != 213 {
            Err(format!(
                "input length for Blake2 F precompile should be exactly 213 bytes, got: {}",
                input.len()
            ))
        } else {
            Ok((
                u32_from_be(&input[..4]) as usize,
                vec![
                    u64_from_le(&input[4..12]),
                    u64_from_le(&input[12..20]),
                    u64_from_le(&input[20..28]),
                    u64_from_le(&input[28..36]),
                    u64_from_le(&input[36..44]),
                    u64_from_le(&input[44..52]),
                    u64_from_le(&input[52..60]),
                    u64_from_le(&input[60..68]),
                ],
                input[68..196].to_vec(),
                vec![u64_from_le(&input[196..204]), u64_from_le(&input[204..212])],
                input[212] > 0,
            ))
        }
    }

    struct PyBlake2<'a> {
        py: Python<'a>,
        module: &'a PyModule,
    }

    impl<'a> PyBlake2<'a> {
        fn new(py: Python<'a>) -> Self {
            let result = PyModule::from_code(py, include_str!("blake2.py"), "blake2.py", "blake2");

            match result {
                Err(e) => {
                    e.print(py);
                    panic!("Python exception when loading blake2.py");
                }
                Ok(module) => Self { py, module },
            }
        }

        fn extract_blake2b_parameters(&self, input_bytes: &[u8]) -> PyResult<TFCompressArgs> {
            use pyo3::types::PyBytes;

            let input_bytes = PyBytes::new(self.py, input_bytes);

            let py_val = self
                .module
                .call("extract_blake2b_parameters", (input_bytes,), None)?;

            py_val.extract()
        }

        fn blake2b_compress(
            &self,
            rounds: usize,
            h_starting_state: &[u64],
            block: &[u8],
            t_offset_counters: &[u64],
            final_block_flag: bool,
        ) -> PyResult<Vec<u8>> {
            use pyo3::types::PyTuple;

            let rounds = rounds.to_object(self.py);
            let h_starting_state = PyTuple::new(self.py, h_starting_state);
            let block = block.to_object(self.py);
            let t_offset_counters = PyTuple::new(self.py, t_offset_counters);
            let final_block_flag = final_block_flag.to_object(self.py);

            let py_val = self.module.call(
                "blake2b_compress",
                (
                    rounds,
                    h_starting_state,
                    block,
                    t_offset_counters,
                    final_block_flag,
                ),
                None,
            )?;

            py_val.extract()
        }
    }

    const FAST_EXAMPLES: &[(&str, &str)] = &[
        (
            "0000000048c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
            "08c9bcf367e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d282e6ad7f520e511f6c3e2b8c68059b9442be0454267ce079217e1319cde05b",
        ),
        (
            "0000000c48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
            "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923",
        ),
        (
            "0000000c48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000",
            "75ab69d3190a562c51aef8d88f1c2775876944407270c42c9844252c26d2875298743e7f6d5ea2f2d3e8d226039cd31b4e426ac4f2d3d666a610c2116fde4735",
        ),
        (
            "0000000148c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
            "b63a380cb2897d521994a85234ee2c181b5f844d2c624c002677e9703449d2fba551b3a8333bcdf5f2f7e08993d53923de3d64fcc68c034e717b9293fed7a421",
        ),
    ];

    const ERROR_EXAMPLES: &[&str] = &[
        "",
        "00000c48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
        "000000000c48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
        "0000000c48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000002",
    ];

    const SLOW_EXAMPLES: &[(&str, &str)] = &[
        (
            "001e848048c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
            "fc59093aafa9ab43daae0e914c57635c5402d8e3d2130eb9b3cc181de7f0ecf9b22bf99a7815ce16419e200e01846e6b5df8cc7703041bbceb571de6631d2615",
        ),
        //(  // This example takes a couple of minute to run
        //    "ffffffff48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
        //    "fc59093aafa9ab43daae0e914c57635c5402d8e3d2130eb9b3cc181de7f0ecf9b22bf99a7815ce16419e200e01846e6b5df8cc7703041bbceb571de6631d2615",
        //),
    ];

    #[test]
    fn test_py_blake2b_compress_success() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let blake2 = PyBlake2::new(py);

        for (inp, expected) in FAST_EXAMPLES {
            let input_bytes = hex::decode(inp).unwrap();

            let blake2_params = blake2.extract_blake2b_parameters(&input_bytes).unwrap();
            let (rounds, h_starting_state, block, t_offset_counters, final_block_flag) =
                blake2_params;

            let result_bytes = blake2
                .blake2b_compress(
                    rounds,
                    &h_starting_state,
                    &block,
                    &t_offset_counters,
                    final_block_flag,
                )
                .unwrap();

            assert_eq!(hex::encode(result_bytes), *expected);
        }
    }

    #[test]
    fn test_py_extract_blake2b_parameters_error() {
        use pyo3::exceptions::ValueError;

        let gil = Python::acquire_gil();
        let py = gil.python();
        let blake2 = PyBlake2::new(py);

        for inp in ERROR_EXAMPLES {
            let input_bytes = hex::decode(inp).unwrap();

            let err = blake2.extract_blake2b_parameters(&input_bytes).unwrap_err();

            assert!(err.is_instance::<ValueError>(py));
        }
    }

    #[test]
    fn test_rust_blake2b_compress_success() {
        for (inp, expected) in FAST_EXAMPLES {
            let input_bytes = hex::decode(inp).unwrap();

            let blake2_params = extract_blake2b_parameters(&input_bytes).unwrap();
            let (rounds, h_starting_state, block, t_offset_counters, final_block_flag) =
                blake2_params;

            let rust_result_bytes = blake2b_compress(
                rounds as usize,
                (
                    h_starting_state[0],
                    h_starting_state[1],
                    h_starting_state[2],
                    h_starting_state[3],
                    h_starting_state[4],
                    h_starting_state[5],
                    h_starting_state[6],
                    h_starting_state[7],
                ),
                &block,
                (t_offset_counters[0], t_offset_counters[1]),
                final_block_flag,
            )
            .to_vec();

            assert_eq!(hex::encode(rust_result_bytes), *expected);
        }
    }
}
