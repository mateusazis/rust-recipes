use rand::RngCore;
use rand::SeedableRng;
use std::simd::Simd;
use std::time::Duration;
use std::time::Instant;
use std::vec::Vec;

// TODO: use a Vec instead; too large to allocate on the stack
const VEC_LEN: usize = 1024 * 1024 * 32;
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
    v0.resize(VEC_LEN, 0);
    v1.resize(VEC_LEN, 0);

    let mut random_num_generator = rand::rngs::SmallRng::from_entropy();

    println!("building arrays...");
    for _i in 0..VEC_LEN {
        v0.push(random_num_generator.next_u32() % 1000);
        v1.push(random_num_generator.next_u32() % 1000);
    }

    let (d0, result0) = benchmark(&|| {
        let mut result = Vec::<u32>::with_capacity(VEC_LEN);
        result.resize(VEC_LEN, 0);
        println!("start1");
        for i in 0..VEC_LEN {
            result[i] = v0[i] + v1[i];
        }
        println!("end1");
        result
    });
    println!("Done sequential sum");

    let (d1, result1) = benchmark(&|| {
        let mut result = Vec::<u32>::with_capacity(VEC_LEN);
        result.resize(VEC_LEN, 0);
        println!("start2");
        for i in (0..VEC_LEN).step_by(SIMD_LANES) {
            let a = Simd::<u32, SIMD_LANES>::from_slice(&v0[i..i + SIMD_LANES]);
            let b = Simd::<u32, SIMD_LANES>::from_slice(&v1[i..i + SIMD_LANES]);
            let sum = a + b;
            let sum_slice = sum.as_array();
            result[i..i + SIMD_LANES].copy_from_slice(sum_slice);
        }
        println!("end2");
        result
    });
    println!("Done sum with SIMD");

    assert_eq!(result0, result1);
    println!(
        "Added {} elements; without SIMD: {}ms, with SIMD: {}ms",
        VEC_LEN,
        d0.as_millis(),
        d1.as_millis()
    );
}
