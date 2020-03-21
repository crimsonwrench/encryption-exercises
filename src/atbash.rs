fn replace(ch: &char, alphabet: &String, alphabet_size: &usize) -> char {
    match alphabet
        .chars()
        .position(|alphabet_ch: char| alphabet_ch == *ch)
    {
        Some(index) => alphabet.chars().nth(alphabet_size - index - 1).unwrap(),
        None => *ch,
    }
}

pub fn encode(alphabet: &str, message: &str) -> String {
    let alphabet_uppercase: String = alphabet.to_uppercase();
    let alphabet_lowercase: String = alphabet.to_lowercase();
    let alphabet_size: usize = alphabet.chars().count();

    message
        .chars()
        .map(|ch: char| match ch {
            'A'..='Z' | 'А'..='Я' | 'Ё' => replace(&ch, &alphabet_uppercase, &alphabet_size),
            'a'..='z' | 'а'..='я' | 'ё' => replace(&ch, &alphabet_lowercase, &alphabet_size),
            _ => ch,
        })
        .collect()
}

mod tests {
    #[test]
    fn atbash_symmetry() {
        use crate::atbash;
        use std::collections::HashMap;

        let mut tests = HashMap::new();

        tests.insert(
            "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ",
            vec![
                "тест",
                "ТЕСТ",
                "Съешь ещё этих мягких французских булок, да выпей же чаю",
            ],
        );
        tests.insert(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            vec![
                "test",
                "TEST",
                "The quick brown fox jumps over the lazy dog",
            ],
        );

        for (alphabet, sentences) in tests {
            sentences.iter().for_each(|sentence| {
                let encoded_string: String = atbash::encode(alphabet, sentence);
                assert_eq!(
                    atbash::encode(alphabet, encoded_string.as_str()),
                    String::from(*sentence)
                );
            })
        }
    }
}
