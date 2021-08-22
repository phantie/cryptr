use std::{collections::HashMap, hash::Hash};

fn is_cypherable(value: &str) -> bool {
    fn is_string_alphabetic(value: &str) -> bool {
        value.chars().all(|c| c.is_alphabetic())
    }

    fn is_string_lowercase(value: &str) -> bool {
        value.to_lowercase() == value
    }

    is_string_alphabetic(value) & is_string_lowercase(value)
}

fn get_transition_map(shift: usize) -> HashMap<char, char> {
    let letters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .to_lowercase()
        .chars()
        .collect();
    let shift = shift % letters.len();
    letters
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            (*c, {
                if idx + shift >= letters.len() {
                    letters[idx + shift - letters.len()]
                } else {
                    letters[idx + shift]
                }
            })
        })
        .collect::<HashMap<_, _>>()
}

fn invert_hashmap<K, V>(map: &HashMap<K, V>) -> HashMap<V, K>
where
    V: Eq + Hash + Clone,
    K: Clone,
{
    map.iter()
        .map(|(k, v)| ((*v).clone(), (*k).clone()))
        .collect::<HashMap<_, _>>()
}

pub fn encrypt(value: &str, shift: usize) -> Option<String> {
    match is_cypherable(value) {
        false => None,
        true => {
            let transition_map = get_transition_map(shift);
            Some(
                value
                    .chars()
                    .map(|c| transition_map.get(&c).unwrap())
                    .collect::<String>(),
            )
        }
    }
}

pub fn decrypt(value: &str, shift: usize) -> Option<String> {
    match is_cypherable(value) {
        false => None,
        true => {
            let transition_map = invert_hashmap(&get_transition_map(shift));
            Some(
                value
                    .chars()
                    .map(|c| transition_map.get(&c).unwrap())
                    .collect::<String>(),
            )
        }
    }
}