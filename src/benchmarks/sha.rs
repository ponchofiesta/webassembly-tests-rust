use sha2::{Sha256, Digest, Sha512};

pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.input(data);
    hex::encode(hasher.result())
}

pub fn sha512(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.input(data);
    hex::encode(hasher.result())
}