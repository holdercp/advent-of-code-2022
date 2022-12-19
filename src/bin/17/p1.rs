use std::collections::HashSet;

use crate::JetPattern;

pub fn solve() -> String {
    let patterns = super::parse_input();
    let shape_order = super::get_shape_arr();

    const LEFT_WALL: u32 = 0;
    const RIGHT_WALL: u32 = 6;
    const FLOOR: u32 = 0;
    const LIMIT: u32 = 2022;

    let mut rocks_fallen = 0;
    let mut tower_height = 0;

    let mut chamber = Vec::new();

    loop {
        let rock = build_rock(shape_order[rocks_fallen % 5]);

        rocks_fallen += 1;
        if rocks_fallen > LIMIT {
            break;
        }
        // Expand chamber by shape height
        // Calculate where shape will shift in 4 cycles

        // Update tower height
    }
}

enum Movement {
    Jet,
    Fall,
}

impl Movement {
    fn toggle(movement: Movement) -> Movement {
        match movement {
            Movement::Fall => Movement::Jet,
            Movement::Jet => Movement::Fall,
        }
    }
}
