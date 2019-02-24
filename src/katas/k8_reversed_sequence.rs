#![allow(unused)]

/*
    Kata (8kyu): Reversed sequence
    Url: https://www.codewars.com/kata/reversed-sequence/train/rust

    Get the number n (n>0) to return the reversed sequence from n to 1.

    Example: n=5 >> [5,4,3,2,1]
*/

fn reverse_seq(n: u32) -> Vec<u32> {
    (0 .. n).map(|x| n - x).collect()
}

#[test]
fn sample_test() {
    assert_eq!(reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
    assert_eq!(reverse_seq(6), [6, 5, 4, 3, 2, 1].to_vec());
}