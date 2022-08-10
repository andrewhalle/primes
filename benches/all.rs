use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use primes::{
    eratosthenes::{og, skip_2},
    live::first::Primes as LiveFirstPrimes,
    live::second::Primes as LiveSecondPrimes,
    Primes,
};

fn bench_em(c: &mut Criterion) {
    let mut group = c.benchmark_group("Primes");
    // For each tuple, the first number is the largest prime number we will search for for
    // eratosthenes-based algorithms and the second number is the index of that same prime
    // in an array of prime numbers.
    for i in [(821641, 65536), (9999991, 664579), (499999993, 26355867)].iter() {
        group.bench_with_input(BenchmarkId::new("division", i.1), i, |b, i| {
            b.iter(|| {
                let _: Vec<u32> = Primes::new().take(i.1).collect();
            })
        });
        group.bench_with_input(BenchmarkId::new("live_first_prime", i.1), i, |b, i| {
            b.iter(|| {
                let _: Vec<u64> = LiveFirstPrimes::new().take(i.1).collect();
            })
        });
        group.bench_with_input(BenchmarkId::new("live_second_prime", i.1), i, |b, i| {
            b.iter(|| {
                let _: Vec<u64> = LiveSecondPrimes::new().take(i.1).collect();
            })
        });
        group.bench_with_input(BenchmarkId::new("sieve_og", i.0), i, |b, i| {
            b.iter(|| {
                let _ = og::primes(i.0);
            })
        });
        group.bench_with_input(BenchmarkId::new("sieve_skip_2", i.0), i, |b, i| {
            b.iter(|| {
                let _ = skip_2::primes(i.0);
            })
        });
    }
    group.finish();
}

criterion_group!(
    all,
    bench_em,
);
criterion_main!(all);
