#![allow(unused)]

/*
    Kata (8kyu): Jenny's secret message
    Url: https://www.codewars.com/kata/jennys-secret-message/train/rust

    Jenny has written a function that returns a greeting for a user.
    However, she's in love with Johnny, and would like to greet him slightly
    different. She added a special case to her function, but she made a mistake.

    Can you help her?
*/

fn greet(input: &str) -> String {
    match input {
        "Johnny" => "Hello, my love!".to_owned(),
        other => format!("Hello, {}!", other),
    }
}

#[test]
fn greets_some_people_normally() {
    assert_eq!(greet("Jim"),   "Hello, Jim!");
    assert_eq!(greet("Jane"),  "Hello, Jane!");
    assert_eq!(greet("Simon"), "Hello, Simon!");
}

#[test]
fn greets_johnny_special() {
    assert_eq!(greet("Johnny"), "Hello, my love!");
}