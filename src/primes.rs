pub struct Primes {
    primes: Vec<usize>,
    current: usize,
    max: usize
}

impl Primes {
    pub fn new(max: usize) -> Self {
        Primes {
            primes: vec![],
            current: 2,
            max
        }
    }
}

impl Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {

        if self.current >= self.max {
            return None;
        }

        for i in self.current..usize::max_value() {
            if self.primes.iter().all(|&x| i % x != 0) {
                self.primes.push(i);
                self.current = i + 1;
                return Some(i);
            }
        }

        None
    }
}

pub struct MutuallyPrimes {
    current: usize,
    with: usize,
    max: usize,
}

impl MutuallyPrimes {
    fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
        while a != 0 {
            let old_a = a;
            a = b % a;
            b = old_a;
        }
        b
    }

    pub fn new(with: usize, max: usize) -> Self {
        MutuallyPrimes {
            current: with,
            with,
            max,
        }
    }
}

impl Iterator for MutuallyPrimes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {

        if self.current >= self.max {
            return None;
        }

        for i in self.current..usize::max_value() {
            let value: usize = Self::greatest_common_divisor(self.with, i);

            if value == 1 {
                self.current = i + 1;
                return Some(i);
            }
        }

        None
    }
}
