use std::collections::HashMap;
use std::iter::StepBy;
use std::ops::RangeFrom;

pub struct Primes {
    curr: u32,
    iters: HashMap<u32, Vec<StepBy<RangeFrom<u32>>>>,
}

impl Primes {
    pub fn new() -> Self {
        Self {
            curr: 2,
            iters: Default::default(),
        }
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.iters.contains_key(&self.curr) {
            // This .unwrap() is safe because we checked .contains_key() above.
            let iters = self.iters.remove(&self.curr).unwrap();
            for mut iter in iters {
                // This .unwrap() is safe because our iterators never end, by
                // construction.
                let next = iter.next().unwrap();
                let entry = self.iters.entry(next).or_default();
                entry.push(iter);
            }
            self.curr += 1;
        }

        // self.curr is prime, so add an iter for it.
        let entry = self.iters.entry(self.curr * 2).or_default();
        entry.push((self.curr * 3..).step_by(self.curr as usize));

        let retval = Some(self.curr);
        self.curr += 1;
        retval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let primes: Vec<u32> = Primes::new().take(16).collect();
        assert_eq!(
            &primes[..],
            &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53]
        );
    }
}
