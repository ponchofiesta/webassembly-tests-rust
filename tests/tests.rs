extern crate webassembly_tests_rust;

use webassembly_tests_rust::{fibonacci, hanoi};

#[test]
fn test_fibonacci() {
    assert_eq!(6765, fibonacci(20));
}

#[test]
fn test_hanoi() {

    let expect = "A->C\nA->B\nC->B\nA->C\nB->A\nB->C\nA->C\n";
    let actual = hanoi(3, "A", "B", "C");

    assert_eq!(expect, actual);
}
