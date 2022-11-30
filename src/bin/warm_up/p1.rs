use std::fs;

fn parse_input() -> Vec<u32> {
    let depths = fs::read_to_string("src/bin/warm_up/input.txt")
        .expect("Should have been able to read the file");

    let depths: Vec<u32> = depths
        .split_whitespace()
        .map(|d| d.parse().expect("Could not parse string to number"))
        .collect();

    depths
}

pub fn solve() {
    let depths = parse_input();
    let mut count: u32 = 0;
    let mut prev = &depths[0];

    for d in &depths[1..] {
        if d > prev {
            count += 1;
        }
        prev = d;
    }

    println!("{}", count);
}
