#![allow(unused)]

/*
    Kata (7kyu): Shortest Word
    Url: https://www.codewars.com/kata/shortest-word/train/rust

    Simple, given a string of words, return the length of the shortest word(s).

    String will never be empty and you do not need to account for different
    data types.
*/

fn find_short(s: &str) -> usize {
    s.split_whitespace().map(str::len).min().unwrap_or(0)
}

#[test]
fn returns_expected() {
    assert_eq!(
        find_short("bitcoin take over the world maybe who knows perhaps"),
        3
    );
    assert_eq!(
        find_short("turns out random test cases are easier than writing out basic ones"),
        3
    );
    assert_eq!(
        find_short("lets talk about javascript the best language"),
        3
    );
    assert_eq!(
        find_short("i want to travel the world writing code one day"),
        1
    );
    assert_eq!(find_short("Lets all go on holiday somewhere very cold"), 2);
}
