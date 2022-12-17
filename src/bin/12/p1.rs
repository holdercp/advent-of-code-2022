use std::collections::{HashSet, VecDeque};

use crate::Point;

pub fn solve() -> u32 {
    let (start, _, target, graph) = super::parse_input();

    let mut visited: HashSet<&Point> = HashSet::new();
    let mut q: VecDeque<(u32, &Point)> = VecDeque::new();

    q.push_back((0, &start));
    visited.insert(&start);

    while !q.is_empty() {
        let (paths_taken, current) = q.pop_front().unwrap();

        if current == &target {
            return paths_taken;
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

    1
}
