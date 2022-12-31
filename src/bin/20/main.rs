use std::fs;

pub mod p1;
pub mod p2;

fn parse_input() -> Vec<(i32, usize)> {
    let input =
        fs::read_to_string("src/bin/20/input.txt").expect("Should have been able to read the file");

    input
        .lines()
        .enumerate()
        .map(|(i, l)| (l.parse().unwrap(), i))
        .collect()
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}
