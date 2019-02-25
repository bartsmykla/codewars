#![allow(unused)]

/*
    Kata (8kyu): Reversed Strings
    Url: https://www.codewars.com/kata/reversed-strings

    Complete the solution so that it reverses the string value passed into it.

    solution("world") // returns "dlrow"
*/

fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}

#[test]
fn sample_test() {
    assert_eq!(solution("world"), "dlrow");
}