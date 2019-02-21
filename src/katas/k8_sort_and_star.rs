#![allow(unused)]

/*
    Kata: https://www.codewars.com/kata/sort-and-star/rust

    You will be given an vector of string(s).
    You must sort it alphabetically (case-sensitive!!)
    and then return the first value.

    The returned value must be a string, and have "***"
    between each of its letters.

    You should not remove or add elements from/to the array.
*/

fn two_sort(arr: &[&str]) -> String {
    let mut arr_clone = arr.to_vec();
    arr_clone.sort();

    arr_clone[0]
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("***")

}

#[test]
fn sample_test_cases() {
    assert_eq!(
        two_sort(&["bitcoin", "take", "over", "the", "world", "maybe",
            "who", "knows", "perhaps"]), "b***i***t***c***o***i***n",);
    assert_eq!(
        two_sort(&["turns", "out", "random", "test", "cases", "are",
            "easier", "than", "writing", "out", "basic", "ones"]), "a***r***e");
}

/*
    I really like this solution:
    https://www.codewars.com/kata/reviews/5b430affc4255efac30008fa/groups/5b7449597705efcc95002acb

        arr.iter()
            .min()
            .unwrap()
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("***")
*/