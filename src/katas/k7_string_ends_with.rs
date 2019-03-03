#![allow(unused)]

/*
    Kata (7kyu): String ends with?
    Url: https://www.codewars.com/kata/string-ends-with/train/rust

    Complete the solution so that it returns true if the first argument(string)
    passed in ends with the 2nd argument (also a string).

    Examples:
        solution("abc", "bc") //returns true
        solution("abc", "d") //returns false
*/

fn solution(word: &str, ending: &str) -> bool {
    if ending.len() > word.len() {
        return false;
    }

    &word[word.len() - ending.len()..] == ending
}

// Better Solution from:
// https://www.codewars.com/kata/reviews/599765fc0b2b3683ba002ba6/groups/599766210b2b3683ba002baa
fn solution_v2(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

#[test]
fn returns_expected() {
    assert_eq!(true, solution("abc", "c"));
    assert_eq!(false, solution("strawberry", "banana"));
}
