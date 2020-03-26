pub struct Symmetric {
    chars_lc: String,
    chars_uc: String,
    size: usize
}

impl Symmetric {
    pub fn new(characters: &str) -> Self {
        Symmetric {
            chars_lc: String::from(characters).to_lowercase(),
            chars_uc: String::from(characters).to_uppercase(),
            size: characters.chars().count()
        }
    }

    pub fn atbash(&self, message: &str) -> String {
        message
            .chars()
            .map(|ch: char| {
                let alphabet_case: &String = self.get_case(&ch);
                match alphabet_case.chars().position(|alphabet_ch: char| alphabet_ch == ch) {
                    Some(position) => self.shift_character(alphabet_case, self.size, -(position as i32  + 1)),
                    None => ch
                }
            }).collect()
    }

    pub fn caesar(&self, message: &str, shift: i32) -> String {
        message
            .chars()
            .map(|ch: char| {
                let alphabet_case: &String = self.get_case(&ch);
                match alphabet_case.chars().position(|alphabet_ch: char| alphabet_ch == ch) {
                    Some(position) => self.shift_character(alphabet_case, position, shift),
                    None => ch
                }
            }).collect()
    }

    pub fn gronsfeld(&self, message: &str, key: &Vec<i32>) -> String {
        message
            .chars()
            .enumerate()
            .map(|(index, ch)| {
                let alphabet_case: &String = self.get_case(&ch);
                match alphabet_case.chars().position(|alphabet_ch: char| alphabet_ch == ch) {
                    Some(position) => self.shift_character(alphabet_case, position, *key.iter().nth(index % key.len()).unwrap()),
                    None => ch
                }
            }).collect()
    }

    fn get_case(&self, char: &char) -> &String {
        if char.is_lowercase() {
            return &self.chars_lc;
        }

        &self.chars_uc
    }

    fn shift_character(&self, alphabet: &String, position: usize, value: i32) -> char {
        alphabet
            .chars()
            .nth(((position as i32 + value).rem_euclid(self.size as i32)) as usize)
            .unwrap()
    }
}

mod tests {

    #[test]
    fn atbash_symmetry() {
        use crate::symmetric::Symmetric;
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
            let cipher = Symmetric::new(alphabet);
            sentences.iter().for_each(|sentence| {
                let encoded_string: String = cipher.atbash(sentence);
                assert_eq!(
                    cipher.atbash(&encoded_string),
                    String::from(*sentence)
                );
            })
        }
    }

    #[test]
    fn caesar_symmetry() {
        use crate::symmetric::Symmetric;
        use rand::thread_rng;
        use rand::Rng;

        let latin_alphabet: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let random_shift: i32 = thread_rng().gen_range(-1 * latin_alphabet.len() as i32, latin_alphabet.len() as i32);

        let cipher = Symmetric::new(latin_alphabet);

        vec![
            "The quick brown fox jumps over the lazy dog",
            "test",
            "TEST"
        ].iter()
            .for_each(|sentence| {
                let encoded_sentence: String = cipher.caesar(*sentence, random_shift);
                assert_eq!(
                    cipher.caesar(&encoded_sentence, -1 * random_shift),
                    String::from(*sentence)
                )
            })
    }

    #[test]
    fn gronsfeld_symmetry() {
        use crate::symmetric::Symmetric;
        use std::collections::HashMap;
        use rand::Rng;
        use rand::thread_rng;

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
            let cipher = Symmetric::new(alphabet);

            // Generate a key of random length from 1 to alphabet length and of random numbers from 1 to 9
            let random_shift_length = thread_rng().gen_range(1, alphabet.len());
            let key: Vec<i32> = (0..random_shift_length).map(|_| thread_rng().gen_range(1, 9)).collect();

            sentences.iter().for_each(|sentence| {
                let encoded_string: String = cipher.gronsfeld(sentence, &key);

                // Decode the string by reversing the key and applying it on the encoded string
                let key_reversed: Vec<i32> = key.iter().map(|el| -el).collect();

                assert_eq!(
                    cipher.gronsfeld(&encoded_string, &key_reversed),
                    String::from(*sentence)
                );
            })
        }
    }
}