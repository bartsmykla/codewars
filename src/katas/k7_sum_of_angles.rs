#![allow(unused)]

/*
    Kata (7kyu): Sum of angles
    Url: https://www.codewars.com/kata/sum-of-angles/train/rust

    Find the total sum of angles in an n sided shape. N will be greater than 2.
*/

fn angle(n: u32) -> u32 {
    (n - 2) * 180
}

#[test]
fn normal_tests() {
    assert_eq!(angle(3), 180);
    assert_eq!(angle(4), 360);
    assert_eq!(angle(5), 540);
}
