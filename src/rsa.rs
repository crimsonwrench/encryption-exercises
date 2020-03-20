use crate::primes::{MutuallyPrimes, Primes};
use rand::seq::IteratorRandom;

struct NumberE {
    current: usize,
    euler: usize,
    d: usize,
}

impl NumberE {
    pub fn new(euler: usize, d: usize) -> Self {
        NumberE {
            current: 1,
            euler,
            d,
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
    n: usize,
}

impl Rsa {
    pub fn new() -> Self {
        let keys: (usize, usize, usize) = Self::generate_rsa();
        Rsa {
            public_part: keys.0,
            private_part: keys.1,
            n: keys.2,
        }
    }

    pub fn encode(&self, string: &str) -> String {
        string
            .to_string()
            .into_bytes()
            .iter()
            .map(|&u| mod_exp::mod_exp(u as usize, self.public_part, self.n).to_string())
            .collect::<Vec<String>>()
            .join("-")
    }

    pub fn decode(&self, string: &str) -> Option<String> {
        let letters: Vec<u8> = string
            .split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&str| match str.parse::<usize>() {
                Ok(_parsed) => Some(mod_exp::mod_exp(
                    str.parse::<usize>().unwrap(),
                    self.private_part,
                    self.n,
                ) as u8),
                Err(error) => {
                    println!("Could not parse input {:?}", error);
                    None
                }
            })
            .flatten()
            .collect();

        String::from_utf8(letters).ok()
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
        let d: usize = MutuallyPrimes::new(euler, euler * 2)
            .choose(&mut rng)
            .unwrap();

        // Get a number matching condition: (e * d) % euler == 1
        let e = NumberE::new(euler, d).next().unwrap();

        (e, d, n)
    }
}

mod tests {
    use crate::rsa::Rsa;

    #[test]
    fn ensure_integrity() {
        let rsa: Rsa = Rsa::new();
        let messages: [&str; 3] = [
            "The quick brown fox jumps over the lazy dog",
            "Test123",
            "!@#$%^&",
        ];

        for &message in &messages {
            let encoded: String = rsa.encode(message);
            assert_eq!(rsa.decode(encoded.as_str()), Some(String::from(message)))
        }
    }
}
