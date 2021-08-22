use std::collections::HashMap;

pub fn encrypt(value: &str, key: &str) -> Option<String> {
    if crate::utils::string::is_alphabetic_lowercase(value)
        & crate::utils::string::is_alphabetic_lowercase(key)
    {
        let letters: Vec<char> = crate::utils::string::ENG_ALPHA
            .to_lowercase()
            .chars()
            .collect();

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

pub fn decrypt(value: &str, key: &str) -> Option<String> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_test() {
        let plain = "theydrinkthetea";
        let e = super::encrypt(plain, "duh");
        assert_eq!(e, Some("wblbxylhrwblwyh".to_owned()));
        // println!("{:?}", e);
    }
}
