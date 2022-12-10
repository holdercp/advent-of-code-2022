use std::fs;

pub mod p1;
pub mod p2;

fn parse_input() -> Vec<i32> {
    let input =
        fs::read_to_string("src/bin/10/input.txt").expect("Should have been able to read the file");

    let mut instructions = Vec::new();

    input.lines().for_each(|l| {
        if l.starts_with("addx") {
            l.split_whitespace()
                .flat_map(|s| s.parse::<i32>())
                .for_each(|v| {
                    instructions.push(0);
                    instructions.push(v);
                })
        } else {
            instructions.push(0);
        }
    });

    instructions
}

fn main() {
    println!("Part 1: {}", p1::solve());

    let output = p2::solve();

    for r in output {
        println!("{:?}", r)
    }
}
