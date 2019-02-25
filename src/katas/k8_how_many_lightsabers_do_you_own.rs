#![allow(unused)]

/*
    Kata (8kyu): How many lightsabers do you own?
    Url: https://www.codewars.com/kata/how-many-lightsabers-do-you-own/train/rust

    Inspired by the development team at Vooza,
    write the function howManyLightsabersDoYouOwn that

    accepts the name of a programmer, and
    returns the number of lightsabers owned by that person.
    The only person who owns lightsabers is Zach, by the way.
    He owns 18, which is an awesome number of lightsabers. Anyone else owns 0.

    No starting help here -- you'll need to know how to write a function
    that accepts a parameter and returns a value based on that parameter.

    how_many_lightsabers_do_you_own("Adam") // => 0
    how_many_lightsabers_do_you_own("Zach") // => 18
*/

fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
    match name {
        "Zach" => 18,
        _ => 0,
    }
}

#[test]
fn returns_zero_for_an_empty_string() {
    assert_eq!(how_many_lightsabers_do_you_own(""), 0);
}

#[test]
fn returns_0_for_other_people() {
    assert_eq!(how_many_lightsabers_do_you_own("Adam"), 0);
}

#[test]
fn returns_18_for_zach() {
    assert_eq!(how_many_lightsabers_do_you_own("Zach"), 18);
}

#[test]
fn returns_0_when_zach_is_lowercased() {
    assert_eq!(how_many_lightsabers_do_you_own("zach"), 0);
}
