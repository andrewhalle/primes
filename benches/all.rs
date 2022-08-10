use criterion::{black_box, criterion_group, criterion_main, Criterion};

use primes::{
    eratosthenes::{og, skip_2},
    live::first::Primes as LiveFirstPrimes,
    Primes,
};

// The largest prime number we will search for for eratosthenes-based algorithms.
const LARGEST_PRIME_TO_SEARCH: u64 = 821641;

// The index of LARGEST_PRIME_TO_SEARCH in an array of prime numbers.
const LARGEST_PRIME_INDEX: usize = 65536;

fn live_first_primes() {
    let _: Vec<u64> = black_box(LiveFirstPrimes::new().take(LARGEST_PRIME_INDEX).collect());
}

fn sieve_og() {
    let _ = black_box(og::primes(LARGEST_PRIME_TO_SEARCH));
}

fn sieve_skip_2() {
    let _ = black_box(skip_2::primes(LARGEST_PRIME_TO_SEARCH));
}

fn test_division() {
    let _: Vec<u32> = black_box(Primes::new().take(LARGEST_PRIME_INDEX).collect());
}

fn bench_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("primes");

    group.bench_function("live_first_primes", |b| b.iter(|| live_first_primes()));
    group.bench_function("sieve_og", |b| b.iter(|| sieve_og()));
    group.bench_function("sieve_skip_2", |b| b.iter(|| sieve_skip_2()));
    group.bench_function("test_division", |b| b.iter(|| test_division()));

    group.finish();
}

criterion_group!(benches, bench_primes);
criterion_main!(benches);
