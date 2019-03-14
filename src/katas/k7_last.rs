#![allow(unused)]

/*
    Kata (7kyu): Last
    Url: https://www.codewars.com/kata/last/train/rust

    Find the last element of the given argument(s).

    Examples
    last(&[1, 2, 3, 4])     // =>  4
    last(&['x', 'y', 'z'])  // => 'z'
    In javascript and CoffeeScript a list will be an array,
    a string or the list of arguments.
*/

fn last<T: Clone>(slice: &[T]) -> T {
    slice.last().unwrap().clone()
}

#[test]
fn should_work_for_non_empty_list_of_integers() {
    assert_eq!(last(&[1, 2, 3]), 3);
}

#[test]
fn should_work_for_non_empty_list_of_strings() {
    assert_eq!(last(&["a", "b"]), "b");
}
