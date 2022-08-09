use criterion::{criterion_group, criterion_main, Criterion};

use primes::Primes;

fn test_division(c: &mut Criterion) {
    c.bench_function("test_division", |b| {
        b.iter(|| {
            let _: Vec<u32> = Primes::new().take(1024).collect();
        })
    });
}

criterion_group!(all, test_division);
criterion_main!(all);
