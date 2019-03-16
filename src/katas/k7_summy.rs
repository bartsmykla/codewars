#![allow(unused)]

/*
    Kata (7kyu): Summy
    Url: https://www.codewars.com/kata/summy/train/rust

    Write a function that takes a string which has integers inside it
    separated by spaces, and your task is to convert each integer
    in the string into an integer and return their sum.

    Example
    summy("1 2 3") -> 6
    Good luck!
*/

fn summy(s: &str) -> i32 {
    s.split_whitespace().flat_map(str::parse::<i32>).sum()
}

#[test]
fn sample_tests() {
    assert_eq!(summy("1 2 3"), 6);
    assert_eq!(summy("1 2 3 4"), 10);
    assert_eq!(summy("1 2 3 4 5"), 15);
    assert_eq!(summy("10 10"), 20);
    assert_eq!(summy("0 0"), 0);
}
