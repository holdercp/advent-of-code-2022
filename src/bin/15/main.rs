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
    fn get_min_y(&self) -> i64 {
        self.location.y - self.distance as i64
    }

    fn get_max_y(&self) -> i64 {
        self.location.y + self.distance as i64
    }

    fn get_min_x(&self) -> i64 {
        self.location.x - self.distance as i64
    }

    fn get_max_x(&self) -> i64 {
        self.location.x + self.distance as i64
    }

    fn get_adjacent_locations(&self) -> Vec<Location> {
        let mut locations = Vec::new();

        self.get_boundary().into_iter().for_each(|l| {
            if l.x >= self.location.x {
                locations.push(Location { x: l.x + 1, y: l.y });
            }

            if l.x <= self.location.x {
                locations.push(Location { x: l.x - 1, y: l.y });
            }

            if l.x == self.location.x && l.y == self.get_min_y() {
                locations.push(Location { x: l.x, y: l.y - 1 });
            }

            if l.x == self.location.x && l.y == self.get_max_y() {
                locations.push(Location { x: l.x, y: l.y + 1 });
            }
        });

        locations
    }

    fn get_boundary(&self) -> Vec<Location> {
        let mut boundary = vec![
            Location {
                x: self.location.x,
                y: self.get_max_y(),
            },
            Location {
                x: self.location.x,
                y: self.get_min_y(),
            },
            Location {
                x: self.get_min_x(),
                y: self.location.y,
            },
            Location {
                x: self.get_max_x(),
                y: self.location.y,
            },
        ];

        for i in 1..self.distance {
            boundary.push(Location {
                x: self.location.x + i as i64,
                y: self.get_min_y() + i as i64,
            });
            boundary.push(Location {
                x: self.location.x + i as i64,
                y: self.get_max_y() - i as i64,
            });
            boundary.push(Location {
                x: self.location.x - i as i64,
                y: self.get_min_y() + i as i64,
            });
            boundary.push(Location {
                x: self.location.x - i as i64,
                y: self.get_max_y() - i as i64,
            });
        }

        boundary
    }
}
