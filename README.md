# Poseidon Hash Implementation Comparison

This project compares the performance of different Poseidon hash implementations across various libraries and frameworks.

## Overview

The benchmarks compare Poseidon implementations from:
- **Arkworks** (`ark-crypto-primitives`)
- **Icicle** (GPU-accelerated zk-SNARK library)
- **Horizen** (zkhash library)

## Benchmark Files

### 1. `poseidon_comparison.rs`
Compares **Poseidon v1** implementations between Arkworks and Icicle.

**Key Notes:**
- Results between implementations can differ because ICICLE employs an optimized version of Poseidon (see [Optimized Poseidon spec](https://hackmd.io/@jake/poseidon-spec#Optimized-Poseidon))
- Arkworks `absorb()` already includes the permutation step

### 2. `poseidon2_comparison.rs`
Compares **Poseidon v2** implementations between Horizen and Icicle.

**Key Notes:**
- **Important:** Horizen only performs the permutation argument while Icicle performs the complete hash (absorption + permutation + squeeze)
- For this reason, outputs are different and times obviously favor Horizen, but it is still interesting to see how they compare.

## Running the Benchmarks

```bash
# Run Poseidon v1 comparison
cargo bench --bench poseidon_comparison

# Run Poseidon v2 comparison  
cargo bench --bench poseidon2_comparison
```

