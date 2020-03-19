use crate::primes::{Primes, MutuallyPrimes};
use rand::seq::IteratorRandom;

pub fn generate_rsa() -> (usize, usize, usize) {
    // Initialize lazily-initialized thread-local number generator
    let mut rng = rand::thread_rng();

    // Generate a vector containing two random prime numbers
    let primes: Vec<usize> = Primes::new(10000).choose_multiple(&mut rng, 2);

    // Get product of prime numbers
    let n: usize = primes.iter().product();

    // Get euler's function value for given primes
    let euler: usize = (primes[0] - 1) * (primes[1] - 1);

    // Get random mutually prime number with euler's function in range [euler; euler * 2]
    let d: usize = MutuallyPrimes::new(euler, e * 2).choose(&mut rng).unwrap();

    // TODO: Get a number matching condition: (e * d) mod euler == 1
    let e: usize = 1;

    println!("{:#?}, {:#?}, {:#?}, {:#?}", primes, n, e, d);

    (n, e, d)
}