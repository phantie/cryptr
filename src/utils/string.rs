pub static ENG_ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn is_alphabetic(value: &str) -> bool {
    value.chars().all(|c| c.is_alphabetic())
}

fn is_lowercase(value: &str) -> bool {
    value.to_lowercase() == value
}

pub fn is_alphabetic_lowercase(value: &str) -> bool {
    is_alphabetic(value) & is_lowercase(value)
}
