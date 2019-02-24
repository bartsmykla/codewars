#![allow(unused)]

/*
    Kata (8 kyu): Parse nice int from char problem
    Url: https://www.codewars.com/kata/parse-nice-int-from-char-problem/train/rust

    Ask a small girl - "How old are you?". She always says strange things...
    Lets help her!

    For correct answer program should return int from 0 to 9.

    Assume test input string always valid and may look like "1 year old"
    or "5 years old", etc.. The first char is number only.
*/

fn get_age(age: &str) -> u32 {
    age[..1].parse().unwrap()
}

#[test]
fn basic_tests() {
    assert_eq!(get_age("2 years old"), 2);
    assert_eq!(get_age("4 years old"), 4);
    assert_eq!(get_age("5 years old"), 5);
    assert_eq!(get_age("7 years old"), 7);
}