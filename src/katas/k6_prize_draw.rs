#![allow(unused)]

/*
    Kata: https://www.codewars.com/kata/prize-draw/train/rust

    To participate in a prize draw each one gives his/her firstname.

    Each letter of a firstname has a value which is its rank in
    the English alphabet. A and a have rank 1, B and b rank 2 and so on.

    The length of the firstname is added to the sum of these ranks
    hence a number n. An array of random weights is linked to the firstnames
    and each n is multiplied by its corresponding weight to get
    what they call a winning number.

    Example: names: COLIN,AMANDBA,AMANDAB,CAROL,PauL,JOSEPH weights:
    [1, 4, 4, 5, 2, 1]

    PAUL -> n = length of firstname + 16 + 1 + 21 + 12 = 4 + 50 -> 54
    The weight associated with PAUL is 2 so Paul's winning number is
    54 * 2 = 108.

    Now one can sort the firstnames in decreasing order of the winning numbers.
    When two people have the same winning number sort them alphabetically by
    their firstnames.

    #Task:

    parameters: st a string of firstnames, we an array of weights, n a rank

    return: the firstname of the participant whose rank is n
    (ranks are numbered from 1)

    #Example: names: COLIN,AMANDBA,AMANDAB,CAROL,PauL,JOSEPH weights:
    [1, 4, 4, 5, 2, 1] n: 4

    The function should return: PauL

    Note:
    If st is empty return "No participants".

    If n is greater than the number of participants then return
    "Not enough participants".

    See Examples Test Cases for more examples.
*/

use std::cmp;

fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
    if st.is_empty() {
        return "No participants";
    }

    if we.len() < n {
        return "Not enough participants";
    }

    let mut ranked_people: Vec<_> = st
        .split(',')
        .zip(&we)
        .map(|(name, weight)| {
            (
                name,
                (name
                    .bytes()
                    .map(|c| match c {
                        b'a'..=b'z' => i32::from(c - b'a' + 1),
                        b'A'..=b'Z' => i32::from(c - b'A' + 1),
                        _ => unreachable!(),
                    })
                    .sum::<i32>()
                    + name.len() as i32)
                    * weight,
            )
        })
        .collect();

    ranked_people.sort_unstable_by_key(|&(name, rank)| (cmp::Reverse(rank), name));

    ranked_people[n - 1].0
}

fn testing(st: &str, we: Vec<i32>, n: usize, exp: &str) {
    assert_eq!(rank(st, we, n), exp)
}

#[test]
fn basics_rank() {
    testing(
        "Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin",
        vec![4, 2, 1, 4, 3, 1, 2],
        4,
        "Benjamin",
    );
    testing(
        "Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden",
        vec![1, 3, 5, 5, 3, 6],
        2,
        "Matthew",
    );
    testing(
        "Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth",
        vec![3, 1, 4, 4, 3, 2],
        4,
        "Abigail",
    );
    testing("Lagon,Lily", vec![1, 5], 2, "Lagon");
    testing(
        "William,Willaim,Olivia,Olivai,Lily,Lyli",
        vec![1, 1, 1, 1, 1, 1],
        1,
        "Willaim",
    );
}

/*
    What I learned or want I want to remember?
        - If you want to support some cases via match statement, you can use
          unreachable!() macro in the case of `_`.
        - I need to read more about it, but clippy suggested usage of
          `i32::from` instead of `as i32` in places where I was converting
          u8 to i32
        - Instead of enumerating I and then using index to get a weight
          I could use .zip() method on the names with the weights vector
          as a parameter
        - Function signature was given for me by CodeWars,
          but when I would create it I should use &[i32] slice
          as a weight parameter. It would give me option to pass
          any slice-like type
        - Instead of doing something like `c_as_u8 - 64` to get
          a number of character (a -> 1, b -> 2, c -> 3 ...)
          I can do `c_as_u8 - b'a'`
*/
