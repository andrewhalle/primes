use criterion::{criterion_group, criterion_main, Criterion};

use primes::{
    eratosthenes::{og, skip_2},
    Primes,
};

// The largest prime number we will search for for eratosthenes-based algorithms.
const LARGEST_PRIME_TO_SEARCH: u64 = 9999991;
// The index of LARGEST_PRIME_TO_SEARCH in an array of prime numbers.
const LARGEST_PRIME_INDEX: usize = 664579;

fn sieve_og(c: &mut Criterion) {
    c.bench_function("sieve_og", |b| {
        b.iter(|| {
            let _ = og::primes(LARGEST_PRIME_TO_SEARCH);
        })
    });
}

fn sieve_skip_2(c: &mut Criterion) {
    c.bench_function("sieve_skip_2", |b| {
        b.iter(|| {
            let _ = skip_2::primes(LARGEST_PRIME_TO_SEARCH);
        })
    });
}

fn test_division(c: &mut Criterion) {
    c.bench_function("test_division", |b| {
        b.iter(|| {
            let _: Vec<u32> = Primes::new().take(LARGEST_PRIME_INDEX).collect();
        })
    });
}

criterion_group!(all, test_division, sieve_og, sieve_skip_2);
criterion_main!(all);