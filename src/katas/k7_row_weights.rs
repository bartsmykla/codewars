#![allow(unused)]

/*
    Kata (7kyu): Row Weights
    Url: https://www.codewars.com/kata/row-weights/train/rust

    Scenario
    Several people are standing in a row divided into two teams.
    The first person goes into team 1, the second goes into team 2,
    the third goes into team 1, and so on.

    Task
    Given an array of positive integers (the weights of the people),
    return a new array/tuple of two integers, where the first one
    is the total weight of team 1, and the second one
    is the total weight of team 2.

    Notes
    Array size is at least 1.
    All numbers will be positive.
    Input >> Output Examples
    1- rowWeights([13, 27, 49])  ==>  return (62, 27)
    Explanation:
    The first element 62 is the total weight of team 1,
    and the second element 27 is the total weight of team 2.

    2- rowWeights([50, 60, 70, 80])  ==>  return (120, 140)
    Explanation:
    The first element 120 is the total weight of team 1,
    and the second element 140 is the total weight of team 2.

    3- rowWeights([80])  ==>  return (80, 0)
    Explanation:
    The first element 80 is the total weight of team 1,
    and the second element 0 is the total weight of team 2.
*/

fn row_weights(array: Vec<u32>) -> (u32, u32) {
    array.into_iter().enumerate().fold(
        (0, 0),
        |(a, b), (i, n)| {
            if i % 2 == 0 {
                (a + n, b)
            } else {
                (a, b + n)
            }
        },
    )
}

#[test]
fn basic_tests() {
    assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
    assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
    assert_eq!(row_weights(vec![80]), (80, 0));
}
