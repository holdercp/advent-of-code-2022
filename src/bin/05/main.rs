use std::fs;

pub mod p1;
pub mod p2;

fn build_supply(raw: &str) -> Vec<Stack> {
    let mut lines: Vec<&str> = raw.lines().collect();

    // We don't care about the stack num from the input
    lines.pop();

    lines.reverse();

    let mut supply: Vec<Stack> = vec![Stack { crates: Vec::new() }; 9];

    for line in lines {
        let cols: Vec<char> = line.chars().collect();
        let chunks = cols.chunks(4);

        for (i, chunk) in chunks.enumerate() {
            if chunk[0] == '[' {
                supply[i].crates.push(chunk[1]);
            }
        }
    }

    supply
}

fn build_procedure(raw: &str) -> Vec<Step> {
    let mut steps: Vec<Step> = Vec::new();

    for line in raw.lines() {
        let nums: Vec<usize> = line.split_whitespace().flat_map(|w| w.parse()).collect();

        steps.push(Step(nums[0], nums[1], nums[2]));
    }

    steps
}

fn parse_input() -> (Vec<Stack>, Vec<Step>) {
    let input =
        fs::read_to_string("src/bin/05/input.txt").expect("Should have been able to read the file");

    let input: Vec<&str> = input.split("\n\n").collect();

    let supply = build_supply(input[0]);
    let procedure = build_procedure(input[1]);

    (supply, procedure)
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

#[derive(Clone)]
struct Stack {
    crates: Vec<char>,
}

struct Step(usize, usize, usize);
