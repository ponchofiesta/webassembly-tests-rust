extern crate webassembly_tests_rust;

use webassembly_tests_rust::tests::{fibonacci, sort};
use webassembly_tests_rust::tests::hanoi;
use webassembly_tests_rust::tests::sort::User;
use webassembly_tests_rust::tests::prime::prime;
use webassembly_tests_rust::tests::aes::aes;

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
fn test_aes() {
    let key = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let iv = [17u8, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
    let data = "abcdefghijklmnop".as_bytes();

    let expect = vec![1u8];
    let actual = aes(&key, &iv, &data);

    assert_eq!(&expect, &actual);
}
