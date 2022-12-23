use std::{collections::HashMap, fs};

pub mod p1;
pub mod p2;

fn parse_input() -> Scan {
    let input =
        fs::read_to_string("src/bin/23/input.txt").expect("Should have been able to read the file");

    let mut scan = Scan::new();

    for (y, row) in input.lines().rev().enumerate() {
        for (x, col) in row.chars().enumerate() {
            let pos = Position {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };

            if col == '#' {
                scan.grove.insert(pos, Elf);
            };
        }
    }

    scan
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

struct Scan {
    grove: HashMap<Position, Elf>,
    direction_ord: [Direction; 4],
    direction_i: usize,
}

impl Scan {
    fn new() -> Self {
        Self {
            grove: HashMap::new(),
            direction_ord: [Direction::N, Direction::S, Direction::W, Direction::E],
            direction_i: 0,
        }
    }

    fn increment_direction(&mut self) {
        self.direction_i = (self.direction_i + 1) % self.direction_ord.len();
    }

    fn check_adjacent(&self, p: &Position) -> HashMap<Direction, Option<&Elf>> {
        HashMap::from([
            (Direction::N, self.grove.get(&Position::new(p.x, p.y + 1))),
            (
                Direction::NE,
                self.grove.get(&Position::new(p.x + 1, p.y + 1)),
            ),
            (
                Direction::NW,
                self.grove.get(&Position::new(p.x - 1, p.y + 1)),
            ),
            (Direction::E, self.grove.get(&Position::new(p.x + 1, p.y))),
            (Direction::W, self.grove.get(&Position::new(p.x - 1, p.y))),
            (Direction::S, self.grove.get(&Position::new(p.x, p.y - 1))),
            (
                Direction::SE,
                self.grove.get(&Position::new(p.x + 1, p.y - 1)),
            ),
            (
                Direction::SW,
                self.grove.get(&Position::new(p.x - 1, p.y - 1)),
            ),
        ])
    }

    fn calc_proposed_move(&self, p: &Position) -> Option<Position> {
        let adj = self.check_adjacent(p);

        if adj.values().all(|p| p.is_none()) {
            return None;
        }

        for i in self.direction_i..self.direction_i + 4 {
            let dir = &self.direction_ord[i % self.direction_ord.len()];

            match dir {
                Direction::N => {
                    if adj.get(&Direction::N).unwrap().is_none()
                        && adj.get(&Direction::NE).unwrap().is_none()
                        && adj.get(&Direction::NW).unwrap().is_none()
                    {
                        return Some(Position::new(p.x, p.y + 1));
                    }
                }

                Direction::S => {
                    if adj.get(&Direction::S).unwrap().is_none()
                        && adj.get(&Direction::SE).unwrap().is_none()
                        && adj.get(&Direction::SW).unwrap().is_none()
                    {
                        return Some(Position::new(p.x, p.y - 1));
                    }
                }

                Direction::W => {
                    if adj.get(&Direction::W).unwrap().is_none()
                        && adj.get(&Direction::SW).unwrap().is_none()
                        && adj.get(&Direction::NW).unwrap().is_none()
                    {
                        return Some(Position::new(p.x - 1, p.y));
                    }
                }

                Direction::E => {
                    if adj.get(&Direction::E).unwrap().is_none()
                        && adj.get(&Direction::SE).unwrap().is_none()
                        && adj.get(&Direction::NE).unwrap().is_none()
                    {
                        return Some(Position::new(p.x + 1, p.y));
                    }
                }

                _ => return None,
            };
        }

        None
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Elf;

#[derive(Hash, Eq, PartialEq)]
enum Direction {
    N,
    NE,
    NW,
    S,
    SE,
    SW,
    E,
    W,
}
