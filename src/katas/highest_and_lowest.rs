use std::collections::BinaryHeap;

fn high_and_low(numbers: &str) -> String {
    let mut parsed = numbers
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap()).collect::<BinaryHeap<i32>>()
        .into_sorted_vec();

    format!("{} {}", parsed.last().unwrap(), parsed.first().unwrap())
}

#[test]
fn sample_test() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}