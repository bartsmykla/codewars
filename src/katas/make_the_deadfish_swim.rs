#![allow(unused)]

fn parse(code: &str) -> Vec<i32> {
    let mut result = vec![];
    let mut num = 0;

    for c in code.chars() {
        match c {
            'i' => num += 1,
            'd' => num -= 1,
            's' => num *= num,
            'o' => result.push(num),
            _ => {}
        }
    }

    result
}

#[test]
fn sample_tests() {
    assert_eq!(parse("iiisdoso"), vec![8, 64]);
    assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
}
