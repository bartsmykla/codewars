#![allow(unused)]

fn potatoes(p0: i64, w0: i64, p1: i64) -> i64 {
    (w0 as f64 * ((100.0 - p0 as f64) / (100.0 - p1 as f64))) as i64
}

fn dotest(p0: i64, w0: i64, p1: i64, exp: i64) {
    let ans = potatoes(p0, w0, p1);
    assert_eq!(ans, exp)
}

#[test]
fn tests_potatoes() {
    dotest(99, 100, 98, 50);
    dotest(82, 127, 80, 114);
}
