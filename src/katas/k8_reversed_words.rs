#![allow(unused)]

/*
    Kata (8kyu): Reversed Words
    Url: https://www.codewars.com/kata/reversed-words/train/rust

    Complete the solution so that it reverses all of the word
     within the string passed in.

    Example:

    reverse_words("The greatest victory is that which requires no battle")
    // should return "battle no requires which that is victory greatest The"
*/

fn reverse_words(str: &str) -> String {
    let reversed: Vec<_> = str.split_whitespace().rev().collect();

    reversed.join(" ")
}

#[test]
fn returns_expected() {
    assert_eq!(reverse_words("hello world!"), "world! hello");
}

