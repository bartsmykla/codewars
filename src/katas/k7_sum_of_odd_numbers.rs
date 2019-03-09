#![allow(unused)]

/*
    Kata (7kyu): Sum of odd numbers
    Url: https://www.codewars.com/kata/sum-of-odd-numbers/train/rust

    Given the triangle of consecutive odd numbers:

                 1
              3     5
           7     9    11
       13    15    17    19
    21    23    25    27    29
    ...

    Calculate the row sums of this triangle from the row index
    (starting at index 1) e.g.:

        row_sum_odd_numbers(1); # 1
        row_sum_odd_numbers(2); # 3 + 5 = 8
*/

// my first solution
fn row_sum_odd_numbers(n: i64) -> i64 {
    let starting_point = (0..n).map(|n| n * 2).sum::<i64>() + 1;
    let ending_point = starting_point + (n - 1) * 2;

    (starting_point..=ending_point)
        .filter(|&n| n % 2 == 1)
        .sum()
}

fn row_sum_odd_numbers_v2(n: i64) -> i64 {
    n.pow(3)
}

#[test]
fn returns_expected() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(2), 8);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}
