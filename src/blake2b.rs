use std::convert::TryInto;

const SIGMA_LEN: usize = 10;
const SIGMA: [[usize; 16]; SIGMA_LEN] = [
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

const WORDBITS: usize = 64;
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

const ROT1: usize = 32;
const ROT2: usize = 24;
const ROT3: usize = 16;
const ROT4: usize = 63;

#[inline]
fn u64_from_le(input: &[u8]) -> u64 {
    u64::from_le_bytes(input.try_into().unwrap())
}

#[inline]
fn eight_words(input: &[u8]) -> [u64; 8] {
    [
        u64_from_le(&input[..8]),
        u64_from_le(&input[8..16]),
        u64_from_le(&input[16..24]),
        u64_from_le(&input[24..32]),
        u64_from_le(&input[32..40]),
        u64_from_le(&input[40..48]),
        u64_from_le(&input[48..56]),
        u64_from_le(&input[56..64]),
    ]
}

#[inline]
fn sixteen_words(input: &[u8]) -> [u64; 16] {
    [
        u64_from_le(&input[..8]),
        u64_from_le(&input[8..16]),
        u64_from_le(&input[16..24]),
        u64_from_le(&input[24..32]),
        u64_from_le(&input[32..40]),
        u64_from_le(&input[40..48]),
        u64_from_le(&input[48..56]),
        u64_from_le(&input[56..64]),
        u64_from_le(&input[64..72]),
        u64_from_le(&input[72..80]),
        u64_from_le(&input[80..88]),
        u64_from_le(&input[88..96]),
        u64_from_le(&input[96..104]),
        u64_from_le(&input[104..112]),
        u64_from_le(&input[112..120]),
        u64_from_le(&input[120..128]),
    ]
}

#[inline]
fn two_words(input: &[u8]) -> [u64; 2] {
    [u64_from_le(&input[..8]), u64_from_le(&input[8..16])]
}

pub type CompressArgs = (usize, [u64; 8], [u64; 16], [u64; 2], bool);

/// Decode blake2 precompile input parameters from the tightly packed encoding in the byte sequence
/// `input`.
///
/// See here: https://eips.ethereum.org/EIPS/eip-152#specification
pub fn decode_parameters(input: &[u8]) -> Result<CompressArgs, String> {
    if input.len() != 213 {
        return Err(format!(
            "input length for blake2 F precompile should be exactly 213 bytes, got: {}",
            input.len()
        ));
    }

    let rounds = u32::from_be_bytes((&input[..4]).try_into().unwrap()) as usize;
    let starting_state = eight_words(&input[4..68]);
    let block = sixteen_words(&input[68..196]);
    let offset_counters = two_words(&input[196..212]);
    let final_block_flag = match input[212] {
        0 => false,
        1 => true,
        x => {
            return Err(format!("incorrect final block indicator flag, got: {}", x));
        }
    };

    Ok((
        rounds,
        starting_state,
        block,
        offset_counters,
        final_block_flag,
    ))
}

/// Rotate bits in the unsigned 64-bit integer `x` to the right by `n` bits.
///
/// See here: https://tools.ietf.org/html/rfc7693#section-2.3
#[inline]
fn rotate_bits(x: u64, n: usize) -> u64 {
    (x >> n) ^ (x << (WORDBITS - n))
}

/// The blake2b mixing function G.
///
/// See here: https://tools.ietf.org/html/rfc7693#section-3.1
#[allow(non_snake_case)]
#[inline]
fn G(v: &mut [u64; 16], a: usize, b: usize, c: usize, d: usize, x: u64, y: u64) {
    // RFC 7693 includes the use of mod operations with operand 2 ** 64.  We omit those because we
    // get them for free with u64 arithmetic.
    v[a] = v[a] + v[b] + x;
    v[d] = rotate_bits(v[d] ^ v[a], ROT1);
    v[c] = v[c] + v[d];
    v[b] = rotate_bits(v[b] ^ v[c], ROT2);
    v[a] = v[a] + v[b] + y;
    v[d] = rotate_bits(v[d] ^ v[a], ROT3);
    v[c] = v[c] + v[d];
    v[b] = rotate_bits(v[b] ^ v[c], ROT4);
}

/// The blake2b compression function F.
///
/// See here: https://tools.ietf.org/html/rfc7693#section-3.2
#[allow(non_snake_case)]
pub fn F(
    rounds: usize,
    starting_state: &[u64],
    block: &[u64],
    offset_counters: &[u64],
    final_block_flag: bool,
) -> [u8; 64] {
    let h = starting_state;
    let m = block;
    let t = offset_counters;

    let mut v = [
        h[0],         // 0
        h[1],         // 1
        h[2],         // 2
        h[3],         // 3
        h[4],         // 4
        h[5],         // 5
        h[6],         // 6
        h[7],         // 7
        IV[0],        // 8
        IV[1],        // 9
        IV[2],        // 10
        IV[3],        // 11
        IV[4] ^ t[0], // 12
        IV[5] ^ t[1], // 13
        if final_block_flag {
            MASKBITS ^ IV[6]
        } else {
            IV[6]
        }, // 14
        IV[7],        // 15
    ];

    for r in 0..rounds {
        let s = &SIGMA[r % SIGMA_LEN];

        G(&mut v, 0, 4, 8, 12, m[s[0]], m[s[1]]);
        G(&mut v, 1, 5, 9, 13, m[s[2]], m[s[3]]);
        G(&mut v, 2, 6, 10, 14, m[s[4]], m[s[5]]);
        G(&mut v, 3, 7, 11, 15, m[s[6]], m[s[7]]);

        G(&mut v, 0, 5, 10, 15, m[s[8]], m[s[9]]);
        G(&mut v, 1, 6, 11, 12, m[s[10]], m[s[11]]);
        G(&mut v, 2, 7, 8, 13, m[s[12]], m[s[13]]);
        G(&mut v, 3, 4, 9, 14, m[s[14]], m[s[15]]);
    }

    let result_words = [
        (h[0] ^ v[0] ^ v[8]).to_le_bytes(),
        (h[1] ^ v[1] ^ v[9]).to_le_bytes(),
        (h[2] ^ v[2] ^ v[10]).to_le_bytes(),
        (h[3] ^ v[3] ^ v[11]).to_le_bytes(),
        (h[4] ^ v[4] ^ v[12]).to_le_bytes(),
        (h[5] ^ v[5] ^ v[13]).to_le_bytes(),
        (h[6] ^ v[6] ^ v[14]).to_le_bytes(),
        (h[7] ^ v[7] ^ v[15]).to_le_bytes(),
    ];

    let mut result = [0u8; 64];
    for (i, word_bytes) in result_words.into_iter().enumerate() {
        for (j, x) in word_bytes.into_iter().enumerate() {
            result[i * 8 + j] = *x;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

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
        ( // 2,000,000 rounds
            "001e848048c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
            "a86f2348a6afc9a7ccb3ae6e92818eb34f57f4e0d618580efa1c9b0a35ea84998c22afe92c41e4b538f213f8f35deb37e47fc6a8eca34f645da18231f59c6190",
        ),
        ( // 8,000,000 rounds
            "007a120048c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
            "6d2ce9e534d50e18ff866ae92d70cceba79bbcd14c63819fe48752c8aca87a4bb7dcc230d22a4047f0486cfcfb50a17b24b2899eb8fca370f22240adb5170189",
        ),
    ];

    #[test]
    fn test_f_success() {
        for (inp, expected) in FAST_EXAMPLES {
            let input_bytes = hex::decode(inp).unwrap();
            let blake2_params = decode_parameters(&input_bytes).unwrap();
            let (rounds, starting_state, block, offset_counters, final_block_flag) = blake2_params;

            let result_bytes = F(
                rounds,
                &starting_state,
                &block,
                &offset_counters,
                final_block_flag,
            )
            .to_vec();

            assert_eq!(hex::encode(result_bytes), *expected);
        }
    }

    #[test]
    fn test_f_slow() {
        for (inp, expected) in SLOW_EXAMPLES {
            let input_bytes = hex::decode(inp).unwrap();
            let blake2_params = decode_parameters(&input_bytes).unwrap();
            let (rounds, starting_state, block, offset_counters, final_block_flag) = blake2_params;

            let result_bytes = F(
                rounds,
                &starting_state,
                &block,
                &offset_counters,
                final_block_flag,
            )
            .to_vec();

            assert_eq!(hex::encode(result_bytes), *expected);
        }
    }

    /// Check slow running test vector 8 from EIP 152
    /// (https://eips.ethereum.org/EIPS/eip-152#test-vector-8)
    #[test]
    #[ignore]
    fn test_f_eip_152_vec_8() {
        let (inp, expected) = ( // 2 ** 32 - 1 rounds
            "ffffffff48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
            "fc59093aafa9ab43daae0e914c57635c5402d8e3d2130eb9b3cc181de7f0ecf9b22bf99a7815ce16419e200e01846e6b5df8cc7703041bbceb571de6631d2615",
        );

        let input_bytes = hex::decode(inp).unwrap();
        let blake2_params = decode_parameters(&input_bytes).unwrap();
        let (rounds, starting_state, block, offset_counters, final_block_flag) = blake2_params;

        let t_start = std::time::SystemTime::now();

        let result_bytes = F(
            rounds,
            &starting_state,
            &block,
            &offset_counters,
            final_block_flag,
        )
        .to_vec();

        if let Ok(elapsed) = t_start.elapsed() {
            eprintln!("test_f_eip_152_vec_8: took {} secs", elapsed.as_secs());
        }

        assert_eq!(hex::encode(result_bytes), *expected);
    }

    #[test]
    fn test_decode_parameters_error() {
        for inp in ERROR_EXAMPLES {
            let input_bytes = hex::decode(inp).unwrap();

            if decode_parameters(&input_bytes).is_ok() {
                panic!("expected Result::Err but got Result::Ok");
            }
        }
    }
}

#[cfg(test)]
mod bench {
    extern crate test;

    use super::*;

    use test::Bencher;

    fn rounds_benchmark(rounds: usize, bencher: &mut Bencher) {
        let input_bytes = hex::decode("0000000048c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001").unwrap();

        let blake2_params = decode_parameters(&input_bytes).unwrap();
        let (_, starting_state, block, offset_counters, final_block_flag) = blake2_params;

        bencher.iter(|| {
            F(
                rounds,
                &starting_state,
                &block,
                &offset_counters,
                final_block_flag,
            )
            .to_vec()
        });
    }

    #[bench]
    fn bench_100_000_rounds(bencher: &mut Bencher) {
        rounds_benchmark(100_000, bencher);
    }

    #[bench]
    fn bench_2_000_000_rounds(bencher: &mut Bencher) {
        rounds_benchmark(2_000_000, bencher);
    }

    #[bench]
    fn bench_8_000_000_rounds(bencher: &mut Bencher) {
        rounds_benchmark(8_000_000, bencher);
    }
}
