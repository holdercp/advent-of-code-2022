use std::{
    collections::{HashMap, VecDeque},
    fs,
};

pub mod p1;
pub mod p2;

fn parse_input() -> HashMap<usize, Monkey> {
    let input =
        fs::read_to_string("src/bin/11/input.txt").expect("Should have been able to read the file");

    let mut monkeys = HashMap::new();

    input.split("\n\n").enumerate().for_each(|(i, m)| {
        monkeys.insert(i, Monkey::build(m));
    });

    monkeys
}

fn main() {
    parse_input();
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

#[derive(Debug)]
enum OperationType {
    Multiply,
    Add,
}

#[derive(Debug)]
enum Operand {
    Integer(u64),
    Item,
}

#[derive(Debug)]
struct Operation(OperationType, Operand, Operand);

impl Operation {
    fn build(raw: &str) -> Operation {
        let op_raw = &raw[17..];

        let op: Vec<&str> = op_raw.split_whitespace().collect();

        let opd1 = match op[0] {
            "old" => Operand::Item,
            other => Operand::Integer(other.parse().unwrap()),
        };

        let opd2 = match op[2] {
            "old" => Operand::Item,
            other => Operand::Integer(other.parse().unwrap()),
        };

        let opt = match op[1] {
            "*" => OperationType::Multiply,
            "+" => OperationType::Add,
            _ => panic!("bad operation type"),
        };

        Operation(opt, opd1, opd2)
    }

    fn execute(&self, item: &Item) -> u64 {
        let Operation(opt, opd1, opd2) = self;

        match opt {
            OperationType::Multiply => {
                let a = match opd1 {
                    Operand::Integer(int) => int,
                    Operand::Item => &item.worry,
                };
                let b = match opd2 {
                    Operand::Integer(int) => int,
                    Operand::Item => &item.worry,
                };

                a * b
            }
            OperationType::Add => {
                let a = match opd1 {
                    Operand::Integer(int) => int,
                    Operand::Item => &item.worry,
                };
                let b = match opd2 {
                    Operand::Integer(int) => int,
                    Operand::Item => &item.worry,
                };

                a + b
            }
        }
    }
}

#[derive(Debug)]
struct Test {
    divisor: u64,
    t: usize,
    f: usize,
}

impl Test {
    fn build(raw: Vec<&str>) -> Test {
        let divisor: u64 = raw[0][19..].parse().unwrap();
        let t: usize = raw[1][25..].parse().unwrap();
        let f: usize = raw[2][26..].parse().unwrap();

        Test { divisor, t, f }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: Test,
    inspected: u64,
}

impl Monkey {
    fn build(raw: &str) -> Monkey {
        let lines: Vec<&str> = raw.lines().map(|l| l.trim()).collect();

        let items = Item::build(lines[1]);
        let operation = Operation::build(lines[2]);
        let test = Test::build(lines[3..].to_vec());

        Monkey {
            items,
            operation,
            test,
            inspected: 0,
        }
    }

    fn inspect_item(&mut self) -> Item {
        self.inspected += 1;
        self.items.pop_front().unwrap()
    }

    fn test(&self, item: &Item) -> usize {
        if item.worry % self.test.divisor == 0 {
            self.test.t
        } else {
            self.test.f
        }
    }
}

#[derive(Debug)]
struct Item {
    worry: u64,
}

impl Item {
    fn new(worry: u64) -> Self {
        Self { worry }
    }

    fn build(raw: &str) -> VecDeque<Item> {
        let items_raw = &raw[16..];
        items_raw
            .split(", ")
            .flat_map(|n| n.parse::<u64>())
            .map(Item::new)
            .collect()
    }
}
