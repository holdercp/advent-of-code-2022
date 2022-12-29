use std::{collections::HashMap, fs};

pub mod p1;
pub mod p2;

fn read_input() -> String {
    fs::read_to_string("src/bin/22/input.txt").expect("Should have been able to read the file")
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
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
