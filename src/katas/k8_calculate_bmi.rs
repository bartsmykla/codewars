#![allow(unused)]

/*
    Kata (8kyu): Calculate BMI
    Url: https://www.codewars.com/kata/calculate-bmi/train/rust

    Write function bmi that calculates body mass index
    (bmi = weight / height ^ 2).

    if bmi <= 18.5 return "Underweight"

    if bmi <= 25.0 return "Normal"

    if bmi <= 30.0 return "Overweight"

    if bmi > 30 return "Obese"
*/

fn bmi(weight: u32, height: f32) -> &'static str {
    match weight as f32 / height.powi(2) {
        bmi if bmi <= 18.5 => "Underweight",
        bmi if bmi <= 25.0 => "Normal",
        bmi if bmi <= 30.0 => "Overweight",
        bmi if bmi > 30.0 => "Obese",
        _ => unreachable!(),
    }
}

#[test]
fn basic_unit_test() {
    assert_eq!(bmi(80, 1.80), "Normal");
}