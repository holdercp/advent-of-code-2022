use std::collections::HashMap;

use regex::Regex;

use crate::{Direction, Instruction, Point, SidePoint, SideTile, TileValue};

const SIDE_SIZE: usize = 50;

fn parse_input() -> ((SidePoint, HashMap<SidePoint, SideTile>), Vec<Instruction>) {
    let input = super::read_input();

    let input: Vec<&str> = input.split("\n\n").collect();

    let map_raw = input[0].replace(' ', "");

    let mut sides = [
        [[' '; SIDE_SIZE]; SIDE_SIZE],
        [[' '; SIDE_SIZE]; SIDE_SIZE],
        [[' '; SIDE_SIZE]; SIDE_SIZE],
        [[' '; SIDE_SIZE]; SIDE_SIZE],
        [[' '; SIDE_SIZE]; SIDE_SIZE],
        [[' '; SIDE_SIZE]; SIDE_SIZE],
    ];

    // Hardcode my input layout
    /*
           [3][1]
           [0]
        [4][2]
        [5]
    */

    for (y, row) in map_raw.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if y < SIDE_SIZE {
                if x < SIDE_SIZE {
                    sides[3][y][x] = c;
                } else {
                    sides[1][y][x % SIDE_SIZE] = c;
                }
            }

            if (SIDE_SIZE..SIDE_SIZE * 2).contains(&y) {
                sides[0][y % SIDE_SIZE][x % SIDE_SIZE] = c;
            }

            if (SIDE_SIZE * 2..SIDE_SIZE * 3).contains(&y) {
                if x < SIDE_SIZE {
                    sides[4][y % SIDE_SIZE][x] = c;
                } else {
                    sides[2][y % SIDE_SIZE][x % SIDE_SIZE] = c;
                }
            }

            if y >= SIDE_SIZE * 3 {
                sides[5][y % SIDE_SIZE][x] = c;
            }
        }
    }

    let map = build_map(sides);

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

fn build_map(sides: [[[char; 50]; 50]; 6]) -> (SidePoint, HashMap<SidePoint, SideTile>) {
    let mut map = HashMap::new();

    for (i, s) in sides.iter().enumerate() {
        for (y, row) in s.iter().enumerate() {
            for (x, char) in row.iter().enumerate() {
                let up = if y == 0 {
                    match i {
                        0 => SidePoint(3, Point::new(x, SIDE_SIZE - 1)),
                        1 => SidePoint(5, Point::new(x, SIDE_SIZE - 1)),
                        2 => SidePoint(0, Point::new(x, SIDE_SIZE - 1)),
                        3 => SidePoint(5, Point::new(0, x)),
                        4 => SidePoint(0, Point::new(0, x)),
                        5 => SidePoint(4, Point::new(x, SIDE_SIZE - 1)),
                        _ => panic!("bad side"),
                    }
                } else {
                    SidePoint(i, Point::new(x, y - 1))
                };

                let down = if y == SIDE_SIZE - 1 {
                    match i {
                        0 => SidePoint(2, Point::new(x, 0)),
                        1 => SidePoint(0, Point::new(SIDE_SIZE - 1, x)),
                        2 => SidePoint(5, Point::new(SIDE_SIZE - 1, x)),
                        3 => SidePoint(0, Point::new(x, 0)),
                        4 => SidePoint(5, Point::new(x, 0)),
                        5 => SidePoint(1, Point::new(x, 0)),
                        _ => panic!("bad side"),
                    }
                } else {
                    SidePoint(i, Point::new(x, y + 1))
                };

                let left = if x == 0 {
                    match i {
                        0 => SidePoint(4, Point::new(y, 0)),
                        1 => SidePoint(3, Point::new(SIDE_SIZE - 1, y)),
                        2 => SidePoint(4, Point::new(SIDE_SIZE - 1, y)),
                        3 => SidePoint(4, Point::new(0, y.abs_diff(SIDE_SIZE - 1))),
                        4 => SidePoint(3, Point::new(0, y.abs_diff(SIDE_SIZE - 1))),
                        5 => SidePoint(3, Point::new(y, 0)),
                        _ => panic!("bad side"),
                    }
                } else {
                    SidePoint(i, Point::new(x - 1, y))
                };

                let right = if x == SIDE_SIZE - 1 {
                    match i {
                        0 => SidePoint(1, Point::new(y, SIDE_SIZE - 1)),
                        1 => SidePoint(2, Point::new(SIDE_SIZE - 1, y.abs_diff(SIDE_SIZE - 1))),
                        2 => SidePoint(1, Point::new(SIDE_SIZE - 1, y.abs_diff(SIDE_SIZE - 1))),
                        3 => SidePoint(1, Point::new(0, y)),
                        4 => SidePoint(2, Point::new(0, y)),
                        5 => SidePoint(2, Point::new(y, SIDE_SIZE - 1)),
                        _ => panic!("bad side"),
                    }
                } else {
                    SidePoint(i, Point::new(x + 1, y))
                };

                let neighbors = HashMap::from([
                    (Direction::Up, up),
                    (Direction::Down, down),
                    (Direction::Left, left),
                    (Direction::Right, right),
                ]);

                map.insert(
                    SidePoint(i, Point::new(x, y)),
                    SideTile::new(
                        SidePoint(i, Point::new(x, y)),
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
    }

    (SidePoint(3, Point::new(0, 0)), map)
}

fn update_wrapped_dir(tile: &SideTile, dir: &Direction) -> Direction {
    match dir {
        Direction::Up => match tile.location.0 {
            0 => Direction::Up,
            1 => Direction::Up,
            2 => Direction::Up,
            3 => Direction::Right,
            4 => Direction::Right,
            5 => Direction::Up,
            _ => panic!("bad dir change"),
        },
        Direction::Down => match tile.location.0 {
            0 => Direction::Down,
            1 => Direction::Left,
            2 => Direction::Left,
            3 => Direction::Down,
            4 => Direction::Down,
            5 => Direction::Down,
            _ => panic!("bad dir change"),
        },
        Direction::Left => match tile.location.0 {
            0 => Direction::Down,
            1 => Direction::Left,
            2 => Direction::Left,
            3 => Direction::Right,
            4 => Direction::Right,
            5 => Direction::Down,
            _ => panic!("bad dir change"),
        },
        Direction::Right => match tile.location.0 {
            0 => Direction::Up,
            1 => Direction::Left,
            2 => Direction::Left,
            3 => Direction::Right,
            4 => Direction::Right,
            5 => Direction::Up,
            _ => panic!("bad dir change"),
        },
    }
}

fn get_row_val(tile: &SideTile) -> usize {
    match tile.location.0 {
        0 => tile.location.1.y + SIDE_SIZE,
        1 => tile.location.1.y,
        2 => tile.location.1.y + (SIDE_SIZE * 2),
        3 => tile.location.1.y,
        4 => tile.location.1.y + (SIDE_SIZE * 2),
        5 => tile.location.1.y + (SIDE_SIZE * 3),
        _ => panic!("bad row calc"),
    }
}

fn get_col_val(tile: &SideTile) -> usize {
    match tile.location.0 {
        0 => tile.location.1.x + SIDE_SIZE,
        1 => tile.location.1.x + (SIDE_SIZE * 2),
        2 => tile.location.1.x + SIDE_SIZE,
        3 => tile.location.1.x + SIDE_SIZE,
        4 => tile.location.1.x,
        5 => tile.location.1.x,
        _ => panic!("bad col calc"),
    }
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

                    if next_tile.location.0 != tile.location.0 {
                        dir = update_wrapped_dir(tile, &dir);
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

    (1_000 * (get_row_val(tile) as u32 + 1)) + (4 * (get_col_val(tile) as u32 + 1)) + dir_value
}
