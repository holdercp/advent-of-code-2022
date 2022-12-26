use std::{collections::HashMap, fs};

use regex::Regex;

pub mod p1;
pub mod p2;

fn parse_input() -> ((Point, HashMap<Point, Tile>), Vec<Instruction>) {
    let input =
        fs::read_to_string("src/bin/22/input.txt").expect("Should have been able to read the file");

    let input: Vec<&str> = input.split("\n\n").collect();

    let map_raw = input[0];

    let width = map_raw.lines().max_by_key(|l| l.len()).unwrap().len();
    let height = map_raw.lines().count();

    let mut grid = vec![vec![' '; width]; height];

    for (y, row) in map_raw.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            grid[y][x] = c;
        }
    }

    let map = build_map(grid);

    let re = Regex::new(r"L|R|\d+").unwrap();

    let instructions_raw = input[1];
    let instructions = re
        .captures_iter(instructions_raw)
        .map(|c| {
            if &c[0] == "R" {
                Instruction::RotateRight
            } else if &c[0] == "L" {
                Instruction::RotateLeft
            } else {
                Instruction::Move(c[0].parse().unwrap())
            }
        })
        .collect();

    (map, instructions)
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

fn build_map(grid: Vec<Vec<char>>) -> (Point, HashMap<Point, Tile>) {
    let mut map = HashMap::new();

    let map_height = grid.len();
    let map_width = grid[0].len();

    let mut start = None;

    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if char == &' ' {
                continue;
            }

            if y == 0 && start.is_none() {
                start = Some(Point::new(x, y));
            }

            // Up
            let up = if y == 0 || grid[y - 1][x] == ' ' {
                let wrap = grid
                    .iter()
                    .enumerate()
                    .filter(|(yy, r)| yy > &y && r[x] == ' ')
                    .min_by_key(|(yy, _r)| yy.to_owned());

                if let Some((yy, _r)) = wrap {
                    Point::new(x, yy - 1)
                } else {
                    Point::new(x, map_height - 1)
                }
            } else {
                Point::new(x, y - 1)
            };

            // Down
            let down = if y == map_height - 1 || grid[y + 1][x] == ' ' {
                let wrap = grid
                    .iter()
                    .enumerate()
                    .filter(|(yy, r)| yy < &y && r[x] == ' ')
                    .max_by_key(|(yy, _r)| yy.to_owned());

                if let Some((yy, _r)) = wrap {
                    Point::new(x, yy + 1)
                } else {
                    Point::new(x, 0)
                }
            } else {
                Point::new(x, y + 1)
            };

            // Left
            let left = if x == 0 || grid[y][x - 1] == ' ' {
                let wrap = grid
                    .iter()
                    .enumerate()
                    .filter(|(yy, _r)| yy == &y)
                    .flat_map(|(_yy, r)| {
                        r.iter().enumerate().filter(|(xx, c)| xx > &x && c == &&' ')
                    })
                    .min_by_key(|(xx, _c)| xx.to_owned());

                if let Some((xx, _r)) = wrap {
                    Point::new(xx - 1, y)
                } else {
                    Point::new(map_width - 1, y)
                }
            } else {
                Point::new(x - 1, y)
            };

            // Right
            let right = if x == map_width - 1 || grid[y][x + 1] == ' ' {
                let wrap = grid
                    .iter()
                    .enumerate()
                    .filter(|(yy, _r)| yy == &y)
                    .flat_map(|(_yy, r)| {
                        r.iter().enumerate().filter(|(xx, c)| xx < &x && c == &&' ')
                    })
                    .max_by_key(|(xx, _c)| xx.to_owned());

                if let Some((xx, _r)) = wrap {
                    Point::new(xx + 1, y)
                } else {
                    Point::new(0, y)
                }
            } else {
                Point::new(x + 1, y)
            };

            let neighbors = HashMap::from([
                (Direction::Up, up),
                (Direction::Down, down),
                (Direction::Left, left),
                (Direction::Right, right),
            ]);

            map.insert(
                Point::new(x, y),
                Tile::new(
                    Point::new(x, y),
                    if char == &'#' {
                        TileValue::Wall
                    } else {
                        TileValue::Vacant
                    },
                    neighbors,
                ),
            );
        }
    }

    (start.unwrap(), map)
}

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Tile {
    location: Point,
    value: TileValue,
    neighbors: HashMap<Direction, Point>,
}

impl Tile {
    fn new(location: Point, value: TileValue, neighbors: HashMap<Direction, Point>) -> Self {
        Self {
            location,
            value,
            neighbors,
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
enum TileValue {
    Vacant,
    Wall,
}

#[derive(Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Instruction {
    Move(u32),
    RotateLeft,
    RotateRight,
}
