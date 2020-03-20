mod primes;
mod rsa;

use crate::rsa::Rsa;
use std::io;

fn main() {
    println!("---Generating rsa keys---");
    let rsa: Rsa = Rsa::new();
    loop {
        println!("---Encryption exercises---");
        println!("(1) Encode input using RSA algorithm");
        println!("(2) Decode input using RSA algorithm");
        println!("(0) Exit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match command {
            1 => encode_rsa(&rsa),
            2 => decode_rsa(&rsa),
            0 => break,
            _ => {
                println!("Unknown command!");
                continue;
            }
        }
    }
}

fn encode_rsa(rsa: &Rsa) {
    println!("Enter the message to be encoded");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let encoded_message: String = rsa.encode(input.trim());
    println!("Encoded message: {}", encoded_message);
}

fn decode_rsa(rsa: &Rsa) {
    println!("Enter the message to be decoded");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let decoded_message: String = rsa.decode(input.trim()).unwrap();
    println!("Decoded message: {}", decoded_message);
}
