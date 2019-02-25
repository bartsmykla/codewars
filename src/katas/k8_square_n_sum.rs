#![allow(unused)]

/*
    Kata (8kyu): Square(n) Sum
    Url: https://www.codewars.com/kata/square-n-sum

    Complete the square sum method so that it squares each number passed
    into it and then sums the results together.

    For example: squareSum([1, 2, 2]) should return 9 because
    1^2 + 2^2 + 2^2 = 9.
*/

fn square_sum(vec: Vec<i32>) -> i32 {
    vec.into_iter().map(|x| x * x).sum()
}

#[test]
fn returns_expected() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
}