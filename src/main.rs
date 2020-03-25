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
        println!("(5) Encode input using Gronsfeld cipher");
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
            5 => encode_gronsfeld(),
            0 => break,
            _ => {
                println!("Unknown command!");
                continue;
            }
        }
    }
}

fn encode_rsa(rsa: &Rsa) {
    let input: String = get_message();
    let encoded_message: String = rsa.encode(input.trim());
    println!("Encoded message: {}", encoded_message);
}

fn decode_rsa(rsa: &Rsa) {
    let input: String = get_message();
    let decoded_message: String = rsa.decode(input.trim()).unwrap();
    println!("Decoded message: {}", decoded_message);
}

fn encode_atbash() {
    let input: String = get_message();
    let alphabet: String = get_alphabet();
    println!("Encoded message: {}", Symmetric::new(&alphabet).atbash(&input));
}

fn encode_caesar() {
    println!("Enter the message to be encoded");
    let mut message = String::new();

    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    let alphabet: String = get_alphabet();
    let mut shift = String::new();

    println!("Enter shift value");
    io::stdin()
        .read_line(&mut shift)
        .expect("Failed to read line");

    let shift: i32 = match shift.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("Encoded message: {}", Symmetric::new(&alphabet).caesar(&message, shift));
}

fn encode_gronsfeld() {
    let message: String = get_message();
    let alphabet: String = get_alphabet();

    println!("Enter shift key one by one. Enter blank to stop");

    let mut key: Vec<i32> = Vec::new();
    let mut input = String::new();

    loop {
        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => key.push(num),
            Err(_) => break,
        };
    }

    println!("Encoded message: {}", Symmetric::new(&alphabet).gronsfeld(&message, &key));
}

fn get_message() -> String {
    println!("Enter the message to be encoded");
    let mut message = String::new();

    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    message
}

fn get_alphabet() -> String {
    println!("---Select alphabet---");
    println!("(1) Cyrillic");
    println!("(2) Latin");

    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    let command: usize = match command.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    match command {
        1 => String::from("АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"),
        _ => String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }
}
