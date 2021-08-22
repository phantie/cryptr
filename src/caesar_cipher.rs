use std::{collections::HashMap, hash::Hash};

fn is_cipherable(value: &str) -> bool {
    fn is_string_alphabetic(value: &str) -> bool {
        value.chars().all(|c| c.is_alphabetic())
    }

    fn is_string_lowercase(value: &str) -> bool {
        value.to_lowercase() == value
    }

    is_string_alphabetic(value) & is_string_lowercase(value)
}

fn get_transition_map(shift: usize, invert: bool) -> HashMap<char, char> {
    let letters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
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

// fn invert_hashmap<K, V>(map: &HashMap<K, V>) -> HashMap<V, K>
// where
//     V: Eq + Hash + Clone,
//     K: Clone,
// {
//     map.iter()
//         .map(|(k, v)| ((*v).clone(), (*k).clone()))
//         .collect::<HashMap<_, _>>()
// }

pub fn encrypt(value: &str, shift: usize) -> Option<String> {
    if is_cipherable(value) {
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
    if is_cipherable(value) {
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
