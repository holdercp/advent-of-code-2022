use std::fs;

pub mod p1;
pub mod p2;

fn parse_input() -> Vec<Tree> {
    let input =
        fs::read_to_string("src/bin/08/input.txt").expect("Should have been able to read the file");

    let mut trees = Vec::new();

    for (y, r) in input.lines().enumerate() {
        for (x, c) in r.chars().enumerate() {
            let height = c.to_digit(10).unwrap();

            trees.push(Tree { height, x, y })
        }
    }

    trees
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

#[derive(Debug)]
struct Tree {
    height: u32,
    x: usize,
    y: usize,
}
