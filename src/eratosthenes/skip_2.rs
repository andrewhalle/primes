use std::num::NonZeroU64;

/// Generate a list of prime numbers up to the given limit, inclusive.
///
/// This uses the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
fn primes(limit: u64) -> Vec<u64> {
    // We will let Somes represent candidates for primes, and Nones be known non-primes.
    let mut candidates: Vec<Option<NonZeroU64>> = (2..limit + 1)
        .into_iter()
        // This filter is OK because later when we step through, we will technically step larger
        // than the number we have found to be prime by 2, but a prime by 2 is already removed by
        // this filter. It works out.
        .filter(|i| *i % 2 != 0 || *i == 2)
        .map(|i| NonZeroU64::new(i))
        .collect();

    // Filter out non-prime numbers.
    for i in 1..candidates.len() {
        if let Some(value) = candidates[i] {
            let value: usize = value.get().try_into().unwrap();

            // Now, we will step through every multiple of value and mark it None, which is
            // what we use to signfify non-prime numbers. Note that we removed the even numbers
            // in the filter in the contruction of candidates above, so here we will do i +
            // value, i + 2*value, i + 3*value, etc. Technically we are skipping over the
            // evens, but that's OK. As an example, consider 3. We would keep three, then mark
            // 6, 9, 12, 15, etc. But because 6 and 12 have been removed, this will just mark 9,
            // 15, 21, etc.
            for step in ((i + value)..candidates.len()).step_by(value) {
                candidates[step] = None
            }
        }
    }

    // At this point, all the Somes are confirmed primes. Filter out the Nones and return them.
    candidates.into_iter().filter_map(|i| i.map(|i| i.get())).collect()
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
