extern crate pyo3;

use pyo3::prelude::*;

#[cfg(test)]
mod tests {
    extern crate hex;

    use super::*;

    type TFCompressArgs = (u64, Vec<u64>, Vec<u8>, Vec<u64>, bool);

    pub struct PyBlake2<'a> {
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
            rounds: u64,
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

    #[test]
    fn test_py_blake2b_compress() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let blake2 = PyBlake2::new(py);

        let examples = vec![
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

        for (inp, expected) in examples {
            let input_bytes = hex::decode(inp).unwrap();

            let blake2_params = blake2.extract_blake2b_parameters(&input_bytes);
            let (rounds, h_starting_state, block, t_offset_counters, final_block_flag) =
                blake2_params.unwrap();

            let result_bytes = blake2
                .blake2b_compress(
                    rounds,
                    &h_starting_state,
                    &block,
                    &t_offset_counters,
                    final_block_flag,
                )
                .unwrap();

            assert_eq!(hex::encode(result_bytes), expected);
        }
    }
}
