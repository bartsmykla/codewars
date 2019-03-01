#![allow(unused)]

/*
    Kata (7kyu): Moves in squared strings (I)
    Url: https://www.codewars.com/kata/moves-in-squared-strings-i/train/rust

    This kata is the first of a sequence of four about "Squared Strings".

    You are given a string of n lines, each substring being n characters long:
    For example:

        s = "abcd\nefgh\nijkl\nmnop"

    We will study some transformations of this square of strings.

    Vertical mirror: vert_mirror (or vertMirror or vert-mirror)
    vert_mirror(s) => "dcba\nhgfe\nlkji\nponm"

    Horizontal mirror: hor_mirror (or horMirror or hor-mirror)
    hor_mirror(s) => "mnop\nijkl\nefgh\nabcd"
    or printed:

    vertical mirror   |horizontal mirror
    abcd --> dcba     |abcd --> mnop
    efgh     hgfe     |efgh     ijkl
    ijkl     lkji     |ijkl     efgh
    mnop     ponm     |mnop     abcd

    #Task:

    Write these two functions
    and

    high-order function oper(fct, s) where

    fct is the function of one variable f to apply to the string s
    (fct will be one of vertMirror, horMirror)

    #Examples:

        s = "abcd\nefgh\nijkl\nmnop"
        oper(vert_mirror, s) => "dcba\nhgfe\nlkji\nponm"
        oper(hor_mirror, s) => "mnop\nijkl\nefgh\nabcd"

    Note:
    The form of the parameter fct in oper changes according to the language.
    You can see each form according to the language in "Sample Tests".

    Bash Note:
    The input strings are separated by , instead of \n.
    The output strings should be separated by \r instead of \n.
    See "Sample Tests".

    Forthcoming katas will study other transformations.
*/

use std::iter::Rev;
use std::str::Chars;

fn hor_mirror(s: String) -> String {
    let mirrored: Vec<_> = s.lines().rev().collect();

    mirrored.join("\n")
}

fn vert_mirror(s: String) -> String {
    let mirrored: Vec<String> = s
        .lines()
        .map(str::chars)
        .map(Chars::rev)
        .map(Rev::collect)
        .collect();

    mirrored.join("\n")
}

fn oper(handler: fn(String) -> String, s: String) -> String {
    handler(s)
}

fn testing1(s: &str, exp: &str) -> () {
    assert_eq!(oper(hor_mirror, s.to_string()), exp)
}

fn testing2(s: &str, exp: &str) -> () {
    assert_eq!(oper(vert_mirror, s.to_string()), exp)
}

#[test]
fn basics_oper() {
    testing1("lVHt\nJVhv\nCSbg\nyeCt", "yeCt\nCSbg\nJVhv\nlVHt");
    testing1("njMK\ndbrZ\nLPKo\ncEYz", "cEYz\nLPKo\ndbrZ\nnjMK");
    testing1("QMxo\ntmFe\nWLUG\nowoq", "owoq\nWLUG\ntmFe\nQMxo");

    testing2(
        "hSgdHQ\nHnDMao\nClNNxX\niRvxxH\nbqTVvA\nwvSyRu",
        "QHdgSh\noaMDnH\nXxNNlC\nHxxvRi\nAvVTqb\nuRySvw",
    );
    testing2(
        "IzOTWE\nkkbeCM\nWuzZxM\nvDddJw\njiJyHF\nPVHfSx",
        "EWTOzI\nMCebkk\nMxZzuW\nwJddDv\nFHyJij\nxSfHVP",
    );
    testing2("cuQW\nxOuD\nfZwp\neqFx", "WQuc\nDuOx\npwZf\nxFqe");
}
