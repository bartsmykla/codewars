/*
    Kata (8kyu): Abbreviate a Two Word Name
    Url: https://www.codewars.com/kata/abbreviate-a-two-word-name/train/rust

    Write a function to convert a name into initials.
    This kata strictly takes two words with one space in between them.

    The output should be two capital letters with a dot separating them.

    It should look like this:
        Sam Harris => S.H

        Patrick Feeney => P.F
*/

fn abbrev_name(name: &str) -> String {
    name.split_whitespace()
        .flat_map(|word| word.chars().take(1))
        .enumerate()
        .map(|(i, c)| {
            if i != 0 {
                format!(".{}", c).to_uppercase()
            } else {
                c.to_string().to_uppercase()
            }
        })
        .collect()
}

#[test]
fn sample_tests() {
    assert_eq!(abbrev_name("Sam Harris"), "S.H");
    assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
    assert_eq!(abbrev_name("Evan Cole"), "E.C");
    assert_eq!(abbrev_name("P Favuzzi"), "P.F");
    assert_eq!(abbrev_name("David Mendieta"), "D.M");
}