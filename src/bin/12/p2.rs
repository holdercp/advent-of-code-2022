use std::collections::{HashMap, HashSet, VecDeque};

use crate::Point;

pub fn solve() -> u32 {
    let (_, starts, target, graph) = super::parse_input();

    let mut shortest = u32::MAX;

    for s in starts {
        let res = find_path_distance(&s, &target, &graph);

        if let Some(d) = res {
            if d < shortest {
                shortest = d;
            }
        }
    }

    shortest
}

fn find_path_distance(
    start: &Point,
    target: &Point,
    graph: &HashMap<Point, Vec<Point>>,
) -> Option<u32> {
    let mut visited: HashSet<&Point> = HashSet::new();
    let mut q: VecDeque<(u32, &Point)> = VecDeque::new();

    q.push_back((0, start));
    visited.insert(start);

    while !q.is_empty() {
        let (paths_taken, current) = q.pop_front().unwrap();

        if current == target {
            return Some(paths_taken);
        }

        let paths = graph.get(current).unwrap();

        for p in paths {
            if visited.contains(p) {
                continue;
            }

            q.push_back((paths_taken + 1, p));
            visited.insert(p);
        }
    }

    None
}
