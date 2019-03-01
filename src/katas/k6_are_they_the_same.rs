#![allow(unused)]

/*
    Kata (6kyu): Are they the "same"?
    Url: https://www.codewars.com/kata/are-they-the-same/train/rust

    Given two arrays a and b write a function comp(a, b)
    (compSame(a, b) in Clojure) that checks whether the two arrays have
    the "same" elements, with the same multiplicities. "Same" means, here,
    that the elements in b are the elements in a squared,
    regardless of the order.

    Examples
        Valid arrays

            a = [121, 144, 19, 161, 19, 144, 19, 11]
            b = [121, 14641, 20736, 361, 25921, 361, 20736, 361]

        comp(a, b) returns true because in b 121 is the square of 11, 14641
        is the square of 121, 20736 the square of 144, 361 the square of 19,
        25921 the square of 161, and so on. It gets obvious if we write
        b's elements in terms of squares:

            a = [121, 144, 19, 161, 19, 144, 19, 11]
            b = [11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19]

        Invalid arrays
        If we change the first number to something else,
        comp may not return true anymore:

            a = [121, 144, 19, 161, 19, 144, 19, 11]
            b = [132, 14641, 20736, 361, 25921, 361, 20736, 361]

        comp(a,b) returns false because in b 132 is not the square
        of any number of a.

            a = [121, 144, 19, 161, 19, 144, 19, 11]
            b = [121, 14641, 20736, 36100, 25921, 361, 20736, 361]

        comp(a,b) returns false because in b 36100 is not the square
        of any number of a.

    Remarks
    a or b might be [] (all languages except R, Shell). a or b might be nil
    or null or None or nothing (except in Haskell, Elixir, C++, Rust, R, Shell,
    PureScript).

    If a or b are nil (or null or None), the problem doesn't make sense
    so return false.

    If a or b are empty then the result is self-evident.
*/

fn comp(mut a: Vec<i64>, mut b: Vec<i64>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    a.sort_unstable_by_key(|&n| n.abs());
    b.sort_unstable_by_key(|&n| n.abs());

    a.into_iter().zip(b).all(|(a, b)| a * a == b)
}

// Still no allocations, with much better best-case times
// O(n + d*log(n)) where d is the number of matching elements before
// the largest non-matching one.
// Solution from user ᚮscottmcmᚭ at Rust community discord server,
// channel: #code-review
fn comp_v2(mut a: Vec<i64>, b: Vec<i64>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    a.iter_mut().for_each(|n| *n *= *n);

    use std::collections::BinaryHeap;
    let mut a = BinaryHeap::from(a);
    let mut b = BinaryHeap::from(b);

    while let (Some(a), Some(b)) = (a.pop(), b.pop()) {
        if a != b {
            return false;
        }
    }

    true
}

fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) {
    assert_eq!(comp(a, b), exp)
}

#[test]
fn tests_comp() {
    let cases = [
        (
            vec![121, 144, 19, 161, 19, 144, 19, 11],
            vec![121, 14641, 20736, 361, 25921, 361, 20736, 361],
            true,
        ),
        (
            vec![121, 144, 19, 161, 19, 144, 19, 11],
            vec![231, 14641, 20736, 361, 25921, 361, 20736, 361],
            false,
        ),
        (
            vec![121, 144, 19, 161, 19, 144, 19, 11],
            vec![121, 14641, 20736, 36100, 25921, 361, 20736, 361],
            false,
        ),
        (vec![], vec![], true),
        (
            vec![121, 144, 19, 161, 19, 144, 19, 11, 1008],
            vec![121, 14641, 20736, 36100, 25921, 361, 20736, 361],
            false,
        ),
        (
            vec![10000000, 100000000],
            vec![100000000000000, 10000000000000000],
            true,
        ),
    ];

    for (a, b, expected) in cases.iter().cloned() {
        testing(a, b, expected);
    }
}
