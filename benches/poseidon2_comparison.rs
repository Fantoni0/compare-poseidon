use std::hint::black_box;
use ark_std::rand::Rng;
use ark_std::{time::Duration, test_rng};
use criterion::{criterion_group, criterion_main, Criterion};
// ICICLE dependencies
use icicle_runtime::{
    memory::{IntoIcicleSlice, IntoIcicleSliceMut},
};
use icicle_bn254::curve::ScalarField as IcFr;
use icicle_core::{
    bignum::BigNum,
    hash::{HashConfig},
    poseidon2::{Poseidon2 as Poseidon2ICICLE},
};
// Horizen dependencies
use zkhash::ark_ff::{BigInt, PrimeField};
use zkhash::{
    fields::{bn256::FpBN256 as HorizenScalar},
    poseidon2::{poseidon2::Poseidon2, poseidon2_instance_bn256::POSEIDON2_BN256_PARAMS},
};

// Define different input rates for testing
// These are the only supported rates from Poseidon2 ICICLE &[ 2, 3, 4, 8, 12, 16, 20, 24];
const RATES: &[usize] = &[2, 3, 4, 8];


// Please note that Horizen only performs the permutation argument while ICICLE performs the complete hash (absorption + permutation + squeeze)
// For this reason outputs are different and times obviously favour Horizen.
// It is still interesting to see how these 2 implementations perform.

// Comparison benchmarks
fn poseidon_comparison_benchmark(c: &mut Criterion, rate: usize) {
    // Generate the same numbers for both implementations
    let mut rng = test_rng();
    let test_data: Vec<i32> = (0..100)
        .map(|_| rng.gen_range(0..1000))
        .collect();

    let test_data_horizen: Vec<HorizenScalar> = test_data
        .iter()
        .map(|e| HorizenScalar::from_bigint(BigInt::from(*e as u32)).unwrap())
        .collect();

    let test_data_ic: Vec<IcFr> = test_data
        .iter()
        .map(|e| IcFr::from(*e as u32))
        .collect();

    // Create Poseidon instances
    // Horizen permutation
    let poseidon = Poseidon2::new(&POSEIDON2_BN256_PARAMS);
    let t = poseidon.get_t();
    // Icicle complete hashes
    let poseidon_hasher_main = Poseidon2ICICLE::new::<IcFr>(rate as u32, None).unwrap();

    let mut group = c.benchmark_group(format!("Poseidon_2_comparison_rate_{}", rate));
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(5));

    // Horizen single hash
    group.bench_function("Horizen", |b| {
        b.iter(|| {
            let perm = poseidon.permutation(black_box(&test_data_horizen[..t]));
            black_box(perm)
        });
    });

    // Icicle single hash
    group.bench_function("icicle", |b| {
        b.iter(|| {
            let input = black_box(&test_data_ic[..rate]);
            let mut outputs_main = vec![IcFr::zero(); 1];

            poseidon_hasher_main
                .hash(
                    input.into_slice(),
                    &HashConfig::default(),
                    outputs_main.into_slice_mut(),
                )
                .unwrap();
            black_box(outputs_main)
        });
    });

    group.finish();
}

// Main benchmark function
fn poseidon_benchmarks(c: &mut Criterion) {
    // Individual implementation benchmarks
    for &rate in RATES {
        poseidon_comparison_benchmark(c, rate);
    }
}

criterion_group!(
    name=benches;
    config = Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs_f32(50.0))
        .warm_up_time(Duration::from_secs_f32(1.0));
    targets = poseidon_benchmarks
);
criterion_main!(benches);
