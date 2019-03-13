#![allow(unused)]

/*
    Kata (7kyu): SequenceSum
    Url: https://www.codewars.com/kata/5436f26c4e3d6c40e5000282/train/rust

    Sum of 'n' Numbers
    A sequence or a series, in mathematics, is a string of objects,
    like numbers, that follow a particular pattern. The individual elements
    in a sequence are called terms.

    For example, some simple patterns include: 3, 6, 9, 12, 15, 18, 21, ...
    Pattern: "add 3 to the previous number to get the next number"

    0, 12, 24, 36, 48, 60, 72, ...
    Pattern: "add 12 to the previous number to get the next number"

    How about generating a more complicated pattern?
    0, 1, 3, 6, 10, 15, 21, 28, ...
    0(0)_
    ,1
    (0+1)
    ,3
    (0+1+2)
    ,6
    (0+1+2+3)_...
    Pattern: "thenth term is the sum of numbers(from 0 ton, inclusive)"

    sum_of_n (or SequenceSum.sumOfN in Java, SequenceSum.SumOfN in C#)
    takes an integer n and returns a List (an Array in Java/C#) of length
    abs(n) + 1. The List/Array contains the numbers in the arithmetic series
    produced by taking the sum of the consecutive integer numbers from 0
    to n inclusive.

    n can also be 0 or a negative value.
    Wikipedia reference for abs value is available here.

    Example:
    5 -> [0, 1, 3, 6, 10, 15]

    -5 -> [0, -1, -3, -6, -10, -15]

    7 -> [0, 1, 3, 6, 10, 15, 21, 28]
*/

fn sum_of_n(n: i32) -> Vec<i32> {
    let mut results = vec![];
    let mut x = 0;

    for i in 0..=n.abs() {
        x += i;

        results.push(x * n.signum());
    }

    results
}

#[test]
fn normal_tests() {
    assert_eq!(sum_of_n(3), vec![0, 1, 3, 6]);
    assert_eq!(sum_of_n(-4), vec![0, -1, -3, -6, -10]);
    assert_eq!(sum_of_n(1), vec![0, 1]);
    assert_eq!(sum_of_n(0), vec![0]);
    assert_eq!(sum_of_n(10), vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
}
