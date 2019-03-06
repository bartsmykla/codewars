#![allow(unused)]

/*
    Kata (7kyu): Maximum Length Difference
    Url: https://www.codewars.com/kata/maximum-length-difference/train/rust

    You are given two arrays a1 and a2 of strings. Each string is composed
    with letters from a to z. Let x be any string in the first array and
    y be any string in the second array.

    Find max(abs(length(x) − length(y)))

    If a1 and/or a2 are empty return -1 in each language except in Haskell (F#)
    where you will return Nothing (None).

    #Example:
        a1 = ["hoqq", "bbllkw", "oox", "ejjuyyy", "plmiis", "xxxzgpsssa",
            "xxwwkktt", "znnnnfqknaz", "qqquuhii", "dvvvwz"]
        a2 = ["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"]

        mxdiflg(a1, a2) --> 13

    Bash note:
        input : 2 strings with substrings separated by ,
        output: number as a string
*/

fn mx_dif_lg(mut a1: Vec<&str>, mut a2: Vec<&str>) -> i32 {
    if a1.is_empty() || a2.is_empty() {
        return -1;
    }

    a1.sort_unstable_by_key(|s| s.len());
    a2.sort_unstable_by_key(|s| s.len());

    let (a1_min, a1_max, a2_min, a2_max) = (
        a1.first().unwrap().len() as i32,
        a1.last().unwrap().len() as i32,
        a2.first().unwrap().len() as i32,
        a2.last().unwrap().len() as i32,
    );

    (a1_min - a2_max).abs().max((a1_max - a2_min).abs())
}

fn dotest(a1: Vec<&str>, a2: Vec<&str>, exp: i32) -> () {
    assert_eq!(mx_dif_lg(a1, a2), exp);
}

#[test]
fn basic_tests() {
    let mut s1 = vec![
        "hoqq",
        "bbllkw",
        "oox",
        "ejjuyyy",
        "plmiis",
        "xxxzgpsssa",
        "xxwwkktt",
        "znnnnfqknaz",
        "qqquuhii",
        "dvvvwz",
    ];
    let mut s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
    dotest(s1, s2, 13);
    s1 = vec![
        "ejjjjmmtthh",
        "zxxuueeg",
        "aanlljrrrxx",
        "dqqqaaabbb",
        "oocccffuucccjjjkkkjyyyeehh",
    ];
    s2 = vec!["bbbaaayddqbbrrrv"];
    dotest(s1, s2, 10);
}
