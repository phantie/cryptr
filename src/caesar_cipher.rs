use std::collections::HashMap;

use crate::Cipher;

fn get_transition_map(shift: usize) -> HashMap<char, char> {
    let letters: Vec<char> = ('a'..='z').collect();

    let shift = shift % letters.len();
    letters
        .iter()
        .enumerate()
        .map(|(idx, c)| (*c, letters[(idx + shift) % letters.len()]))
        .collect::<HashMap<_, _>>()
}

pub fn apply(value: &str, mode: Cipher, shift: usize) -> Option<String> {
    if crate::utils::string::is_alphabetic_lowercase(value) {
        let mut transition_map = get_transition_map(shift);
        if let Cipher::D = mode {
            transition_map = crate::utils::hash_map::invert(&transition_map);
        }
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
    use crate::Cipher;

    #[test]
    fn basic_test() {
        let plain = "wooshhh";
        let encrypted = super::apply(plain, Cipher::E, 3).unwrap();
        assert_ne!(encrypted, plain);
        let decrypted = super::apply(&encrypted, Cipher::D, 3).unwrap();
        assert_eq!(plain, decrypted);
    }
}
