use crate::Cipher;
use std::collections::HashMap;

pub fn apply(value: &str, mode: Cipher, key: &str) -> Option<String> {
    if crate::utils::string::is_alphabetic_lowercase(value)
        & crate::utils::string::is_alphabetic_lowercase(key)
    {
        let letters: Vec<char> = ('a'..='z').collect();

        let letter_to_idx = letters
            .iter()
            .enumerate()
            .map(|(idx, c)| (*c, idx))
            .collect::<HashMap<_, _>>();

        let idx_to_letter = crate::utils::hash_map::invert(&letter_to_idx);

        let mut offsets = key
            .chars()
            .map(|c| *letter_to_idx.get(&c).unwrap())
            .into_iter()
            .cycle();

        Some(
            value
                .chars()
                .map(|c| {
                    idx_to_letter
                        .get(&{
                            let offset = offsets.next().unwrap();
                            let idx = *letter_to_idx.get(&c).unwrap();
                            match mode {
                                Cipher::E => (idx + offset) % letters.len(),
                                Cipher::D => (letters.len() + idx - offset) % letters.len(),
                            }
                        })
                        .unwrap()
                })
                .collect::<String>(),
        )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::Cipher;

    fn enc_dec_success(plain: &str, key: &str) -> bool {
        let encrypted = super::apply(plain, Cipher::E, key).unwrap();
        assert_ne!(encrypted, plain);
        let decrypted = super::apply(&encrypted, Cipher::D, key).unwrap();
        plain == decrypted
    }

    #[test]
    fn cases() {
        assert!(enc_dec_success("theydrinkthetea", "duh"));
        assert!(enc_dec_success("theydrinkthetea", "helloworld"));
        assert!(enc_dec_success("plain", "secret"));
    }
}
