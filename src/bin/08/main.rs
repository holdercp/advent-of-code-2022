use std::fs;

pub mod p1;
pub mod p2;

fn parse_input() -> Vec<Vec<u32>> {
    let input =
        fs::read_to_string("src/bin/08/input.txt").expect("Should have been able to read the file");

    input
        .lines()
        .map(|r| r.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}
