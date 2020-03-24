mod symmetric;
mod primes;
mod rsa;

use crate::rsa::Rsa;
use std::io;
use crate::symmetric::Symmetric;

fn main() {
    println!("Generating rsa keys...");
    let rsa: Rsa = Rsa::new();
    loop {
        println!("---Encryption exercises---");
        println!("(1) Encode input using RSA algorithm");
        println!("(2) Decode input using RSA algorithm");
        println!("(3) Encode input using Atbash cipher");
        println!("(4) Encode input using Caesar cipher");
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
            3 => encode_atbash(),
            4 => encode_caesar(),
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

fn encode_atbash() {
    println!("Enter the message to be encoded");
    let mut message = String::new();

    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    println!("---Select alphabet---");
    println!("(1) Cyrillic");
    println!("(2) Latin");

    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    let command: usize = match command.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let alphabet: &str = match command {
        1 => "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ",
        _ => "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    };

    println!("Encoded message: {}", Symmetric::new(alphabet).atbash(&message));
}

fn encode_caesar() {
    println!("Enter the message to be encoded");
    let mut message = String::new();

    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    println!("---Select alphabet---");
    println!("(1) Cyrillic");
    println!("(2) Latin");

    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    let command: usize = match command.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let alphabet: &str = match command {
        1 => "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ",
        _ => "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    };

    println!("Enter shift");
    let mut shift = String::new();

    io::stdin()
        .read_line(&mut shift)
        .expect("Failed to read line");

    let shift: i32 = match shift.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("Encoded message: {}", Symmetric::new(alphabet).caesar(&message, shift));
}
