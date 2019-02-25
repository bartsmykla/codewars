#![allow(unused)]

/*
    Kata (8kyu): Grasshopper - Terminal game combat function
    Url: https://www.codewars.com/kata/grasshopper-terminal-game-combat-function-1/train/rust

    Create a combat function that takes the player's current health
    and the amount of damage recieved, and returns the player's new health.
    Health can't be less than 0.
*/

fn combat(health: f32, damage: f32) -> f32 {
    (health - damage).max(0_f32)
}

#[test]
fn example_tests() {
    assert_eq!(combat(100.0, 5.0), 95.0);
    assert_eq!(combat(92.0, 8.0), 84.0);
    assert_eq!(combat(20.0, 30.0), 0.0, "Health cannot go below 0");
}