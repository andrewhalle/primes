use primes::Primes;

fn main() {
    let primes: Vec<u32> = Primes::new().take(1 << 21).collect();
    println!("{:?}", primes);
}
