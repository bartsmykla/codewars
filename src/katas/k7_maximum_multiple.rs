#![allow(unused)]

/*
    Kata (7kyu): Maximum Multiple
    Url: https://www.codewars.com/kata/maximum-multiple/train/rust

    Task
    Given a Divisor and a Bound, Find the largest integer N, Such That,

    Conditions :
    N is divisible by divisor

    N is less than or equal to bound

    N is greater than 0.

    Notes
    The parameters (divisor, bound) passed to the function are only
    positive values.
    It's guaranteed that a divisor is Found.
    Input >> Output Examples
    1- maxMultiple (2,7) ==> return (6)
    Explanation:
    (6) is divisible by (2), (6) is less than or equal to bound (7), and (6)
        is > 0.

    2- maxMultiple (10,50)  ==> return (50)
    Explanation:
    (50) is divisible by (10), (50) is less than or equal to bound (50),
        and (50)is > 0.*

    3- maxMultiple (37,200) ==> return (185)
    Explanation:
    (185) is divisible by (37), (185) is less than or equal to bound (200),
        and (185) is > 0.
*/

fn max_multiple(divisor: u32, bound: u32) -> u32 {
    (bound / divisor) * divisor
}

#[test]
fn basic_test() {
    assert_eq!(max_multiple(2, 7), 6);
    assert_eq!(max_multiple(3, 10), 9);
    assert_eq!(max_multiple(7, 17), 14);
    assert_eq!(max_multiple(10, 50), 50);
}
