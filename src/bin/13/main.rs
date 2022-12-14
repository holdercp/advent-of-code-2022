use std::fs;

use serde_json::Value;

pub mod p1;
pub mod p2;

fn parse_input() -> Vec<Pair> {
    let input =
        fs::read_to_string("src/bin/13/input.txt").expect("Should have been able to read the file");

    input
        .split("\n\n")
        .map(|pair| {
            let packets: Vec<&str> = pair.split_whitespace().collect();

            Pair {
                left: serde_json::from_str(packets[0]).unwrap(),
                right: serde_json::from_str(packets[1]).unwrap(),
            }
        })
        .collect()
}

fn parse_input2() -> Vec<Value> {
    let mut input =
        fs::read_to_string("src/bin/13/input.txt").expect("Should have been able to read the file");

    input.extend(["[[2]]\n", "[[6]]"]);
    input = input.replace("\n\n", "");
    input = input.replace("][", "]\n[");

    input.lines().flat_map(serde_json::from_str).collect()
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

#[derive(Debug)]
struct Pair {
    left: Value,
    right: Value,
}
