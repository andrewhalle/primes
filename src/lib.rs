pub mod eratosthenes;

pub struct Primes {
    curr: u32,
    generated_primes: Vec<u32>,
}

impl Primes {
    pub fn new() -> Self {
        let mut generated_primes = Vec::with_capacity(1024);
        generated_primes.push(2);
        generated_primes.push(3);
        generated_primes.push(5);
        generated_primes.push(7);

        Self {
            curr: 2,
            generated_primes,
        }
    }

    fn frontload(&mut self) -> Option<u32> {
        let (retval, new) = match self.curr {
            2 => (Some(2), 3),
            3 => (Some(3), 5),
            5 => (Some(5), 7),
            7 => (Some(7), 11),
            _ => unreachable!(),
        };

        self.curr = new;

        retval
    }

    fn check(&self) -> bool {
        let limit = (self.curr as f64).sqrt() as u32;
        for val in &self.generated_primes {
            if *val > limit {
                break;
            }

            if self.curr % *val == 0 {
                return false;
            }
        }

        return true;
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr <= 7 {
            return self.frontload();
        }

        while !self.check() {
            self.curr += 1;
        }

        self.generated_primes.push(self.curr);
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
