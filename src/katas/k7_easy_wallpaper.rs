#![allow(unused)]

/*
    Kata (7kyu): Easy wallpaper
    Url: https://www.codewars.com/kata/easy-wallpaper/train/rust

    John wants to decorate a room with wallpaper. He has been said that making
    sure he has the right amount of wallpaper is more complex than it sounds.
    He wants a fool-proof method to getting it right.

    John knows that the rectangular room has a length of l meters,
    a width of w meters, a height of h meters. The standard width of the rolls
    he wants to buy is 52 centimeters. The length of a roll is 10 meters.
    He bears in mind however, that it’s best to have an extra
    length of wallpaper handy in case of mistakes or miscalculations
    so he wants to buy a length 15% greater than the one he needs.

    Last time he did these calculations he caught a headache
    so could you help John? Your function wallpaper(l, w, h) should return
    as a plain English word in lower case the number of rolls he must buy.

    #Example: wallpaper(4, 3.5, 3) should return "ten"

    #Notes:
        all rolls (even with incomplete width) are put edge to edge
        0 <= l, w, h (floating numbers), it can happens that w x h x l is zero
        the integer r (number of rolls) will always be less or equal to 20
*/

fn wall_paper(l: f64, w: f64, h: f64) -> String {
    if l <= 0.0 || w <= 0.0 || h <= 0.0 {
        return String::from("zero");
    }

    let result = (2.0 * w * h + 2.0 * l * h) / 5.2 * 1.15;

    let result = match result.ceil() as u64 {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        _ => unreachable!(),
    };

    String::from(result)
}

fn dotest(l: f64, w: f64, h: f64, exp: &str) -> () {
    assert_eq!(wall_paper(l, w, h), exp);
}
#[test]
fn basic_tests() {
    dotest(6.3, 4.5, 3.29, "sixteen");
    dotest(6.3, 5.8, 3.13, "seventeen");
}
