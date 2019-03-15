#![allow(unused)]

/*
    Kata (7kyu): Adding words - Part I
    Url: https://www.codewars.com/kata/adding-words-part-i/train/rust

    This is the first part of this kata series. Second part is here
    and third part is here

    Add two English words together!

    Implement a class Arith (struct struct Arith { value: &'static str }
    in Rust) such that

      //javascript
      var k = new Arith("three");
      k.add("seven"); //this should return "ten"
      //c++
      Arith* k = new Arith("three");
      k->add("seven"); //this should return string "ten"
      //Rust
      let c = Arith{value: "three"};
      c.add("seven") // this should return &str "ten"

    Input - Will be between zero and ten and will always be in lower case

    Output - Word representation of the result of the addition.
    Should be in lower case
*/

use std::ops::Add;

fn str_as_u8(s: &str) -> u8 {
    match s {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "ten" => 10,
        "eleven" => 11,
        "twelve" => 12,
        "thirteen" => 13,
        "fourteen" => 14,
        "fifteen" => 15,
        "sixteen" => 16,
        "seventeen" => 17,
        "eighteen" => 18,
        "twenty" => 20,
        _ => unimplemented!(),
    }
}

fn u8_as_str(n: u8) -> &'static str {
    match n {
        0 => "zero",
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
        20 => "twenty",
        _ => unimplemented!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Arith {
    value: &'static str,
}

impl From<Arith> for u8 {
    fn from(a: Arith) -> Self {
        str_as_u8(a.value)
    }
}

impl From<u8> for Arith {
    fn from(n: u8) -> Self {
        Self {
            value: u8_as_str(n),
        }
    }
}

impl Add<&'static str> for Arith {
    type Output = &'static str;

    fn add(self, rhs: &str) -> Self::Output {
        u8_as_str(u8::from(self) + str_as_u8(rhs))
    }
}

#[test]
fn returns_expected() {
    let c = Arith { value: "three" };
    assert_eq!(c.add("seven"), "ten");
    assert_eq!(c.add("eight"), "eleven");
    assert_eq!(c.add("zero"), "three");
}
