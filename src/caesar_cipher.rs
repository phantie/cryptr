use std::collections::HashMap;

fn get_transition_map(shift: usize, invert: bool) -> HashMap<char, char> {
    let letters: Vec<char> = crate::utils::string::ENG_ALPHA
        .to_lowercase()
        .chars()
        .collect();
    let shift = shift % letters.len();
    letters
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            let pair = (*c, {
                if idx + shift >= letters.len() {
                    letters[idx + shift - letters.len()]
                } else {
                    letters[idx + shift]
                }
            });
            if invert {
                (pair.1, pair.0)
            } else {
                pair
            }
        })
        .collect::<HashMap<_, _>>()
}

pub fn encrypt(value: &str, shift: usize) -> Option<String> {
    if crate::utils::string::is_alphabetic_lowercase(value) {
        let transition_map = &get_transition_map(shift, false);
        Some(
            value
                .chars()
                .map(|c| transition_map.get(&c).unwrap())
                .collect::<String>(),
        )
    } else {
        None
    }
}

pub fn decrypt(value: &str, shift: usize) -> Option<String> {
    if crate::utils::string::is_alphabetic_lowercase(value) {
        let transition_map = &get_transition_map(shift, true);
        Some(
            value
                .chars()
                .map(|c| transition_map.get(&c).unwrap())
                .collect::<String>(),
        )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_test() {
        let plain = "wooshhh";
        let encrypted = super::encrypt(plain, 3).unwrap();
        assert_ne!(encrypted, plain);
        let decrypted = super::decrypt(&encrypted, 3).unwrap();
        assert_eq!(plain, decrypted);
    }
}
