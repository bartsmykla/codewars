#![allow(unused)]

/*
    Kata (7kyu): Reverse words
    Url: https://www.codewars.com/kata/reverse-words/train/rust

    Complete the function that accepts a string parameter,
    and reverses each word in the string.
    All spaces in the string should be retained.

    Examples
        "This is an example!" ==> "sihT si na !elpmaxe"
        "double  spaces"      ==> "elbuod  secaps"
*/

use std::iter::Rev;
use std::str::Chars;

fn reverse_words(s: &str) -> String {
    let reversed: Vec<String> = s
        .split(" ")
        .map(&str::chars)
        .map(Chars::rev)
        .map(Rev::collect)
        .collect();

    reversed.join(" ")
}

#[test]
fn sample_test() {
    assert_eq!(
        reverse_words("The quick brown fox jumps over the lazy dog."),
        "ehT kciuq nworb xof spmuj revo eht yzal .god"
    );
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"), "a b c d");
    assert_eq!(
        reverse_words("double  spaced  words"),
        "elbuod  decaps  sdrow"
    );
}
