#![allow(unused)]

/*
    Kata (7kyu): Form The Minimum
    Url: https://www.codewars.com/kata/form-the-minimum/train/rust

    Task
    Given a list of digits, return the smallest number that could be formed
    from these digits, using the digits only once ( ignore duplicates).

    Notes:
    Only positive integers will be passed to the function (> 0 ),
    no negatives or zeros.
    Input >> Output Examples
    1- minValue ({1, 3, 1})  ==> return (13)
    Explanation:
    (13) is the minimum number could be formed from {1, 3, 1},
    Without duplications

    2- minValue({5, 7, 5, 9, 7})  ==> return (579)
    Explanation:
    (579) is the minimum number could be formed from {5, 7, 5, 9, 7},
    Without duplications

    3- minValue({1, 9, 3, 1, 7, 4, 6, 6, 7}) return  ==> (134679)
    Explanation:
    (134679) is the minimum number could be formed from
    {1, 9, 3, 1, 7, 4, 6, 6, 7}, Without duplications
*/

fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();

    digits.iter().fold(0, |acc, curr| acc * 10 + curr)
}

#[test]
fn basic_test() {
    assert_eq!(min_value(vec![1, 3, 1]), 13);
    assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
    assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}