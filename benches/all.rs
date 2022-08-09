use criterion::{criterion_group, criterion_main, Criterion};

use primes::{
    eratosthenes::{og, skip_2},
    Primes,
};

fn sieve_og(c: &mut Criterion) {
    c.bench_function("sieve_og", |b| {
        b.iter(|| {
            let _ = og::primes(10000);
        })
    });
}

fn sieve_skip_2(c: &mut Criterion) {
    c.bench_function("sieve_skip_2", |b| {
        b.iter(|| {
            let _ = skip_2::primes(10000);
        })
    });
}

fn test_division(c: &mut Criterion) {
    c.bench_function("test_division", |b| {
        b.iter(|| {
            let _: Vec<u32> = Primes::new().take(1024).collect();
        })
    });
}

criterion_group!(all, test_division, sieve_og, sieve_skip_2);
criterion_main!(all);
