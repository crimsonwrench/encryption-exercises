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

    fn next(&mut self) -> Option<usize> {

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
        panic!("Integer overflow")
    }
}
