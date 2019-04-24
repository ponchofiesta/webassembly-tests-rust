extern crate webassembly_tests_rust;

use webassembly_tests_rust::tests::{fibonacci, sort};
use webassembly_tests_rust::tests::hanoi;
use webassembly_tests_rust::tests::sort::User;
use webassembly_tests_rust::tests::prime::prime;

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
    let expect: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19];
    let actual = prime(20);

    assert_eq!(&expect, &actual);
}
