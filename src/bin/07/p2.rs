use std::collections::HashMap;

const TOTAL_SIZE: u32 = 70_000_000;
const REQUIRED_SIZE: u32 = 30_000_000;

pub fn solve() -> u32 {
    let input = super::read_input();

    let mut history: Vec<String> = Vec::new();
    let mut sums: HashMap<String, u32> = HashMap::new();

    for line in input.lines() {
        if let Some(arg) = line.strip_prefix("$ cd ") {
            if arg == ".." {
                history.pop();
            } else {
                let mut dir = String::new();
                for d in history.clone() {
                    dir.push_str(&d);
                }
                dir.push_str(arg);
                history.push(dir);
            }
        } else if line.starts_with(|c: char| c.is_numeric()) {
            let split: Vec<&str> = line.split_whitespace().collect();
            let size: u32 = split[0].parse().unwrap();

            for dir in history.clone() {
                let sum = sums.entry(dir).or_insert(0);
                *sum += size;
            }
        };
    }

    let used_space = sums.get("/").unwrap();
    let free_space = TOTAL_SIZE - used_space;
    let needed_space = REQUIRED_SIZE - free_space;

    *sums.values().filter(|v| v >= &&needed_space).min().unwrap()
}
