#![allow(unused)]

/*
    Kata: https://www.codewars.com/kata/alternating-case-%3C-equals-%3E-alternating-case/rust

    altERnaTIng cAsE <=> ALTerNAtiNG CaSe
    Define String.prototype.toAlternatingCase (or a similar function/method
    such as to_alternating_case/toAlternatingCase/ToAlternatingCase
    in your selected language; see the initial solution for details)
    such that each lowercase letter becomes uppercase
    and each uppercase letter becomes lowercase.

    For example:
        "hello world".toAlternatingCase() === "HELLO WORLD"
        "HELLO WORLD".toAlternatingCase() === "hello world"
        "hello WORLD".toAlternatingCase() === "HELLO world"
        "HeLLo WoRLD".toAlternatingCase() === "hEllO wOrld"
        "12345".toAlternatingCase() === "12345"
            // Non-alphabetical characters are unaffected
        "1a2b3c4d5e".toAlternatingCase() === "1A2B3C4D5E"
        "String.prototype.toAlternatingCase".toAlternatingCase() ===
            "sTRING.PROTOTYPE.TOaLTERNATINGcASE"

    As usual, your function/method should be pure, i.e. it should not mutate
    the original string.
*/

fn invert_case(c: char) -> char {
    if c.is_ascii_lowercase() {
        c.to_ascii_uppercase()
    } else {
        c.to_ascii_lowercase()
    }
}

fn to_alternating_case(s: &str) -> String {
    s.chars().map(invert_case).collect()
}

#[test]
fn example_tests() {
    assert_eq!("HELLO WORLD", to_alternating_case("hello world"));
    assert_eq!("hello world", to_alternating_case("HELLO WORLD"));
    assert_eq!("HELLO world", to_alternating_case("hello WORLD"));
    assert_eq!("hEllO wOrld", to_alternating_case("HeLLo WoRLD"));
    assert_eq!("Hello World", to_alternating_case(&to_alternating_case("Hello World")[..]));
    assert_eq!("12345", to_alternating_case("12345"));
    assert_eq!("1A2B3C4D5E", to_alternating_case("1a2b3c4d5e"));
    assert_eq!("sTRING.tOaLTERNATINGcASE", to_alternating_case("String.ToAlternatingCase"));
}

/*
    The latest version of Rust on codewars.com is 1.15, so I can't use there
    ascii helper methods on chars. Submitted solution is:

    fn invert_case(c: char) -> char {
        use std::ascii::AsciiExt;

        if AsciiExt::is_ascii(&c) {
            if c.is_lowercase() {
                AsciiExt::to_ascii_uppercase(&c)
            } else {
                AsciiExt::to_ascii_lowercase(&c)
            }
        } else {
            c
        }
    }
*/