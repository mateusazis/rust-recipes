use std::simd::Simd;
use std::time::Duration;
use std::time::Instant;
use std::vec::Vec;

// TODO: use a Vec instead; too large to allocate on the stack
const VEC_LEN: usize = 1024 * 1024 * 8;
const SIMD_LANES: usize = 8;

fn benchmark(f: &dyn Fn() -> Vec<u32>) -> (Duration, Vec<u32>) {
    let start = Instant::now();
    let res = f();
    let end = Instant::now();
    return (end - start, res);
}

pub fn main() {
    let mut v0 = Vec::<u32>::with_capacity(VEC_LEN);
    let mut v1 = Vec::<u32>::with_capacity(VEC_LEN);
    for _i in 0..VEC_LEN {
        v0.push(rand::random::<u32>() % 1000);
        v1.push(rand::random::<u32>() % 1000);
    }

    let (d0, result0) = benchmark(&|| {
        let mut result0 = Vec::<u32>::with_capacity(VEC_LEN);
        result0.resize(VEC_LEN, 0);
        for i in 0..v0.len() {
            result0[i] = v0[i] + v1[i];
        }
        result0
    });

    let (d1, result1) = benchmark(&|| {
        let mut result1 = Vec::<u32>::with_capacity(VEC_LEN);
        result1.resize(VEC_LEN, 0);
        for i in (0..v0.len()).step_by(SIMD_LANES) {
            let a = Simd::<u32, SIMD_LANES>::from_slice(&v0[i..i + SIMD_LANES]);
            let b = Simd::<u32, SIMD_LANES>::from_slice(&v1[i..i + SIMD_LANES]);
            let sum = a + b;
            let sum_slice = sum.as_array();
            result1[i..i + SIMD_LANES].copy_from_slice(sum_slice);
        }
        result1
    });

    assert_eq!(result0, result1);
    // println!("Got: {:?}", result0);
    // println!("And: {:?}", result1);
    println!(
        "Without SIMD: {}ms, with SIMD: {}ms",
        d0.as_millis(),
        d1.as_millis()
    );
}
