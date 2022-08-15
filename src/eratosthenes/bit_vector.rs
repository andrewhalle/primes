/// Generate a list of prime numbers up to the given limit, inclusive.
///
/// This uses the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
pub fn primes(limit: u64) -> Vec<u64> {
    // We'll use a bit for each value where 1 indicates prime candidacy and 0 indicates non-prime.
    // First we need to calculate how many u64s can represent our search space.
    let vec_size: usize = ((limit as usize - 1) / 128) + 1;
    let mut candidates = vec![u64::MAX; vec_size];

    // Filter out non-prime numbers.
    let mut candidates_index = 0;
    let mut test_vector = 1;
    for i in 0..(limit/2) {
        let prime_test = candidates[candidates_index] & test_vector;

        if prime_test != 0 {
            let value = i * 2 + 3;
            // Now, we will step through every multiple of value and mark it 0, which is
            // what we use to signfify non-prime numbers.
            for non_prime in (value * 3..(limit + 1)).step_by((value * 2) as usize) {
                let non_prime_index = (((non_prime - 3) / 2) as usize) / 64;
                let mask_vector = 1 << (((non_prime - 3) / 2) % 64);
                let mask_vector = u64::MAX ^ mask_vector;
                candidates[non_prime_index] = candidates[non_prime_index] & mask_vector;
            }
        }

        if test_vector >= (1 << 63) {
            candidates_index += 1;
            test_vector = 1;
        } else {
            test_vector <<= 1;
        }
    }

    // At this point, all the 1's are confirmed primes.
    let mut primes = vec![2];
    let mut value = 3;
    for candidate in candidates {
        for i in 0..64 {
            if (candidate >> i) & 1 == 1 {
                // This is a prime
                primes.push(value);
            }
            value += 2;
            if value > limit {
                break;
            }
        }
    }

    primes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn primes_10() {
        let expected = [2, 3, 5, 7];

        assert_eq!(primes(10), expected);
    }

    #[test]
    fn primes_15() {
        let expected = [2, 3, 5, 7, 11, 13];

        assert_eq!(primes(15), expected);
    }

    #[test]
    fn primes_19() {
        let expected = [2, 3, 5, 7, 11, 13, 17, 19];

        assert_eq!(primes(19), expected);
    }

    #[test]
    fn primes_50() {
        let expected = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        assert_eq!(primes(50), expected);
    }

    #[test]
    /// Compare with primal's output
    fn primal() {
        let expected: Vec<u64> = primal::Sieve::new(1000000)
            .primes_from(2)
            .map(|p| p.try_into().unwrap())
            .collect();

        assert_eq!(primes(1000000), expected);
    }
}
