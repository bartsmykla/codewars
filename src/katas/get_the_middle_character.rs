#![allow(unused)]

fn get_middle(s: &str) -> &str {
    let len = s.len();
    let left = len / 2 - (1 - len % 2);
    let right = len - left;

    &s[left..right]
}

#[test]
fn example_tests() {
    assert_eq!(get_middle("test"),"es");
    assert_eq!(get_middle("testing"),"t");
    assert_eq!(get_middle("middle"),"dd");
    assert_eq!(get_middle("A"),"A");
    assert_eq!(get_middle("of"),"of");
}
