#![allow(unused)]

fn camel_case(str: &str) -> String {
    let mut result = String::new();
    let mut word = String::new();
    let str_len = str.len();

    for (i, c) in str.chars().enumerate() {
        match c {
            ' ' if !word.is_empty() => {
                result.push_str(&word);
                word.clear()
            }
            ' ' if word.is_empty() => {}
            _ if word.is_empty() => {
                let upper_case_c: Vec<char> = c.to_uppercase().collect();

                word.push(upper_case_c[0])
            }
            _ if i == str_len - 1 && !word.is_empty() => {
                word.push(c);

                result.push_str(&word);
            }
            _ => {
                word.push(c);
            }
        }
    }

    result
}

// Rust tests
#[test]
fn sample_test() {
    assert_eq!(camel_case("test case"), "TestCase");
    assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
    assert_eq!(camel_case("say hello "), "SayHello");
    assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
    assert_eq!(camel_case(""), "");
}
