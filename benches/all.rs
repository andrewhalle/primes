use std::fmt;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use primal::{Primes as PrimalPrimes, Sieve as PrimalSieve};

use primes::{
    eratosthenes::{og, skip_2},
    live::first::Primes as LiveFirstPrimes,
    live::second::Primes as LiveSecondPrimes,
    Primes,
};

struct Input {
    largest_prime: u64,
    largest_prime_index: usize,
    search_space: u64,
}

impl Input {
    fn new(search_space: u64) -> Self {
        let primes = skip_2::primes(search_space);

        Self {
            largest_prime: *primes.last().unwrap(),
            largest_prime_index: primes.len(),
            search_space,
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.search_space)
    }
}

fn bench_em(c: &mut Criterion) {
    let mut group = c.benchmark_group("Primes");

    // Each number in this list defines a search space in which we will record the largest prime
    // and the index of that prime
    for i in [
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        500_000_000,
    ]
    .iter()
    {
        let input = Input::new(*i);

        group.bench_with_input(
            BenchmarkId::new("primal_primes", &input),
            &input,
            |b, input| {
                b.iter(|| {
                    let _: Vec<_> = PrimalPrimes::all()
                        .take(input.largest_prime_index)
                        .collect();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("primal_sieve", &input),
            &input,
            |b, input| {
                b.iter(|| {
                    let _: Vec<_> = PrimalSieve::new(input.largest_prime as usize)
                        .primes_from(2)
                        .collect();
                })
            },
        );
        group.bench_with_input(BenchmarkId::new("division", &input), &input, |b, input| {
            b.iter(|| {
                let _: Vec<u32> = Primes::new().take(input.largest_prime_index).collect();
            })
        });
        group.bench_with_input(
            BenchmarkId::new("live_first_prime", &input),
            &input,
            |b, input| {
                b.iter(|| {
                    let _: Vec<u64> = LiveFirstPrimes::new()
                        .take(input.largest_prime_index)
                        .collect();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("live_second_prime", &input),
            &input,
            |b, input| {
                b.iter(|| {
                    let _: Vec<u64> = LiveSecondPrimes::new()
                        .take(input.largest_prime_index)
                        .collect();
                })
            },
        );
        group.bench_with_input(BenchmarkId::new("sieve_og", &input), &input, |b, input| {
            b.iter(|| {
                let _ = og::primes(input.largest_prime);
            })
        });
        group.bench_with_input(
            BenchmarkId::new("sieve_skip_2", &input),
            &input,
            |b, input| {
                b.iter(|| {
                    let _ = skip_2::primes(input.largest_prime);
                })
            },
        );
    }
    group.finish();
}

criterion_group!(all, bench_em);
criterion_main!(all);
