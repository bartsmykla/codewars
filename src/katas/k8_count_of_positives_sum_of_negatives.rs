#![allow(unused)]

/*
    Kata (8kyu): Count of positives / sum of negatives
    Url: https://www.codewars.com/kata/count-of-positives-slash-sum-of-negatives/train/rust

    Given an array of integers.

    Return an array, where the first element is the count of positives numbers
    and the second element is sum of negative numbers.

    If the input array is empty or null, return an empty array.

    Example
        For input [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15],
        you should return [10, -65].
*/

use std::cmp::Ordering::*;

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }

    let (positive, negative) = input.into_iter()
        .fold((0, 0), |(positive, negative), curr| {
            match curr.cmp(&0) {
                Greater => (positive + 1, negative),
                Less => (positive, negative + curr),
                Equal => (positive, negative),
            }
        });

    vec![positive, negative]
}

#[test]
fn returns_expected() {
    let test_data1 = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12,-13, -14, -15,
    ];
    let expected1 = vec![10, -65];
    assert_eq!(count_positives_sum_negatives(test_data1), expected1);
}
