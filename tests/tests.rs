//#![feature(test)]

extern crate webassembly_benchmarks_rust;
extern crate wasm_bindgen;
//extern crate test;

use webassembly_benchmarks_rust::benchmarks::{fibonacci, sort};
use webassembly_benchmarks_rust::benchmarks::base64;
use webassembly_benchmarks_rust::benchmarks::hanoi;
use webassembly_benchmarks_rust::benchmarks::sort::User;
use webassembly_benchmarks_rust::benchmarks::prime::prime;
use webassembly_benchmarks_rust::benchmarks::aes::{aes_encrypt, aes_decrypt};
use webassembly_benchmarks_rust::benchmarks::deflate::{deflate, inflate};
use wasm_bindgen::Clamped;
//use test::Bencher;
use webassembly_benchmarks_rust::benchmarks::sha::{sha256, sha512};

#[test]
fn test_base64() {
    let data = vec![1, 2, 3];
    let expected = "AQID";
    let actual = base64::base64(&data);
    assert_eq!(&expected, &actual);
}

#[test]
fn test_fibonacci() {
    assert_eq!(6765, fibonacci::fibonacci(20));
}

#[test]
fn test_hanoi() {
    let expect = "A->C\nA->B\nC->B\nA->C\nB->A\nB->C\nA->C\n";

    let mut hanoi = hanoi::Hanoi::new();
    let actual = hanoi.hanoi(3, "A", "B", "C");

    assert_eq!(expect, actual);
}

#[test]
fn test_sort() {
    let expect = vec![
        User{id: 2, name: String::from("alf")},
        User{id: 1, name: String::from("hans")},
        User{id: 3, name: String::from("peter")},
    ];
    let mut data = vec![
        User{id: 1, name: String::from("hans")},
        User{id: 3, name: String::from("peter")},
        User{id: 2, name: String::from("alf")},
    ];
    let actual = sort::sort(&mut data);

    assert_eq!(&expect[..], &actual[..]);
}

#[test]
fn test_prime() {
    let expect = vec![2usize, 3, 5, 7, 11, 13, 17, 19];
    let actual = prime(20);

    assert_eq!(&expect, &actual);
}

#[test]
fn test_sha256() {
    let expect = "66840dda154e8a113c31dd0ad32f7f3a366a80e8136979d8f5a101d3d29d6f72";
    let actual = sha256(&vec![1, 2, 3, 4, 5, 6, 7, 8]);

    assert_eq!(&expect, &actual);
}

#[test]
fn test_sha512() {
    let expect = "1818cc2acd207880a07afc360fd0da87e51ccf17e7c604c4eb16be5788322724c298e1fcc66eb293926993141ef0863c09eda383188cf5df49b910aacac17ec5";
    let actual = sha512(&vec![1, 2, 3, 4, 5, 6, 7, 8]);

    assert_eq!(&expect, &actual);
}

#[test]
fn test_aes() {
    let key = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let iv = [17u8, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
    let text = "abcdefghijklmnop";
    let data = text.as_bytes();
    let data = data.to_vec();

    let expect = vec![239u8, 202, 78, 31, 15, 43, 252, 66, 190, 102, 119, 142, 185, 132, 167, 184,
                                12, 193, 161, 159, 255, 132, 215, 89, 4, 218, 165, 255, 191, 96, 35, 63];
    let encrypted = aes_encrypt(&key, &iv, &data);

    assert_eq!(&expect, &encrypted);

    let decrypted = aes_decrypt(&key, &iv, &encrypted);
    let decrypted_text = std::str::from_utf8(&decrypted).unwrap();

    assert_eq!(&text, &decrypted_text);
}

#[test]
fn test_deflate() {
    let text = "abcdefghijklmnop";
    let data = text.as_bytes();

    let expect = vec![75u8, 76, 74, 78, 73, 77, 75, 207, 200, 204, 202, 206, 201, 205, 203, 47, 0, 0];

    let compressed = deflate(data);

    assert_eq!(&expect, &compressed);

    let decompressed = inflate(&compressed);
    let decompressed_text = std::str::from_utf8(&decompressed).unwrap();

    assert_eq!(&text, &decompressed_text);
}

//#[bench]
//fn bench_iterate(b: &mut Bencher) {
//    b.iter(|| webassembly_benchmarks_rust::iterate(n))
//}