#![allow(unused)]

/*
    Kata (7kyu): Frequency sequence
    Url: https://www.codewars.com/kata/frequency-sequence/train/rust

    Return an output string that translates an input string s/$s by replacing
    each character in s/$s with a number representing the number of times
    that character occurs in s/$s and separating each number with
    the character(s) sep/$sep.

        freq_seq("hello world", "-"); // => "1-1-3-3-2-1-1-2-1-3-1"
        freq_seq("19999999", ":"); // => "1:7:7:7:7:7:7:7"
        freq_seq("^^^**$", "x"); // => "3x3x3x2x2x1"
*/

use std::collections::HashMap;

fn freq_seq(s: &str, sep: &str) -> String {
    let counted_chars = s.chars().fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).and_modify(|n| *n += 1).or_insert(1_usize);

        acc
    });

    let result: Vec<_> = s
        .chars()
        .map(|c| counted_chars.get(&c).unwrap().to_string())
        .collect();

    result.join(sep)
}

// Better solution from https://www.codewars.com/kata/reviews/5be079ce974804afd50022ca/groups/5be079cf974804afd50022ce
fn freq_seq_v2(s: &str, sep: &str) -> String {
    s.chars()
        .map(|c| s.matches(c).count().to_string())
        .collect::<Vec<String>>()
        .join(sep)
}

#[test]
fn returns_expected() {
    assert_eq!(freq_seq("hello world", "-"), "1-1-3-3-2-1-1-2-1-3-1");
    assert_eq!(freq_seq("19999999", ":"), "1:7:7:7:7:7:7:7");
    assert_eq!(freq_seq("^^^**$", "x"), "3x3x3x2x2x1");
}
