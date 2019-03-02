#![allow(unused)]

/*
    Kata (7kyu): Build a square
    Url: https://www.codewars.com/kata/build-a-square/train/rust

    I will give you an integer. Give me back a shape that is as long
    and wide as the integer. The integer will be a whole number between
    0 and 50.

    Example
        n = 3, so I expect a 3x3 square back just like below as a string:

        +++
        +++
        +++
*/

fn generate_shape(n: usize) -> String {
    let lines: Vec<_> = (0..n).map(|_| "+".repeat(n)).collect();

    lines.join("\n")
}

// This is solution which I really like from:
// https://www.codewars.com/kata/reviews/5b7da8496243672c8a00092d/groups/5b8d18569e97810d530007c8
fn generate_shape_v2(n: usize) -> String {
    vec!["+".repeat(n); n].join("\n")
}

#[test]
fn sample_test() {
    assert_eq!(generate_shape(3), "+++\n+++\n+++");
}
