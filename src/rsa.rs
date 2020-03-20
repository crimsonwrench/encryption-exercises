use crate::primes::{Primes, MutuallyPrimes};
use rand::seq::IteratorRandom;

struct NumberE {
    current: usize,
    euler: usize,
    d: usize
}

impl NumberE {
    pub fn new(euler: usize, d: usize) -> Self {
        NumberE {
            current: 1,
            euler,
            d
        }
    }
}

impl Iterator for NumberE {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.current..Self::Item::max_value() {
            if (i * self.d) % self.euler == 1 {
                self.current = i + 1;
                return Some(i);
            }
        }

        None
    }
}

pub struct Rsa {
    public_part: usize,
    private_part: usize,
    n: usize
}

impl Rsa {
    pub fn new() -> Self {
        let keys: (usize, usize, usize) = Self::generate_rsa();
        Rsa {
            public_part: keys.0,
            private_part: keys.1,
            n: keys.2
        }
    }

    pub fn encode(&self, string: String) -> Vec<usize> {
        // TODO: Encoding
        vec![1, 2, 3]
    }

    pub fn decode(&self, string: String) -> String {
        // TODO: Decoding
        String::from("Test")
    }

    fn generate_rsa() -> (usize, usize, usize) {
        // Initialize lazily-initialized thread-local number generator
        let mut rng = rand::thread_rng();

        // Generate two random primes within a given max value
        let first_prime: usize = Primes::new(1000).choose(&mut rng).unwrap();
        let second_prime: usize = Primes::new(1000).choose(&mut rng).unwrap();

        // Get product of prime numbers
        let n: usize = first_prime * second_prime;

        // Get euler's function value for given primes
        let euler: usize = (first_prime - 1) * (second_prime - 1);

        // Get mutually prime number with euler's function in range [euler; euler * 2]
        let d: usize = MutuallyPrimes::new(euler, euler * 2).choose(&mut rng).unwrap();

        // Get a number matching condition: (e * d) % euler == 1
        let e = NumberE::new(euler, d).next().unwrap();

        (e, d, n)
    }
}