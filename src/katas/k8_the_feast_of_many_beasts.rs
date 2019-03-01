#![allow(unused)]

/*
    Kata (8kyu): The Feast of Many Beasts
    Url: https://www.codewars.com/kata/the-feast-of-many-beasts/train/rust

    All of the animals are having a feast! Each animal is bringing one dish.
    There is just one rule: the dish must start and end with the same letters
    as the animal's name. For example, the great blue heron is bringing
    garlic naan and the chickadee is bringing chocolate cake.

    Write a function feast that takes the animal's name and dish as arguments
    and returns true or false to indicate whether the beast is allowed to bring
    the dish to the feast.

    Assume that beast and dish are always lowercase strings, and that each has
    at least two letters. beast and dish may contain hyphens and spaces,
    but these will not appear at the beginning or end of the string.
    They will not contain numerals.
*/

fn feast(beast: &str, dish: &str) -> bool {
    let (b_len, d_len) = (beast.len(), dish.len());

    beast[0..1] == dish[0..1] && beast[(b_len - 1)..b_len] == dish[(d_len - 1)..d_len]
}

#[test]
fn sample_test_cases() {
    assert_eq!(feast("great blue heron", "garlic naan"), true);
    assert_eq!(feast("chickadee", "chocolate cake"), true);
    assert_eq!(feast("brown bear", "bear claw"), false);
}
