use std::collections::HashMap;

pub fn solve() -> f32 {
    let (start, target, graph) = super::parse_input();

    // println!("{:?}", graph);

    let mut visited: Vec<&(usize, usize)> = Vec::new();
    let mut unvisited: Vec<&(usize, usize)> = graph.keys().collect();
    unvisited.sort();
    unvisited.reverse();

    let mut distances: HashMap<&(usize, usize), f32> =
        graph.keys().map(|k| (k, f32::INFINITY)).collect();

    distances.insert(&start, 0.0);

    while !unvisited.is_empty() {
        println!("{}", unvisited.len());
        let copy = distances.to_owned();

        let (v, _) = copy
            .iter()
            .filter(|(k, _)| unvisited.contains(k))
            .min_by_key(|(_, v)| **v as u32)
            .unwrap();

        let v_i = unvisited.iter().position(|k| k == v).unwrap();
        unvisited.remove(v_i);

        for n in graph.get(*v).unwrap() {
            if visited.contains(&n) {
                continue;
            }

            let new_dis = distances.get(*v).unwrap() + 1.0;
            let curr_dis = *distances.get(n).unwrap();

            if new_dis < curr_dis {
                distances.insert(n, new_dis);
            }
        }

        visited.push(*v);
    }

    *distances.get(&target).unwrap()
}
