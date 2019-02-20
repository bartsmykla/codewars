/*
    Kata: https://www.codewars.com/kata/convert-a-number-to-a-string/train/rust

    We need a function that can transform a number into a string.

    What ways of achieving this do you know?

    Examples:
        number_to_string(123) //=> "123"
        number_to_string(999) //=> "999"
*/

fn number_to_string(i: i32) -> String {
    i.to_string()
}

#[test]
fn returns_number_as_a_string() {
    assert_eq!(number_to_string(67), "67");
    assert_eq!(number_to_string(1+2), "3");
}
