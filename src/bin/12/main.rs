use std::{collections::HashMap, fs};

pub mod p1;
pub mod p2;

fn parse_input() -> HashMap<char, HashMap<char, u32>> {
    let input =
        fs::read_to_string("src/bin/12/input.txt").expect("Should have been able to read the file");

    let row_size = input.find("\n").unwrap();

    let line: Vec<char> = input.lines().flat_map(|l| l.trim().chars()).collect();

    let graph: HashMap<char, HashMap<char, u32>> = HashMap::new();

    for (i, c) in line.iter().enumerate() {
        let up = i - row_size;
        let down = i + row_size;
        let left = i - 1;
        let right = i + 1;

        let neighbors: HashMap<char, u32> = HashMap::new();

        if up >= 0 {}

        if down >= 0 {}

        if left >= 0 {}

        if right >= 0 {}
    }

    graph
}

fn main() {
    parse_input();
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}
