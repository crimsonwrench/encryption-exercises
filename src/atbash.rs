pub fn encode_atbash(alphabet: &str, message: &str) -> String {
    message
        .chars()
        .map(|ch| {
            return match alphabet
                .to_lowercase()
                .chars()
                .position(|alphabet_ch: char| ch == alphabet_ch)
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
        use crate::atbash::encode_atbash;

        [
            "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        ]
        .iter()
        .for_each(|msg: &&str| {
            let encoded_message: String = encode_atbash(msg, msg);
            let decoded_message: String = encode_atbash(msg, encoded_message.as_str());
            assert_eq!(decoded_message, String::from(*msg));
        });
    }
}
