use std::fs;

pub mod p1;
pub mod p2;

const PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn read_input() -> String {
    fs::read_to_string("src/bin/03/input.txt").expect("Should have been able to read the file")
}

fn convert_to_priority(item: &char) -> usize {
    let pos = PRIORITY.chars().position(|c| c == *item).unwrap();

    pos + 1
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}
