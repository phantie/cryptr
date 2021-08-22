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
                            (idx + offset) % letters.len()
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

    #[test]
    fn basic_test() {
        let plain = "theydrinkthetea";
        let encrypted = super::apply(plain, Cipher::E, "duh").unwrap();
        assert_eq!(encrypted, "wblbxylhrwblwyh".to_owned());
        // let decrypted = super::apply(&encrypted, Cipher::D, "duh").unwrap();
        // // println!("{:?}", e);
        // assert_eq!(plain, decrypted);
    }
}
