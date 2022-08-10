use std::collections::HashMap;

pub struct Primes {
    iterators: HashMap<u64, Vec<u64>>,
    next: u64,
}

impl Primes {
    pub fn new() -> Self {
        Self { iterators: HashMap::default(), next: 2 }
    }
}

impl std::iter::Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut value_iterators) = self.iterators.remove(&self.next) {
            for prime in &value_iterators {
                let next_value = self.next + prime;

                let entry = self.iterators.entry(next_value).or_default();
                entry.push(*prime);
            }
 
            value_iterators.clear();
            self.next += 1;
        }

        let prime = self.next;
        let entry = self.iterators.entry(prime * 2).or_default();
        entry.push(prime);
        self.next += 1;

        Some(prime)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn primes_10() {
        let expected = [2, 3, 5, 7];

        assert_eq!(Primes::new().take(4).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn primes_15() {
        let expected = [2, 3, 5, 7, 11, 13];

        assert_eq!(Primes::new().take(6).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn primes_19() {
        let expected = [2, 3, 5, 7, 11, 13, 17, 19];

        assert_eq!(Primes::new().take(8).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn primes_50() {
        let expected = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        assert_eq!(Primes::new().take(15).collect::<Vec<_>>(), expected);
    }
}
