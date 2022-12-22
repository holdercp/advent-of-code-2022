use std::{collections::HashMap, fs};

use regex::Regex;

pub mod p1;
pub mod p2;

fn parse_input() -> HashMap<String, Monkey> {
    let input =
        fs::read_to_string("src/bin/21/input.txt").expect("Should have been able to read the file");

    let re = Regex::new(r"(\w+): (\w+) ?(\+|-|/|\*)? ?(\w+)?").unwrap();

    let mut monkeys = HashMap::new();

    re.captures_iter(&input).for_each(|c| {
        let name = c.get(1).unwrap().as_str();
        let num_or_other = c.get(2).unwrap().as_str();

        let monkey = if c.get(3).is_none() {
            Monkey {
                name: name.to_string(),
                number: num_or_other.parse().unwrap(),
                operation: Operation::None,
                others: vec![],
            }
        } else {
            let operation = c.get(3).unwrap().as_str();
            let other = c.get(4).unwrap().as_str();

            let operation = match operation {
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                _ => panic!("bad op"),
            };

            Monkey {
                name: name.to_string(),
                operation,
                others: vec![num_or_other.to_string(), other.to_string()],
                number: 0.0,
            }
        };

        monkeys.insert(name.to_string(), monkey);
    });

    monkeys
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

struct Monkey {
    name: String,
    others: Vec<String>,
    operation: Operation,
    number: f64,
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    None,
}
