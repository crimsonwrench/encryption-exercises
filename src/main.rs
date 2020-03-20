mod primes;
mod rsa;

use rsa::Rsa;

fn main() {
    let rsa: Rsa = Rsa::new();
    println!("{:?}", rsa.encode(String::from("Test")));
}
