use std::collections::{HashSet, VecDeque};

use regex::Regex;

pub fn solve() -> u32 {
    let input = super::read_input();

    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();

    let maxes = re.captures_iter(&input).fold((0, 0, 0), |maxes, c| {
        (
            maxes.0.max(c[1].parse().unwrap()),
            maxes.1.max(c[2].parse().unwrap()),
            maxes.2.max(c[3].parse().unwrap()),
        )
    });

    let mut grid: Vec<Vec<Vec<Option<Cube>>>> =
        vec![vec![vec![None; maxes.0 + 5]; maxes.1 + 5]; maxes.2 + 5];

    input.lines().for_each(|l| {
        let coors: Vec<usize> = l.split(',').map(|n| n.parse().unwrap()).collect();

        grid[coors[2] + 3][coors[1] + 3][coors[0] + 3] = Some(Cube);
    });

    compute_surface_area(Point(1, 1, 1), grid)
}

fn compute_surface_area(start: Point, graph: Vec<Vec<Vec<Option<Cube>>>>) -> u32 {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut q: VecDeque<Point> = VecDeque::new();
    let mut surface_count = 0;

    q.append(&mut start.adjacent_points());
    visited.insert(start);

    while !q.is_empty() {
        let point = q.pop_front().unwrap();

        for p in point.adjacent_points() {
            if (0 <= p.2 && p.2 < graph.len().try_into().unwrap())
                && (0 <= p.1 && p.1 < graph[0].len().try_into().unwrap())
                && (0 <= p.0 && p.0 < graph[0][0].len().try_into().unwrap())
            {
                if visited.contains(&p) {
                    continue;
                }

                if graph[p.2 as usize][p.1 as usize][p.0 as usize].is_some() {
                    surface_count += 1;
                    continue;
                }

                q.push_back(p);
                visited.insert(p);
            }
        }
    }

    surface_count
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point(i32, i32, i32);

impl Point {
    fn adjacent_points(&self) -> VecDeque<Point> {
        VecDeque::from([
            Point(self.0 + 1, self.1, self.2),
            Point(self.0 - 1, self.1, self.2),
            Point(self.0, self.1 + 1, self.2),
            Point(self.0, self.1 - 1, self.2),
            Point(self.0, self.1, self.2 + 1),
            Point(self.0, self.1, self.2 - 1),
        ])
    }
}

#[derive(Clone, Debug)]
struct Cube;
