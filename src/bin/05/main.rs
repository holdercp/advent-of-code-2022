use std::fs;

pub mod p1;
pub mod p2;

fn build_supply(raw: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<&str> = raw.lines().collect();

    // We don't care about the stack num from the input
    lines.pop();

    lines.reverse();

    let mut supply: Vec<Vec<char>> = vec![Vec::new(); 9];

    for line in lines {
        let cols: Vec<char> = line.chars().collect();
        let chunks = cols.chunks(4);

        for (i, chunk) in chunks.enumerate() {
            if chunk[0] == '[' {
                supply[i].push(chunk[1]);
            }
        }
    }

    supply
}

fn build_procedure(raw: &str) -> Vec<(usize, usize, usize)> {
    let mut steps: Vec<(usize, usize, usize)> = Vec::new();

    for line in raw.lines() {
        let nums: Vec<usize> = line.split_whitespace().flat_map(|w| w.parse()).collect();

        steps.push((nums[0], nums[1], nums[2]));
    }

    steps
}

fn parse_input() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
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
