#![allow(unused)]

/*
    Kata (7kyu): All Inclusive?
    Url: https://www.codewars.com/kata/all-inclusive/train/rust

    Input:
        a string strng
        an array of strings arr

    Output of function contain_all_rots(strng, arr)
    (or containAllRots or contain-all-rots):

    a boolean true if all rotations of strng are included in arr (C returns 1)
    false otherwise (C returns 0)
    Examples:
    contain_all_rots(
      "bsjq", ["bsjq", "qbsj", "sjqb", "twZNsslC", "jqbs"]) -> true

    contain_all_rots(
      "Ajylvpy", ["Ajylvpy", "ylvpyAj", "jylvpyA", "lvpyAjy", "pyAjylv",
      "vpyAjyl", "ipywee"]) -> false)

    Note:
    Though not correct in a mathematical sense

    we will consider that there are no rotations of strng == ""
    and for any array arr: contain_all_rots("", arr) --> true
    Ref: https://en.wikipedia.org/wiki/String_(computer_science)#Rotations
*/

use std::collections::HashSet;

fn contain_all_rots(string: &str, arr: Vec<&str>) -> bool {
    if string.is_empty() {
        return true;
    }

    let hash_array: HashSet<_> = arr.iter().cloned().map(&str::to_owned).collect();

    for i in 0..string.len() {
        let rotation = string[i..].to_string() + &string[..i];

        if hash_array.get(&rotation).is_none() {
            return false;
        }
    }

    true
}

fn dotest(string: &str, arr: Vec<&str>, exp: bool) -> () {
    assert_eq!(contain_all_rots(string, arr), exp);
}

#[test]
fn basis_tests() {
    dotest("", vec![], true);
    dotest(
        "bsjq",
        vec!["bsjq", "qbsj", "sjqb", "twZNsslC", "jqbs"],
        true,
    );
    dotest(
        "XjYABhR",
        vec![
            "TzYxlgfnhf",
            "yqVAuoLjMLy",
            "BhRXjYA",
            "YABhRXj",
            "hRXjYAB",
            "jYABhRX",
            "XjYABhR",
            "ABhRXjY",
        ],
        false,
    );
}
