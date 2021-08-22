#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod caesar_cipher;

fn main() {
    let plain = "wooshhh";
    let encrypted = caesar_cipher::encrypt(plain, 3);
    let decrypted = caesar_cipher::decrypt(encrypted.as_ref().unwrap(), 3);
    assert_eq!(plain, decrypted.unwrap());
}
