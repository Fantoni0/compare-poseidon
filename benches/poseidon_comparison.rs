use ark_std::rand::Rng;
use ark_std::{time::Duration, test_rng};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
// Arkwork dependencies
use ark_crypto_primitives::{
    sponge::{
        test::Fr,
        poseidon::{PoseidonSponge},
        CryptographicSponge, FieldBasedCryptographicSponge,
    },
};
use ark_crypto_primitives::crh::pedersen::bytes_to_bits;
use ark_crypto_primitives::sponge::poseidon::PoseidonDefaultConfigField;
use ark_ff::{BigInt, BigInteger};

// ICICLE dependencies
use icicle_runtime::{
    memory::{IntoIcicleSlice, IntoIcicleSliceMut},
};
use icicle_core::{
    bignum::BigNum,
    hash::{HashConfig},
    poseidon::{Poseidon},
};
use icicle_bn254::curve::ScalarField as IcFr;

// Define different input rates for testing
// These are the only supported rates from ICICLE &[3, 5, 9, 12];
// These are the parameters supported by Arkworks &[2, 3, 4, 5, 6, 7, 8]
// So we only test 3 and 5 for the comparison
const RATES: &[usize] = &[3, 5];

// Helper function to convert IcFr to Fr
#[allow(dead_code)]
fn ic_fr_to_fr(ic_fr: &IcFr) -> Fr {
    // Convert IcFr to string representation and then parse it
    let ic_bytes = ic_fr.to_bytes_le();
    let ib_bits: Vec<bool> = bytes_to_bits(&ic_bytes);
    Fr::from(BigInt::from_bits_le(&ib_bits))
}

// Comparison benchmarks
// Please note that results between implementations can differ:
// ICICLE employs an optimized version of Poseidon (https://hackmd.io/@jake/poseidon-spec#Optimized-Poseidon)
fn poseidon_comparison_benchmark(c: &mut Criterion, rate: usize) {
    // Generate the same numbers for both implementations
    let mut rng = test_rng();
    let test_data: Vec<i32> = (0..100)
        .map(|_| rng.gen_range(0..1000))
        .collect();
    
    let test_data_ark: Vec<Fr> = test_data
        .iter()
        .map(|e| Fr::from(*e))
        .collect();
    
    let test_data_ic: Vec<IcFr> = test_data
        .iter()
        .map(|e| IcFr::from(*e as u32))
        .collect();
    
    // Create Poseidon parameters 
    let sponge_param = Fr::get_default_poseidon_parameters(rate, true).unwrap();
    let poseidon_hasher_main = Poseidon::new::<IcFr>(rate as u32, None).unwrap();
    
    let mut group = c.benchmark_group(format!("poseidon_comparison_rate_{}", rate));
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(5));
    
    // Arkworks single hash
    group.bench_function("arkworks", |b| {
        b.iter(|| {
            let mut sponge = PoseidonSponge::new(&sponge_param);
            sponge.absorb(black_box(&&test_data_ark[..rate])); // Absorb includes the permutation
            black_box(sponge.squeeze_native_field_elements(1))
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
            /*println!("Result_ic: {:?}", &outputs_main
                    .iter()
                    .map(ic_fr_to_fr)
                    .collect::<Vec<Fr>>());*/
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
