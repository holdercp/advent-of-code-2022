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
    let mut windows = depths.windows(3);
    let first = windows.next().unwrap();

    let (increases, _) = windows.fold((0 as u32, first), |mut acc, window| {
        let prev_sum: u32 = acc.1.iter().sum();
        let curr_sum: u32 = window.iter().sum();

        if curr_sum > prev_sum {
            acc.0 += 1;
        }

        acc.1 = window;

        acc
    });

    println!("{}", increases);
}
