use std::{collections::HashSet, fs};

pub mod p1;
pub mod p2;

fn parse_input() -> Vec<Motion> {
    let input =
        fs::read_to_string("src/bin/09/input.txt").expect("Should have been able to read the file");

    let motions = input
        .lines()
        .map(|l| {
            let split: Vec<&str> = l.split_whitespace().collect();

            let dir: char = split[0].chars().next().unwrap();
            let steps: i32 = split[1].parse().unwrap();

            match dir {
                'U' => Motion::U(steps),
                'D' => Motion::D(steps),
                'L' => Motion::L(steps),
                'R' => Motion::R(steps),
                _ => panic!("bad direction"),
            }
        })
        .collect();

    motions
}

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

enum Motion {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point(i32, i32);

trait Knot {
    fn new() -> Self;

    fn get_position(&self) -> Point;
    fn set_position(&mut self, point: Point);
}

trait Follower
where
    Self: Knot,
{
    fn is_adjacent(&self, leader: &impl Knot) -> bool {
        let Point(x, y) = self.get_position();
        let Point(lx, ly) = leader.get_position();

        let x_dis = x.abs_diff(lx);
        let y_dis = y.abs_diff(ly);

        x_dis <= 1 && y_dis <= 1
    }

    fn step(&mut self, leader: &impl Knot) {
        if self.is_adjacent(leader) {
            return;
        }

        let Point(x, y) = self.get_position();
        let Point(lx, ly) = leader.get_position();

        let x_dis = x.abs_diff(lx);
        let y_dis = y.abs_diff(ly);

        let x_dir = if lx - x < 0 { -1 } else { 1 };
        let y_dir = if ly - y < 0 { -1 } else { 1 };

        if x_dis == 0 {
            self.set_position(Point(x, y + y_dir));
        } else if y_dis == 0 {
            self.set_position(Point(x + x_dir, y));
        } else {
            self.set_position(Point(x + x_dir, y + y_dir));
        }
    }
}

struct Head {
    position: Point,
}

impl Head {
    fn r#move(&mut self, motion: &Motion) {
        let Point(x, y) = self.get_position();

        match motion {
            Motion::U(steps) => self.set_position(Point(x, y + steps)),
            Motion::D(steps) => self.set_position(Point(x, y - steps)),
            Motion::L(steps) => self.set_position(Point(x - steps, y)),
            Motion::R(steps) => self.set_position(Point(x + steps, y)),
        };
    }
}

impl Knot for Head {
    fn new() -> Self {
        Self {
            position: Point(0, 0),
        }
    }

    fn get_position(&self) -> Point {
        self.position
    }

    fn set_position(&mut self, point: Point) {
        self.position = point;
    }
}

struct Segment {
    position: Point,
}

impl Knot for Segment {
    fn new() -> Self {
        Self {
            position: Point(0, 0),
        }
    }

    fn get_position(&self) -> Point {
        self.position
    }

    fn set_position(&mut self, point: Point) {
        self.position = point;
    }
}

impl Follower for Segment {}

struct Tail {
    position: Point,
    visited: HashSet<Point>,
}

impl Tail {
    fn update_visited(&mut self) {
        self.visited.insert(self.get_position());
    }
}

impl Knot for Tail {
    fn new() -> Self {
        Self {
            position: Point(0, 0),
            visited: HashSet::new(),
        }
    }
    fn get_position(&self) -> Point {
        self.position
    }

    fn set_position(&mut self, point: Point) {
        self.position = point;
    }
}

impl Follower for Tail {
    fn step(&mut self, leader: &impl Knot) {
        if self.is_adjacent(leader) {
            return;
        }

        let Point(x, y) = self.get_position();
        let Point(lx, ly) = leader.get_position();

        let x_dis = x.abs_diff(lx);
        let y_dis = y.abs_diff(ly);

        let x_dir = if lx - x < 0 { -1 } else { 1 };
        let y_dir = if ly - y < 0 { -1 } else { 1 };

        if x_dis == 0 {
            self.set_position(Point(x, y + y_dir));
        } else if y_dis == 0 {
            self.set_position(Point(x + x_dir, y));
        } else {
            self.set_position(Point(x + x_dir, y + y_dir));
        }

        self.update_visited();
    }
}
