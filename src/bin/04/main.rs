use std::fs;

pub mod p1;
pub mod p2;

fn read_input() -> String {
    fs::read_to_string("src/bin/04/input.txt").expect("Should have been able to read the file")
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

struct Pair {
    first: Assignment,
    second: Assignment,
}

impl Pair {
    fn build(p: &str) -> Pair {
        let assignments: Vec<&str> = p.split(",").collect();

        Pair {
            first: Assignment::build(assignments[0]),
            second: Assignment::build(assignments[1]),
        }
    }
}

struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn build(a: &str) -> Assignment {
        let sections: Vec<u32> = a.split("-").flat_map(|s| s.parse()).collect();

        Assignment {
            start: sections[0],
            end: sections[1],
        }
    }

    fn contains(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.contains(other) || (self.start <= other.start && self.end >= other.start)
    }
}
