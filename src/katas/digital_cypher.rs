pub fn digital_cypher() {

}

fn encode(msg: String, n: i32) -> Vec<i32> {
    let key: Vec<i32> = n.to_string().chars().map(|d| d.to_digit(10).unwrap() as i32).collect();



    msg.chars().zip(n.to_string().chars().cycle()).map(|(a, b)| {
        println!("a: {}, b: {}", a, b);
    });

//    msg.chars().enumerate().map(|(i, d)| d as i32 - 96 + key[i % key.len()]).collect::<Vec<i32>>()

    vec![]
}

#[test]
fn fixed_tests() {
    assert_eq!(encode(String::from("abcd"), 1), vec![2, 3, 4, 5]);

    assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
    assert_eq!(encode("masterpiece".to_string(), 1939), vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);
}