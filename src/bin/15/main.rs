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

            Sensor::new(
                x1,
                y1,
                Beacon {
                    location: Location { x: x2, y: y2 },
                },
            )
        })
        .collect()
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Beacon {
    location: Location,
}

#[derive(Debug)]
struct Sensor {
    location: Location,
    beacon: Beacon,
    distance: u64,
}

impl Sensor {
    fn new(x: i64, y: i64, beacon: Beacon) -> Self {
        let location = Location { x, y };
        let distance = x.abs_diff(beacon.location.x) + y.abs_diff(beacon.location.y);

        Self {
            location,
            beacon,
            distance,
        }
    }

    fn min_y(&self) -> i64 {
        self.location.y - self.distance as i64
    }

    fn max_y(&self) -> i64 {
        self.location.y + self.distance as i64
    }

    fn min_x(&self) -> i64 {
        self.location.x - self.distance as i64
    }

    fn max_x(&self) -> i64 {
        self.location.x + self.distance as i64
    }

    fn adjacent_locations(&self) -> Vec<Location> {
        let mut adjacent_locations = vec![];

        for x in self.min_x() - 1..self.max_x() + 2 {
            let y_operand: i64 = ((self.distance + 1) - self.location.x.abs_diff(x))
                .try_into()
                .unwrap();

            adjacent_locations.push(Location {
                x,
                y: self.location.y + y_operand,
            });

            if y_operand != 0 {
                adjacent_locations.push(Location {
                    x,
                    y: self.location.y - y_operand,
                });
            };
        }

        adjacent_locations
    }
}
