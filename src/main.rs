mod primes;

use primes::Primes;
use rand::seq::IteratorRandom;

fn main() {
    let mut rng = rand::thread_rng();

    let primes = Primes::new(100).choose_multiple(&mut rng, 2);

    println!("{:#?}", primes)
}
