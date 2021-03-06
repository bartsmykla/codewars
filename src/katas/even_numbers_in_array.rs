#![allow(unused)]

fn even_numbers(array: &[i32], number: usize) -> Vec<i32> {
    array.iter().rfold(Vec::new(), |mut acc, &curr| {
        if curr % 2 == 0 && acc.len() < number {
            acc.insert(0, curr)
        }

        acc
    })
}

#[test]
fn sample_tests() {
    assert_eq!(
        even_numbers(&vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), 3),
        vec!(4, 6, 8),
    );
    assert_eq!(
        even_numbers(&vec!(-22, 5, 3, 11, 26, -6, -7, -8, -9, -8, 26), 2,),
        vec!(-8, 26),
    );
    assert_eq!(
        even_numbers(&vec!(6, -25, 3, 7, 5, 5, 7, -3, 23), 1),
        vec!(6),
    );
}
