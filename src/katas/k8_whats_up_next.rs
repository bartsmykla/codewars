#![allow(unused)]

/*
    Kata (8kyu): What's up next?
    Url: https://www.codewars.com/kata/whats-up-next/train/rust

    Given a sequence of items and a specific item in that sequence,
    return the item immediately following the item specified.
    If the item occurs more than once in a sequence,
    return the item after the first occurrence.
    This should work for a sequence of any type.

    When the item isn't present or nothing follows it,
    the function should return nil in Clojure and Elixir, Nothing in Haskell,
    undefined in JavaScript.

    next_item([1, 2, 3, 4, 5, 6, 7], 3) //=> 4
    next_item(["Joe" "Bob" "Sally"], "Bob") //=> "Sally"
*/

fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    let mut slice_iter = slice.iter().cloned();

    while let Some(item) = slice_iter.next() {
        if item == find {
            return slice_iter.next();
        }
    }

    None
}

#[test]
fn returns_expected() {
    assert_eq!(next_item(&["Joe", "Bob", "Sally"], "Bob"), Some("Sally"),);

    assert_eq!(next_item(&[0, 1], 0), Some(1),);

    assert_eq!(next_item(&[0, 1], 1), None,);

    assert_eq!(
        next_item((1..10).collect::<Vec<_>>().as_slice(), 7),
        Some(8),
    );
}
