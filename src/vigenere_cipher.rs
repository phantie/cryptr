use std::collections::HashMap;

fn encrypt(value: &str, key: &str) -> Option<String> {
    if crate::utils::string::is_alphabetic_lowercase(value) {
        Some("".to_owned())
    } else {
        None
    }
}

fn decrypt(value: &str, key: &str) -> Option<String> {
    unimplemented!()
}
