use crate::Cipher;

pub fn apply(value: usize, mode: Cipher, key: usize) -> Option<usize> {
    Some(value ^ key)
}

#[cfg(test)]
mod tests {
    use crate::Cipher;

    fn enc_dec_success(plain: usize, key: usize) -> bool {
        let encrypted = super::apply(plain, Cipher::E, key).unwrap();
        assert_ne!(encrypted, plain); // possible collision due to XOR, whatev
        let decrypted = super::apply(encrypted, Cipher::D, key).unwrap();
        plain == decrypted
    }

    #[test]
    fn cases() {
        assert!(enc_dec_success(0b1111_1111, 0b1101_0010));
    }
}
