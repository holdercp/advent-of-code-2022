use std::collections::HashMap;

use crate::Location;

const LOWER_BOUND: i64 = 0;
const UPPER_BOUND: i64 = 4_000_000;

pub fn solve() -> u64 {
    let sensors = super::parse_input();

    let mut all_adjacent: Vec<Location> = Vec::new();
    let mut adjacent_location_counts = HashMap::new();

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

    let beacon = adjacent_location_counts
        .iter()
        .max_by_key(|(_, v)| *v)
        .unwrap();

    (beacon.0.x * UPPER_BOUND + beacon.0.y).try_into().unwrap()
}
