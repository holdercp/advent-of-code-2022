use std::fs;

pub mod p1;
pub mod p2;

fn main() {
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {:?}", p2::solve());
}

fn parse_input() -> Vec<Wind> {
    let input =
        fs::read_to_string("src/bin/17/input.txt").expect("Should have been able to read the file");

    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Wind::Right,
            '<' => Wind::Left,
            _ => panic!("bad input"),
        })
        .collect()
}

fn get_shape_arr() -> [Shape; 5] {
    [Shape::Line, Shape::Plus, Shape::J, Shape::I, Shape::Square]
}

struct Rock {
    points: Vec<Point>,
}

impl Rock {
    fn new(s: &Shape, y: i64) -> Self {
        match s {
            Shape::Line => Self {
                points: vec![
                    Point::new(2, y + 3),
                    Point::new(3, y + 3),
                    Point::new(4, y + 3),
                    Point::new(5, y + 3),
                ],
            },
            Shape::Plus => Self {
                points: vec![
                    Point::new(3, y + 5),
                    Point::new(2, y + 4),
                    Point::new(3, y + 4),
                    Point::new(4, y + 4),
                    Point::new(3, y + 3),
                ],
            },
            Shape::J => Self {
                points: vec![
                    Point::new(4, y + 5),
                    Point::new(4, y + 4),
                    Point::new(4, y + 3),
                    Point::new(2, y + 3),
                    Point::new(3, y + 3),
                ],
            },
            Shape::I => Self {
                points: vec![
                    Point::new(2, y + 6),
                    Point::new(2, y + 5),
                    Point::new(2, y + 4),
                    Point::new(2, y + 3),
                ],
            },
            Shape::Square => Self {
                points: vec![
                    Point::new(2, y + 4),
                    Point::new(3, y + 4),
                    Point::new(2, y + 3),
                    Point::new(3, y + 3),
                ],
            },
        }
    }

    fn fall(&self) -> Vec<Point> {
        self.points
            .iter()
            .map(|p| Point { x: p.x, y: p.y - 1 })
            .collect()
    }

    fn shift_left(&self) -> Vec<Point> {
        self.points
            .iter()
            .map(|p| Point { x: p.x - 1, y: p.y })
            .collect()
    }

    fn shift_right(&self) -> Vec<Point> {
        self.points
            .iter()
            .map(|p| Point { x: p.x + 1, y: p.y })
            .collect()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

enum Shape {
    Line,
    Plus,
    J,
    I,
    Square,
}

enum Wind {
    Left,
    Right,
}
