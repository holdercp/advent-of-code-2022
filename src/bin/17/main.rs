use std::fs;

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
        .trim()
        .chars()
        .map(|c| match c {
            '>' => JetPattern::Right,
            '<' => JetPattern::Left,
            _ => panic!("bad input"),
        })
        .collect()
}

fn get_shape_arr() -> [Shape; 5] {
    [Shape::Line, Shape::Plus, Shape::J, Shape::I, Shape::Square]
}

#[derive(Debug)]
struct Rock {
    r#type: String,
    points: Vec<Point>,
}

impl Rock {
    fn new(s: &Shape, y: &usize) -> Self {
        match s {
            Shape::Line => Self {
                r#type: String::from("line"),
                points: vec![
                    Point(4, y + 3),
                    Point(5, y + 3),
                    Point(3, y + 3),
                    Point(2, y + 3),
                ],
            },
            Shape::Plus => Self {
                r#type: String::from("plus"),
                points: vec![
                    Point(3, y + 5),
                    Point(4, y + 4),
                    Point(3, y + 3),
                    Point(2, y + 4),
                    Point(3, y + 4),
                ],
            },
            Shape::J => Self {
                r#type: String::from("j"),
                points: vec![
                    Point(4, y + 5),
                    Point(4, y + 3),
                    Point(3, y + 3),
                    Point(2, y + 3),
                    Point(4, y + 4),
                ],
            },
            Shape::I => Self {
                r#type: String::from("i"),
                points: vec![
                    Point(2, y + 6),
                    Point(2, y + 5),
                    Point(2, y + 3),
                    Point(2, y + 4),
                ],
            },
            Shape::Square => Self {
                r#type: String::from("square"),
                points: vec![
                    Point(2, y + 4),
                    Point(3, y + 4),
                    Point(3, y + 3),
                    Point(2, y + 3),
                ],
            },
        }
    }

    fn top(&self) -> usize {
        self.points[0].1
    }
    fn right(&self) -> usize {
        self.points[1].0
    }
    fn bottom(&self) -> usize {
        self.points[2].1
    }
    fn left(&self) -> usize {
        self.points[3].0
    }
}

#[derive(Debug)]
struct Point(usize, usize);

enum Shape {
    Line,
    Plus,
    J,
    I,
    Square,
}

enum JetPattern {
    Left,
    Right,
}
