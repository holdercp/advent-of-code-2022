use std::collections::HashSet;

use crate::Location;

const ROW: i32 = 2_000_000;

pub fn solve() -> usize {
    let sensors = super::parse_input();

    let beacon_locations: HashSet<Location> = sensors.iter().map(|s| s.beacon.location).collect();

    let mut locations: HashSet<Location> = HashSet::new();

    for s in &sensors {
        if ROW as i32 >= s.get_min_range() && ROW as i32 <= s.get_max_range() {
            let y_diff = ROW.abs_diff(s.location.y);

            let x_min = s.location.x - (s.distance - y_diff) as i32;
            let x_max = s.location.x + (s.distance - y_diff) as i32;

            for x in x_min..x_max + 1 {
                let l = Location { x, y: ROW as i32 };

                locations.insert(l);
            }
        }
    }

    locations.difference(&beacon_locations).count()
}
