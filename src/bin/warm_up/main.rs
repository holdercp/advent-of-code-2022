use std::fs;
pub mod p1;
pub mod p2;

fn parse_input() -> Vec<u32> {
    let depths = fs::read_to_string("src/bin/warm_up/input.txt")
        .expect("Should have been able to read the file");

    let depths: Vec<u32> = depths
        .split_whitespace()
        .map(|d| d.parse().expect("Could not parse string to number"))
        .collect();

    depths
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}
