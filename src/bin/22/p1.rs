use crate::{Direction, Instruction, Point, Tile, TileValue};

use std::collections::HashMap;

use regex::Regex;

fn parse_input() -> ((Point, HashMap<Point, Tile>), Vec<Instruction>) {
    let input = super::read_input();

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

pub fn solve() -> u32 {
    let ((start, map), instructions) = parse_input();

    let mut dir = Direction::Right;
    let mut tile = map.get(&start).unwrap();

    for instruction in instructions {
        match instruction {
            Instruction::RotateLeft => {
                dir = match dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                }
            }
            Instruction::RotateRight => {
                dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }
            }
            Instruction::Move(n) => {
                for _ in 0..n {
                    let next_point = tile.neighbors.get(&dir).unwrap();
                    let next_tile = map.get(next_point).unwrap();

                    if let TileValue::Wall = next_tile.value {
                        break;
                    }

                    tile = next_tile;
                }
            }
        };
    }

    let dir_value = match dir {
        Direction::Up => 3,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 0,
    };

    (1_000 * (tile.location.y as u32 + 1)) + (4 * (tile.location.x as u32 + 1)) + dir_value
}
