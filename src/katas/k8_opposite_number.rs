#![allow(unused)]

/*
    Kata (8kyu): Opposite number
    Url: https://www.codewars.com/kata/opposite-number

    Very simple, given a number, find its opposite.

    Examples:
        1: -1
        14: -14
        -34: 34

    But can you do it in 1 line of code and without any conditionals?
*/

fn opposite(number: i32) -> i32 {
    -number
}

#[test]
fn returns_expected() {
    assert_eq!(opposite(1), -1);
    assert_eq!(opposite(-1), 1);
}
