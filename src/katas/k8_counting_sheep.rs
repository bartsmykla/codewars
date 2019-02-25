#![allow(unused)]

/*
    Kata (8kyu): Counting sheep...
    Url: https://www.codewars.com/kata/counting-sheep-dot-dot-dot/train/rust

    Consider an array of sheep where some sheep may be missing from their place.
    We need a function that counts the number of sheep present in the array
    (true means present).

    For example:

    &[true,  true,  true,  false,
      true,  true,  true,  true ,
      true,  false, true,  false,
      true,  false, false, true ,
      true,  true,  true,  true ,
      false, false, true,  true]

    The correct answer would be 17.

    Hint: Don't forget to check for bad values like null/undefined
*/

fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().fold(0, |acc, curr| {
        match curr {
            true => acc + 1,
            false => acc,
        }
    })
}

#[test]
fn returns_correct_sheep_count() {
    assert_eq!(count_sheep(&[false]), 0);
    assert_eq!(count_sheep(&[true]), 1);
    assert_eq!(count_sheep(&[true, false]), 1);
}