#![allow(unused)]

/*
    Kata (7kyu): Is It Negative Zero (-0)?
    Url: https://www.codewars.com/kata/is-it-negative-zero-0/train/rust

    There exist two zeroes: +0 (or just 0) and -0.

    Write a function that returns true if the input number is -0
    and false otherwise (True and False for Python).

    In JavaScript / TypeScript / CoffeeScript the input will be a number.

    In Python / Java / C / Haskell the input will be a float.
*/

fn is_negative_zero(n: f32) -> bool {
    n == 0.0 && n.is_sign_negative()
}

use std::f32;

#[test]
fn sample_tests() {
    assert_eq!(is_negative_zero(-0.0), true);
    assert_eq!(is_negative_zero(f32::NEG_INFINITY), false);
    assert_eq!(is_negative_zero(-5.0), false);
    assert_eq!(is_negative_zero(-4.0), false);
    assert_eq!(is_negative_zero(-3.0), false);
    assert_eq!(is_negative_zero(-2.0), false);
    assert_eq!(is_negative_zero(-1.0), false);
    assert_eq!(is_negative_zero(-f32::MIN), false);
    assert_eq!(is_negative_zero(0.0), false);
    assert_eq!(is_negative_zero(f32::MIN), false);
    assert_eq!(is_negative_zero(1.0), false);
    assert_eq!(is_negative_zero(2.0), false);
    assert_eq!(is_negative_zero(3.0), false);
    assert_eq!(is_negative_zero(4.0), false);
    assert_eq!(is_negative_zero(5.0), false);
    assert_eq!(is_negative_zero(f32::INFINITY), false);
}
