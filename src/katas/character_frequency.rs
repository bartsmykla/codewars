#![allow(unused)]

use std::collections::BTreeMap;

fn letter_frequency(input: &str) -> BTreeMap<char, i32> {
    input.to_lowercase().chars().fold(BTreeMap::new(), |mut acc, c| {
        if c < 'a' || c > 'z' {
            return acc;
        }

        if let Some(v) = acc.get_mut(&c) {
            *v += 1
        }

        if acc.get_mut(&c).is_none() {
            acc.insert(c, 1);
        }

        acc
    })
}

fn letter_frequency_v2(input: &str) -> BTreeMap<char, i32> {
    input.to_lowercase()
        .chars()
        .filter(|x| x.is_alphabetic())
        .fold(BTreeMap::new(), |mut acc, chr| {
            *acc.entry(chr).or_insert(0) += 1;

            acc
        })
}

#[test]
fn simpleword() {
    let answer: BTreeMap<char, i32> =
        [('a', 2),
            ('c', 1),
            ('l', 1),
            ('t', 1),
            ('u', 1)]
            .iter().cloned().collect();

    assert_eq!(letter_frequency("actual"), answer);
}

#[test]
fn sequence() {
    let answer: BTreeMap<char, i32> =
        [('a', 3),
            ('b', 2),
            ('f', 1),
            ('p', 1),
            ('s', 1),
            ('t', 2),
            ('u', 1),
            ('x', 5)]
            .iter().cloned().collect();

    assert_eq!(letter_frequency("AaabBF UttsP xxxxx"), answer);
}