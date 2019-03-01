#![allow(unused)]

/*
    Kata (7kyu): Two to One
    Url: https://www.codewars.com/kata/two-to-one/train/rust

    Take 2 strings s1 and s2 including only letters from ato z.
    Return a new sorted string, the longest possible,
    containing distinct letters,

    each taken only once - coming from s1 or s2.

    Examples:
        a = "xyaabbbccccdefww"
        b = "xxxxyyyyabklmopq"
        longest(a, b) -> "abcdefklmopqwxy"

        a = "abcdefghijklmnopqrstuvwxyz"
        longest(a, a) -> "abcdefghijklmnopqrstuvwxyz"
*/

fn longest(a1: &str, a2: &str) -> String {
    let mut result = Vec::with_capacity(a1.len() + a2.len());
    result.extend_from_slice(a1.as_bytes());
    result.extend_from_slice(a2.as_bytes());

    result.sort();
    result.dedup();

    String::from_utf8(result).unwrap()
}

fn testing(s1: &str, s2: &str, exp: &str) {
    assert_eq!(&longest(s1, s2), exp)
}

#[test]
fn basic_tests() {
    testing("aretheyhere", "yestheyarehere", "aehrsty");
    testing(
        "loopingisfunbutdangerous",
        "lessdangerousthancoding",
        "abcdefghilnoprstu",
    );
}
