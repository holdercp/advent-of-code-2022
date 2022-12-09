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

trait Location {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

struct Head {
    x: i32,
    y: i32,
}

impl Head {
    fn r#move(&mut self, motion: &Motion) {
        match motion {
            Motion::U(steps) => self.y += steps,
            Motion::D(steps) => self.y -= steps,
            Motion::L(steps) => self.x -= steps,
            Motion::R(steps) => self.x += steps,
        };
    }
}

impl Location for Head {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

struct Segment {
    x: i32,
    y: i32,
}

impl Location for Segment {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

impl Segment {
    fn step(&mut self, leader: &impl Location) {
        if !self.is_adjacent(leader) {
            let x_dis = self.x.abs_diff(leader.x());
            let y_dis = self.y.abs_diff(leader.y());

            let x_dir = if leader.x() - self.x < 0 { -1 } else { 1 };
            let y_dir = if leader.y() - self.y < 0 { -1 } else { 1 };

            if x_dis == 0 {
                self.y += y_dir;
            } else if y_dis == 0 {
                self.x += x_dir;
            } else {
                self.x += x_dir;
                self.y += y_dir;
            }
        }
    }

    fn is_adjacent(&self, leader: &impl Location) -> bool {
        let x_dis = self.x.abs_diff(leader.x());
        let y_dis = self.y.abs_diff(leader.y());

        x_dis <= 1 && y_dis <= 1
    }
}

struct Tail {
    x: i32,
    y: i32,
    visited: HashSet<(i32, i32)>,
}

impl Tail {
    fn step(&mut self, leader: &impl Location) {
        if !self.is_adjacent(leader) {
            let x_dis = self.x.abs_diff(leader.x());
            let y_dis = self.y.abs_diff(leader.y());

            let x_dir = if leader.x() - self.x < 0 { -1 } else { 1 };
            let y_dir = if leader.y() - self.y < 0 { -1 } else { 1 };

            if x_dis == 0 {
                self.y += y_dir;
            } else if y_dis == 0 {
                self.x += x_dir;
            } else {
                self.x += x_dir;
                self.y += y_dir;
            }

            self.update_visited();
        }
    }

    fn is_adjacent(&self, leader: &impl Location) -> bool {
        let x_dis = self.x.abs_diff(leader.x());
        let y_dis = self.y.abs_diff(leader.y());

        x_dis <= 1 && y_dis <= 1
    }

    fn update_visited(&mut self) {
        let space = (self.x, self.y);

        self.visited.insert(space);
    }
}
