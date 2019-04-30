extern crate aes_soft as aes;
extern crate block_modes;

use self::aes::Aes128;
use self::block_modes::{BlockMode, Cbc};
use self::block_modes::block_padding::Pkcs7;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub fn aes_encrypt(key: &[u8], iv: &[u8], data: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_var(&key, &iv).unwrap();
    let encrypted = cipher.encrypt_vec(data);
    encrypted
}

pub fn aes_decrypt(key: &[u8], iv: &[u8], data: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_var(&key, &iv).unwrap();
    let decrypted = cipher.decrypt_vec(&data).unwrap();
    decrypted
}