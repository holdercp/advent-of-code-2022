use std::fs;

use regex::Regex;

pub mod p1;
pub mod p2;

fn parse_input() -> Vec<Sensor> {
    let input =
        fs::read_to_string("src/bin/15/input.txt").expect("Should have been able to read the file");

    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();

            let x1 = caps.get(1).unwrap().as_str().parse().unwrap();
            let y1 = caps.get(2).unwrap().as_str().parse().unwrap();

            let x2 = caps.get(3).unwrap().as_str().parse().unwrap();
            let y2 = caps.get(4).unwrap().as_str().parse().unwrap();

            Sensor {
                location: Location { x: x1, y: y1 },
                beacon: Beacon {
                    location: Location { x: x2, y: y2 },
                },
                distance: x1.abs_diff(x2) + y1.abs_diff(y2),
            }
        })
        .collect()
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Beacon {
    location: Location,
}

#[derive(Debug)]
struct Sensor {
    location: Location,
    beacon: Beacon,
    distance: u32,
}

impl Sensor {
    fn get_min_range(&self) -> i32 {
        self.location.y - self.distance as i32
    }

    fn get_max_range(&self) -> i32 {
        self.location.y + self.distance as i32
    }
}
