extern crate webassembly_tests_rust;

use webassembly_tests_rust::fibonacci;

#[test]
fn test_fibonacci() {
    assert_eq!(6765, fibonacci(20));
}
