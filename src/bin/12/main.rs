use std::{collections::HashMap, fs};

pub mod p1;
pub mod p2;

fn parse_input() -> (Point, Vec<Point>, Point, HashMap<Point, Vec<Point>>) {
    let input =
        fs::read_to_string("src/bin/12/input.txt").expect("Should have been able to read the file");

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();

    build_graph(&grid)
}

fn build_graph(grid: &Vec<Vec<char>>) -> (Point, Vec<Point>, Point, HashMap<Point, Vec<Point>>) {
    let h_bound = grid[0].len() - 1;
    let v_bound = grid.len() - 1;

    let mut highest: Point = Point(0, 0);
    let mut start: Point = Point(0, 0);
    let mut starts: Vec<Point> = Vec::new();

    let mut graph: HashMap<Point, Vec<Point>> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let elevation = &grid[y][x];

            let mut neighbors: Vec<Point> = Vec::new();

            if y > 0 && can_traverse(elevation, &grid[y - 1][x]) {
                neighbors.push(Point(x, y - 1));
            }

            if y < v_bound && can_traverse(elevation, &grid[y + 1][x]) {
                neighbors.push(Point(x, y + 1));
            }

            if x > 0 && can_traverse(elevation, &grid[y][x - 1]) {
                neighbors.push(Point(x - 1, y));
            }

            if x < h_bound && can_traverse(elevation, &grid[y][x + 1]) {
                neighbors.push(Point(x + 1, y));
            }

            graph.insert(Point(x, y), neighbors);

            if *elevation == 'E' {
                highest = Point(x, y);
            }

            if *elevation == 'S' || *elevation == 'a' {
                starts.push(Point(x, y));

                if *elevation == 'S' {
                    start = Point(x, y);
                }
            }
        }
    }

    (start, starts, highest, graph)
}

fn can_traverse(a: &char, b: &char) -> bool {
    let elevation_map: HashMap<char, usize> = [
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('S', 1),
        ('E', 26),
    ]
    .iter()
    .cloned()
    .collect();

    elevation_map.get(b).unwrap() <= &(elevation_map.get(a).unwrap() + 1)
}

fn main() {
    parse_input();
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}

#[derive(Eq, Hash, PartialEq)]
struct Point(usize, usize);
