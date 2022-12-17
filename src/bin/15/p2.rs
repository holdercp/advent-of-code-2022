use std::collections::HashMap;

const LOWER_BOUND: i64 = 0;
const UPPER_BOUND: i64 = 4_000_000;

pub fn solve() -> u64 {
    let sensors = super::parse_input();

    sensors.get(6).unwrap().adjacent_locations();

    let mut adjacent_location_counts = HashMap::new();

    for s in &sensors {
        s.adjacent_locations()
            .iter()
            .filter(|al| {
                LOWER_BOUND <= al.x
                    && LOWER_BOUND <= al.y
                    && UPPER_BOUND > al.x
                    && UPPER_BOUND > al.y
            })
            .for_each(|fl| {
                let count = adjacent_location_counts.entry(*fl).or_insert(0);
                *count += 1;
            });
    }

    let beacon = adjacent_location_counts
        .iter()
        .max_by_key(|(_, v)| *v)
        .unwrap();

    (beacon.0.x * UPPER_BOUND + beacon.0.y).try_into().unwrap()
}
