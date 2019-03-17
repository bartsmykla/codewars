#![allow(unused)]

/*
    Kata (7kyu): Remove duplicate words
    Url: https://www.codewars.com/kata/remove-duplicate-words/train/rust

    Your task is to remove all duplicate words from string, leaving only single
    (first) words entries.

    Example:

    Input:

    'alpha beta beta gamma gamma gamma delta alpha beta beta gamma gamma gamma \
    delta'

    Output:

    'alpha beta gamma delta'
*/

use std::collections::HashSet;

fn remove_duplicate_words(s: &str) -> String {
    let mut dict = HashSet::new();
    let mut result = vec![];

    for word in s.split_whitespace() {
        if dict.get(word).is_none() {
            dict.insert(word);
            result.push(word);
        }
    }

    result.join(" ")
}

#[test]
fn sample_test_cases() {
    assert_eq!(
        remove_duplicate_words(
            "alpha beta beta gamma gamma gamma delta alpha beta beta \
             gamma gamma gamma delta"
        ),
        "alpha beta gamma delta"
    );
    assert_eq!(
        remove_duplicate_words("my cat is my cat fat"),
        "my cat is fat"
    );
}
