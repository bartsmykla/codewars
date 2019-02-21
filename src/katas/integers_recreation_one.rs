#![allow(unused)]

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..=n).fold(Vec::new(), |mut acc, curr| {
        let sum:u64 = (1..=curr)
            .fold(Vec::new(), |mut s_acc, s_curr| {
                if curr % s_curr == 0 {
                    s_acc.push(s_curr * s_curr);
                }

                s_acc
            })
            .iter()
            .sum();


        if (sum as f64).sqrt().fract() == 0.0 {
            acc.push((curr, sum));
        }

        acc
    })
}

fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) {
    assert_eq!(list_squared(m, n), exp)
}

#[test]
fn basics_list_squared() {
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(42, 250, vec![(42, 2500), (246, 84100)]);
    testing(300, 600, vec![]);
}