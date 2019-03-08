#![allow(unused)]

/*
    Kata (7kyu): Deodorant Evaporator
    Url: https://www.codewars.com/kata/deodorant-evaporator/train/rust

    This program tests the life of an evaporator containing a gas.

    We know the content of the evaporator (content in ml),
    the percentage of foam or gas lost every day (evap_per_day)
    and the threshold (threshold) in percentage beyond which the evaporator
    is no longer useful. All numbers are strictly positive.

    The program reports the nth day (as an integer) on which
    the evaporator will be out of use.

    Note: Content is in fact not necessary in the body
    of the function"evaporator", you can use it or not use it, as you wish.
    Some people might prefer to reason with content, some other
    with percentages only. It's up to you but you must keep it as a parameter
    because the tests have it as an argument.
*/

fn evaporator(mut content: f64, evap_per_day: i32, threshold: i32) -> i32 {
    let mut i = 0;
    let threshold = threshold as f64 * 0.01 * content;

    while content > threshold {
        i += 1;
        content -= content * evap_per_day as f64 * 0.01;
    }

    i
}

// Better than mine solution from:
// https://www.codewars.com/kata/reviews/5c80cf640e78f10001892841/groups/5c81e0100e78f1000149fffe
fn evaporator_v2(mut content: f64, evap_per_day: i32, threshold: i32) -> i32 {
    let evap_per_day = evap_per_day as f64 / 100.0;
    let threshold = threshold as f64 / 100.0;

    (threshold.log2() / (1.0 - evap_per_day).log2()).ceil() as i32
}

fn dotest(_content: f64, evap_per_day: i32, threshold: i32, exp: i32) -> () {
    assert_eq!(exp, evaporator(_content, evap_per_day, threshold));
}

#[test]
fn basic_tests() {
    dotest(10.0, 10, 10, 22);
    dotest(10.0, 10, 5, 29);
}
