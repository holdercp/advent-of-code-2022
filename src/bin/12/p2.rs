use std::collections::{HashMap, HashSet, VecDeque};

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
    start: &(usize, usize),
    target: &(usize, usize),
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> Option<u32> {
    let mut visited: HashSet<&(usize, usize)> = HashSet::new();
    let mut q: VecDeque<(u32, &(usize, usize))> = VecDeque::new();

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
