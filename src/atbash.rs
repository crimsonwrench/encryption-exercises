pub fn encode(alphabet: &str, message: &str) -> String {
    message
        .chars()
        .map(|ch| {
            return match alphabet
                .chars()
                .position(|alphabet_ch: char| ch.to_uppercase().next().unwrap() == alphabet_ch)
            {
                Some(index) => {
                    let result: char = alphabet
                        .chars()
                        .nth(alphabet.chars().count() - index - 1)
                        .unwrap();

                    if ch.is_lowercase() {
                        return Some(result.to_lowercase().next().unwrap());
                    }

                    Some(result)
                }
                None => Some(ch),
            };
        })
        .flatten()
        .collect()
}

mod tests {
    #[test]
    fn test_encoding() {
        use crate::atbash::encode;

        [
            "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        ]
        .iter()
        .for_each(|msg: &&str| {
            let encoded_message: String = encode(msg, msg);
            let decoded_message: String = encode(msg, encoded_message.as_str());
            assert_eq!(decoded_message, String::from(*msg));
        });
    }
}
