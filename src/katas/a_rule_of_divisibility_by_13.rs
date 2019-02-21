#![allow(unused)]

use std::string::ToString;

fn thirt(n: i64) -> i64 {
    let result: i64 = n
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .rev()
        .zip([1, 10, 9, 12, 3, 4].iter().cycle())
        .map(|(a, b)| i64::from(a * b))
        .sum();


    if n == result {
        return result;
    }

    thirt(result)
}

fn testequal(n: i64, exp: i64) {
    assert_eq!(exp, thirt(n))
}

#[test]
fn basics_thirt() {
    testequal(1234567, 87);
    testequal(8529, 79);
    testequal(85299258, 31);
    testequal(5634, 57);
}

