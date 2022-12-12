use std::fs;

pub mod p1;
pub mod p2;

fn read_input() -> String {
    fs::read_to_string("src/bin/12/input.txt").expect("Should have been able to read the file")
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}
