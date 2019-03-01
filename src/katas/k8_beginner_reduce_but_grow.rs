#![allow(unused)]

/*

    Kata (8kyu): Beginner - Reduce but Grow
    Url: https://www.codewars.com/kata/beginner-reduce-but-grow/rust

    Given a non-empty array of integers, return the result of multiplying
    the values together in order. Example:

    [1, 2, 3, 4] => 1 * 2 * 3 * 4 = 24
*/

fn grow(array: Vec<i32>) -> i32 {
    array.iter().product()
}

#[test]
fn basic_test() {
    assert_eq!(grow(vec![1, 2, 3]), 6);
    assert_eq!(grow(vec![4, 1, 1, 1, 4]), 16);
    assert_eq!(grow(vec![2, 2, 2, 2, 2, 2]), 64);
}

/*
    Questions?
        - What is the name of a method over iterators to get the product
          of all numbers from them combined?

          `.product()`
*/
