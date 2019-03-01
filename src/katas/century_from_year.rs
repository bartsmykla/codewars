#![allow(unused)]

/*
    Kata: https://www.codewars.com/kata/5a3fe3dde1ce0e8ed6000097/train/rust

    Introduction
    The first century spans from the year 1 up to and including the year 100,
    The second - from the year 101 up to and including the year 200, etc.

    Task :
    Given a year, return the century it is in.

    Input , Output Examples ::
        centuryFromYear(1705)  returns (18)
        centuryFromYear(1900)  returns (19)
        centuryFromYear(1601)  returns (17)
        centuryFromYear(2000)  returns (20)
    Hope you enjoy it .. Awaiting for Best Practice Codes

    Enjoy Learning !!!
*/

fn century(year: u32) -> u32 {
    (year as f32 / 100.0).ceil() as u32
}

#[test]
fn basic_tests() {
    assert_eq!(century(1705), 18);
    assert_eq!(century(1900), 19);
    assert_eq!(century(1601), 17);
    assert_eq!(century(2000), 20);
    assert_eq!(century(89), 1);
}
