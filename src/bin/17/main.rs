use std::{collections::HashSet, fs};

pub mod p1;
pub mod p2;

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

fn parse_input() -> Vec<JetPattern> {
    let input =
        fs::read_to_string("src/bin/17/input.txt").expect("Should have been able to read the file");

    input
        .chars()
        .map(|c| match c {
            '>' => JetPattern::Right,
            '<' => JetPattern::Left,
            _ => panic!("bad input"),
        })
        .collect()
}

enum JetPattern {
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

struct Chamber {
    border: HashSet<Point>,
    floor: HashSet<Point>,
    left_boundary: HashSet<Point>,
    right_boundary: HashSet<Point>,
}

impl Chamber {
    fn new(border: HashSet<Point>) -> Self {
        Self { border }
    }

    fn update_border(points: HashSet<Point>) {}
}

trait Rock {
    fn new(start: Point) -> Self;

    fn fall(&mut self, floor: &HashSet<Point>) -> Result<HashSet<&Point>, HashSet<Point>>;

    fn shift_left(&mut self, left_bound: &HashSet<Point>);

    fn shift_right(&mut self, right_bound: &HashSet<Point>);

    fn get_points(&self) -> HashSet<&Point>;
}

struct Line {
    left: Point,
    mid_left: Point,
    mid_right: Point,
    right: Point,
}

impl Rock for Line {
    fn new(start: Point) -> Self {
        let left = Point {
            x: start.x - 3,
            y: start.y,
        };

        let mid_left = Point {
            x: start.x - 2,
            y: start.y,
        };

        let mid_right = Point {
            x: start.x - 1,
            y: start.y,
        };

        Self {
            left,
            mid_left,
            mid_right,
            right: start,
        }
    }

    fn fall(&mut self, floor: &HashSet<Point>) -> Result<HashSet<&Point>, HashSet<Point>> {
        let has_collision = floor.iter().any(|f| {
            self.get_points()
                .iter()
                .any(|p| f.x == p.x && f.y == p.y - 1)
        });

        if has_collision {
            Err(self.get_points().into_iter().cloned().collect())
        } else {
            self.left.y -= 1;
            self.mid_left.y -= 1;
            self.mid_right.y -= 1;
            self.right.y -= 1;

            Ok(self.get_points())
        }
    }

    fn shift_left(&mut self, bounds: &HashSet<Point>) {
        let has_collision = bounds.iter().any(|b| {
            self.get_points()
                .iter()
                .any(|p| b.y == p.y && b.x == p.x - 1)
        });

        self.left.x -= 1;
        self.mid_left.x -= 1;
        self.mid_right.x -= 1;
        self.right.x -= 1;
    }

    fn shift_right(&mut self, bounds: &HashSet<Point>) {
        let has_collision = bounds.iter().any(|b| {
            self.get_points()
                .iter()
                .any(|p| b.y == p.y && b.x == p.x + 1)
        });

        self.left.x += 1;
        self.mid_left.x += 1;
        self.mid_right.x += 1;
        self.right.x += 1;
    }

    fn get_points(&self) -> HashSet<&Point> {
        HashSet::from([&self.left, &self.mid_left, &self.mid_right, &self.right])
    }
}

struct Plus {
    top_point: Point,
    left_point: Point,
    right_point: Point,
    bottom_point: Point,
}

struct J {
    top_point: Point,
    left_point: Point,
    right_point: Point,
}

struct I {
    top_point: Point,
    bottom_point: Point,
}

struct Block {
    top_point: Point,
    left_point: Point,
    right_point: Point,
    bottom_point: Point,
}
