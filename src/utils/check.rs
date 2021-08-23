use std::fmt::Debug;

// use crate::Cipher;

// pub fn enc_dec_success<F, P, K>(f: F, plain: P, key: K) -> bool 
//     where P: PartialEq + Debug, F: Fn(&P, Cipher, &K) -> Option<P> {
//     let encrypted = f(&plain, Cipher::E, &key).unwrap();
//     assert_ne!(encrypted, plain);
//     let decrypted = f(&encrypted, Cipher::D, &key).unwrap();
//     plain == decrypted
// }