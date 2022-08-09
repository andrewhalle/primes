/// Generate a list of prime numbers up to the given limit, inclusive.
///
/// This uses the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
fn primes(limit: u64) -> Vec<u64> {
    // We will let Somes represent candidates for primes, and Nones be known non-primes.
    let mut candidates: Vec<u64> = (2..limit + 1)
        .into_iter()
        .collect();

    // Filter out non-prime numbers.
    for i in 0..candidates.len() {
        let value: usize = candidates[i].try_into().unwrap();

        if value != 0 {
            for step in ((i + value)..candidates.len()).step_by(value) {
                candidates[step] = 0;
            }
        }
    }

    // At this point, all the non-0 values are confirmed primes. Filter out the 0's and return what's left.
    candidates.into_iter().filter(|i| *i != 0).collect()
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
}
