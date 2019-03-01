#![allow(unused)]

/*
    Kata (8kyu): String repeat
    Url: https://www.codewars.com/kata/string-repeat

    Write a function called repeatStr which repeats the given string
    exactly n times.

    repeatStr(6, "I") // "IIIIII"
    repeatStr(5, "Hello") // "HelloHelloHelloHelloHello"
*/

fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}

#[test]
fn example_tests() {
    assert_eq!(repeat_str("a", 4), "aaaa");
    assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
    assert_eq!(repeat_str("abc", 2), "abcabc");
}
