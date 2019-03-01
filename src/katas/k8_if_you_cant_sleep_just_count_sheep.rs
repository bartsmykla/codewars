#![allow(unused)]

/*
    Kata (8kyu): If you can't sleep, just count sheep!!
    Url: https://www.codewars.com/kata/if-you-cant-sleep-just-count-sheep/train/rust

    Given a non-negative integer, 3 for example, return a string
    with a murmur: "1 sheep...2 sheep...3 sheep...".
    Input will always be valid, i.e. no negative integers.
*/

fn count_sheep(n: u32) -> String {
    (0..n).map(|i| format!("{} sheep...", i + 1)).collect()
}

#[test]
fn returns_expected() {
    assert_eq!(count_sheep(1), "1 sheep...");
    assert_eq!(count_sheep(2), "1 sheep...2 sheep...");
    assert_eq!(count_sheep(3), "1 sheep...2 sheep...3 sheep...");
}
