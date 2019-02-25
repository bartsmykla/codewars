#![allow(unused)]

/*
    Kata (8kyu): Get Nth Even Number
    Url: https://www.codewars.com/kata/get-nth-even-number/train/rust

    Return the Nth Even Number

    nthEven(1) //=> 0, the first even number is 0
    nthEven(3) //=> 4, the 3rd even number is 4 (0, 2, 4)

    nthEven(100) //=> 198
    nthEven(1298734) //=> 2597466

    The input will not be 0.
*/

fn nth_even(n: u32) -> u32 {
    (n - 1) * 2
}

#[test]
fn example_tests() {
    assert_eq!(nth_even(1), 0);
    assert_eq!(nth_even(2), 2);
    assert_eq!(nth_even(3), 4);
    assert_eq!(nth_even(100), 198);
    assert_eq!(nth_even(1298734), 2597466);
}