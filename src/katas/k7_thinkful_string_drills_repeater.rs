#![allow(unused)]

/*
    Kata: https://www.codewars.com/kata/thinkful-string-drills-repeater/train/rust

    Write a class function named repeat() that takes two arguments
    (a string and a long integer), and returns a new string where
    the input string is repeated that many times.

    Example:
        Repeater.repeat("a", 5) should return "aaaaa"
*/

fn repeater(string: &str, n: u32) -> String {
    string.repeat(n as usize)
}

#[test]
fn basic_test() {
    assert_eq!(repeater("a", 5), "aaaaa");
    assert_eq!(repeater("Na", 16), "NaNaNaNaNaNaNaNaNaNaNaNaNaNaNaNa");
    assert_eq!(repeater("Wub ", 6), "Wub Wub Wub Wub Wub Wub ");
}
