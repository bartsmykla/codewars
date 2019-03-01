#![allow(unused)]

/*
    Kata: https://www.codewars.com/kata/exes-and-ohs/train/rust

    Check to see if a string has the same amount of 'x's and 'o's.
    The method must return a boolean and be case insensitive.
    The string can contain any char.

    Examples input/output:

        XO("ooxx") => true
        XO("xooxx") => false
        XO("ooxXm") => true
        XO("zpzpzpp") => true // when no 'x' and 'o' is present
            should return true
        XO("zzoo") => false
*/

fn xo(string: &'static str) -> bool {
    let (xs, os) = string.bytes().fold((0_usize, 0_usize), |(xs, os), c| {
        match c.to_ascii_lowercase() {
            b'x' => (xs + 1, os),
            b'o' => (xs, os + 1),
            _ => (xs, os),
        }
    });

    xs == os
}

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}

/*
    One of the other solutions I really like:

    fn xo(string: &'static str) -> bool {
      string.chars().fold(0, |a, c|{
        match c {
          'x' | 'X' => a + 1,
          'o' | 'O' => a - 1,
          _ => a
        }
      }) == 0
    }
*/
