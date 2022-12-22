use std::collections::HashMap;

use crate::{Monkey, Operation};

pub fn solve() -> f64 {
    let monkeys = super::parse_input();

    let root = monkeys.get("root").unwrap();

    compute_monkey(root, &monkeys)
}

fn compute_monkey(m: &Monkey, monkeys: &HashMap<String, Monkey>) -> f64 {
    if m.others.is_empty() {
        return m.number;
    }

    let other1 = monkeys.get(&m.others[0]).unwrap();
    let other2 = monkeys.get(&m.others[1]).unwrap();

    match m.operation {
        Operation::Add => compute_monkey(other1, monkeys) + compute_monkey(other2, monkeys),
        Operation::Subtract => compute_monkey(other1, monkeys) - compute_monkey(other2, monkeys),
        Operation::Multiply => compute_monkey(other1, monkeys) * compute_monkey(other2, monkeys),
        Operation::Divide => compute_monkey(other1, monkeys) / compute_monkey(other2, monkeys),
        Operation::None => panic!("this shouldn't have happened"),
    }
}
