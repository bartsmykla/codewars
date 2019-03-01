#![allow(unused)]

/*
    Kata (7kyu): Simple beads count
    Url: https://www.codewars.com/kata/simple-beads-count/train/rust

    Two red beads are placed between every two blue beads.
    There are N blue beads. After looking at the arrangement below work out
    the number of red beads.

    @ @@ @ @@ @ @@ @ @@ @ @@ @

    Implement count_red_beads(n) (in PHP count_red_beads($n); in Java,
    Javascript, TypeScript, C, C++ countRedBeads(n)) so that it returns
    the number of red beads.

    If there are less than 2 blue beads return 0.
*/

fn count_red_beads(n: u32) -> u32 {
    n.saturating_sub(1) * 2
}

// Solution I've submitted, which is worse than above
fn count_red_beads_v2(n: u32) -> u32 {
    if n < 2 {
        return 0;
    }

    (n - 1) * 2
}

#[test]
fn test() {
    assert_eq!(count_red_beads(0), 0);
    assert_eq!(count_red_beads(1), 0);
    assert_eq!(count_red_beads(3), 4);
    assert_eq!(count_red_beads(5), 8);
}

/*
    What I learned or want I want to remember?
        - `u32::saturating_sub` is very interesting method, and I learned that
          there is much more similar at unsigned types.
          Remember and read more about it.
*/
