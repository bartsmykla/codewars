#![allow(unused)]

/*
    Kata: up AND down (6kyu)
    Url: https://www.codewars.com/kata/up-and-down/train/rust

    Don't be afraid, the description is rather long but - hopefully
    - it is in order that the process be well understood.

    You are given a string s made up of substring s(1), s(2), ..., s(n)
    separated by whitespaces. Example: "after be arrived two My so"

    #Task Return a string t having the following property:

    length t(O) <= length t(1) >= length t(2) <= length t(3) >= length t(4)
        .... (P)

    where the t(i) are the substring of s; you must respect the following rule:

    at each step from left to right, you can only move either
    already consecutive strings or strings that became consecutive
    after a previous move. The number of moves should be minimum.

    #Let us go with our example:

    The length of "after" is greater than the length of "be".
    Let us move them->"be after arrived two My so"

    The length of "after" is smaller than the length of "arrived".
    Let us move them -> "be arrived after two My so"

    The length of "after" is greater than the length of "two"
        ->"be arrived two after My so"

    The length of "after" is greater than the length of "My".
    Good! Finally the length of "My" and "so" are the same, nothing to do.
    At the end of the process, the substrings s(i) verify:

    length s(0) <= length s(1) >= length s(2) <= length s(3) >= length (s4)
        <= length (s5)

    Hence given a string s of substrings s(i) the function arrange
    with the previous process should return a unique string t
    having the property (P).

    It is kind of roller coaster or up and down. When you have property (P),
    to make the result more "up and down" visible t(0), t(2), ...
    will be lower cases and the others upper cases.

    arrange("after be arrived two My so") should return
    "be ARRIVED two AFTER my SO"

    Notes:
    The string "My after be arrived so two" has the property (P) but
    can't be obtained by the described process so it won't be accepted
    as a result. The property (P) doesn't give unicity by itself.
    Process: go from left to right, move only consecutive strings when needed.
    For the first fixed tests the needed number of moves to get property (P)
    is given as a comment so that you can know if your process follows the rule.
*/

fn arrange(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut words: Vec<_> = s.split_whitespace().map(&str::to_string).collect();

    for i in 0..words.len() {
        if i + 1 < words.len() {
            let (a_len, b_len) = (words[i].len(), words[i + 1].len());

            let should_swap = if i % 2 == 0 {
                a_len > b_len
            } else {
                a_len < b_len
            };

            if should_swap {
                words.swap(i, i + 1)
            }
        }

        if i % 2 == 0 {
            words[i] = words[i].to_lowercase();
        } else {
            words[i] = words[i].to_uppercase();
        }
    }

    words.join(" ")
}

use std::cell::Cell;

// Method .as_slice_of_cells is available only in nightly, experimental
fn arrange_nightly(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut words: Vec<_> = s.split_whitespace().collect();

    for (i, words) in Cell::from_mut(&mut words[..])
        .as_slice_of_cells()
        .windows(2)
        .enumerate()
    {
        let a = &words[0];
        let b = &words[1];

        let (a_len, b_len) = (a.get().len(), b.get().len());

        let should_swap = if i % 2 == 0 {
            a_len > b_len
        } else {
            a_len < b_len
        };

        if should_swap {
            a.swap(b);
        }
    }

    let mut result = String::with_capacity(s.len());

    for (i, word) in words.into_iter().enumerate() {
        if i != 0 {
            result.push(' ');
        }

        if i % 2 == 0 {
            result.extend(word.chars().flat_map(char::to_lowercase));
        } else {
            result.extend(word.chars().flat_map(char::to_uppercase));
        }
    }

    result
}

fn testing(s: &str, exp: &str) {
    assert_eq!(arrange(s), exp.to_string())
}

#[test]
fn basics_arrange() {
    testing("after be arrived two My so", "be ARRIVED two AFTER my SO");
    testing(
        "who hit retaining The That a we taken",
        "who RETAINING hit THAT a THE we TAKEN",
    );
    testing(
        "on I came up were so grandmothers",
        "i CAME on WERE up GRANDMOTHERS so",
    );
}

/*
    The arrange_nightly() function was written on the example of function
    written by user scottmcm on Rust's community discord server,
    channel: #code-review (23.02.2019)
    https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=b5ced4ed81466c945845181d68600911

    What I have learned:
        - It's the first time I saw the `.flat_map`, which not only let you
          map over the iterator where the map function/closure needs to return
          Sized values (ie. i32, i64 ...), but it also can return iterators,
          results, options etc.
        - It's also the first time, where I used `.extend` method like that,
          which is very helpful, and instead of doing `.push(c)` for every
          character, I can put iterator which produces characters inside.
          Very useful.
        - It's also the first time when I used `Cell` and
          its method: `.as_slice_of_cells` (which is not stable yet).
          I definitely need to read more about Cells.
        - Another thing I haven't seen yet are the window iterators.
          I read a little bit about them, but it would be good to play
          with them more.
*/
