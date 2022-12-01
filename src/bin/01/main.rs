use std::fs;

pub mod p1;
pub mod p2;

fn parse_input() -> String {
    let cals =
        fs::read_to_string("src/bin/01/input.txt").expect("Should have been able to read the file");

    cals
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}
