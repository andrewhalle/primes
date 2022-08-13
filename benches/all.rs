use std::fmt;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use primes::{
    eratosthenes::{bit_vector, og, skip_2},
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
        let primes: Vec<_> = primal::Sieve::new(search_space as usize)
            .primes_from(2)
            .collect();

        Self {
            largest_prime: *primes.last().unwrap() as u64,
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
                    let _: Vec<_> = primal::Primes::all()
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
                    let _: Vec<_> = primal::Sieve::new(input.largest_prime as usize)
                        .primes_from(2)
                        .collect();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("primal_streaming_sieve", &input),
            &input,
            |b, input| {
                b.iter(|| {
                    let _: Vec<_> = (1..input.largest_prime_index)
                        .map(|i| primal::StreamingSieve::nth_prime(i as usize))
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
        group.bench_with_input(
            BenchmarkId::new("bit_vector", &input),
            &input,
            |b, input| {
                b.iter(|| {
                    let _ = bit_vector::primes(input.largest_prime);
                })
            },
        );
    }
    group.finish();
}

criterion_group!(all, bench_em);
criterion_main!(all);
