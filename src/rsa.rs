use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;

pub struct Rsa {
    public_part: usize,
    private_part: usize,
    n: usize,
}

impl Rsa {
    pub fn new(prime_limit: usize) -> Self {
        let keys: (usize, usize, usize) = Self::generate_rsa(prime_limit);

        Rsa {
            public_part: keys.0,
            private_part: keys.1,
            n: keys.2,
        }
    }

    fn generate_primes(limit: usize) -> Vec<usize> {
        let mut is_prime: Vec<bool> = vec![true; limit+1];
        is_prime[0] = false;

        if limit >= 1 {
            is_prime[1] = false;
        }

        for num in 2..limit+1 {
            if is_prime[num] {
                let mut multiple = num * num;
                while multiple <= limit {
                    is_prime[multiple] = false;
                    multiple += num;
                }
            }
        }

        is_prime
            .iter()
            .enumerate()
            .filter_map(|(pr, &prime)| if prime { Some(pr) } else { None } )
            .collect()
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

    pub fn decode(&self, code_chain: &str) -> Option<String> {
        let decoded_chunks: Vec<u8> = code_chain
            .split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&chunk| match chunk.trim().parse::<usize>() {
                Ok(parsed) => Some(mod_exp::mod_exp(parsed, self.private_part, self.n) as u8),
                Err(error) => {
                    println!("Could not parse chunk \"{}\": {:?}", chunk.trim(), error);
                    None
                }
            })
            .flatten()
            .collect();

        String::from_utf8(decoded_chunks).ok()
    }

    fn generate_rsa(prime_limit: usize) -> (usize, usize, usize) {
        let mut rng = rand::thread_rng();

        let primes: Vec<usize> = Self::generate_primes(prime_limit);

        let first_prime: usize = *primes.choose(&mut rng).unwrap();
        let second_prime: usize = *primes.choose(&mut rng).unwrap();

        // Get common modulus
        let n: usize = first_prime * second_prime;

        // Get euler's function value for given primes
        let euler: usize = (first_prime - 1) * (second_prime - 1);

        // Get up to 100 matching public key exponents and choose a random one
        let e: usize = PublicKeyExponent::new(euler, 100).choose(&mut rng).unwrap();

        // Get a private key exponent satisfying the congruence relation: (e * d) === 1 (mod euler)
        let d = PrivateKeyExponent::new(euler, e).next().unwrap();

        (e, d, n)
    }
}

struct PublicKeyExponent {
    current: usize,
    with: usize,
    max_count: usize,
    matches: Vec<usize>,
}

impl PublicKeyExponent {
    fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
        while a != 0 {
            let old_a = a;
            a = b % a;
            b = old_a;
        }
        b
    }

    fn new(with: usize, max_count: usize) -> Self {
        PublicKeyExponent {
            current: 1,
            with,
            max_count,
            matches: vec![],
        }
    }
}

impl Iterator for PublicKeyExponent {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.matches.len() >= self.max_count {
            return None;
        }

        for i in self.current..self.with {
            if Self::greatest_common_divisor(self.with, i) == 1 {
                self.matches.push(i);
                self.current = i + 1;
                return Some(i);
            }
        }

        None
    }
}

struct PrivateKeyExponent {
    current: usize,
    euler: usize,
    e: usize,
}

impl PrivateKeyExponent {
    fn new(euler: usize, e: usize) -> Self {
        PrivateKeyExponent {
            current: 1,
            euler,
            e,
        }
    }
}

impl Iterator for PrivateKeyExponent {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for d in self.current..Self::Item::max_value() {
            if (d * self.e) % self.euler == 1 {
                self.current = d + 1;
                return Some(d);
            }
        }

        None
    }
}

mod tests {
    #[test]
    fn ensure_integrity() {
        use crate::rsa::Rsa;

        let rsa: Rsa = Rsa::new(10000);
        [
            "The quick brown fox jumps over the lazy dog", // Latin alphabet
            "!@#$%^&",                                     // Special characters
            "Съешь же ещё этих мягких французских булок, да выпей чаю", // Cyrillic alphabet
        ]
        .iter()
        .for_each(|msg: &&str| {
            let encoded: String = rsa.encode(msg);
            assert_eq!(rsa.decode(encoded.as_str()), Some(String::from(*msg)))
        });
    }
}
