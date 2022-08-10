use rustc_hash::FxHashMap;

pub struct Primes {
    iterators: FxHashMap<u64, Vec<u64>>,
    next: u64,
    // This allows us to avoid freeing vecs only to allocate new ones later. It's the warehouse
    // pattern.
    used_vecs: Vec<Vec<u64>>,
}

impl Primes {
    pub fn new() -> Self {
        Self { iterators: FxHashMap::default(), next: 2, used_vecs: vec![] }
    }
}

impl std::iter::Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // We can special case for 2 to go faster by storing half as many iterators.
        if self.next == 2 {
            self.next += 1;
            return Some(2);
        }

        while let Some(mut value_iterators) = self.iterators.remove(&self.next) {
            for prime in &value_iterators {
                // Because we special case'd 2, we need to skip one prime and go to the next one.
                let next_value = self.next + prime*2;

                let entry = self.iterators.entry(next_value).or_insert_with(|| {
                    // There isn't a Vec for our next iterator - let's see if we have one in the
                    // warehouse that we can reuse.
                    match self.used_vecs.pop() {
                        Some(used_vec) => {
                            used_vec
                        },
                        None => vec![],
                    }
                });
                entry.push(*prime);
            }
 
            value_iterators.clear();
            self.used_vecs.push(value_iterators);
            // Because we special case'd 2, we need to skip one prime and go to the next one.
            self.next += 2;
        }

        let prime = self.next;
        // Because we special case'd 2, we need to skip one prime and go to the next one.
        let entry = self.iterators.entry(prime * 3).or_insert_with(|| {
            // There isn't a Vec for our next iterator - let's see if we have one in the
            // warehouse that we can reuse.
            match self.used_vecs.pop() {
                Some(used_vec) => {
                    used_vec
                },
                None => vec![],
            }
        });
        entry.push(prime);
        self.next += 2;

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
