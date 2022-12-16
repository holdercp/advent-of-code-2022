use std::collections::{HashMap, HashSet};

use crate::Location;

const LOWER_BOUND: i32 = 0;
const UPPER_BOUND: i32 = 4_000_000;

pub fn solve() -> i32 {
    let sensors = super::parse_input();

    let mut all_adjacent: Vec<Location> = Vec::new();
    let mut adjacent_location_counts = HashMap::new();

    println!("loaded");

    for s in sensors {
        let als = s.get_adjacent_locations();
        let filtered = als.iter().filter(|al| {
            LOWER_BOUND <= al.x && LOWER_BOUND <= al.y && UPPER_BOUND > al.x && UPPER_BOUND > al.y
        });

        all_adjacent.extend(filtered);
    }

    println!("got adjacent");

    for al in all_adjacent {
        let count = adjacent_location_counts.entry(al).or_insert(0);
        *count += 1;
    }

    println!("got adjacent counts");

    let candidates: Vec<&Location> = adjacent_location_counts
        .iter()
        .filter(|(k, v)| **v >= 4)
        .map(|(k, v)| k)
        .collect();

    println!("got adjacent counts 2");

    println!("{:?}", candidates);
    0
}
