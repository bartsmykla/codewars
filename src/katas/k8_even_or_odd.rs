#![allow(unused)]

/*
    Kata (8kyu): Even or Odd
    Url: https://www.codewars.com/kata/even-or-odd/train/rust

    Create a function (or write a script in Shell) that takes an integer
    as an argument and returns "Even" for even numbers or "Odd" for odd numbers.
*/

fn even_or_odd(i: i32) -> &'static str {
    if i & 1 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

#[test]
fn returns_expected() {
    assert_eq!(even_or_odd(0), "Even");
    assert_eq!(even_or_odd(2), "Even");
    assert_eq!(even_or_odd(1), "Odd");
    assert_eq!(even_or_odd(7), "Odd");
    assert_eq!(even_or_odd(-1), "Odd");
}
