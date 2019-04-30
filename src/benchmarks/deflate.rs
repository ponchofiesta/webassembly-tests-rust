extern crate deflate;

use self::deflate::{deflate_bytes};
use inflate::{inflate_bytes};

pub fn deflate(data: &[u8]) -> Vec<u8> {
    deflate_bytes(data)
}

pub fn inflate(data: &[u8]) -> Vec<u8> {
    inflate_bytes(data).unwrap()
}