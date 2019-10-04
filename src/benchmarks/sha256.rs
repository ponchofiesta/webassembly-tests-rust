use sha2::{Sha256, Digest};

pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.input(data);
    hex::encode(hasher.result())
}