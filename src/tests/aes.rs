use openssl::{symm};

pub fn aes(key: &[u8], iv: &[u8], data: &[u8]) -> Vec<u8> {
    let cipher = symm::Cipher::aes_128_cbc();
    let encrypted = symm::encrypt(cipher, &key, Some(&iv), &data).unwrap();
    encrypted
}
