use std::collections::HashMap;

use crate::Position;

pub fn solve() -> u32 {
    let mut scan = super::parse_input();

    for _round in 0..10 {
        let mut proposed_moves: HashMap<Position, Position> = HashMap::new();
        let mut proposed_move_counts: HashMap<Position, u32> = HashMap::new();

        for pos in scan.grove.keys() {
            let proposed_move = scan.calc_proposed_move(pos);

            if let Some(p) = proposed_move {
                proposed_moves.insert(p, *pos);

                let count = proposed_move_counts.entry(p).or_insert(0);
                *count += 1;
            }
        }

        for (to, from) in proposed_moves {
            if proposed_move_counts.get(&to).unwrap() == &1 {
                let elf = scan.grove.remove(&from).unwrap();
                scan.grove.insert(to, elf);
            }
        }

        scan.increment_direction();
    }

    let length_min = scan.grove.keys().min_by_key(|p| p.x).unwrap().x;
    let length_max = scan.grove.keys().max_by_key(|p| p.x).unwrap().x + 1;
    let height_min = scan.grove.keys().min_by_key(|p| p.y).unwrap().y;
    let height_max = scan.grove.keys().max_by_key(|p| p.y).unwrap().y + 1;

    (length_max.abs_diff(length_min) * height_max.abs_diff(height_min)) - scan.grove.len() as u32
}
