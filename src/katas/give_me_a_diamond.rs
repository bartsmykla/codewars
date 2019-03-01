#![allow(unused)]

use std::iter::repeat;

fn print(n: i32) -> Option<String> {
    if n < 1 || n % 2 == 0 {
        return None;
    }

    let mut bottom = String::new();
    let mut top = String::new();
    let mut middle: String = repeat('*').take(n as usize).collect();

    middle.push('\n');

    for k in 0..n / 2 {
        let stars_amount: usize = 2 * k as usize + 1;
        let spaces_amount = (n / 2) - k;
        let stars = repeat('*').take(stars_amount);
        let spaces = repeat(' ').take(spaces_amount as usize);
        let mut result: String = spaces.chain(stars).collect();

        result.push('\n');
        bottom.push_str(result.as_str());
        top.insert_str(0, result.as_str());
    }

    Some(format!("{}{}{}", bottom, middle, top))
}

#[test]
fn basic_test() {
    assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
    assert_eq!(print(-3), None);
    assert_eq!(print(1), Some("*\n".to_string()));
}
