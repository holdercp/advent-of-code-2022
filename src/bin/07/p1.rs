use std::collections::HashMap;

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

    sums.into_values().filter(|v| v <= &100000).sum()
}
