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

struct Rock {
    points: Vec<Point>,
}

impl Rock {
    fn new(s: Shape, p: Point) -> Self {
        match s {
            Shape::Line => Self {
                points: vec![
                    Point(2, p.1 + 3),
                    Point(3, p.1 + 3),
                    Point(4, p.1 + 3),
                    Point(5, p.1 + 3),
                ],
            },
            Shape::Plus => Self {
                points: vec![
                    Point(2, p.1 + 4),
                    Point(3, p.1 + 3),
                    Point(3, p.1 + 4),
                    Point(3, p.1 + 5),
                    Point(4, p.1 + 4),
                ],
            },
        }
    }
}

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
