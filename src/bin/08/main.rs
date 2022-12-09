use std::fs;

pub mod p1;
pub mod p2;

fn parse_input() -> Vec<Vec<Tree>> {
    let input =
        fs::read_to_string("src/bin/08/input.txt").expect("Should have been able to read the file");

    input
        .lines()
        .enumerate()
        .map(|(y, r)| {
            r.chars()
                .enumerate()
                .map(|(x, c)| {
                    let height = c.to_digit(10).unwrap();
                    Tree { height, x, y }
                })
                .collect::<Vec<Tree>>()
        })
        .collect()
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

#[derive(Copy, Clone, Debug)]
struct Tree {
    height: u32,
    x: usize,
    y: usize,
}
