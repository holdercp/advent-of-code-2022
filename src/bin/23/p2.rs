use std::collections::HashMap;

use crate::Position;

pub fn solve() -> u64 {
    let mut scan = super::parse_input();

    let mut round: u64 = 0;

    loop {
        round += 1;

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

        if proposed_move_counts.values().all(|c| c > &1) {
            break;
        }

        for (to, from) in proposed_moves {
            if proposed_move_counts.get(&to).unwrap() == &1 {
                let elf = scan.grove.remove(&from).unwrap();
                scan.grove.insert(to, elf);
            }
        }

        scan.increment_direction();
    }

    round
}
